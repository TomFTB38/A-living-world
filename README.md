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
| Tourner le bloc à poser | R (un quart de tour ; pour les troncs : mode d'orientation) |
| Ouvrir une porte, une trappe | Clic droit (accroupi : poser contre elle) |
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

**Le cycle.** On part sans rien. Branches et cailloux se ramassent d'un clic droit, l'herbe donne de la fibre, le gravier du silex. À la main : couteau, hache, pioche, pelle, marteau et lance de pierre, corde, torche, bandage, feu de camp, sac de couchage. La hache abat l'arbre (les rondins vont dans le sac), le bois donne les planches et l'établi ; à l'établi, outils de silex, coffre, lit, gourde ; puis four (lingots de cuivre, d'étain, de bronze, de fer), enclume (outils et armures de métal) et banc de taille (pierre taillée). Jusqu'au fer, tout se fait seul ; l'acier et les pièces de maître demandent un artisan de village (forgeron, armurier, tanneur…), qui peut transmettre sa technique.

**Le corps.** Vie, faim, soif, endurance, chaleur ressentie (biome, altitude, heure, météo, vêtements, feu, abri, eau) et fatigue. Pas de jauges : six icônes (cœur, estomac, goutte, poumons, thermomètre, œil) remplies exactement à la valeur du besoin, qui apparaissent quand il change et restent quand il devient dangereux ; Alt les montre toutes. Mode immersif, dynamique ou permanent, animations réduites : menu Échap. Dormir (lit, sac de couchage) fait passer la nuit, soigne et fixe le point de réapparition.

**Le reste.** Usure des outils, charge portée (surchargé, on ralentit), vêtements et armures sur cinq emplacements, coffres de 27 cases, feux qui consomment leur combustible, eau à faire bouillir, cultures de blé, chasse (le couteau dépèce mieux), mort avec dépouille qui garde tout pendant plusieurs jours de jeu. Trois profils : Exploration, Survie standard, Survie exigeante.

**Monde vivant.** Des loups en meute dans la taïga, et la nuit hors des forêts : ils flairent le joueur (plus loin la nuit), le poursuivent et mordent ; blessés, ils fuient. L'armure amortit les morsures. Les sangliers des prairies broutent sans s'occuper de vous, mais chargent celui qui les frappe. L'arc tire de vraies flèches (vitesse, gravité) qui abattent le gibier de loin ou se plantent dans le décor, d'où on les ramasse en marchant dessus. La canne à pêche se lance vers l'eau ; quand ça mord, il faut ferrer vite (clic droit) : truite en eau courante, perche en eau dormante, omble dans le froid du nord et des hauteurs. Carottes et pommes de terre poussent à l'état sauvage dans les prairies et se sèment comme le blé ; ragoût, poisson grillé, pommes de terre cuites.

**Construire.** Dalles de bois, de pierre taillée et de pavé (en haut ou en bas selon l'endroit visé ; deux font un bloc), escaliers (dans le sens du regard ; ils se montent sans sauter, comme les dalles), pentes de chaume pour les toits, porte sur deux blocs et trappe (clic droit pour ouvrir), échelle (on grimpe en avançant, on redescend accroupi), fenêtre à carreaux de verre (sable cuit au four), poteaux, colonnes et murs fins. R tourne d'un quart de tour avant de poser ; l'aperçu prend la forme exacte du bloc, vert si la pose est possible, rouge sinon. Les collisions suivent la géométrie dessinée (`engine/core/src/shapes.rs`, une seule source pour le dessin, les collisions et l'aperçu).

**Villages.** Des bourgs générés selon le biome (prairie, taïga, steppe, désert, montagne, hautes terres), avec leurs artisans : commande, amélioration, réparation, transmission ; réputation, rumeurs, journal (J). Les villageois vivent leur journée, ouvrent les portes, saluent, reculent devant une arme tirée et échangent des marchandises (`echanges.txt`).

**Carte.** Mini-carte en boussole en haut à droite, grande carte (M) qui se déplie sur ce qu'on a exploré.

**Combat.** Clic pour frapper, tenu pour un coup lourd ; bouclier en main gauche, clic droit tenu pour la garde, parade au bon moment. Ours en forêt profonde et taïga, bouquetins dans les hauteurs, lézards au désert. Les habits et le bouclier se voient sur le personnage.

**Parcours.** Écran titre, plusieurs mondes (dossier `mondes/`), profil choisi à la création, premiers pas guidés, commandes réglables (deux touches par action, ZQSD et WASD), sensibilité et champ de vision.

**Les données.** Objets (201), recettes (120) et métiers sont dans `engine/core/data/` (`objets.txt`, `recettes.txt`, `artisans.txt`) : on ajoute une ligne, pas de code ; une erreur de saisie fait échouer les tests. Le livre de recettes (E) a ses catégories, une recherche, des favoris (clic droit) et ne montre que ce qu'on a découvert.

**Le scénario de validation** du cahier des charges (ramasser, premier outil, abattre un arbre, établi, meilleure hache, abri, feu, eau, repas, coffre, rechargement à l'identique) est joué de bout en bout par `engine/core/tests/scenario.rs`, avec les vraies actions du joueur.

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
