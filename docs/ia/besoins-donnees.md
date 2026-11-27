# Ce que l'IA doit recevoir des parties

> Écrit par Claude, session S3, le 2026-11-27. Pour la session 4, qui étudie où et comment
> regrouper les données. Proposition, rien n'est décidé.

## 1. Le principe

L'IA n'a pas besoin de films ni de captures : elle a besoin du **replay**, c'est-à-dire de quoi
**rejouer exactement** la partie avec le moteur (format dans `besoins-moteur.md`, E9). Tout le
reste (positions, scores, tours) se recalcule en rejouant.

## 2. Une partie envoyée

| Champ | Contenu | Pourquoi |
|---|---|---|
| `format` | version du format de replay | lire les anciens fichiers |
| `rules_version` | version des règles | rejouer avec le bon moteur |
| `encoding_version` | version du codage de l'état | savoir quels modèles peuvent apprendre dessus |
| `app_version` | version de l'APK | repérer un défaut d'une version |
| `seed`, `map_settings` | graine et réglages de la carte | recréer l'état de départ |
| `players` | pour chaque place : humain ou IA ; pour une IA, sa version et son niveau ; clan | savoir qui a joué quoi |
| `player_id` | identifiant **anonyme et aléatoire** du joueur humain, propre au jeu, jamais lié à un compte ni à un appareil | limiter le poids d'un joueur, IA fantôme (avec accord), sans savoir qui il est |
| `actions` | suite des numéros d'actions, avec le tour et la place | le cœur : de quoi rejouer |
| `checks` | empreinte de l'état tous les N tours et à la fin | vérifier le rejeu, repérer une partie truquée |
| `result` | gagnant, scores, tour de fin, abandon ou non | tri rapide sans rejouer |
| `day` | date au jour près | suivre l'évolution dans le temps |
| `think_time` (facultatif) | temps de réflexion par tour, arrondi | repérer des joueurs automatiques |

Taille attendue : quelques kilooctets par partie, moins après compression.

**Ce qui ne doit jamais y être** : nom, adresse e-mail, compte, modèle ou identifiant de
l'appareil, position, adresse IP conservée, texte libre du joueur.

## 3. Ce que le stockage doit permettre

1. **Recevoir** des replays envoyés depuis des téléphones parfois hors ligne (envoi différé,
   reprise, pas de doublon : une empreinte du replay sert de clé).
2. **Contrôler** : rejouer chaque replay avec le moteur de la version indiquée avant de
   l'accepter ; rejeter les parties impossibles ou dont l'empreinte ne correspond pas.
3. **Trier** par version des règles, niveau des joueurs, date, type de partie (humain contre IA,
   humain contre humain).
4. **Exporter** en lot vers l'entraînement (un fichier par jour ou par version, par exemple).
5. **Effacer** les parties d'un joueur sur sa demande (via son identifiant anonyme).
6. **Suivre l'évolution de l'IA** : classement de chaque version d'IA, taux de victoire des
   humains par niveau et par version, nombre de parties par jour. C'est le tableau de bord que
   l'utilisateur voudra voir.

## 4. Volumes

| Hypothèse | Parties par jour | Données par jour |
|---|---|---|
| L'utilisateur et quelques proches | 10 à 100 | < 1 Mo |
| 1 000 joueurs actifs | ~10 000 | ~20 à 50 Mo |

Les parties entre IA de l'entraînement restent sur la machine d'entraînement : elles ne passent
pas par le stockage.

## 5. Consentement

- L'envoi des parties est **désactivé par défaut** et demandé clairement (« aider l'IA à
  progresser en partageant vos parties anonymes »).
- La création d'une IA fantôme à partir des parties d'un joueur demande un accord à part.
- Le jeu reste entièrement jouable sans rien envoyer.
