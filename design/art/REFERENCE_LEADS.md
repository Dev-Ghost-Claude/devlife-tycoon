# Reference leads (2026-10-16)

Collected after the first voxel-desk renders of the old CRT computer looked flat and dull. Two scouts searched: one browsed art sites and looked at the images, the other researched technique. These are links and our own notes only: ideas, never copied assets.
Blocked while browsing: Reddit (browser pane), Pinterest (login wall), itch.io game listings (Cloudflare check).

## A. Props: the 1978 computer, TVs, desks, clutter

| Lead | Takeaway for us |
|---|---|
| [Apple II Computer, voxel art](https://sketchfab.com/3d-models/apple-ii-computer-voxel-art-ee8d760bdb0946c3806971665f217466) (Sketchfab) | Closest to our 1978 target: a CRT on the case, a long keyboard, a floppy drive at the side. Strong box silhouette, one blue screen, keys as a two-tone grid. |
| [Voxel Commodore 64](https://www.behance.net/gallery/83676833/Voxel-Commodore-64) (Behance) | Read the machine as three boxes (monitor, slab keyboard, drive). Cream wedge with dark-brown keys, a blue screen with a light-blue border, tiny green and red LEDs. |
| [Retro voxels](https://www.behance.net/gallery/130725651/Retro-voxels) (m4ndrill, Behance) | A beige CRT with a thick back hump, a glowing green screen, a keyboard in a different colour from the case, and a curly cable (the cheap detail that says "1978"). |
| [Objects](https://www.behance.net/gallery/98147187/Objects) (Joanna Ngai, Behance) | CRTs with faces on the screen. A two-colour screen plus one accent reads even at thumbnail size, and screens can carry the animal cast's faces. |
| [Dale the Computer](https://sketchfab.com/3d-models/dale-the-computer-095ac10d139f4b438a879419be9c986f), [Voxel Retro PC](https://sketchfab.com/3d-models/voxel-retro-pc-177a81e4f7574af3baa3da77a3449fdb) (Sketchfab) | Under four colours. The screen is a big dark rectangle with one bright icon. |
| [Voxel Technology Pack](https://zeppelin-games.itch.io/voxel-technology-pack) (Zeppelin Games, itch.io) | Cream case against a deep-set black screen and a dark bezel: that contrast carries the whole read. |
| [Game Consoles](https://voxelbytebot.itch.io/game-consoles) (Xiao Jing, itch.io) | Chunky one-voxel buttons. The same shape in six colourways gives cheap variants for upgrades. |
| [Voxel Universe](https://studiozooka.itch.io/voxel-universe) (Studio Zooka, itch.io) | A clutter checklist (floppy, mug, books, magazine, pizza box), each readable in about 8×8 voxels. |
| [Voxel bedroom](https://sketchfab.com/3d-models/voxel-bedroom-38d2a8d509b04d5f83f5487260da76ce) (Diego Lima, Sketchfab) | The best match for the teenager's room: a beige PC on a wooden desk, a lamp, a shelf with a CRT, a pinboard, a cat on the bed. Prop density and pastel clutter. |
| [Isometric voxel room furniture](https://pablogamedev.itch.io/isometric-voxel-room-furniture-png) (PabloGameDev, itch.io) | How many props fit in one small room. |

## B. Render and look

| Lead | Takeaway for us |
|---|---|
| [Sermstyle's bedroom music studio](https://sketchfab.com/3d-models/sermstyles-voxel-bedroom-music-studio-6e0cb1990b244450b2531b5a4ef12123) (G.West, Sketchfab) | The whole room in one hue (purple) at 3 or 4 values. Screen, neon and window blown out with wide bloom. The model for our night scene. |
| [Voxel Magic Pub](https://maria-starko.itch.io/voxel-magic-pub) (itch.io) | A warm lamp pool against a cool violet window. Shadows tinted purple-brown, never black. Strong corner shading. Wood tone varies per plank. |
| [Voxel Room](https://www.artstation.com/artwork/OdzwJ), [Voxel Bedroom](https://www.artstation.com/artwork/Zz2O0) (Alexander Ng, ArtStation) | The same room by day (white backdrop) and at dusk (salmon backdrop, window glow): time of day as a backdrop and light swap. Thick, light wall caps. |
| [One Room 2020](https://www.artstation.com/artwork/WKYdXN) (Yannick Castaing, ArtStation) | A dark scene with one accent hue and a thin light line on the cut walls. |
| [MagicaVoxel rooms on ArtStation](https://www.artstation.com/search?sort_by=relevance&query=magicavoxel%20room) | A good page for browsing more rooms. |

## C. Technique (for the voxel desk now, and Godot's look step later)

- **Light:** three terms in linear space instead of flat tints. A warm key light, a cool sky/ground hemisphere and a soft cool fill: `C = albedo × (hemi×ao + key×max(0,n·L)×mix(1,ao,0.5) + fill×max(0,n·Lf))`. Starting values are in the scout's notes below; tune by eye.
- **Shadows** lean blue or purple, highlights lean yellow or orange ([hue shifting](https://lospec.com/articles/pixel-art-outlines-part-2-using-color)). Never pure black.
- **Contact shadow:** march a few rays from top faces toward the light, soft over one or two voxels, tinted cool.
- **Outline ink** is a darker shade of the neighbouring colour, not black ([Lospec](https://lospec.com/articles/pixel-art-outlines-part-2-using-color)). Skip creases the corner shading already darkens.
- **Corner shading:** our ray-cast corner shading matches the classic method ([0fps](https://0fps.net/2013/07/03/ambient-occlusion-for-minecraft-like-worlds/)), and our darkest value of 0.62 fits.
- **Godot outline:** the [3D pixel-art outline shader](https://godotshaders.com/shader/3d-pixel-art-outline-highlight-post-processing-shader/) (CC0 / MIT) uses depth steps for dark lines and normal changes for light ones, which is our planned design. Its thresholds need retuning for an orthographic camera (1.5 voxels = 0.0625 m).
- **Godot shadows with an orthographic camera:** set the light's shadow mode to Orthogonal and set a max distance; raise the normal bias before the bias ([forum](https://forum.godotengine.org/t/3d-shadow-problem-in-othognal-mode/102064)). Start with SDFGI off.
- **Era palette** (Apple II lo-res, [source](https://meresh.com/a2colors.html)): #000000 #930B7C #1F35D3 #BB36FF #00760C #7E7E7E #07A8E0 #9DACFF #624C00 #F9561D #FF81EC #43C800 #DCCD16 #5DF784 #FFFFFF.

## Top ideas, ranked by the scouts

1. **The screen is the hero:** a big dark rectangle with one bright element, made emissive with bloom, spilling light onto the desk.
2. **One hue family per room** at 3 or 4 values plus one accent, with shadows tinted toward that hue.
3. **The 1978 computer as three boxes:** the case, a keyboard slab in a different colour, and a CRT or TV with a back hump. Add a curly cable and a cassette or floppy drive.
4. **A thick diorama frame:** light wall caps, a visible floor slab, soft contact shadows, corner shading strongest under furniture.
5. **Time-of-day backdrops:** pale by day, salmon at dusk, deep violet at night, with the geometry unchanged.
