# Architecture proposée de l'IA

> Écrit par Claude, session S3, le 2026-11-27. Proposition : rien n'est décidé, rien n'est codé (D-11).
> S'appuie sur `etude.md` (méthodes) et `besoins-moteur.md` (ce que les règles doivent offrir).

## 1. Vue d'ensemble

```
                 ┌──────────────────────────── PC / machine louée ───────────────────────────┐
                 │  Règles (même code que le jeu, liées à Python)                           │
                 │    ├─ évolution des IA à utilité (CMA-ES, ligue)        → profils .json   │
                 │    ├─ imitation des parties + jeu contre soi-même       → réseau .onnx    │
                 │    └─ tournoi de validation + classement               → galerie d'IA    │
                 └───────────────────────────────▲───────────────────┬──────────────────────┘
                              replays contrôlés   │                   │ modèles validés
                                                  │                   ▼
           ┌──────── stockage des parties (S4) ───┴───┐     ┌── publication (Releases GitHub) ──┐
           │ replays anonymes, rejoués et filtrés     │     │ fichier + version + empreinte     │
           └──────────────────▲───────────────────────┘     └───────────────┬───────────────────┘
                              │ (avec l'accord du joueur, plus tard)        │ téléchargé si compatible
           ┌──────────────────┴──────────────────── téléphone ─────────────┴───────────────────┐
           │ Règles (cœur natif)  ←→  Écran                                                      │
           │ Joueur IA = profil d'utilité (+ réseau si niveau élevé) + planification à budget fixe│
           │ Adaptation locale : style du joueur, choix du niveau ; replays gardés hors ligne     │
           └─────────────────────────────────────────────────────────────────────────────────────┘
```

## 2. Le joueur IA sur le téléphone

Un « joueur IA » se compose de trois pièces interchangeables :

1. **Évaluation** : note un état ou une action. D'abord une fonction d'utilité (poids dans un
   petit fichier JSON, « profil »), plus tard un réseau (`.onnx`, valeur + politique).
2. **Décision** : comment l'évaluation devient un coup. Glouton (meilleure action), planification
   du tour par évolution, ou recherche arborescente courte. Chacune a un **budget de temps**.
3. **Personnalité** : un profil par clan (poids, préférences), défini avec la session 2.

Le **niveau de difficulté** = un choix de ces trois pièces dans la galerie validée. Les niveaux
bas n'ont besoin d'aucun réseau et marchent sur tous les téléphones.

L'IA ne lit que la **vue de son joueur** (pas de triche avec le brouillard), et elle passe par les
mêmes règles que l'humain : elle ne peut pas jouer un coup interdit.

## 3. L'entraînement

- Langage : **Python** (PyTorch, `cma`), appelant les règles compilées (voir
  `besoins-moteur.md`, § 4).
- Tout entraînement produit une **fiche** : version du code d'entraînement, version des règles et
  du codage, graines, données utilisées, durée, classement obtenu.
- Le **tournoi de validation** est le seul chemin vers la publication : une version candidate
  joue contre la version en service et contre la galerie ; elle n'est publiée que si elle gagne
  nettement et ne recule contre personne (`etude.md`, § 4).
- Ce banc d'essai (tournoi, classement, rejeu) se construit **avant** toute IA sophistiquée : il
  sert à toutes les étapes.

## 4. La distribution des modèles

| Quoi | Où | Taille visée |
|---|---|---|
| IA de départ (profils + éventuellement un réseau) | dans l'APK | < 5 Mo en tout |
| Nouvelles versions | fichiers publiés à part (Releases GitHub du dépôt public), listés dans un petit index : nom, version d'IA, version des règles, empreinte | < 5 Mo chacune |

- Le jeu interroge l'index de temps en temps (s'il y a du réseau), télécharge un modèle
  compatible, vérifie son empreinte, puis le propose ou l'utilise. Il garde toujours le modèle
  précédent.
- Un modèle ne contient que des nombres, pas de code : il ne peut rien exécuter sur le téléphone.
- La brique `maj-auto-github` de la Library couvre déjà le téléchargement depuis les Releases
  pour l'APK ; elle pourra servir de base (à confirmer quand on y sera).

## 5. Le retour des parties (plus tard, étape R4)

- Le téléphone garde les replays des parties terminées (quelques kilooctets chacun).
- Avec l'accord explicite du joueur, il les envoie au stockage choisi par la session 4.
- Le stockage les **rejoue** avec le moteur de la bonne version avant de les accepter.
- Détail des champs : `besoins-donnees.md`.

## 6. Ce qui dépend des autres sessions

| Session | Question |
|---|---|
| S1 | Langage des règles et liaison vers Python ; moteur d'inférence (ONNX Runtime ou LiteRT) selon la technologie choisie |
| S2 | Taille de carte, nombre de joueurs, brouillard, hasard des combats, clans et leurs préférences |
| S4 | Où vivent les replays et les modèles ; consentement ; anonymisation |
| S6 | Comment montrer le niveau et la personnalité de l'IA (rien de visible sans aperçu) |

## 7. Les cas encore ouverts, et ce qu'ils changent

> Ajouté le 2026-11-27 à la demande de la session maître : S2 propose les options, l'utilisateur
> tranche au tri. L'architecture ci-dessus vaut dans tous les cas ; seules les pièces ci-dessous
> changent.

### 7.1 Information complète ou brouillard de guerre

| Pièce | Information complète | Brouillard |
|---|---|---|
| Ce que l'IA lit | l'état entier | la vue de son joueur seulement (`besoins-moteur.md`, E6) |
| Recherche arborescente et planification du tour | directement sur l'état | sur des **états tirés au sort** compatibles avec la vue (plusieurs tirages, résultats moyennés) : coût multiplié par le nombre de tirages, d'où un budget plus serré sur le téléphone |
| IA à utilité | critères sur tout ce qui est sur la carte | critères sur ce qui est vu, plus une **mémoire** de ce qui a été vu (dernière position connue des unités ennemies) |
| Réseau | plans de l'état | plans de la vue, plus un plan « vu il y a N tours » ; un réseau plus difficile à entraîner (plus de parties nécessaires) |
| Contrôle des replays et triche | rejeu simple | rejeu simple aussi (le serveur a l'état complet) ; une IA qui lirait l'état complet sur le téléphone tricherait : à tester par le banc d'essai |
| Coût global | référence | **environ 1,5 à 3 fois plus** de calcul d'entraînement (estimation) et plus de travail sur l'IA à utilité |

Avis pour l'IA : l'information complète est nettement plus simple. Un brouillard reste faisable
avec les méthodes retenues ; il vaut mieux qu'il soit décidé **avant** que S1 fige l'état du
moteur, car la vue par joueur et le tirage d'états se prévoient dès le début.

### 7.2 Combats avec ou sans hasard

| Pièce | Combats déterministes | Combats avec hasard |
|---|---|---|
| Recherche | une action donne un seul état suivant | une action donne plusieurs états possibles : nœuds de hasard dans la recherche, ou plusieurs tirages par action |
| IA à utilité | calcule le résultat exact d'une attaque | calcule l'espérance (dégâts moyens, chance de détruire) |
| Évaluation des IA | moins de parties pour départager deux versions | plus de parties pour départager (le hasard brouille les résultats) : tournois 2 à 4 fois plus longs (estimation) |
| Reproductibilité | sans souci | sans souci, le hasard vient de la graine de l'état (E3) |

Avis pour l'IA : les deux marchent ; les combats sans hasard sont plus simples et plus lisibles
pour le joueur. Un hasard limité (par exemple seulement sur la carte de départ) garde les
avantages des deux.

### 7.3 Taille de carte et nombre de joueurs

| Pièce | Petite carte, 2 joueurs | Grande carte, 4 joueurs ou plus |
|---|---|---|
| Espace d'actions et plans du réseau | petits, fixés par la carte la plus grande permise | plus grands : réseau plus gros ou plus lent sur le téléphone |
| Méthodes | toutes, y compris les méthodes à deux joueurs telles quelles | récompense par rang ou par score ; ligue plus difficile (alliances de fait, « faiseur de roi ») |
| Vitesse d'entraînement | référence | parties plus longues, donc moins de parties par seconde |

Avis pour l'IA : prévoir dès le départ **une taille maximale** de carte et de nombre de joueurs ;
le moteur et le codage se bâtissent sur ce maximum, et toutes les cartes plus petites passent.
Commencer l'entraînement sur des parties à 2 joueurs, puis étendre.
