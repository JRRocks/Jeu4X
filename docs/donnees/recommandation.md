# Recommandation : regrouper les données par étapes

> Session 4, 2026-11-27. Proposition ; rien n'est décidé ni construit (D-11).
> Détail et comparatif : `etude.md`. Idées : `catalogue.md`.

## En une phrase

**Le téléphone garde chaque partie sous forme de replay compact ; l'IA se mesure surtout contre
elle-même sur GitHub Actions, avec un tableau de bord gratuit sur GitHub Pages ; quand d'autres
joueurs arrivent, leurs parties partent par lots vers un petit point de collecte Cloudflare
gratuit ; les nouveaux modèles reviennent par les Releases GitHub.**

## Étape 0 : maintenant, un seul joueur (l'utilisateur) — 0 €

| Quoi | Idées |
|---|---|
| Replay compact et carnet des parties sur le téléphone | DO-001, DO-002 |
| Export d'un fichier, puis copie automatique sur le Drive de l'utilisateur | DO-003, puis DO-004 |
| Banc d'essai IA contre IA sur GitHub Actions (gratuit, dépôt public) | DO-005 |
| Classement des versions et tableau de bord sur GitHub Pages | DO-006, DO-007 |
| Modèle d'IA inclus dans l'APK | `etude.md`, § 6 |

Aucun compte à créer (hors Google Cloud si DO-004 est retenu), aucune donnée personnelle
envoyée, aucun formulaire. C'est déjà suffisant pour voir l'IA progresser : la mesure qui compte
vient des milliers de parties IA contre IA, pas des parties humaines.

## Étape 1 : une centaine de joueurs — 0 €

| Quoi | Idées |
|---|---|
| Consentement, identifiant jetable, effacement, durée de conservation, politique de confidentialité | DO-013, DO-014, DO-019 |
| Envoi par lots vers Cloudflare (Worker + R2 + D1, offre gratuite) | DO-008, DO-009 |
| Vérification par rejeu et quarantaine avant tout usage | DO-010, DO-012 |
| Jeton Play Integrity, si le jeu passe par le Play Store | DO-011 |
| Modèles livrés par fichiers de Release, sans nouvel APK | DO-015 |

Un compte Cloudflare gratuit à créer par l'utilisateur, le moment venu.
Alternative si l'utilisateur préfère rester chez Google : Firebase (Firestore + App Check),
gratuit aussi à cette échelle, mais avec une dépendance plus forte.

## Étape 2 : des milliers de joueurs — 0 à 8 $ par mois

| Quoi | Idées |
|---|---|
| Même architecture ; Workers Paid à 5 $ par mois et stockage R2 au-delà de 10 Go (≈ 0 à 3 $) quand les quotas gratuits sont dépassés, vers 1 000 à 10 000 joueurs très actifs | DO-009 |
| Déploiement progressif des modèles | DO-016 |
| Écran « progrès de l'IA » dans le jeu | DO-018 |
| Facultatif : jeu de données public sur Hugging Face | DO-017 |
| Sortie de secours si Cloudflare change ses règles : petit serveur loué ≈ 4,60 à 7 € par mois | DO-021 |

## Ce qui est écarté, et pourquoi

- **GitHub pour recevoir les parties** : le dépôt est public et il faudrait un secret dans l'APK.
- **Le PC comme serveur exposé à Internet** : fragile et risqué pour un non-codeur (le PC reste le bon endroit pour *entraîner* l'IA).
- **Supabase gratuit** : s'endort après une semaine sans activité ; Pro à 25 $ trop cher pour ce volume.
- **Drive pour regrouper plusieurs joueurs** : chacun écrit dans son propre Drive, invisible des autres.

## Questions pour l'utilisateur

1. **Partage des parties des joueurs, quand il y en aura** : a) demandé à la première partie, « non » par défaut, et rien n'est envoyé pour les moins de 15 ans *(recommandé)* ; b) demandé, « oui » coché par défaut ; c) jamais de collecte, seulement l'IA contre elle-même.
2. **Point de collecte de l'étape 1** : a) Cloudflare *(recommandé : gratuit, sans mise en veille, sans carte bancaire)* ; b) Firebase (tout chez Google) ; c) petit serveur loué, prix fixe.
3. **Copie automatique de tes propres parties sur ton Drive à l'étape 0** : a) oui, avec la brique `google-drive-appdata` *(recommandé)* ; b) non, l'export manuel suffit pour l'instant.
4. **Tableau de bord de l'IA** : a) page web publique sur GitHub Pages, données d'IA seulement *(recommandé)* ; b) page privée (demande un service avec connexion) ; c) pas de tableau de bord pour l'instant.
