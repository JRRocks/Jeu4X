# Mission de la session 4 : le regroupement des données

> Écrite par la session maître le 2026-11-27. Branche : `s4-donnees`. Dossier : `docs/donnees/`. Préfixe des idées : `DO-`.
> Lire d'abord `CLAUDE.md`, `docs/ORGANISATION.md` et `docs/DECISIONS.md`.

## Ce que l'utilisateur a demandé

Travailler sur le concept de regroupement des données : sur un serveur ? Sur une session locale quelque part ? Ce regroupement servira à voir les évolutions de l'IA.

C'est une étude (D-11) : tu ne codes rien, tu ne crées aucun compte, tu n'engages aucune dépense.

## Ce qu'il faut couvrir

1. **Ce qu'on regroupe** : parties (replays compacts : graine + actions), résultats, version de l'IA, clans joués, durée ; volume estimé par partie et pour 1, 100 et 10 000 joueurs. Les besoins de l'IA viennent de la session 3 (`docs/ia/besoins-donnees.md` sur la branche `s3-ia`, dès qu'il existe : `git fetch origin s3-ia`) ; en attendant, fais des hypothèses et écris-les.
2. **Où** : tout sur le téléphone (avec export), le PC de l'utilisateur comme petit serveur, son Google Drive (brique `google-drive-appdata` de la Library), GitHub, services gratuits ou peu chers (Firebase, Supabase, Cloudflare, PocketBase, Hugging Face pour les modèles et les jeux de données…), petit serveur loué, formule mixte (le téléphone garde et envoie quand il peut).
3. **Critères** : coût à chaque échelle ; vie privée et RGPD (consentement, identifiant anonyme, minimisation, durée de conservation, effacement, mineurs, formulaire « sécurité des données » du Play Store) ; sécurité (données falsifiées, triche, IA empoisonnée) ; effort d'entretien pour quelqu'un qui ne code pas ; jeu hors ligne ; dépendance à un fournisseur.
4. **Voir l'évolution de l'IA** : quels indicateurs (taux de victoire par version, classement Elo, stratégies, équilibre des clans) et où les regarder (tableau de bord en page web, depuis le téléphone).
5. **Le retour** : comment les nouveaux modèles d'IA repartent vers les téléphones (avec la session 3).
6. **Une recommandation par étapes** : maintenant (un seul joueur), puis une centaine, puis davantage.

## Livrables

- `docs/donnees/etude.md` (comparatif), `recommandation.md`, `schema-donnees.md`, `catalogue.md` (`DO-NNN`), `sources.md`, `JOURNAL.md`.
- Un rapport à la session maître : la recommandation, ce qu'elle coûte, les questions pour l'utilisateur.
