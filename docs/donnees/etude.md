# Étude : où regrouper les données de parties pour suivre l'IA

> Session 4, 2026-11-27. Étude seulement (D-11) : aucun code, aucun compte, aucune dépense.
> Tarifs relevés le 2026-11-27 sur les pages officielles (`sources.md`), hors taxes sauf mention.
> Hypothèses et volumes : `schema-donnees.md`. Idées numérotées : `catalogue.md`.

## 1. Ce qu'on regroupe, et ce qu'on ne regroupe pas

- **Les parties des joueurs** : replay compact (graine + actions), résultat, versions (app, moteur, IA), clans, durée. **≈ 2 Ko par partie** (1 à 5 Ko).
- **Les évaluations IA contre IA** : seulement leurs résumés (quelques centaines d'octets par série de 1 000 parties).
- **Les modèles d'IA** : 100 Ko à 5 Mo chacun, dans l'autre sens (serveur → téléphones).
- **Pas** les parties d'entraînement de l'IA contre elle-même : elles naissent et restent sur la machine d'entraînement (PC ou session cloud, S3). C'est de loin le plus gros volume, et il n'a rien à faire sur un serveur de collecte.

| Échelle | Volume brut par an | Envois par jour (lots quotidiens) |
|---|---|---|
| 1 joueur | < 5 Mo | 1 |
| 100 joueurs | ≈ 60 Mo | ≈ 100 |
| 10 000 joueurs | ≈ 6 Go | ≈ 10 000 |

**Conclusion du volume** : même à 10 000 joueurs, les données tiennent dans les offres gratuites.
Ce qui décide du choix, ce n'est donc pas le stockage mais **la vie privée, la triche, l'entretien
et la dépendance à un fournisseur**.

## 2. Les formules comparées

Notes de 1 (mauvais) à 5 (très bon). Coûts mensuels.

| Formule | Regroupe plusieurs joueurs ? | Coût 1 / 100 / 10 000 | Vie privée | Sécurité anti-triche | Entretien (non-codeur) | Hors ligne | Dépendance |
|---|---|---|---|---|---|---|---|
| A. Tout sur le téléphone + export | Non (à la main) | 0 / — / — | 5 | 3 | 5 | 5 | 5 |
| B. PC de l'utilisateur comme serveur | Oui | 0 / 0 / 0 (électricité) | 4 | 3 | 1 | 4 | 4 |
| C. Google Drive de chaque joueur (`google-drive-appdata`) | **Non** : chacun son Drive | 0 / 0 / 0 | 4 | 2 | 4 | 4 | 3 |
| D. GitHub (dépôt, Issues, Releases) comme réception | Oui | 0 | **1** (public) | 1 | 3 | 4 | 3 |
| E. Firebase (Firestore + App Check) | Oui | 0 / 0 / 0 à quelques $ | 3 | 4 | 4 | 4 | 2 |
| F. Supabase | Oui | 0 / 0 ou 25 $ / 25 $ | 3 | 3 | 3 | 4 | 3 |
| G. Cloudflare (Worker + R2 + D1) | Oui | 0 / 0 / 0 à 5 $ | 4 | 4 | 4 | 4 | 3 |
| H. Petit serveur loué (PocketBase ou PostgreSQL) | Oui | 4,57 à 7 € TTC à toute échelle | 4 | 4 | **1** | 4 | 5 |
| I. Mixte : téléphone qui garde et envoie vers E, F, G ou H | Oui | celui du service | 4 | 4 | 4 | **5** | selon le service |

### A. Tout sur le téléphone, avec export

Base locale, bouton d'export (DO-002, DO-003). Parfait pour un seul joueur. Ne passe pas à
l'échelle : personne n'envoie de fichier à la main chaque semaine.

### B. Le PC de l'utilisateur comme petit serveur

Gratuit, mais il faut le laisser allumé, l'exposer à Internet (tunnel, ouverture de port), le
sécuriser et le mettre à jour. C'est la formule la plus fragile pour quelqu'un qui ne code pas ;
en revanche, le PC reste **le bon endroit pour entraîner l'IA** (S3), pas pour recevoir les parties.

### C. Google Drive (brique Library `google-drive-appdata`)

Brique éprouvée (dépôt manuel), gratuite, privée, sans serveur. **Mais elle ne regroupe pas** :
chaque joueur écrit dans son propre Drive, que personne d'autre ne peut lire. Utile à l'étape 0,
avec la variante « dossier visible » (`drive.file`), pour que les parties de l'utilisateur
arrivent seules sur son Drive et soient relues par le PC. Points de vigilance de la fiche :
envoi périodique jamais vérifié en réel ; dans un dépôt public, ne déclarer que le SHA-1 de la
clé release. Non vérifié : si le dossier caché `appDataFolder` compte dans les 15 Go du compte
(très probable, sans phrase officielle).

### D. GitHub comme réception

Écarté pour les parties des joueurs : le dépôt est public (D-01), il faudrait un jeton d'écriture
dans l'APK (un secret, donc extractible), et chaque partie deviendrait publique. **GitHub reste
excellent pour tout le reste** : Releases pour distribuer les modèles (fichiers < 2 Gio, sans limite
de bande passante), Actions gratuites et illimitées pour un dépôt public (banc d'essai IA contre IA,
DO-005), Pages pour le tableau de bord (site ≤ 1 Go, 100 Go de trafic par mois).

### E. Firebase

Firestore gratuit : 1 Gio, 50 000 lectures et 20 000 écritures par jour, sur le plan Spark comme
Blaze. Écrire une partie par document tient jusqu'à ≈ 20 000 parties par jour (au-delà de
10 000 joueurs). Points forts : console claire, SDK Android, App Check avec Play Integrity.
Points faibles : depuis le 3 février 2026, Cloud Storage et Cloud Functions exigent le plan Blaze
(carte bancaire, facturation à l'usage, risque de facture en cas d'abus) ; dépendance forte à
Google (format de données propre, sortie plus difficile).

### F. Supabase

Base PostgreSQL standard (sortie facile). Gratuit : 500 Mo, mais **mise en pause après une semaine
sans activité** : avec un seul joueur qui fait une pause, le service s'endort. Pro à 25 $ par mois :
trop cher pour ce que le projet a à stocker.

### G. Cloudflare : Worker + R2 + D1

Gratuit : 100 000 requêtes par jour (Workers), 10 Go et sortie gratuite (R2), 5 Go et 100 000
lignes écrites par jour (D1). À 10 000 joueurs, on reste dans la gratuité ; sinon, Workers Paid à
5 $ par mois couvre très largement. Pas de mise en veille, pas de carte bancaire pour l'offre
gratuite. Les lots bruts sont de simples fichiers dans R2 : faciles à télécharger ailleurs, d'où une
dépendance modérée. Il faut un petit programme (le Worker, écrit par S1), mais une fois en place,
rien à entretenir.

### H. Petit serveur loué

Hetzner CX23 à 5,99 € HT (indisponible le jour du relevé), OVHcloud VPS-1 à 3,81 € HT (4,57 € TTC).
PocketBase : un seul exécutable, licence MIT, mais encore en version 0.x (compatibilité non garantie
d'une version à l'autre). Prix fixe et liberté totale, mais mises à jour de sécurité, sauvegardes,
certificats et pannes à la charge du projet : à garder en sortie de secours (DO-021).

### I. Mixte

Le téléphone garde tout (A), le jeu reste jouable hors ligne, et un lot part quand le réseau est là
(DO-008) vers un service (G de préférence). C'est la forme recommandée dès qu'il y a plus d'un joueur.

## 3. Vie privée et RGPD

- **Un identifiant d'installation aléatoire est une donnée personnelle** (pseudonyme) selon la CNIL :
  le RGPD s'applique dès qu'on collecte, même sans nom ni e-mail.
- **Minimisation** : rien d'autre que la partie (`schema-donnees.md`, « ce qu'on ne met pas ») ; le
  serveur ne garde pas l'adresse IP ; date au jour près.
- **Consentement** : question explicite à la première partie, modifiable ensuite (DO-013). Base légale
  à confirmer : le consentement est le plus simple à défendre. *L'intérêt légitime* pourrait suffire
  pour des statistiques de jeu sans identifiant, mais c'est une appréciation juridique que cette
  étude ne tranche pas.
- **Mineurs** : en France, en dessous de 15 ans, le consentement doit être donné conjointement avec
  un parent. Un jeu de stratégie attirera des moins de 15 ans : soit on demande l'âge (« moins de
  15 ans ? alors rien n'est envoyé »), soit le partage reste désactivé par défaut. Question pour
  l'utilisateur.
- **Durée de conservation** : 24 mois pour les replays bruts, puis agrégats seulement (DO-019).
- **Effacement** : bouton « Effacer mes données » qui envoie l'`install_id` à supprimer puis en tire un
  nouveau (DO-014). Sans compte, l'obligation Play Store d'un lien web de suppression de compte ne
  s'applique pas.
- **Play Store, formulaire « Sécurité des données »** : les données pseudonymes se déclarent. À
  déclarer (lecture de la page, à confirmer au moment de publier) : « Activité dans l'application »
  et « Appareil ou autres identifiants », finalité « Analyse » et « Fonctionnalités de l'app »,
  chiffrées en transit, non partagées, effacement sur demande possible. Une politique de
  confidentialité en page web est requise (GitHub Pages suffit).
- **Livraison hors Play Store** : tant que le jeu est distribué en APK direct, le formulaire ne
  s'applique pas, mais le RGPD, si.

## 4. Sécurité : données falsifiées, triche, IA empoisonnée

1. **Rejeu de chaque partie** (DO-010) : comme le moteur est déterministe, on rejoue graine + actions.
   Coup illégal ou résultat qui ne colle pas : la partie est écartée. On ne peut pas inventer une
   victoire.
2. **Intégrité de l'appli** (DO-011) : Play Integrity atteste que l'envoi vient de l'APK officiel.
   Quota gratuit par défaut : 10 000 jetons par jour, juste suffisant à 10 000 joueurs en lots
   quotidiens (extensible si l'app est sur le Play Store). À vérifier pour un APK distribué hors
   Play Store (S1).
3. **Anti-empoisonnement** (DO-012) : plafond de parties par installation, filtres de parties
   absurdes, poids faible des parties humaines face aux parties IA contre IA, et surtout **aucun
   modèle ne part vers les téléphones sans battre l'ancien au banc d'essai** (DO-005).
4. **Pas de secret dans l'APK** : le point d'envoi n'accepte qu'écrire (jamais lire) et limite le
   débit par installation.

## 5. Voir l'évolution de l'IA

| Indicateur | Source | Ce qu'il montre |
|---|---|---|
| Taux de victoire d'une version contre la précédente et contre des IA de référence fixes (aléatoire, simple) | Banc IA contre IA | Le progrès brut, à graines égales |
| Classement OpenSkill des versions (DO-006) | Banc + parties humaines | Une courbe unique, valable à 3 ou 4 joueurs |
| Taux de victoire de l'IA contre les humains, par niveau de difficulté | Parties des joueurs | Si l'IA est trop forte ou trop faible pour les vrais joueurs |
| Matrice clan contre clan, taux de choix de chaque clan | Les deux | L'équilibre des clans (utile à S2) |
| Répartition des actions (expansion, guerre, recherche, économie) et premiers coups typiques | Rejeu des replays | Les stratégies que l'IA invente |
| Durée des parties, taux d'abandon | Parties des joueurs | Le plaisir de jeu |

**Où les regarder** : une page web statique sur GitHub Pages (DO-007), régénérée par GitHub Actions
à chaque évaluation ; elle s'ouvre au téléphone, ne coûte rien et ne publie que des agrégats sans
identifiant. Plus tard, un écran dans le jeu (DO-018). Les deux sont visibles : aperçu d'abord.

## 6. Le retour : les nouveaux modèles vers les téléphones

1. **Étape 0** : le modèle est inclus dans l'APK ; une nouvelle IA = une nouvelle version livrée.
2. **Ensuite** : le modèle devient un fichier joint d'une Release GitHub, avec un manifeste (version,
   empreinte `sha256`, version de moteur compatible). Le jeu vérifie au lancement, comme la brique
   `maj-auto-github`, télécharge, vérifie l'empreinte et garde l'ancien en secours (DO-015).
3. **Plus tard** : déploiement progressif à une part des joueurs, comparaison, généralisation ou
   retour arrière (DO-016). Hugging Face Hub (gratuit pour les dépôts publics) est une alternative
   pour héberger les modèles avec leur fiche descriptive, sans intérêt tant que GitHub suffit.

Format et taille du modèle : à arrêter avec S3.
