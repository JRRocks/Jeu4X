# Clé d'essai (publique, volontairement)

`cle-essai.keystore` signe seulement la version d'essai `fr.jeu4x.app.essai` (« Jeu4X essai »), qui ne sera jamais distribuée (D-17). Elle est versionnée pour que chaque essai s'installe par-dessus le précédent, quelle que soit la machine qui compile. Mot de passe : `essai-public`, alias `essai`. Ce n'est pas un secret : n'importe qui peut signer une application qui remplace « Jeu4X essai » sur un téléphone, ce qui est sans conséquence pour une version d'essai privée.

Le vrai jeu `fr.jeu4x.app` est signé d'une autre clé, jamais versionnée (`docs/environnement/signature.md`).
