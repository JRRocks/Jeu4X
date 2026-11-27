# Sources de la session 3

> Une ligne par source : lien, date de consultation, ce qu'on en retient. On résume, on ne recopie pas.
> Toutes les pages ci-dessous ont été ouvertes le 2026-11-27. Quand l'éditeur refusait la lecture
> automatique, la notice a été lue par Crossref ou par une page miroir (indiqué).

| Source | Lien | Consulté le | Ce qu'on en retient |
|---|---|---|---|
| OpenSpiel (DeepMind) | https://github.com/google-deepmind/open_spiel · https://openspiel.readthedocs.io/en/latest/api_reference.html | 2026-11-27 | Interface d'un état : actions permises, appliquer, copier, tenseurs d'observation et d'information, joueur courant ; algorithmes MCTS, AlphaZero, CFR |
| PyO3 et maturin | https://pyo3.rs · https://www.maturin.rs | 2026-11-27 | Bibliothèques Rust appelables depuis Python ; maturin les construit et les publie |
| cargo-ndk | https://github.com/bbqsrc/cargo-ndk | 2026-11-27 | Compiler du Rust avec le NDK Android |
| uniffi (Mozilla) | https://github.com/mozilla/uniffi-rs | 2026-11-27 | Liaisons Rust générées vers Kotlin, Swift, Python |
| ONNX Runtime Mobile | https://onnxruntime.ai/docs/tutorials/mobile/ | 2026-11-27 | Paquet `onnxruntime-android`, accélération NNAPI et XNNPACK |
| LiteRT | https://developers.google.com/edge/litert | 2026-11-27 | Successeur de TensorFlow Lite, Android 7 (API 24) et plus |
| Tribes (Perez-Liebana et al., AIIDE 2020) | https://github.com/GAIGResearch/Tribes · https://doi.org/10.1609/aiide.v16i1.7438 | 2026-11-27 | Analyse seulement. Cartes 11×11 ; ~10^15 tours possibles en moyenne ; RHEA meilleure IA (63 % contre MCTS), IA à règles deuxième, MCTS troisième ; toutes battues par un humain |
| Stratega (Dockhorn et al., AIIDE 2020 ; Perez-Liebana et al., arXiv 2020) | https://github.com/GAIGResearch/Stratega · https://arxiv.org/abs/2009.05643 | 2026-11-27 | Cadre pour jeux de stratégie au tour par tour |
| Online Evolution for Multi-Action Adversarial Games (Justesen, Mahlmann, Togelius, EvoApplications 2016) | https://doi.org/10.1007/978-3-319-31204-0_38 (notice lue sur https://lup.lub.lu.se/record/8569733) | 2026-11-27 | Sur Hero Academy, faire évoluer le tour entier bat MCTS et les IA gloutonnes de loin |
| AlphaZero (Silver et al., Science 2018) | https://doi.org/10.1126/science.aar6404 (via Crossref) · https://arxiv.org/abs/1712.01815 (prépublication 2017) | 2026-11-27 | Jeu contre soi-même, réseau valeur + politique, recherche guidée |
| Policy improvement by planning with Gumbel (Danihelka et al., ICLR 2022) | https://openreview.net/forum?id=bERaNdoegnO | 2026-11-27 | Nettement meilleur que l'existant avec peu de simulations |
| Expert Iteration (Anthony, Tian, Barber, 2017) | https://arxiv.org/abs/1705.08439 | 2026-11-27 | Même principe qu'AlphaZero, sur Hex |
| MuZero (Schrittwieser et al., Nature 2020) | https://arxiv.org/abs/1911.08265 · https://doi.org/10.1038/s41586-020-03051-4 | 2026-11-27 | Apprend un modèle du jeu : inutile quand on a les règles |
| AlphaStar (Vinyals et al., Nature 2019) | https://doi.org/10.1038/s41586-019-1724-z | 2026-11-27 | Ligue : exploiteurs principaux contre le champion, exploiteurs de ligue contre tous les anciens |
| Population Based Training (Jaderberg et al., 2017) | https://arxiv.org/abs/1711.09846 | 2026-11-27 | Réglages appris par une population au fil de l'entraînement |
| Evolution Strategies as a Scalable Alternative to RL (Salimans et al., 2017) | https://arxiv.org/abs/1703.03864 | 2026-11-27 | Stratégies d'évolution parallèles à grande échelle |
| pycma | https://github.com/CMA-ES/pycma | 2026-11-27 | CMA-ES en Python |
| EvoTorch | https://github.com/nnaisense/evotorch | 2026-11-27 | Algorithmes évolutionnaires sur PyTorch (CMA-ES, PGPE, SNES, GA, MAP-Elites…) |
| NEAT (Stanley et Miikkulainen, Evolutionary Computation 10(2), 2002) | https://doi.org/10.1162/106365602320169811 (via Crossref) | 2026-11-27 | Évolution de la structure et des poids d'un réseau |
| PPO (Schulman et al., 2017) | https://arxiv.org/abs/1707.06347 | 2026-11-27 | Renforcement sans recherche |
| TrueSkill (Herbrich, Minka, Graepel, NIPS 2006) | https://papers.nips.cc/paper_files/paper/2006/file/f44ee263952e65b3610b8ba51229d1f9-Paper.pdf | 2026-11-27 | Classement pour un nombre quelconque de joueurs ou d'équipes |
| FAQ de Google Colab | https://research.google.com/colaboratory/faq.html | 2026-11-27 | GPU gratuits à limites variables, sans garantie |
| Kaggle Notebooks | https://www.kaggle.com/docs/notebooks | 2026-11-27 | GPU gratuits ; aucun quota officiel lisible sur la page : non vérifié |
| GitHub Actions : facturation, limites, conditions | https://docs.github.com/en/billing/concepts/product-billing/github-actions · https://docs.github.com/en/actions/reference/limits · https://docs.github.com/en/site-policy/github-terms/github-terms-for-additional-products-and-features | 2026-11-27 | Gratuit pour un dépôt public ; 6 h par tâche ; interdit sur les machines hébergées toute activité sans rapport avec la production, le test ou la publication du projet |
