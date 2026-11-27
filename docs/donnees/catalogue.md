# Catalogue des idées : regroupement des données (`DO-`)

> Session 4, 2026-11-27. La recherche propose, l'utilisateur trie (D-07, D-11).
> Numéros jamais réutilisés. Coût : S (quelques heures), M (quelques jours), L (une ou plusieurs semaines).
> Étape : 0 = un seul joueur, 1 = une centaine, 2 = davantage (voir `recommandation.md`).

| N° | Titre | L'idée en une ligne | Intérêt | Coût | Étape |
|---|---|---|---|---|---|
| DO-001 | Replay compact | Une partie = graine + actions, ≈ 2 Ko | Indispensable : tout le reste en dépend | S | 0 |
| DO-002 | Carnet des parties sur le téléphone | Base locale de toutes les parties jouées, hors ligne | Rien ne se perd, même sans réseau | S | 0 |
| DO-003 | Export d'un fichier | « Partager mes parties » : un fichier à envoyer où l'on veut | Le plus simple pour l'étape 0 | S | 0 |
| DO-004 | Copie sur le Drive de l'utilisateur | Brique `google-drive-appdata`, dossier visible, pour que le PC ou Claude les relise | Automatique, privé, gratuit | M | 0 |
| DO-005 | Banc d'essai IA contre IA | Des milliers de parties entre versions d'IA, sur le PC ou en CI, résumées en un fichier | La vraie mesure du progrès, sans aucun joueur | M | 0 |
| DO-006 | Classement des versions d'IA | Note OpenSkill (Elo adapté à plus de 2 joueurs) par version | Un chiffre clair : « la v5 bat la v4 » | S | 0 |
| DO-007 | Tableau de bord en page web | Page statique sur GitHub Pages, régénérée à chaque évaluation, lisible au téléphone | Voir l'évolution de l'IA depuis le téléphone | M | 0 |
| DO-008 | Envoi par lots quand le réseau est là | File locale + envoi en arrière-plan, reprise après échec | Jeu hors ligne et collecte quand même | M | 1 |
| DO-009 | Petit point de collecte Cloudflare | Un Worker reçoit les lots, les range dans R2 (fichiers) et D1 (index) | Gratuit jusqu'à des milliers de joueurs, jamais mis en veille | M | 1 |
| DO-010 | Vérification par rejeu | Chaque replay reçu est rejoué : actions illégales ou résultat faux = rejeté | Anti-triche quasi gratuit grâce au moteur déterministe | M | 1 |
| DO-011 | Jeton d'intégrité Play | Chaque lot porte un jeton Play Integrity vérifié au serveur | Écarte les APK modifiés et les robots | M | 1 |
| DO-012 | Quarantaine anti-empoisonnement | Les parties humaines n'entrent dans l'entraînement qu'après filtres, plafond par joueur et pondération | L'IA ne peut pas être « dressée » par quelques tricheurs | M | 1 |
| DO-013 | Consentement et effacement | Question claire à la première partie, réglage pour changer d'avis, bouton « effacer mes données » | Obligatoire (RGPD) dès qu'on collecte | S | 1 |
| DO-014 | Identifiant d'installation jetable | UUID aléatoire, sans lien avec le téléphone ni le compte, réinitialisé à l'effacement | Minimisation : on ne sait jamais qui joue | S | 1 |
| DO-015 | Modèles d'IA en fichiers de Release | Le modèle part en fichier joint d'une Release GitHub, avec un manifeste (version, empreinte) | Nouvelle IA sans nouvel APK ; réutilise `maj-auto-github` | M | 1 |
| DO-016 | Déploiement progressif d'un modèle | La nouvelle IA ne va d'abord qu'à 20 % des joueurs ; on compare, puis on généralise ou on revient | Pas de mauvaise surprise pour tous | M | 2 |
| DO-017 | Jeu de données public | Parties anonymisées et agrégées publiées sur Hugging Face | Visibilité, aide d'autres chercheurs ; facultatif | M | 2 |
| DO-018 | Écran « progrès de l'IA » dans le jeu | Le joueur voit le niveau de l'IA monter, et sa part dans ce progrès | Motive à jouer et à partager ses parties ; écran = aperçu d'abord | M | 2 |
| DO-019 | Durée de conservation | Replays bruts gardés 24 mois, puis seulement les agrégats | Obligatoire (RGPD), et réduit le stockage | S | 1 |
| DO-020 | Revoir une partie | Le replay se rejoue à l'écran, coup par coup | Bonus du replay compact : déboguer l'IA, montrer une belle partie | M | 1 |
| DO-021 | Serveur loué à prix fixe | Un petit VPS (≈ 4 à 5 € par mois) avec PocketBase ou PostgreSQL | Sortie de secours si un service gratuit change ses règles | L | 2 |

---

## DO-001 · Replay compact

- **Description** : la partie est enregistrée sous forme graine + paramètres + suite des actions, compressée. Format détaillé dans `schema-donnees.md`, § 1.
- **Pourquoi** : 50 à 200 fois plus petit qu'un enregistrement de chaque état ; permet de tout recalculer plus tard (nouveaux indicateurs, nouvelles données d'entraînement) ; rend la vérification anti-triche possible (DO-010).
- **Coût** : S côté données ; dépend du moteur.
- **Dépendances** : moteur déterministe (S1), format d'action arrêté avec S3.
- **Sources** : principe standard des jeux au tour par tour (replays d'échecs PGN, replays de jeux de stratégie) ; `sources.md`.

## DO-002 · Carnet des parties sur le téléphone

- **Description** : une petite base locale (SQLite) garde toutes les parties ; aucune dépendance réseau.
- **Pourquoi** : le jeu reste jouable hors ligne ; c'est aussi la file d'attente de DO-008.
- **Coût** : S. **Dépendances** : DO-001, technologie choisie par S1 (D-04).

## DO-003 · Export d'un fichier

- **Description** : un bouton crée un fichier (`parties-AAAA-MM-JJ.jeu4x`) et ouvre le partage Android.
- **Pourquoi** : à l'étape 0, l'utilisateur est le seul joueur ; ce fichier suffit à alimenter le PC.
- **Coût** : S. **Dépendances** : DO-002. Visible (un bouton) : aperçu d'abord.

## DO-004 · Copie sur le Drive de l'utilisateur

- **Description** : la brique Library `google-drive-appdata`, variante dossier visible en `drive.file`, dépose chaque jour les nouvelles parties dans un dossier du Drive de l'utilisateur, que le PC (ou une session Claude avec le connecteur Drive) relit.
- **Pourquoi** : automatique et privé, sans serveur. **Limite** : chaque joueur écrit dans **son** Drive ; cela ne regroupe pas les parties de plusieurs joueurs. Bon pour l'étape 0 seulement.
- **Coût** : M (console Google Cloud, client OAuth, empreinte de la clé release).
- **Dépendances** : brique éprouvée pour le dépôt manuel, pas pour l'envoi périodique (fiche Library) ; dans un dépôt public, ne déclarer que le SHA-1 release (fiche Library, avertissement).

## DO-005 · Banc d'essai IA contre IA

- **Description** : script qui fait jouer N parties (graines fixes) entre deux versions d'IA et écrit un résumé (`schema-donnees.md`, § 3).
- **Pourquoi** : c'est la mesure principale du progrès, à toute échelle ; ne contient aucune donnée personnelle, donc publiable.
- **Coût** : M. **Dépendances** : moteur sans écran (S1), IA (S3). Partagé avec S3.

## DO-006 · Classement des versions d'IA

- **Description** : note OpenSkill (variante libre de TrueSkill) calculée sur les résultats de DO-005 et, plus tard, des parties contre humains.
- **Pourquoi** : l'Elo classique ne vaut que pour deux joueurs ; OpenSkill gère 3 ou 4 joueurs et les équipes.
- **Coût** : S. **Sources** : bibliothèque `openskill` (licence MIT), `sources.md`.

## DO-007 · Tableau de bord en page web

- **Description** : page statique (graphiques) générée par la CI à partir des résumés, publiée sur GitHub Pages ; s'ouvre au téléphone.
- **Pourquoi** : gratuit, rien à entretenir, pas de serveur. Ne publie que des données d'IA contre IA et des agrégats.
- **Coût** : M. **Dépendances** : DO-005. Visible : aperçu d'abord.

## DO-008 · Envoi par lots

- **Description** : chaque jour, ou au retour du Wi-Fi, les parties non envoyées partent en un lot ; en cas d'échec, on réessaie plus tard ; une partie envoyée est marquée, jamais renvoyée (dédoublonnage par `game_id`).
- **Coût** : M. **Dépendances** : DO-002, DO-009, DO-013.

## DO-009 · Petit point de collecte Cloudflare

- **Description** : un Worker (petit programme hébergé) reçoit le lot, vérifie le jeton (DO-011), range le lot brut dans R2 et une ligne par partie dans D1.
- **Pourquoi** : offre gratuite large, pas de mise en veille, pas de carte bancaire pour l'offre gratuite, 5 $ par mois au-delà. Alternatives comparées dans `etude.md`.
- **Coût** : M (un compte Cloudflare, à créer par l'utilisateur le moment venu).

## DO-010 · Vérification par rejeu

- **Description** : avant d'entrer dans les statistiques ou l'entraînement, chaque replay est rejoué par le moteur ; s'il contient un coup illégal ou si le résultat annoncé ne correspond pas, il est écarté.
- **Pourquoi** : on ne peut pas falsifier une victoire sans jouer une vraie partie. Seule triche restante : jouer avec l'aide d'un programme, sans grand danger ici.
- **Coût** : M. **Dépendances** : moteur déterministe et rapide sans écran (S1) ; se fait hors du Worker (sur le PC ou en CI), sur les lots.

## DO-011 · Jeton d'intégrité Play

- **Description** : l'API Play Integrity atteste que la requête vient de l'APK officiel sur un vrai téléphone.
- **Pourquoi** : écarte les envois fabriqués à la main. **Limite** : suppose une distribution par le Play Store ou au moins une app reconnue ; à vérifier avec la livraison par APK direct (S1).
- **Coût** : M.

## DO-012 · Quarantaine anti-empoisonnement

- **Description** : une partie humaine n'entre dans un jeu d'entraînement qu'après DO-010, avec un plafond de parties par `install_id`, l'écart des parties absurdes (abandon au tour 1, coups répétés) et un poids faible face aux parties IA contre IA.
- **Pourquoi** : un petit groupe ne peut pas « dresser » l'IA à perdre d'une certaine façon. Chaque nouveau modèle passe le banc DO-005 contre l'ancien avant d'être livré.
- **Coût** : M. **Dépendances** : S3.

## DO-013 · Consentement et effacement

- **Description** : à la première partie, une question simple (« Partager mes parties, sans rien qui m'identifie, pour faire progresser l'IA ? Oui / Non ») ; réglage pour changer d'avis ; bouton « Effacer mes données » ; politique de confidentialité en page web.
- **Pourquoi** : le RGPD l'exige pour des données pseudonymes ; le Play Store exige la politique et le formulaire « sécurité des données ».
- **Coût** : S. Visible : aperçu d'abord.

## DO-014 · Identifiant d'installation jetable

- **Description** : `install_id` tiré au hasard, sans lien avec l'identifiant publicitaire, le téléphone ou le compte Google.
- **Coût** : S.

## DO-015 · Modèles d'IA en fichiers de Release

- **Description** : chaque modèle validé devient un fichier joint d'une Release GitHub du dépôt public, avec un manifeste (`schema-donnees.md`, § 4) ; le jeu vérifie l'empreinte `sha256` avant de l'utiliser et garde l'ancien en secours.
- **Pourquoi** : gratuit, déjà maîtrisé (brique `maj-auto-github`) ; la nouvelle IA arrive sans réinstaller. À l'étape 0, le modèle peut simplement être inclus dans l'APK.
- **Coût** : M.

## DO-016 · Déploiement progressif d'un modèle

- **Description** : le manifeste porte une part (`rollout: 0.2`) ; chaque téléphone tire au sort s'il prend le nouveau modèle ; le tableau de bord compare les deux groupes.
- **Coût** : M. **Dépendances** : DO-009, DO-015.

## DO-017 · Jeu de données public

- **Description** : chaque trimestre, un extrait sans `install_id` (agrégats et replays sans lien entre eux) publié sur Hugging Face.
- **Pourquoi** : facultatif ; à n'envisager qu'avec un consentement qui le mentionne.
- **Coût** : M.

## DO-018 · Écran « progrès de l'IA »

- **Description** : dans le jeu, une courbe du niveau de l'IA et « vos parties ont servi à entraîner la version 7 ».
- **Coût** : M. Visible : aperçu d'abord (S6 pour le style).

## DO-019 · Durée de conservation

- **Description** : replays bruts 24 mois, puis effacés ; agrégats gardés.
- **Coût** : S (une tâche planifiée).

## DO-020 · Revoir une partie

- **Description** : le replay se relit à l'écran tour par tour.
- **Pourquoi** : outil de débogage pour S1 et S3 ; plus tard, fonction de jeu.
- **Coût** : M. Visible : aperçu d'abord.

## DO-021 · Serveur loué à prix fixe

- **Description** : un VPS d'entrée de gamme avec PocketBase (un seul exécutable, base SQLite, interface d'administration web) ou PostgreSQL.
- **Pourquoi** : prix fixe et aucune dépendance à un service ; **mais** mises à jour de sécurité et sauvegardes à faire soi-même : lourd pour quelqu'un qui ne code pas. Sortie de secours, pas premier choix.
- **Coût** : L.
