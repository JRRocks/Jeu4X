# Journal de bord

> Journal général, tenu par la session maître. Entrée la plus récente **en haut**. On ne modifie
> ni ne supprime une entrée passée : une erreur se corrige par une nouvelle entrée.
> Chaque session de travail tient aussi le sien dans `docs/<dossier>/JOURNAL.md`.

Gabarit :

```markdown
## AAAA-MM-JJ · <titre écrit pour l'utilisateur>

- **Auteur** : Claude, session <maître ou Sx> (ou Antigravity)
- **Branche et commit** :
- **Objectif** :
- **Réalisé** :
- **Décisions** :
- **Vérifications** : <commandes lancées et résultats>
- **Non vérifié sur téléphone** : <points précis, reportés dans le plan>
- **Problèmes et dette** :
- **Prochaine étape** :
```

---

## 2026-11-27 · Premiers livrables, choix de la technologie et de la façon de trier

- **Auteur** : Claude, session maître
- **Branche et commit** : `main`
- **Objectif** : relayer les premiers livrables et les décisions de l'utilisateur.
- **Réalisé** : S3 a livré les besoins du moteur pour l'entraînement (vitesse, reproductibilité, actions numérotées), transmis à S1. S1 a livré un comparatif mesuré par essais jetables : 15 200 parties par seconde en Rust, 12 800 en Kotlin, 7 400 en JavaScript, 590 en Python ; quand Python joue coup par coup, Rust est environ 5 fois plus rapide que Kotlin, à égalité quand le moteur joue par lots. S2 (54 idées `GD-`), S3 (19 idées `IA-`) et S4 (21 idées `DO-`) ont livré leurs catalogues.
- **Décisions** : D-12 (technologie : un mélange par points forts, entraînement sur ordinateur idéalement en Rust, IA déjà entraînée sur le téléphone), D-13 (tri sur une page à cocher).
- **Vérifications** : commits des branches relus (présence des fichiers, identité `noreply`). Sources de S2 et S4 consultées en ligne ; celles de S3, d'abord citées de mémoire, revérifiées à ma demande : 24 sources, 19 tiennent, 3 corrigées, 1 non vérifiable, 1 affirmation retirée.
- **Non vérifié sur téléphone** : rien à installer encore.
- **Problèmes et dette** : les vitesses de S1 viennent d'essais jetables non versionnés ; à refaire sur le vrai moteur.
- **Prochaine étape** : S1 revoit le mélange de technologies puis enchaîne sur l'étape B ; page de tri à publier ; livrables de S5 et S6.

## 2026-11-27 · Création du projet et lancement des six sessions

- **Auteur** : Claude, session maître
- **Branche et commit** : `main`, commit de départ
- **Objectif** : poser le dépôt, sa mémoire et l'organisation, puis lancer les six sessions de travail.
- **Réalisé** : dépôt public créé par l'utilisateur ; mémoire posée (`CLAUDE.md`, plan, décisions, organisation, une mission par session) ; six sessions de travail lancées, chacune sur sa branche (`docs/ORGANISATION.md`).
- **Décisions** : D-01 à D-11.
- **Vérifications** : contrôle `depot-public` avant le push. Le commit initial créé par GitHub portait une identité personnelle : historique remplacé par ce commit à identité `noreply`, avec l'accord de l'utilisateur ; il doit activer la protection de son adresse dans les réglages GitHub.
- **Non vérifié sur téléphone** : rien à installer encore.
- **Problèmes et dette** : aucun.
- **Prochaine étape** : comparatif des technologies (S1) à présenter à l'utilisateur ; premiers rapports des sessions 2 à 6.
