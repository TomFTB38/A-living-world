# A Living World

Prototype jouable de la forêt tempérée, en JavaScript et WebGL2, sans dépendance.
Il valide la direction artistique et l'architecture. La météo (clair, nuageux, pluie, brouillard) suit la seed et se règle dans le menu ; le son est synthétisé à la volée, sans fichier. Le moteur final est prévu en Rust avec wgpu
(voir le dossier fondateur).

## Lancer

Ouvrir `dist/index.html` dans Chrome, Edge ou Firefox, avec une carte graphique compatible WebGL2.

| Action | Commande |
| --- | --- |
| Se déplacer | ZQSD ou WASD |
| Regarder | Souris (clic dans le jeu pour capturer le pointeur) |
| Casser / poser | Clic gauche maintenu / clic droit |
| Sauter, remonter dans l'eau | Espace |
| Sprint / accroupi | Maj / Ctrl |
| Objet | 1 à 9, molette |
| Inventaire | E |
| Survie / créatif | G |
| Voler en créatif | F, ou Espace deux fois ; Ctrl pour descendre |
| Orientation du bloc | R |
| Caméra 1re / 3e personne | C |
| Avancer l'heure / figer le temps | T / Y |
| Profilage | F3 |
| Sauvegarde rapide | F5 |
| Menu | Échap |

## Développer

```sh
node tools/build.mjs            # assemble dist/index.html
npm test                        # tests de génération, maillage, sauvegarde
python3 tools/play_test.py      # bout en bout avec de vraies entrées (Playwright + Chrome)
python3 tools/creative_test.py  # inventaire et mode créatif
python3 tools/poses.py out.png  # planche des poses et des vues
python3 tools/capture.py out.png -q haut -r 8 -w 1672 -H 941   # capture réelle du moteur
python3 tools/imgstats.py out.png refs/ref6.png                # mesures objectives
```

## Organisation

| Dossier | Rôle |
| --- | --- |
| `src/core` | Hachage, bruits, mathématiques |
| `src/world` | Registre de blocs, génération, maillage, lumière voxel, stockage |
| `src/render` | Matériaux procéduraux, shaders, pipeline différé |
| `src/game` | Boucle, streaming, joueur, personnage, HUD, sauvegarde |
| `tests`, `tools` | Tests automatisés, build, captures |
| `refs` | Les sept références |

Le monde est une fonction pure de la seed, plus un delta des modifications du joueur.
Ajouter un bloc revient à ajouter une ligne dans `src/world/blocks.js`.
