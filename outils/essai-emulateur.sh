#!/usr/bin/env bash
# Vérifie l'APK d'essai sur un émulateur Android (CI, D-19) :
#   1. il s'installe et s'ouvre ;
#   2. l'écran affiche « Moteur de règles 1 : contrôle réussi » (moteur Rust chargé) ;
#   3. un plantage provoqué ouvre l'écran « … s'est arrêté ».
# Usage : outils/essai-emulateur.sh <apk> <dossier de sortie>
# Sortie : capture-accueil.png, capture-plantage.png, resume-emulateur.txt.
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
if ecran_contient "Moteur de règles 1 : contrôle réussi"; then
  note "OK  ouverture, moteur Rust chargé : « Moteur de règles 1 : contrôle réussi »"
else
  note "ÉCHEC « Moteur de règles 1 : contrôle réussi » absent de l'écran"
  echec=1
fi
adb exec-out screencap -p > "$SORTIE/capture-accueil.png"
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
if ecran_contient "Moteur de règles 1 : contrôle réussi"; then
  note "OK  relance normale après le rapport"
else
  note "ÉCHEC relance normale après le rapport"
  echec=1
fi
[ $echec -eq 0 ] && note "Résultat : tout est vert" || note "Résultat : au moins un échec"
exit $echec
