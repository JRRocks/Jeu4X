# Sources de la session 3

> Une ligne par source : lien, date de consultation, ce qu'on en retient. On résume, on ne recopie pas.

| Source | Lien | Consulté le | Ce qu'on en retient |
|---|---|---|---|
| OpenSpiel (DeepMind), interface des jeux | https://github.com/google-deepmind/open_spiel | 2026-11-27 (connaissance, lien à revérifier) | Modèle d'interface : actions permises, appliquer, copier, tenseur d'observation, joueur courant ; bibliothèque d'algorithmes (MCTS, AlphaZero, CFR) |
| PyO3 et maturin | https://pyo3.rs | 2026-11-27 (connaissance) | Bibliothèque Rust appelable depuis Python |
| cargo-ndk | https://github.com/bbqsrc/cargo-ndk | 2026-11-27 (connaissance) | Compiler du Rust pour Android |
| uniffi (Mozilla) | https://github.com/mozilla/uniffi-rs | 2026-11-27 (connaissance) | Liaisons Rust vers Kotlin générées |
| ONNX Runtime Mobile | https://onnxruntime.ai | 2026-11-27 (connaissance) | Inférence de modèles sur Android |
| LiteRT (ex-TensorFlow Lite) | https://ai.google.dev/edge/litert | 2026-11-27 (connaissance) | Inférence de modèles sur Android |
| Tribes, cadre de recherche en IA pour un 4X au tour par tour sur petite carte (Perez-Liebana et al., AIIDE 2020) | https://github.com/GAIGResearch/Tribes | 2026-11-27 (connaissance, à revérifier) | Analyse seulement : les recherches arborescentes souffrent des nombreuses actions par tour ; banc d'essai de référence |
| Stratega, cadre pour jeux de stratégie au tour par tour (Dockhorn et al., 2020) | https://github.com/GAIGResearch/Stratega | 2026-11-27 (connaissance, à revérifier) | Interface « état, actions, vue par joueur » pour la stratégie |
| Online Evolutionary Planning in Hero Academy (Justesen, Mahlmann, Togelius, 2016) | référence d'article, lien à retrouver | 2026-11-27 (connaissance) | Faire évoluer le tour entier bat la recherche arborescente quand on joue plusieurs actions par tour |
| AlphaZero (Silver et al., Science 2018) | référence d'article | 2026-11-27 (connaissance) | Jeu contre soi-même, réseau valeur + politique, recherche guidée |
| Policy improvement by planning with Gumbel, « Gumbel AlphaZero » (Danihelka et al., ICLR 2022) | référence d'article | 2026-11-27 (connaissance) | Reste efficace avec très peu de simulations |
| Thinking Fast and Slow with Deep Learning and Tree Search, « Expert Iteration » (Anthony et al., 2017) | référence d'article | 2026-11-27 (connaissance) | Même principe qu'AlphaZero, formulé à part |
| MuZero (Schrittwieser et al., Nature 2020) | référence d'article | 2026-11-27 (connaissance) | Apprend un modèle du jeu : inutile quand on a les règles |
| AlphaStar, ligue et exploiteurs (Vinyals et al., Nature 2019) | référence d'article | 2026-11-27 (connaissance) | Ligue de versions passées et d'exploiteurs contre les cycles |
| Population Based Training (Jaderberg et al., 2017) | référence d'article | 2026-11-27 (connaissance) | Réglages appris au fil de l'entraînement par une population |
| Evolution Strategies as a Scalable Alternative to RL (Salimans et al., 2017) | référence d'article | 2026-11-27 (connaissance) | Stratégies d'évolution parallèles à grande échelle |
| CMA-ES (Hansen) et bibliothèque `cma` | https://github.com/CMA-ES/pycma | 2026-11-27 (connaissance) | Optimiseur sans dérivée pour quelques centaines de poids |
| NEAT (Stanley et Miikkulainen, 2002) | référence d'article | 2026-11-27 (connaissance) | Évolution de la structure d'un réseau |
| PPO (Schulman et al., 2017) | référence d'article | 2026-11-27 (connaissance) | Renforcement sans recherche |
| TrueSkill (Herbrich et al., 2006) | référence d'article | 2026-11-27 (connaissance) | Classement adapté aux parties à plusieurs joueurs |
