#!/usr/bin/env bash
# Vérifie l'APK d'essai sur un émulateur Android (CI, D-19) :
#   1. il s'installe et s'ouvre ;
#   2. l'écran « Nouvelle partie » s'affiche ; « Commencer » lance une partie contre
#      l'ordinateur (plateau lu dans le moteur Rust : « Tour 1 », « Votre tour ») ;
#      toucher « Fin du tour » fait jouer l'ordinateur, puis rend la main au tour 2 ;
#   3. un plantage provoqué ouvre l'écran « … s'est arrêté ».
# Usage : outils/essai-emulateur.sh <apk> <dossier de sortie>
# Sortie : capture-accueil.png (plateau), capture-tour-rouge.png (début du tour 2), capture-plantage.png, resume-emulateur.txt.
set -uo pipefail
APK=$1; SORTIE=$2
PAQUET=fr.jeu4x.app.essai
mkdir -p "$SORTIE"
RESUME="$SORTIE/resume-emulateur.txt"
echec=0
note() { echo "$1" | tee -a "$RESUME"; }

ecran_contient() { # attend jusqu'à 30 s qu'un texte soit à l'écran
  local texte=$1
  for _ in $(seq 1 15); do
    adb shell uiautomator dump /sdcard/ecran.xml >/dev/null 2>&1
    if adb shell cat /sdcard/ecran.xml 2>/dev/null | grep -qF "$texte"; then return 0; fi
    sleep 2
  done
  return 1
}

: > "$RESUME"
note "Essai à l'émulateur : $(basename "$APK")"
note "Émulateur : Android $(adb shell getprop ro.build.version.release | tr -d '\r') (API $(adb shell getprop ro.build.version.sdk | tr -d '\r')), $(adb shell getprop ro.product.cpu.abi | tr -d '\r')"
adb shell settings put global window_animation_scale 0
adb shell settings put global transition_animation_scale 0
adb shell settings put global animator_duration_scale 0

if adb install -r "$APK" >/dev/null; then note "OK  installation"; else note "ÉCHEC installation"; exit 1; fi

adb logcat -c
adb shell am start -W -n "$PAQUET/fr.jeu4x.app.MainActivity" >/dev/null
toucher() { # touche le centre du premier nœud qui porte ce texte
  adb shell uiautomator dump /sdcard/ecran.xml >/dev/null 2>&1
  local cadre
  cadre=$(adb shell cat /sdcard/ecran.xml | tr '>' '\n' | grep -A2 "text=\"$1\"" | grep -o 'bounds="[^"]*"' | head -1)
  read -r x1 y1 x2 y2 <<<"$(echo "$cadre" | grep -oE '[0-9]+' | tr '\n' ' ')"
  [ -n "${y2:-}" ] || return 1
  adb shell input tap $(( (x1 + x2) / 2 )) $(( (y1 + y2) / 2 ))
}

if ecran_contient "Nouvelle partie" && toucher "Commencer"; then
  note "OK  écran « Nouvelle partie », « Commencer » touché"
else
  note "ÉCHEC écran « Nouvelle partie » ou bouton « Commencer » absent"
  if ecran_contient "Moteur de règles"; then note "      l'écran de contrôle du moteur s'affiche à la place : bibliothèque Rust en défaut"; fi
  echec=1
fi
if ecran_contient "Votre tour" && ecran_contient "Fin du tour"; then
  note "OK  partie lancée depuis le moteur Rust : « Votre tour », l'ordinateur a joué avant si besoin"
else
  note "ÉCHEC plateau absent (« Votre tour » introuvable)"; echec=1
fi
sleep 2
adb exec-out screencap -p > "$SORTIE/capture-accueil.png"
tour_avant=$(adb shell cat /sdcard/ecran.xml | grep -oE 'text="Tour [0-9]+' | head -1 | grep -oE '[0-9]+')
if toucher "Fin du tour"; then
  sleep 1
  if ecran_contient "Votre tour" && ecran_contient "Tour $(( ${tour_avant:-1} + 1 ))"; then
    note "OK  « Fin du tour » touché : l'ordinateur joue, puis c'est de nouveau notre tour (tour $(( ${tour_avant:-1} + 1 )))"
  else
    note "ÉCHEC « Fin du tour » touché, mais pas de retour à notre tour"; echec=1
  fi
  sleep 2  # la vue glisse vers nos unités (D-21) : laisser finir l'animation
  adb exec-out screencap -p > "$SORTIE/capture-tour-rouge.png"
else
  note "ÉCHEC bouton « Fin du tour » introuvable"; echec=1
fi
echo "--- logcat de l'appli (ouverture) ---"; adb logcat -d | grep -E "jeu4x|Jeu4X|AndroidRuntime" | tail -40

adb shell am force-stop "$PAQUET"
adb logcat -c
adb shell am start -n "$PAQUET/fr.jeu4x.app.MainActivity" --ez fr.jeu4x.app.PLANTAGE_DE_TEST true >/dev/null
if ecran_contient "est arrêté"; then
  note "OK  plantage provoqué : l'écran du rapport de plantage s'ouvre"
  if ecran_contient "Plantage de test demandé"; then note "OK  le rapport contient la cause du plantage"; else note "ÉCHEC cause absente du rapport"; echec=1; fi
else
  note "ÉCHEC écran du rapport de plantage absent"
  echec=1
fi
adb exec-out screencap -p > "$SORTIE/capture-plantage.png"
echo "--- logcat de l'appli (plantage) ---"; adb logcat -d | grep -E "jeu4x|Jeu4X|AndroidRuntime" | tail -40

# Après le rapport vu, l'appli repart normalement.
adb shell am force-stop "$PAQUET"
adb shell am start -W -n "$PAQUET/fr.jeu4x.app.MainActivity" >/dev/null
if ecran_contient "Nouvelle partie"; then
  note "OK  relance normale après le rapport"
else
  note "ÉCHEC relance normale après le rapport"
  echec=1
fi
[ $echec -eq 0 ] && note "Résultat : tout est vert" || note "Résultat : au moins un échec"
exit $echec
