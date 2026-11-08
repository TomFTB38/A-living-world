# A Living World

Prototype jouable de la forêt tempérée, en JavaScript et WebGL2, sans dépendance.
Il valide la direction artistique et l'architecture. La météo (clair, nuageux, pluie, brouillard) suit la seed et se règle dans le menu ; les bruitages sont de vraies prises de son libres (`assets/sounds/CREDITS.md`), la musique est composée à la volée. Le moteur final se construit en Rust avec wgpu dans `engine/` : `engine/core` reprend exactement le monde du prototype, sans dépendance ; `engine/game` ajoute la fenêtre et le rendu.

## Lancer

Ouvrir `dist/index.html` dans Chrome, Edge ou Firefox, avec une carte graphique compatible WebGL2.

| Action | Commande |
| --- | --- |
| Se déplacer | ZQSD ou WASD |
| Regarder | Souris (clic dans le jeu pour capturer le pointeur) |
| Casser / poser | Clic gauche maintenu / clic droit |
| Frapper un animal | Clic gauche en le visant |
| Ramasser, cueillir | Clic droit sur des branches, des cailloux, un buisson à baies |
| Manger / boire | Clic droit avec un aliment en main / les mains libres face à l'eau |
| Fabriquer | E, puis clic sur une recette (Maj + clic : autant que possible) |
| Réapparaître | Entrée ou le bouton de l'écran de mort |
| Sauter, remonter dans l'eau | Espace |
| Sprint / accroupi | Maj / Ctrl |
| Objet | 1 à 9, molette |
| Inventaire | E |
| Survie / créatif | G |
| Voler en créatif | F, ou Espace deux fois ; Ctrl pour descendre |
| Orientation du bloc (regard, debout, sens du regard, en travers) | R |
| Caméra 1re / 3e personne | C |
| Avancer l'heure / figer le temps | T / Y |
| Profilage | F3 |
| Sauvegarde rapide | F5 |
| Menu | Échap |

## Survie (moteur final)

On commence les mains vides. Au sol traînent des branches et des cailloux, qui se ramassent d'un clic droit (une branche, un bâton ; un caillou, un caillou) ; l'herbe arrachée donne parfois de la fibre et des graines (toujours de la fibre au couteau). Branches, cailloux et fibre font, à la main, le couteau, la hache et la pioche de pierre ; la hache donne le bois, le bois les planches et l'établi, où se font les outils d'acier.
Les blocs résistent : la terre cède en deux secondes, le bois en sept à mains nues, la pierre en trois à la pioche de pierre et ne donne rien sans pioche. Des fissures montrent où en est le bloc. Les myrtilliers et les sorbiers se cueillent d'un clic droit et retrouvent leurs baies deux jours plus tard.
Quatre jauges : vie, faim, soif, endurance, et le souffle sous l'eau. Courir, sauter, rouler et chaque coup (dans le vide, un bloc ou un animal) coûtent de l'endurance ; à bout de souffle, on marche et on frappe moins fort. La faim et la soif baissent avec l'effort (la soif deux fois plus vite au désert) ; à zéro, elles blessent. Bien nourri, on guérit.
Ce qui blesse : les chutes de plus de trois blocs et demi, la noyade, les épines de cactus, les flammes, la faim, la soif, la ruade d'un cerf. À la mort, tout l'inventaire reste dans un sac, là où l'on est tombé ; on réapparaît au point de départ et on le reprend en marchant dessus.
On chasse cerfs, renards et lièvres (viande crue, cuir) et on boit dans l'eau visée. La viande crue rend malade : elle se cuit près d'un feu de camp. Les recettes sont dans `engine/core/src/survival.rs`.

## Développer

```sh
node tools/build.mjs            # assemble dist/index.html, sons compris
npm test                        # tests de génération, maillage, sauvegarde
python3 tools/play_test.py      # bout en bout avec de vraies entrées (Playwright + Chrome)
python3 tools/creative_test.py  # inventaire et mode créatif
python3 tools/audio_test.py      # rend chaque son et mesure le signal
python3 tools/poses.py out.png  # planche des poses et des vues
python3 tools/capture.py out.png -q haut -r 8 -w 1672 -H 941   # capture réelle du moteur
python3 tools/imgstats.py out.png refs/ref6.png                # mesures objectives
```

## Moteur final

```sh
cd engine/core && cargo test --release        # parité exacte avec le prototype, chargement, physique
node engine/tools/parity.mjs                  # régénère les empreintes depuis le prototype
python3 engine/tools/wgsl_test.py out.png "[x,y,z]" "[x,y,z]"   # rend la vraie scène avec WebGPU dans Chrome
```

GitHub compile `engine/game` sous Windows à chaque envoi et publie le programme dans la version `moteur-dev`.

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
