# Sources

> Consultées le 2026-11-27. Résumées, jamais recopiées.

| Sujet | Lien | Ce qu'on en retient |
|---|---|---|
| Firebase, tarifs | https://firebase.google.com/pricing | Firestore gratuit : 1 Gio, 50 000 lectures, 20 000 écritures par jour ; Functions sur Blaze seulement |
| Firebase, Cloud Storage | https://firebase.google.com/docs/storage/faqs-storage-changes-announced-sept-2024 | Plan Blaze exigé depuis le 3 février 2026 |
| Play Integrity, quotas | https://developer.android.com/google/play/integrity/setup | 10 000 jetons par jour par défaut |
| Supabase, tarifs | https://supabase.com/pricing | Gratuit 500 Mo, pause après 1 semaine ; Pro 25 $ |
| Cloudflare Workers | https://developers.cloudflare.com/workers/platform/pricing/ | 100 000 requêtes par jour gratuites ; 5 $ par mois au-delà ; D1 5 Go gratuits |
| Cloudflare R2 | https://developers.cloudflare.com/r2/pricing/ | 10 Go gratuits, sortie gratuite |
| PocketBase | https://pocketbase.io · https://github.com/pocketbase/pocketbase | MIT, v0.42.3, compatibilité non garantie avant 1.0 |
| Hugging Face, stockage | https://huggingface.co/docs/hub/storage-limits | Public gratuit « au mieux », 100 Go privés |
| GitHub Releases | https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases | Fichier < 2 Gio, pas de limite de bande passante |
| GitHub, gros fichiers | https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github | Dépôt < 1 Go conseillé, 100 Mio par fichier au plus |
| GitHub Pages | https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits | 1 Go, 100 Go de trafic par mois |
| GitHub Actions | https://docs.github.com/en/billing/concepts/product-billing/github-actions | Gratuit sans limite pour un dépôt public |
| Drive `appDataFolder` | https://developers.google.com/workspace/drive/api/guides/appdata | Caché, supprimable par l'utilisateur ; quota non précisé |
| Hetzner | https://www.hetzner.com/cloud/cost-optimized/ | CX23 5,99 € HT, indisponible ce jour-là |
| OVHcloud | https://www.ovhcloud.com/fr/vps/ | VPS-1 3,81 € HT (4,57 € TTC) |
| Play, sécurité des données | https://support.google.com/googleplay/android-developer/answer/10787469?hl=fr | Données pseudonymes à déclarer ; identifiants d'installation déclarés |
| Play, suppression de compte | https://support.google.com/googleplay/android-developer/answer/13327111?hl=fr | Obligatoire seulement s'il y a des comptes |
| CNIL, anonymisation | https://www.cnil.fr/fr/lanonymisation-de-donnees-personnelles | Une donnée pseudonymisée reste personnelle |
| CNIL, mineurs | https://www.cnil.fr/fr/recommandation-4-rechercher-le-consentement-dun-parent-pour-les-mineurs-de-moins-de-15-ans | Moins de 15 ans : consentement conjoint avec un parent |
| Library, brique Drive | `JRRocks/Library`, `moteurs/google-drive-appdata/README.md` (fiche du 2026-10-27) | Dépôt manuel éprouvé ; envoi périodique non vérifié ; SHA-1 release seul en dépôt public |
| OpenSkill | https://github.com/vivekjoshy/openskill.py | Classement multi-joueurs, licence MIT (non relu en détail ce jour) |

Non vérifiés : `appDataFolder` dans les 15 Go ; tarif Firestore au-delà de la gratuité ; prix de Hugging Face PRO.
