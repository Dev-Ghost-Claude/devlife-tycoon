# DevLife Tycoon: world and interior art direction

Date: 2026-10-12. Status: first full proposal. It starts the talk. The designer and the engineer will change it.

Sources read: `design/GAME_DESIGN.md`, `design/ui/UI_PROPOSAL.md` (for the 16 UI colours), `src/world.rs`, `src/animal.rs`, `src/voxel.rs`, `src/pixel_camera.rs`, `src/clock.rs`, `src/gameplay.rs`, `assets/maps/street.txt`, `assets/maps/computer_shop.txt`, `assets/models/crt_computer.txt`, `art/clawd_pixel.lua`, and the five reference images (1 diorama map, 2 and 3 city block, 4 apartment cutaway, 5 detail target).

Reference rule: we take principles only. We never copy a building, a composition, a sign text or a character from any reference. Sign names in this file are invented placeholders.

---

## 0. The direction in twelve lines

1. **Detail target: reference 5.** Every pixel is a decision. Characters are 35 px tall with shaded faces. Floors have wear. Shelves have labels. Glass looks like glass.
2. **Same scale as ours.** Reference 5 has a 35 px adult. At our 24 px per metre, a 1.7 m adult is 35 px. So the scale is right. Only the number of decisions per pixel is too low.
3. **Today's voxel is too coarse for things you look at.** 1 voxel = 1/8 m = about 2 x 1 px on the ground and 2.6 px tall. A reference-5 pixel is about 1/24 m.
4. **Recommendation (section 2): three resolutions plus a look pass.** Coarse 1/8 m voxels for structure. Fine 1/24 m voxels for animals, props and facade parts. Painted "plates" (bitmaps made in Aseprite or as ASCII) for signs, posters and floor wear. A look pass adds outlines, a four-step light ramp, baked corner shadows and a palette lock.
5. **We do not draw direction sprites for characters** and we do not build a sprite-stack renderer. Reasons are in section 2.
6. **Gate first.** A short spike and a model preview tool come before any bulk art. If fine voxels fail the spike, we fall back to 1/16 m, then to coarse plus plates plus the look pass.
7. **The street is dense but calm.** About 60 percent of the reference city density, 35 percent of ground left quiet, 70 percent muted colour and 30 percent accents.
8. **Every building has a silhouette code, a colour code and a sign shape** so the player finds the computer shop, the café, the arcade, the bank, the municipality, the school and home at a glance.
9. **Building kit (section 4):** base types, facade modules, storefront modules with sign slots, roof modules and corner pieces, all with voxel sizes.
10. **Rooms (section 5):** a cutaway with two back walls, a dark floor plinth, split levels with stairs, 45 to 55 percent of tiles filled, and a story told by props. Two full ASCII layouts: the 1978 bedroom and the 1978 computer shop.
11. **Palette (section 6):** the bedroom era has 60 on-screen colours (32 world, 12 cast, 16 shade tones). It grows each era. A palette lock enforces it.
12. **Build order (section 7):** spike, look pass, street skeleton, storefront kit, fine animals, rooms, split levels, night life, era palette, polish.

### Units used in this file

| Name | Size | Meaning |
|---|---|---|
| tile | 1 m | one character of a map |
| **v** | 1/8 m, 8 per tile | the coarse voxel (today's `VOXEL`) |
| **f** | 1/24 m, 24 per tile | the fine voxel. 1 v = 3 f. |
| px | 1 canvas pixel | at zoom 1 there are 24 px per metre |

Rules for reading sizes:

- **All sizes are in v unless marked f.** A fine item has the same physical size. Its grid is just 3 times denser per axis (multiply by 3).
- If the spike fails and the project stays at coarse only, every size in this file still works. Only the amount of detail inside drops.
- On screen at zoom 1: a tile is a diamond 34 x 17 px. 1 m of height is 20.8 px. 1 v moves 2.1 px sideways and 1.1 px down along a ground axis, and is 2.6 px tall. 1 f is 0.71 x 0.35 px on the ground and 0.87 px tall.
- Useful heights on screen: character (1.4 m) 29 px, door (2.25 m) 47 px, one floor (3 m) 62 px, lamp post (3.25 m) 68 px.
- An object of height h metres hides about 1.2 x h tiles of ground diagonally behind it.
- The camera has four resting views (yaw 45, 135, 225, 315 degrees). Q and E turn a quarter at a time. Every face of every building can be seen.

---

## 1. What makes each reference work

### 1.1 References 2 and 3: the dense city block

Measured by eye. The numbers are estimates.

**Density.**
- Almost no wall is blank. The longest plain stretch is about 4 windows wide. Every window has a sill, a balcony or a blind.
- The picture is about 35 percent ground, 60 percent buildings, 5 percent greenery.
- Roads are the only quiet area. The eye rests there. Everything else is busy.
- There are about 25 pedestrians, 4 vehicles, 8 trees, 6 lamps, 5 traffic lights, 10 cones, 8 bollards and 3 bins in one view.

**Silhouette variety.**
- No two neighbouring roofs match. We see a crenellated top, a conical turret, a red pitched roof with a dormer, flat roofs with clutter, a glass atrium roof, a stepped terrace.
- Heights differ by 3 to 4 times, from a 2-floor bar to a 10-floor tower.
- Edges are broken everywhere: bay windows, balconies on every window of one building, a chamfered corner entrance, stone steps, rooftop huts, awnings.
- The tallest or most detailed building sits on the corner where the roads meet. Everything else steps down away from it. This is the focal point.

**Signage.** There are six kinds of sign, each used once or twice:
1. a huge mural on a blind side wall (about 8 percent of the picture),
2. lettering standing on the roof edge,
3. a vertical blade sign hung from the wall,
4. a lit fascia board above the shopfront,
5. striped awnings with a name on the valance,
6. posters in glass cases, window decals and small door plaques.

Signs are the most saturated and most contrasting things in the scene. They sit on dark panels. Names are 5 to 8 letters. Colour and shape tell you the kind of shop before you read a word.

**Rooftop life.** Each roof is a tiny scene with 3 to 5 items and one story: a sofa and a fire pit (a hangout), a shed with leaning boards (someone's workshop), a radio mast with dishes, air-conditioning boxes and pipes, a flag, a pool and a parasol table, ivy on the walls. A roof is a place, not a lid.

**Street life.**
- Pedestrians are tiny (about 5 percent of the picture height) in many clothing colours. Some walk alone, some in a group. One group of about 10 walks in a line, which tells a story.
- Four different vehicles: a hatchback, a round old car, a pink bus, a sports car. A cyclist and a dog walker show scale.
- Hard crisp shadows under every person, car and tree. Shadows run to the lower right.
- Bunting across the street, traffic cones in clusters, bollards in rows, a post box, a litter bin: the small things that make a street feel used.

**Layering.**
- Foreground: a low strip (mini-golf) with small bright detail.
- Midground: the hero corner building, full detail.
- Background: taller buildings, lighter in value and lower in contrast. They fade toward the sky.
- Each layer overlaps the next. There is no gap of empty space between layers.

**Light and colour.**
- About 70 percent of the picture is muted warm grey and tan (stone, brick, asphalt). About 30 percent is accents: red roofs, teal and green window glass, a pink bus, yellow road lines, orange cones.
- Light comes from the upper left. Roofs and tops are brightest, left faces middle, right faces darkest. Cast shadows are cool blue-violet.
- Each window is tinted a little differently (teal, amber, green, dark). This cheap randomness reads as "people live here".
- Greenery is a clean dark green with a lighter top.

**Ground detail.** Kerbs have a light top and dark face. Yellow lines run along the kerb. A yellow hatched junction box, a zebra crossing, road arrows, slab joints, manhole covers, drains and tree grates break up the ground.

**What we must not take:** the buildings themselves, the sign names and wording, the cast of characters and the composition of the corner.

### 1.2 Reference 4: the apartment cutaway

- **Cutaway.** Two back walls stand full height. Their top edge is a light strip (the wall cap). The front is open. The floor has a thick dark edge, so the room looks like an object on a plinth. The background is flat dark.
- **Footprint is not a box.** It is an L. The bathroom floor is one step lower, in grey tile, while the rest is wood. A doormat tab sticks out at the entrance. The kitchen sits in an alcove. Zones are marked by floor material and level changes.
- **Layering front to back.** The tallest things (wardrobe, bookcase) are at the back walls. Middle height things (desk, sofa, table) are in the middle. The front is low: bathroom fixtures, doormat, shoes. Nothing blocks anything important.
- **Lived-in.** About 45 objects are visible. Clutter: a vacuum cleaner, a pair of shoes, boxes on top of the wardrobe, a backpack leaning on the sofa, three mugs, a laptop, bread on the table, two plants, 15 books, two framed pictures, towels in the bath. Each is placed beside furniture, not in the walking lane.
- **Light.** The window is the key light. Bright sky shows in the glass. Orange wood and peach walls are warm. One cool blue accent (rug, monitor, vacuum, jacket) balances them. Furniture has a soft darker shadow on the floor, about 1 to 2 px wide.
- **Floor.** Wood in two tones with plank lines. Contrast is low, so it does not fight the furniture.
- **Story.** You can guess who lives here (a gamer or programmer who cooks and hikes) from the objects alone.

### 1.3 Reference 5: the detail target

This is the bar we aim at. Principles only.

- **Scale.** The image is about 300 x 230 native pixels, scaled up 2.4 times. A character is 35 native px tall. At 24 px per metre that is 1.7 m. So its world scale is ours.
- **Characters.** 35 px tall. Hair has strands. Glasses are 2 px. Faces have 2 to 3 shade tones. Coats have fold lines. Shoes and trousers are separate colours. Each character has a 3 to 4 tone ramp per material.
- **Selective outlines.** Each object has a 1 px outline in a darker version of its own colour, not black. Crates have a warm dark brown outline. Glass has a light cyan rim.
- **Floors and walls have texture.** The concrete floor has irregular lighter wear streaks, a brown stain, hairline cracks and slab joints. Walls have panels with seams. No surface is one flat colour.
- **Glass language.** A display case is a light cyan edge line, a darker inside, and tiny items (3 to 6 px) inside. No transparency is needed.
- **Dense, labelled shelving.** Cabinets of small drawers, each with a yellow tag. Pegboards with tools. Items have clear identity at 3 to 6 px: each is a shape and 2 colours.
- **Multi-level floors.** A raised platform about 1 m high with 5 steps, handrails with posts (orange metal), a thick dark front face on each level.
- **Warm and cool light.** Red carpet and orange walls (warm) against grey-green metal and cyan glass (cool). A hanging lamp throws a lighter orange pool with a soft cone onto the carpet.
- **Props as storytelling.** Crates with a symbol, a bottle on the floor, a sign with a tiny logo, a hand-lettered price strip.
- **Cutaway.** Dark void around the room. A light cap line on the cut wall edges.

What this needs that we do not have yet: finer shapes, hand-shaded ramps, outlines, textured surfaces, glass rims, labelled detail, stairs and rails, painted light pools. Section 2 handles each.

### 1.4 Reference 1: the diorama map

Context only. Already decided and written in `GAME_DESIGN.md`: one strong diagonal, big landmarks, quiet ground, a cut model base, no blur. This file keeps one idea from it: **quiet ground makes detail readable**. We use it again in rooms (the plinth) and in streets (35 percent of ground without props).

### 1.5 What the three city and room references share (our six rules)

1. **Contrast of busy and quiet.** Dense detail next to a calm area.
2. **Muted body, bright accents.** About 70/30.
3. **Silhouette first.** You can tell what a thing is from its outline filled with one colour.
4. **Tall at the back, low at the front.** In every picture.
5. **Every object has a small story or purpose.** Nothing is only decoration.
6. **Light has a direction and a temperature.** Warm light, cool shadow.

---
## 2. Reaching this detail level

The designer's words: reference 5 is "the level of detail I want in the game with respect to items, animals, world design". This section is honest about our tech and picks a path.

### 2.1 Where we stand, in numbers

All numbers are for our camera: orthographic, 30 degrees pitch, 45 degrees yaw, 24 px per metre at zoom 1.

| Voxel size | Ground step (right x down) | Height | One wall face covers | Faces per pixel | Adult animal (1.4 m) |
|---|---|---|---|---|---|
| **1/8 m (today)** | 2.1 x 1.1 px | 2.6 px | 5.5 px² | 0.18 | 11 rows |
| 1/12 m | 1.4 x 0.7 px | 1.7 px | 2.5 px² | 0.41 | 17 rows |
| 1/16 m | 1.1 x 0.5 px | 1.3 px | 1.4 px² | 0.73 | 22 rows |
| **1/24 m** | 0.7 x 0.35 px | 0.87 px | 0.6 px² | 1.6 | 34 rows |
| 1/32 m | 0.5 x 0.27 px | 0.65 px | 0.3 px² | 2.9 | 45 rows |

The adult animal is about 29 px tall on screen in every row. Only the number of rows inside those 29 px changes.

What this tells us:

1. **The scale is right.** A 1.7 m adult is 35 px at 24 px per metre. That is reference 5's character height. We keep 24 px per metre.
2. **We make 1 colour decision where the reference makes 5 or 6.** Today a wall voxel face covers 5.5 px². In the reference every pixel is its own decision. The gap is 2.3 times in each direction.
3. **Today's 1/8 m voxel is almost exactly the classic 2:1 pixel step** (2 px right, 1 px down). That is why our edges look clean. Going finer is not free: colour boundaries then fall between pixels and thin features alias. This is the main price of fine voxels.
4. **1/16 m is one voxel face per pixel. 1/24 m is 1.6 faces per pixel.** At 1/24 m the screen cannot show every voxel. The renderer picks the front voxel at each pixel centre, and 1-voxel features can drop out. So at 1/24 m the real minimum feature is 2 voxels (about 1.4 to 1.7 px).
5. **Why still prefer 1/24 m.** It gives 34 rows for a 29 px animal (no vertical detail is lost, 1/16 m would lose a quarter). Curves (heads, ears, tails, bottles) look smoother. And 1 v = exactly 3 f, so coarse and fine pieces share one lattice, and fine models reduce to coarse ones by a clean 3 x 3 x 3 majority vote.

### 2.2 What a 3D voxel render does well and badly next to hand-placed pixels

**It does well (the reference cannot do these):**
- Turns to four views (Q and E) with no extra drawing.
- Real sun and lamp light, real shadows, day and night.
- Species, outfits and sizes combine freely (a fox in a blue coat, a bear in a red one) with no new drawing.
- Furniture can be placed and turned by the player.
- Stairs and split levels are real shapes.

**It does badly (the reference does these by hand):**
- Clean 1 px lines on curves and along diagonals.
- Pixel clusters that anti-alias an edge by choice.
- Floor wear that follows the screen grid. A texture on a 3D floor makes diamond-shaped noise, not square pixel clusters.
- Expressive animation frames (squash, smear).
- Painted light pools that follow the artist's eye. We use real lights instead.

**Honest expectation (judgement, not measured):** with the plan below we reach about 85 percent of reference 5 for animals and props, about 75 percent for walls and floors (with plates), and about 65 percent for the street at a distance. The missing part is hand-placed micro-detail. We cover this gap with outlines, careful palettes and painted plates.

### 2.3 The options

Ratings: good, OK, poor.

| Option | Look | Q/E turn | Room zoom | Split levels, stairs | Light, shadow | Cost for Claude to author | Speed and memory |
|---|---|---|---|---|---|---|---|
| **A. All-fine voxels (1/24 m everywhere)** | good for things. Adds nothing to flat ground and walls. | good (it is 3D) | OK | good | good, shadow map must grow | high: 27 times more cells per shape | **poor**: ground 35 MB, one building 130 MB, slow meshing |
| **B1. Aseprite slices to voxels** (authoring format only) | same as A | good | OK | good | good | OK for flat things (rugs, shelves, signs). Awkward for faces. Gives the designer an editable file | same as A |
| **B2. Sprite-stack renderer** (stacked screen quads) | stacked look, banded sides | good (rotates freely, its one real merit) | OK | poor | poor: baked light, fake shadow | high: a new renderer | OK |
| **C. Hand-drawn 8-direction sprites for animals and items** | best crispness | 8 facings cover every view, but turning looks stepped | good | poor: depth sorting against 3D floors | poor: baked, blob shadows, no sun | **very high**: 8 facings x ~14 frames = 112 frames per cast type. 4 species now, 20 later | very good |
| **C2. Hand-drawn sprites for room props only** (one fixed camera) | very good in rooms | n/a (rooms are fixed) | good | OK | OK with palette swaps | about 60 props x 4 turns = 240 sprites | very good |
| **D. Recommended mix** (section 2.4) | about 75 to 85 percent | good | good | good | good, dynamic | medium, with tools | good |

**A, all-fine voxels.**
- Pros: one system, everything lights and rotates correctly.
- Cons: `VoxelGrid` stores `Option<Color>` (about 20 bytes a cell). The street ground at 1/24 m is about 1.7 million cells (35 MB). One 8 x 6 tile, 3 floor building is about 6.6 million cells (130 MB). The naive mesher walks every cell. Ground and walls are flat, so extra voxels buy almost nothing there.
- Verdict: **no for structure, yes for things.**

**B1, slices.**
- The text model format already works in horizontal layers (`layer` blocks in `crt_computer.txt`). Aseprite is installed and `art/clawd_pixel.lua` already drives it in batch mode. A slice model is just a voxel model.
- Good for: rugs, floor tiles, shelf levels, table tops, book stacks, signs lying flat.
- Bad for: faces, clothes, anything where the detail is on a vertical surface. Thinking in slices for a face is slow and error-prone.
- Verdict: **use Aseprite as the editor and the preview for 2D parts (bitmaps), not for sculpting.**

**B2, sprite-stack renderer.**
- The merit is smooth rotation at any angle. We do not need that: Q and E give 4 resting views.
- It breaks the real sun and shadows, depth against 3D walls and floors, and split levels. It is a second renderer to maintain.
- Verdict: **no.**

**C, direction sprites for characters.**
- Best crispness, but the cost explodes. On screen an animal can face 8 directions, so every pose needs 8 facings (the camera's quarter turns only change which facing is used). About 14 frames per facing (idle 2, walk 6, sit 2, three actions). 112 frames per cast type, times species (4 now, 20 or more later), times body sizes (a mouse and a giraffe). Outfits can be palette swaps but hats and coats cannot.
- Light is baked, so the sun cannot shade them. Shadows are blobs. Facings snap in 45 degree steps, so turning looks stepped.
- Claude cannot hand-place that many good pixels by prompting. A generator would only assemble parts, and the parts are the real art.
- The same cost problem applies to items. Even the 4-view version (one sprite per resting view, so about 25 street props x 4 = 100 sprites) pops at quarter turns, casts no real shadow and cannot be lit by the sun. A fine voxel prop costs about the same to make and turns for free.
- Verdict: **no.**

**C2, sprite props in rooms.**
- Rooms are the one place with a fixed camera. Props there could be hand-painted sprites.
- Cost: two art pipelines, and the same bench or desk would exist twice (street and room).
- Verdict: **keep as an escape hatch.** Use it only if fine-voxel furniture fails the quality bar in the spike.

**D, the mix we recommend.** Next section.

### 2.4 Recommendation: three resolutions plus a look pass

**Three resolutions.**

| Layer | Grid | What is made at this layer | Why |
|---|---|---|---|
| **R1 Structure** | coarse, 1/8 m | street ground, building shells, roofs, room walls and floors, tree crowns | big and flat. Memory and speed. |
| **R2 Things** | **fine, 1/24 m** | animals, furniture and props, vehicles, street furniture, facade parts (windows, doors, shopfronts, balconies, awnings, drainpipes, aerials, signs' frames) | these are looked at. Small, many copies, so instancing makes them cheap. |
| **R3 Paint** | bitmaps at 24 px per metre, one voxel deep | signs, posters, price cards, shop window displays, cabinet fronts, wall murals, floor wear and stains, rugs, faces | hand-placed pixels. Made in Aseprite or as ASCII text. Loaded as a thin layer of fine voxels. |

Anything not named stays coarse. Particles (smoke, steam, rain, sparks) and UI use plain 2D sprites.

**The look pass (render side).**
1. **Selective outline.** One pixel, in a darker shade of the object's own colour (reference 5's crates), drawn at canvas size from an object-ID and normal edge pass. Glass gets a light rim instead. This is the single biggest jump for the effort.
2. **Palette lock.** Voxels store palette indices, not RGB. A shader maps index plus a light step (lit, side, shade, shadow) to a colour from the era's palette. The palette has four time keyframes (dawn, noon, dusk, night). This is how the palette grows with the eras (section 6) and how the world turns cold when the storyteller does.
3. **Baked corner shadows.** The authoring code puts a darker voxel line where two surfaces meet (wall base, under an eave, inside a window reveal). Cheap, and it is how hand-pixel art gets soft shadows in corners.
4. **Shadow setup.** Bigger sun shadow map, shorter range, tuned bias.
5. **Snap moving things.** A walking animal's drawn position snaps to whole canvas pixels along the screen axes (the trick the camera already uses). Without this, fine detail swims.

**What we do not do:** no sprite stacking, no direction sprites for animals, no 1/32 m voxels, no dithering in the world.

### 2.5 Pixel-art rules for fine voxels

1. **Minimum feature: 2 f in at least one direction** (about 1.4 to 1.7 px). A 1 f accent (eye glint, button) is allowed only if losing it is harmless.
2. **Colour regions are at least 2 x 2 f** on any face. No single-voxel speckle at fine scale except in marked "noise" areas.
3. **Every item is read at 29 px first.** Judge it at the default zoom, not at a 4 times upscale.
4. **Silhouette test.** Fill the model with one colour. Can you still tell what it is from all four views? If not, fix the shape before the colour.
5. **Ramps of 3.** Each material has base, shade and a light accent. Add a fourth only for skin, glass or metal.
6. **No lines along a ground diagonal that are 1 f wide.** They break into dashes. Make them 2 f.
7. **Glass is opaque.** A 2 f light rim, a darker inside, the contents visible. No alpha.
8. **Outline is a render job.** Do not fake outlines with voxels.

### 2.6 The spike: the gate before bulk art

Three tests in the real renderer. About 2 to 3 days of engineer time. The result decides the fine unit and goes into this file.

**Spike 1: one animal.**
- A fox built from one description at three sizes: 11 rows (1/8 m), 22 rows (1/16 m), 34 rows (1/24 m). The face is painted from a bitmap (see 2.8).
- Render at 24 px per metre, in all four views, standing and walking, with and without position snapping. Save each at 1 times and 4 times.
- **Pass:** (a) eyes, nose and mouth read at 1 times in all four views; (b) the outline hugs the silhouette with no noise; (c) walking does not flicker 2 f features once snapping is on; (d) no shadow acne on the body; (e) 15 animals cost 2 ms or less per frame.

**Spike 2: one prop and one module.**
- A fine desk with a typewriter, a lamp and a mug. A fine window module on a coarse wall. A bitmap sign plate (a 28 x 8 bitmap).
- Render at 24 and 48 px per metre.
- **Pass:** the window has visible relief and a recess from all four views. The sign reads by shape and colour at 24 px per metre. The desk's items are named by a stranger shown a screenshot.

**Spike 3: the look pass on today's street.**
- Outline plus baked corner shadows plus the palette lock with two palettes (day, night).
- **Pass:** the street looks clearly richer with no new models. The designer says so.

**Decision rule.** Take the smallest fine unit that passes. If 1/24 m shows noise, try 1/16 m (the sections below still hold: only multiply by 2 instead of 3). If neither passes, stay coarse and rely on plates, outline, palette and kit (about 70 percent of the plan survives).

### 2.7 What the engineer must build to support this (in order of need)

| # | Piece | Size | Why |
|---|---|---|---|
| 1 | **Model preview tool.** `cargo run -- --preview <model>` renders a turntable (4 yaws, day and night) at 1 times and 4 times to PNG. Reuse `capture.rs`. | M | Claude cannot judge pixel art from code. It must look at renders. |
| 2 | **Palette-indexed grids.** `VoxelGrid` stores a `u8` index and a palette per model. | S to M | 20 times less memory. Needed for the palette lock. |
| 3 | **Model format v2.** Header with `grid coarse` or `grid fine`. Keep `palette` and `layer`. Add `plate` (a bitmap on a face) and `view front/side/back/top` (a painted view that carves and colours the model). Plus an Aseprite Lua exporter for the same data. | M | The authoring path in 2.8. |
| 4 | **Instanced facade modules.** A building is a coarse shell plus placed module meshes that share handles. | M | Section 4. |
| 5 | **Look pass shaders** (outline, palette lock, glass rim). | M to L | Section 2.4. |
| 6 | **Snap moving things** to whole canvas pixels. | S | Stops swimming. |
| 7 | **Shadow tuning.** Shadow map 4096, range about 40 m, bias. | S | Fine voxels alias in shadows. |
| 8 | **Auto LOD.** Reduce a fine grid to coarse by majority colour over each 3 x 3 x 3 block. Use at far zoom. | S | Free. |
| 9 | **Height map and stairs** for rooms and streets (per-tile level, feet follow it, 6 v per level). | M | Section 5.3. |
| 10 | **Fade for occluding buildings** (dither the front building to about 35 percent when the player is behind it). | M | Section 3.3. |
| 11 | **Greedy mesher** for large flat coarse areas. | S, optional | Fewer triangles. |

### 2.8 How Claude authors fine models

1. Write a generator in Rust (like `animal.rs`) or Lua (like `art/clawd_pixel.lua`). Helpers: `box`, `ellipsoid`, `mirror_x`, `scatter(seed)`, `wear(seed)`, `paint_view(front, rows)`, `plate(face, rows)`.
2. Draw faces and fronts as **ASCII bitmaps**. A letter is a colour. This is how Claude draws pixel art well: small bitmaps, 8 to 32 px wide.
3. Render with the preview tool. Read the PNG. Judge at 1 times.
4. Repeat until it passes three checks: silhouette (one colour), palette (only listed swatches), four views.
5. Save the generator and the text model in `art/`. Keep an `.aseprite` file for any bitmap so the designer can open and edit it.

Example: a fox face painted on the front of the head. 11 x 8 f. `.` means keep the carved colour.

```
# fox face, front view. F fur, L cream, D ink, W glint, N nose
.FFFFFFFFF.
FFFFFFFFFFF
FFDDFFFDDFF
FFDWFFFDWFF
FFFFFFFFFFF
LLFFFFFFFLL
.LLLLNLLLL.
..LLLLLLL..
```

Run Aseprite the way the existing script does: `/Applications/Aseprite.app/Contents/MacOS/aseprite -b --script art/<name>.lua --script-param out=<file>`.

### 2.9 What changes for zoom and room framing

- Whole-pixel zoom was chosen for 1/8 m voxels (4, 3, 2 px). Fine voxels are 1 px edge only at 24 px per metre.
- **New street zoom steps:** 16 px per metre (far, coarse LOD only), **24 (default)**, 48 (close-up, fine voxels are 2 px). The 32 step goes away.
- **Room zoom.** Pick per room. Rooms with width + depth of 12 tiles or less use 48 px per metre. Rooms up to 30 use 24. A 6 x 5 bedroom at 48 px per metre is 374 x 312 px. A 12 x 8 shop at 24 is 340 x 232 px.
- Characters are 29 px tall at 24 and 58 px at 48. That is fine: the camera is closer.
- If the project stays coarse-only, use zoom 1.5 (4 px per voxel) for small rooms.

### 2.10 Cost and performance budget (estimates, not measured)

| Item | Count | Effort for Claude (S = a short generator and 2 to 3 previews, M = about a day, L = several days) |
|---|---|---|
| Animal body plan (fine) | 1 | L |
| Animal heads, tails, ears, per species | 4 now | M each |
| Outfits (shirt, pants, coat, hat) | 12 | S each |
| Furniture and props | about 60 for era 1 | S each |
| Facade modules | about 30 | S to M each |
| Shopfronts with signs | 7 for era 1 | M each |
| Street furniture and vehicles | about 25 | S each |
| Plates (signs, posters, floor wear) | about 80 | S each |
| Rooms | 2 for now | L each |

**Frame budget (full street view, day).** About 500,000 triangles, about 1,000,000 with the shadow pass. About 300 draw calls with shadows, because the same mesh is reused. A fine animal is about 2,500 faces (5,000 triangles, 0.4 MB). A fine prop is about 1,500 faces. Fifteen animals cost about 75,000 triangles. A modern laptop GPU handles this easily. The risk is draw calls and mesh build time, not triangles: build each model once and clone the handle.

### 2.11 What we will not match

Hand-placed anti-aliasing on every curve. Hand-drawn animation frames with smear. A painted light pool exactly where the artist wants it. Perfectly clean 1 px lines at every angle. We trade them for a world that turns, lights and shadows correctly, with a cast the player builds by choice. That trade is the right one for this game.

---
## 3. Translation rules for our world (3 px per coarse voxel, 640 x 360)

Reference 2 and 3 show about 5 buildings across at 735 px. At our default zoom 640 px shows about 5 buildings of 8 tiles across the diagonal. So building size on screen matches the reference. Prop and character size on screen also matches (pedestrian about 5 percent of picture height: 29 px of 640 x 360). That means we can borrow the reference's density, scaled down to about 60 percent, with no change of scale.

### 3.1 What survives at this resolution and what to simplify

| Reference feature | In our world | Rule |
|---|---|---|
| 1 px hairlines (rails, wires, aerials) | one 2 f line (fine) or 1 v (coarse) | Rails: a top rail plus posts every 4 f. Wires are not drawn. The only wire is bunting (2 f line, 2 x 2 f flags every 8 f). |
| Window glazing bars | 2 f bars on fine window modules | One vertical plus one horizontal bar. No bars on coarse windows. |
| Sign lettering | pictogram first, letters second | Hero letters 7 x 11 f (stroke 2 f), 5 letters or fewer. Coarse letters 3 x 5 v. Small text becomes a scribble of 2 f lines. |
| Faces on far pedestrians | eyes as 2 x 2 f, a nose, ears. No mouth. | Mouths only in room view or close zoom. |
| 25 pedestrians | 10 to 14 on screen at peak | Group size 3 at most. |
| Brick and stone patterns | speckle plus a few plates | Real brick plate (bricks 4 x 2 f) only on feature walls. Elsewhere speckle of 2 shades. |
| Soft cast shadows | real sun shadows | Plus baked corner shadows (a darker voxel line, 1 v). |
| Window colours differ | one state per window by hash | Section 3.7. |
| Rooftop story | 3 to 6 items per roof | Section 3.2. |
| Ivy, vines | leaf patches, 6 x 10 v | On 30 percent of side and back walls. |
| Hanging lamp light pool | point light plus a soft decal ellipse on the floor | The light has no shadow in the street. |

### 3.2 Density targets

A **frontage** is 12 tiles of one building face with its pavement. A **block** is 4 frontages, about 32 tiles of street edge.

| Item | Target | Notes |
|---|---|---|
| Street furniture per frontage | 3 to 4 pieces | bench, bin, post box, phone box, bollard group, hydrant, bike rack, meter, bus stop |
| Lamps | 1 per 8 tiles of kerb | stagger, never in a straight even row |
| Trees or planters | 1 per frontage | 3 species by era (round, tall, small) |
| Wall details per building face | 3 or more | drainpipe, vent, AC box, lamp, poster, meter box, notice case |
| Props per 100 ground tiles (10 x 10 m) | pavement 6 to 10, road 2 to 4, grass 4 to 8 | cones, crates, bins, signs, cars, bushes |
| Quiet ground | at least 35 percent of visible ground has no props | the "quiet ground" rule from the diorama |
| Parked vehicles | 2 per 12 tiles of parking lane | colours from the era list, never two of one colour in a row |
| Moving vehicles | 3 at most on screen | they stop for the player on a crossing |
| Pedestrians | peak 10 to 14, day 5 to 8, evening 6 to 10, night 1 to 3 | peak = 08:00 to 09:00, 12:00 to 13:00, 17:00 to 19:00 |
| Ambient birds | 4 to 8 | see the open question in section 8 |
| Roof items | 1 to 2 on a roof under 24 tiles, 3 to 6 on a bigger roof | and one "roof story" for each 3 roofs |
| Signs per block | 4 fascia, 3 blade, 1 mural or billboard, 1 roof sign, about 8 small (window, menu board, poster, hours), 3 traffic and street signs: about 20 | max 2 saturated colours per sign |
| Animated props on screen | 6 at most | 2 flags, 2 smoke or steam, 2 blinking signs |
| Colour share | 70 percent muted body, 30 percent accents | by area on screen |

### 3.3 Silhouette rules for buildings

1. **Height rhythm.** In a row of 4 buildings: at least 2 different heights (a difference of at least 1 floor), at least 2 roof types, no 3 equal widths in a row.
2. **Corner rule.** The corner building of a block is the tallest (+1 floor) and has the corner entrance.
3. **Relief.** At least 25 percent of a facade's area is off the base plane by 1 v or more (recess, sill, string course, bay, balcony, awning).
4. **Awnings** on 60 percent of shopfronts. **Balconies** on 50 percent of flats. **Bay windows** on one in two house facades. **Setbacks** (an upper floor stepped in by 8 v to make a terrace) on one in five buildings.
5. **Roof clutter** is chosen by era (section 4.5). Roof clutter must break the roofline from every one of the four views.
6. **Every face is presentable.** The camera turns, so the back and sides get windows, a drainpipe, a vent and a poster or ivy. Only the street face gets the storefront.
7. **Projecting signs** (blade signs, roof towers) are visible from more views than flat signs. Use at least one on every enterable building.
8. **Height near the walkable ground.** A building that is H metres tall hides 1.2 x H tiles behind it. So: buildings next to the walkable area are at most 3 floors (about 9 m, hides 11 tiles). Towers (4 floors or more) stand at least 10 tiles from walkable ground, or are downtown only. Taller front buildings fade (dither to about 35 percent) when the player is behind them.
9. **Blind side walls** are the places for murals (24 x 16 v plates) and billboards. Never leave a blind wall plain.

### 3.4 Street furniture by era

Sizes in v (W x D x H). Era 1 is the bedroom era, about 1978. Later eras list only what is added or replaced.

**Era 1 (1978).**

| Item | Size | Notes |
|---|---|---|
| Sodium street lamp | pole 2 x 2 x 26, arm 10 long, head 6 x 3 x 2 | steel or concrete grey. Orange glow `F2A850`. Replaces the iron post. |
| Bench | 14 x 5 x 7 | wood slats, iron ends |
| Litter bin | 4 x 4 x 7 | wire or steel, lid |
| Pillar post box | 6 x 6 x 12 | rust red, round top |
| Phone booth | 8 x 8 x 20 | steel frame, glass panes (opaque glass look), a dial phone inside |
| Bus stop | pole 2 x 2 x 22, sign 8 x 1 x 6, shelter 24 x 8 x 18 | shelter has a bench and a lit ad panel 8 x 1 x 12 |
| Parking meter | 2 x 2 x 11, head 3 x 3 x 3 | coin type |
| Hydrant | 3 x 3 x 6 | butter or rust |
| Newspaper box | 5 x 5 x 7 | three colours |
| Sweets vending machine | 8 x 6 x 14 | on the pavement outside the shop |
| Traffic light | pole 2 x 2 x 22, head 3 x 3 x 8 | three 2 x 2 lamps stacked. States are emissive. |
| Crossing beacon | pole 2 x 2 x 20, black and white bands, globe 3 x 3 x 3 | flashing amber. Also marks crossings for the player. |
| Bollard | 2 x 2 x 5 | white and black |
| Street name sign | 8 x 1 x 3 | on a lamp or a wall |
| Bike rack | 12 x 3 x 5 | |
| Planter | 8 x 4 x 4 | leaf plus pink or mustard flowers |
| A-board | 5 x 3 x 8 | café menu, shop sale |
| Cones | 2 x 2 x 3 | in clusters of 3 to 5 near roadworks |
| Bunting (festival days) | line along the street at 22 v height | 2 x 2 f flags in 3 colours |

**Era 2, cartridge wars.** Video rental sign, neon tubes, cash machine in a bank wall, big mesh satellite dish in a rich garden, wedge-shaped cars, BMX bikes, cable TV boxes on poles.
**Era 3, disc and 3D.** Mini satellite dishes on every fourth flat balcony, card phone booths, glossy plastic signs, the first lattice mast on a tall roof, "@" signs on an internet café, round-shaped cars.
**Era 4, online.** Cell masts on roofs, LED signs, grey street cabinets (8 x 4 x 12), security domes on corners, security shutters, three-colour recycling bins, flat-screen shop ads (emissive 8 x 1 x 6).
**Era 5, pocket.** White-blue LED street lights, EV chargers (4 x 3 x 10), bike-share racks, delivery scooters, solar panels on two thirds of flat roofs, phone store glass fronts, bus stop with a screen.
**Era 6, the Silicon Drought.** Boarded shop windows (plywood plates 12 x 1 x 14), "to let" signs, parts-queue ropes outside repair stalls, half the street lights dark, fewer cars and more bikes, rust and patches on signs.
**Era 7, after.** A mix of old and new: green walls, repair stalls, mod shops, fresh paint over rust.

### 3.5 Ground detail

All numbers in v unless stated. The street is built from coarse tiles (8 x 8 v). Detail below is painted as plates (R3) or set voxel by voxel.

**Street section (south to north), main street, 16 tiles:**
building line | pavement 3 | parking lane 2 | travel lane 3 | travel lane 3 | parking lane 2 | pavement 3 | building line.

Side street, 10 tiles: pavement 2, lane 3, lane 3, pavement 2. Today's test street has a 3-tile road. Cars are 4 m long and 1.75 m wide, so make roads at least 6 tiles.

| Detail | Spec |
|---|---|
| **Kerb** | The pavement is 1 v higher than the road (pavement top at y = 0, road top at y = -1 v). A 1 v wide kerb stone strip in a lighter colour (`plastic`) runs along the pavement edge. The sun then casts a free shadow. Animals' feet follow the tile height. |
| **Gutter** | A 1 v wide darker line (`asphalt-d`) on the road next to the kerb. |
| **Yellow line** | A 1 v wide `mustard` line 1 v in from the kerb, on parking lanes. |
| **Centre line** | 2 v wide, dash 12 v, gap 12 v, `plastic`. Keep it 2 v so it reads at every zoom. |
| **Zebra crossing** | 3 tiles along the traffic (24 v). Stripes 4 v wide, gaps 4 v, running along the traffic. The pavement kerb dips to the road level for 3 tiles, with a ramp. Crossing beacons at both ends. One crossing per block, at the middle. |
| **Stop line** | 2 v wide, 8 v before a crossing. |
| **Road arrows, bus lane text** | plates, only at junctions. |
| **Manhole** | 6 x 6 v octagon in `steel`, one per 30 road tiles. |
| **Drain grate** | 4 x 2 v dark rectangle at the kerb, one per 10 kerb tiles. |
| **Pavement slabs** | 4 v slabs, a joint line of 1 f. Two tones of `putty` and `clay` by hash. Joints are painted, not modelled. |
| **Wear** | Lighter or darker streaks (6 to 10 v long, 2 v wide, along the 2:1 diagonal) at doors, crossings and bus stops. As plates. |
| **Desire paths** | On grass, where the player and pedestrians walk often, the grass turns to `sand` dirt over days (3 shades, ragged edge 1 to 2 v). A counter per tile. Rebuild a chunk once a day, not every frame. |
| **Tree pits** | 8 x 8 v with a dark soil square and a `steel` grate. |
| **Oil stains** | 6 x 4 v at parking lanes, 1 per 6 spaces. |
| **Puddles** | Rain only: 6 x 4 v blue-grey plates, 4 per 100 tiles. Ground colours go 15 percent darker. |
| **Verge, hedges, low walls** | Hedge 1 tile x 6 v tall, leaf with bites. Low brick wall 4 v high with a lighter coping. |

### 3.6 How life is shown without clutter

**Pedestrians on schedules.**
- Six archetypes: **commuter** (08:00 to the bus stop, 17:30 back), **student** (08:00 school or campus, 15:30 home), **shopkeeper** (opens 09:00, lunch 12:30, closes 18:00), **lunch-goer** (12:00 café), **arcade kid** (16:00 to 21:00), **night owl** (23:00 to 02:00).
- Each character visits 3 to 5 stations a day (home, work or school, café, shop, park, bus stop). They walk between by the shortest path.
- Only characters inside about 28 tiles of the player are drawn. Others are tracked abstractly and appear at their next position.
- Six poses only: walk, stand, sit, talk (two characters face each other at 1 tile, heads bob), look-at (turn the head for 2 s at a window or poster), carry (a bag, a newspaper).
- Era props for pedestrians: a shopping bag (3 x 2 x 4), a newspaper (3 x 1 x 4), a boom box (10 x 3 x 5) for arcade kids, a skateboard (8 x 2 x 1). **No pets.** In this world animals are people.

**Vehicles.**
- Parked: 2 per 12 tiles of parking lane, static, with a worn look (a dent plate, a dirty roof).
- Moving: at most 3 on screen, on loops at 4 tiles per second. They stop at crossings for the player and for pedestrians.
- Era 1 body colours: `orange`, `mustard`, `denim`, `putty`, `moss`, `rust`. Glass `glass`. Tyres `ink`.

**Ambient creatures.** 4 to 8 small birds (3 x 2 x 2 v) hopping on pavements, flushed when the player is within 2 tiles, flying 2 frames of flap, back after 20 s. A few moths around lamps at night (1 v, emissive). Nothing else.

**Laundry lines.** Between flats' balconies and in back lanes. 12 to 24 v long at 2.2 m, with 3 to 6 cloth plates (3 x 5 v) in `plastic`, `salmon`, `denim`, `butter`. Era 1 and 2 only on the street face. Later only in back lanes.

**Smoke and steam.** A chimney puff (2 x 2 x 2 v, grey-white) every 3 s on home evenings and cold mornings. A café vent steams at 07:00 to 10:00 and 12:00 to 14:00. An arcade door lets out a coloured glow, not smoke.

**Flags.** Municipality, school and bank flagpoles. A flag is 8 x 5 v with a 2-frame flutter.

### 3.7 Time of day on buildings

**Windows.** Each window has a state chosen by hash of (building, floor, bay):

| State | Day share | Evening (17:00 to 23:00) | Late (23:00 to 02:00) |
|---|---|---|---|
| dark glass | 72 percent | 30 percent | 75 percent |
| curtain or blind drawn | 15 percent | 15 percent | 15 percent |
| open (frame shifted 2 v) | 8 percent | 5 percent | 0 |
| lit warm (`butter`) | 0 | 40 percent | 8 percent |
| lit TV blue (flicker, 2 shades) | 0 | 5 percent | 2 percent |
| AC unit in the window | 5 percent | 5 percent | 0 |

Lit windows share **three emissive materials**, one for each schedule group (evening 17:00 to 23:00, late 17:00 to 02:00, night owl 00:00 to 06:00 at 3 percent of windows). The clock sets their brightness. One mesh of lit panes per building, never one entity per window.

**Shop windows and signs.** Shops close at their hours. At closing, the roller shutter module replaces the glass (D5). Lit signs stay on until 23:00. Arcade bulbs chase (2 Hz) from 15:00 to 22:00. A closed shop's fascia sign is dim, not off.

**Lamps.** On from 17:30 to 06:00 in winter and from 19:30 in summer. A pool of light (an ellipse of 6 tiles across, `butter` at 25 percent) on the ground under each. Sodium orange, not white.

### 3.8 The dark undertow in the street

The surface stays warm and cute. The storyteller's chill level (0 to 3) cools it a little at a time. These are the only world changes it makes:

| Chill | Change |
|---|---|
| 0 | none |
| 1 | the sun and ambient colour hue shifts 8 degrees toward blue. One lamp in 8 flickers. |
| 2 | saturation drops 6 percent. Lit windows turn off in a row as the player passes. Shadows are 10 percent darker. |
| 3 | an additional 6 percent less saturation. One window per block stays lit all night with a figure that does not move (a still silhouette plate). Pedestrians turn their heads to the player at 3 tiles. |

Chill touches only the palette keyframes and a few props. It never adds new gameplay objects.

---

## 4. The building kit

A parametric vocabulary. The engineer implements each piece as a function that writes voxels (coarse shell) or returns a mesh (fine module), with the sizes below.

### 4.1 Unit contract

- 1 tile = 8 v = 24 f. A **bay** is 2 tiles (16 v), the width of one window rhythm unit. A **half-bay** is 8 v.
- **Floor heights.** Upper floors: 24 v (3 m). Ground floor: shop 32 v (4 m), civic 28 v, home 24 v. A floor slab line (string course) is 2 v high and sticks out 1 v.
- **Plinth:** 3 v high, `taupe`, at the base of stone and brick buildings.
- **Depth layers.** The wall plane is layer 0. Layer -2 and -1 are recesses (window glass at -2, door panel at -2). Layers +1 are trim (sills, pilasters, string courses). Layers +2 to +6 are bays, balconies and awnings. Layers +6 to +12 are blade signs and canopies. Flat features (flush with the wall) do not read. Use relief of 1 v or more.
- **Fine modules snap** to a 4 v horizontal grid inside the bay and to the floor line vertically.
- **Draw the back too.** All four faces use modules. Only the street face uses the storefront.

### 4.2 Base types

| Code | Base | Footprint (tiles) | Floors | Default roof | Used for |
|---|---|---|---|---|---|
| B1 | Row shop | 6 to 8 wide x 6 deep | 2 or 3 | flat, or mansard | computer shop, café, bank branch |
| B2 | Detached house | 6 x 6 plus porch, garage 2 x 3 | 2 | gable | home |
| B3 | Walk-up flats | 8 x 6 | 3 or 4 | flat | home (flats), dorm later |
| B4 | Civic block | 10 x 8 | 3 | flat plus clock gable | municipality office, bank (large) |
| B5 | Long low block | 14 to 16 x 6, plus gym 8 x 8 | 2 | flat, barrel on the gym | high school |
| B6 | Hall or shed | 8 to 12 x 8 | 1 (height 36 v) | sawtooth or gable | arcade, garage, warehouse |
| B7 | Tower | 8 x 8 | 6 to 12 | flat plus mast | downtown (trips) |
| B8 | Kiosk or booth | 1 x 1 to 3 x 2 | 1 (height 16 to 24 v) | flat or small gable | newsstand, bus shelter, ticket booth |

Operators on any base: **setback** (stepped terrace), **L-shape** (cut a corner of at most 3 x 3 tiles), **chamfered corner** (4 steps of 2 v), **annex** (a lower box on one side).

### 4.3 Facade modules

Sizes W x H x D in v. D is the relief out from the wall plane (negative is recess). All are fine modules (R2) on coarse shells, except where marked coarse.

| Code | Module | Size | Notes |
|---|---|---|---|
| W1 | Sash window | 8 x 12 x -1; sill 10 x 2 x +1 | frame 1 v, one horizontal and one vertical bar of 2 f. Sill at 9 v from the floor. Default window. |
| W2 | Tall window | 8 x 16 x -1 | civic and bank. Sill at 5 v. |
| W3 | Small window | 4 x 6 x -1 | bathrooms, stairs |
| W4 | Strip window | 32 x 10 x -1 | school and office. Mullion every 8 v. |
| W5 | Bay window | 12 x 16 x +4 | three glazed faces, small roof cap 1 v. Houses. |
| W6 | Shop window | by storefront formula | see 4.4 |
| W7 | Arched window | 8 x 14 x -1 | top 2 steps rounded. Bank, civic. |
| W8 | Dormer | 10 x 10 x +8 | gable top 6 v. House roofs. |
| W9 | Skylight | 8 x 6 x +2 | on a roof slope or flat roof |
| B1 | Balcony | 14 x 1 x +6 slab, rail 5 v high | posts every 4 f, top rail 2 f. Flats, 2nd floor and up. |
| B2 | Juliet rail | window width x 5 x +1 | |
| A1 | Shop awning | storefront width x 3 drop x +6 | stripes 2 v wide in two colours. Valance 3 v. |
| A2 | Flat canopy | 16 x 1 x +8 | bank, civic, arcade (with bulb row) |
| A3 | Porch roof | 12 x 1 x +6 on two 2 x 2 posts | house |
| D1 | Door | 8 x 18 x -2 | frame 1 v, panel, handle 2 f, step 1 v |
| D2 | Double door | 12 x 18 x -2 | glazed upper 10 x 8. Civic, school. |
| D3 | Glazed shop door | 10 x 18 x -2 | glass 10 v tall |
| D4 | Garage door | 20 x 14 x 0 | ribs every 2 v |
| D5 | Roller shutter | storefront width x 16 x 0 | ribs every 1 v. Shows when closed. |
| D6 | Steps | 1 v rise x 3 v run, 2 to 6 steps | with 2 v wide cheek walls |
| P1 | Pilaster or quoin | 4 x floor height x +1 | quoins alternate 2 v and 4 v long |
| P2 | String course | 2 high x +1 | at each floor line (today it is 1 v: make it 2) |
| P3 | Cornice | 3 high x +2 | dentils: gap every 2 v |
| P4 | Plinth | 3 high x 0 | `taupe` or `umber` |
| X1 | Drainpipe | 1 x 1 x +1, floor to roof | on both street-side corners. `steel`. A bracket every 12 v. |
| X2 | Window AC unit | 6 x 4 x +4 | era 1: boxy, `steel` with a dark grille |
| X3 | Vent grille | 4 x 3 x +1 | |
| X4 | Wall lamp | 2 x 2 x +2 | emissive at night |
| X5 | Poster plate | 4 x 6 x 0 | R3, 2 or 3 colours |
| X6 | Notice case | 8 x 10 x +2 | R3 inside, glass rim |
| X7 | Ivy patch | 6 x 10 x +1 | leaf voxels, bites |
| X8 | Meter box | 3 x 4 x +2 | |
| X9 | Shutters (pair) | 2 x 12 x +1 each | houses, some café windows |

### 4.4 Storefronts and sign slots

**Storefront formula** for a ground floor of width W v (a multiple of 8) and height 32 v:

```
[pilaster 4] [window a] [door 12] [window b] [pilaster 4]
window a + window b = W - 20; a = floor((W-20)/2), b = the rest
y 0..2    stallriser (wall colour, 3 v)
y 3..18   glazing (16 v)
y 2..19   door (18 v tall)
y 19..23  awning band: awning attaches at y 23, lip at y 19, projects 6 v
y 25..31  fascia sign slot (7 v)
```

**Sign slots.**

| Slot | Position | Size | Contents |
|---|---|---|---|
| **F fascia** | y 25..31, between the pilasters | (W - 8) x 7 | a pictogram (8 x 7 v) plus a word of 5 letters or fewer. A 1 v border. |
| **B blade** | at a pilaster, y 22..29, hung 1 v from a bracket | 8 x 8 x 1, sticking out 8 v | a pictogram only. Visible from along the street. |
| **R roof** | stands on the parapet | W x 4 x 8 on 2 v legs | letters or a shape. For arcades and large shops. |
| **T tower** | at a roof corner, vertical | 8 x 2 x 28 | arcade and cinema style. Pink or `butter`. Frame of bulbs. |
| **V valance** | the lip of the awning | W x 3 | stripes or a word 3 v tall |
| **G glass** | on the window | 6 x 3 | "OPEN" (lit), hours card |
| **P poster or menu** | beside the door | 6 x 8 | R3 plate, lit box |
| **M mural** | a blind side wall | 24 x 16 | R3 plate. Era-themed. |

**Text and pictogram rules.**
1. **Pictogram first.** The sign must read by shape and colour at 24 px per metre. Letters help only at close zoom.
2. Hero letters: coarse 3 x 5 v. Fine 7 x 11 f, stroke 2 f, 1 f gap. 5 letters or fewer.
3. A sign has 2 colours plus a dark panel. Never 3 saturated colours.
4. Era type: 1978 uses rounded fat letters in `orange`, `mustard`, `teal`, `pink` on `night` or `umber`.
5. Never reuse a word from the reference images.

**Kind colours** (also the colour of the district map's pins). The panel and letters are chosen for contrast of 4.5 to 1 or better (the Refactoring UI rule for text), measured on the swatch values in section 6.

| Kind | Frame, awning, pin | Panel and letters (contrast) | Pictogram |
|---|---|---|---|
| Computer shop | `teal` | `cream` on shade-teal `1D4E5B` (8.1) | chip (8 x 7 with pins) |
| Café | `orange` | `ink` on `orange` (5.1) | cup with steam |
| Arcade | `pink` | `butter` on `night` (9.9) | joystick or coin |
| Bank | `moss` | `cream` on `pine` (6.0) | coin stack |
| Municipality | `denim` | `cream` on `denim` (5.8) | tower in a shield |
| School | `rust` | `cream` on `rust-d` (6.9) | book |
| Home | `mustard` | `ink` on `mustard` (7.0) | house number plate |
| Bus stop | `steel` | `ink` on `sky` | bus |

The gold gem over each enterable door stays (it is the game's own marker). It is not part of the building kit.

### 4.5 Roof modules and clutter

**Roofs.**

| Code | Roof | Spec |
|---|---|---|
| R1 | Flat with parapet | parapet 3 v high, 1 v thick, with a 1 v lighter coping. Roof surface speckled gravel (two shades of `asphalt-d`). |
| R2 | Gable | pitch 1 up per 2 across (a 6 tile span rises 12 v). Eaves overhang 2 v. Tile courses: a darker row every 2 v. Gable wall triangle with a 2 x 2 vent. |
| R3 | Hip | as gable, four sloped faces |
| R4 | Mansard | steep lower slope (2 up per 1 across, 10 v) then a flat top. A dormer every 2 bays. |
| R5 | Shed | one slope, 1 up per 4 across |
| R6 | Barrel | gym: arch rising 16 v in 4 steps |
| R7 | Clock gable | civic: 16 wide, 6 high, 10 tall clock face plate 6 x 6 |

**Rooftop clutter. Right era only.** Sizes in v.

| Item | Size | Era 1 (1978) | Later |
|---|---|---|---|
| Chimney stack with pots | 6 x 6 x 10, pots 2 x 2 x 3 | houses 1 or 2 | rarer |
| **TV aerial** | mast 1 x 1 x 14, 3 crossbars of 10, 8 and 6 at heights 8, 11 and 14 | on every house, 3 to 5 on a block of flats | falls to 1 in 4 by era 3 |
| CB radio whip | 1 x 1 x 14 on a 3 x 3 x 2 base | 1 roof in 6 | gone by era 3 |
| Vent pipe | 2 x 2 x 8 | yes | yes |
| Water tank on legs | round 12 x 12 x 12, legs 6 | flats of 4 floors | rarer |
| Roof hatch | 8 x 8 x 8 | flats, civic | yes |
| Rooftop HVAC box | 14 x 8 x 6, fan circle | school, civic, big shops | era 2 on, many roofs |
| Flagpole | 1 x 1 x 28, flag 8 x 5 | civic, school, bank | yes |
| Billboard | 32 x 2 x 16 on legs 6 | blind wall or roof, 1 per block | bigger and glossier |
| Skylight | 8 x 6 x 2 | yes | yes |
| Laundry line | see 3.6 | flats | gone |
| Satellite dish | big mesh 24 diameter (era 2), 6 diameter (era 3+) | **none** | era 2 on |
| Cell mast | lattice 4 x 4 x 40 | **none** | era 4 on |
| Solar panels | 16 x 8 x 2, tilted | **none** | era 5 on, 2 of 3 flat roofs |

**Roof stories (pick one for each 3 roofs, era 1):** a lawn chair and a radio (someone sunbathes); a pigeon-free washing line; a kid's den of crates; a telescope on a tripod (3 x 3 x 8); a rooftop garden of 4 pots; a workman's ladder and a bucket.

### 4.6 Corner pieces

| Code | Piece | Spec |
|---|---|---|
| C1 | Square corner | quoins (P1) from the plinth to the cornice |
| C2 | Chamfered corner entrance | cut the corner in 4 steps of 2 v. Put a D3 door in the cut face. The sign wraps (F fascia on both faces). A 4 x 4 v column at the cut. |
| C3 | Wrap shopfront | a storefront on both faces, joined by one column |
| C4 | End-of-terrace blind wall | a mural plate (24 x 16 v), a drainpipe, an AC unit |
| C5 | Passage arch | 8 wide x 18 tall, leads to a back lane |
| C6 | Turret | round tower 12 x 12 v, conical top. Civic only. |

### 4.7 The parametric recipe

```rust
struct BuildingSpec {
    base: Base,                 // Row, Detached, Walkup, Civic, Long, Hall, Tower, Kiosk
    tiles: UVec2,               // footprint
    floors: u8,
    ground: GroundKind,         // Shop 32 | Civic 28 | Home 24
    facade: FacadeStyle,        // wall palette, window set, string course, cornice, plinth
    shopfront: Option<Shopfront>, // storefront formula + sign slots + awning
    roof: Roof,                 // R1 to R7
    roof_items: Vec<RoofItem>,  // from the era table, count by roof area
    corner: Option<CornerPiece>,
    setbacks: Vec<Setback>,     // (floor, side, depth in v)
    era: Era,
    seed: u32,
}
```

Rules:
1. Window module for bay `i` on floor `f` is picked from `facade.window_set` by `hash(seed, i, f)`.
2. A facade uses at most 3 window types. Rhythm is ABA or AAB across bays. The ground floor differs from the upper floors.
3. Balconies go on every second bay of floors 2 and up (flats), as one continuous run per floor.
4. `era` filters every list (roof items, signs, AC, lamps).
5. Window state (day, evening, late) comes from section 3.7 by `hash(seed, i, f)`.
6. Build a coarse shell from the footprint. Place fine modules as children that share mesh handles. One building mesh for the shell and one lit-pane mesh per schedule group.

### 4.8 The seven 1978 buildings

All six enterable buildings and home. Names are placeholders.

**Computer shop ("MICROS").**
- Base B1, 8 tiles wide, 6 deep. Floors: shop 32 v plus 1 floor of 24 v. Walls `brick` with `rust` speckle. Trim `plastic`. Plinth `taupe`.
- Storefront as in 4.4 with W = 64: windows 22 v each. Display stand in the left window: 3 small machines on a stepped stand (each 6 x 5 x 6 v), a price card.
- Awning A1: stripes `teal` and `plastic`. Fascia F: `teal` frame, deep-teal panel, `cream` chip pictogram and the word "MICROS". Blade B: chip.
- Upper floor: two W1 windows per bay, one with an AC unit, curtains in the other.
- Roof R1. One TV aerial, one vent pipe, a rooftop chair story or a CB whip (the owner is a hobbyist).
- At the door: a magazine spinner rack (4 x 4 x 14). A glass "OPEN" (lit).
- **Tell:** brick row with striped teal awning, a chip blade sign, a roof aerial.

**Café ("THE CUP").**
- Base B1, 6 wide, 6 deep. Floors: 32 v plus 24 v flat above. Walls `sand`, trim `plastic`, plinth `umber`.
- Wrap shopfront (C3 or C2 if on a corner): glass from y 3 to 20. Café half curtains (a `plastic` plate, 8 v tall, in the lower half of the window).
- Awning A1: `orange` and `plastic` stripes. Fascia F: `orange` panel, `ink` cup pictogram and the word "THE CUP". Blade: cup.
- Outside: 2 round tables (6 x 6 x 6) with 4 chairs (3 x 3 x 6), an A-board (menu).
- Roof R1 with a steam vent (4 x 4 x 6) that puffs at meal times. Window box under the upper windows.
- **Tell:** low, glassy, orange and cream stripes, little tables on the pavement, steam.

**Arcade ("COIN-OP").**
- Base B6 front, 6 wide x 8 deep, 1 storey of 36 v. Walls `umber` and `night`. Trim `pink`.
- Entrance: A2 canopy (16 x 1 x +8) with a row of 8 `butter` bulbs along the edge that chase. Two D2 doors with black glass (`ink`).
- A big window with 3 cabinet screens glowing (emissive 4 x 3 v, 3 colours). A poster plate beside the door.
- **Tower sign T** (8 x 2 x 28, `pink`, a joystick pictogram, bulb frame) at one front corner of the roof.
- Upper wall above the canopy is a flat mural plate in `night` and `pink`.
- **Tell:** dark box with a tall pink tower sign and a chasing bulb row.

**Bank ("SAVINGS").**
- Base B1 or B4 small, 6 wide, 6 deep. Floors: 32 v plus 24 v. Walls `plastic` and `putty` (stone). Plinth `clay`, 4 v high.
- Symmetrical. Four P1 pilasters. Central porch: A2 canopy, two columns (3 x 3 x 28), 3 steps (D6), two D2 doors in `mustard` (brass).
- W7 arched windows on the ground floor, W2 on the upper floor. Cornice P3 4 v high. Roof R1 with a 4 v parapet.
- Fascia F: `moss` frame, `pine` panel, `cream` coin stack and the word "SAVINGS". A **time and temperature board** on a pole at the corner (6 x 2 x 18, lit digits, 4 digits of 3 x 5 v). It shows the game clock. A player can tell the time from the street.
- **Tell:** a stone block with columns, steps, brass doors and the clock pole.

**Municipality office ("TOWN OFFICE").**
- Base B4, 10 wide, 8 deep. Floors: 28 v plus 2 x 24 v. Walls `sand`, trim `denim`, plinth `clay`.
- A central portico: 4 columns (3 x 3 x 28), 6 steps (24 v wide), two D2 doors. Side wings stepped back 8 v.
- W2 tall windows in rows. Clock gable R7 on the roof, centred. A flagpole (1 x 1 x 28, flag 8 x 5, `denim` and `plastic`) in front.
- A notice case X6 on the left of the door (4 notices). Two parking spaces and a white car with a stripe.
- **Tell:** a big pale block, central portico with steps, a clock gable, a flagpole.

**High school ("HIGH").**
- Base B5, 16 wide, 6 deep, plus a gym annex 8 x 8. Floors 28 v plus 24 v. Walls `sand` brick with 2 `rust` bands (2 v high) at the floor lines. Trim `plastic`.
- W4 strip windows, 70 percent of the facade length on both floors. An entrance bay 4 tiles wide projects 2 tiles, with D2 doors, 4 steps, a clock plate (6 x 6 v).
- Roof R1 with 2 HVAC boxes, a chimney (boiler, 6 x 6 x 14) and a flagpole. Gym roof R6.
- A freestanding sign board (24 x 2 x 12 on two posts) with a changeable-letters plate. A chain-link fence (1 v posts every 4 v, 2 v rails, 14 v high), a bike rack (12 x 3 x 5), a basketball hoop (pole 2 x 2 x 24, backboard 6 x 1 x 4).
- **Tell:** a long low brick bar with continuous glass bands, a barrel-roof gym, a fence and a flagpole.

**Home, a house.**
- Base B2, 6 x 6 plus a porch 3 x 2. Floors: 24 v plus 24 v. Walls `plastic` render above a `brick` base (6 v), trim `denim`. Roof R2 in `rust` with a chimney (6 x 6 x 10, 2 pots) and a TV aerial on it.
- D1 door in `salmon` or `denim`, A3 porch roof, house number plate (2 x 2, R3). A W5 bay window on the ground floor, two W1 windows above with curtains.
- Garage or carport on the side (D4). Front garden 3 tiles deep: lawn, hedge (6 v), low wall with a gate, letterbox (3 x 2 x 3 on a 1 x 1 x 6 post), a bin, a washing line at the side, a bicycle.
- Richer homes are bigger with a garage and garden. Poorer homes are a terraced house with no garden, a bin on the pavement and one aerial.
- **Tell:** a pitched roof with a chimney and an aerial, a hedge and a gate.

**Home, flats.**
- Base B3, 8 x 6. Floors: 24 v x 4. Walls `putty` and `steel` panels, trim `plastic`. Entry canopy (A2) with a buzzer panel (3 x 4 plate). Balconies B1 on floors 2 to 4. Laundry on 2 balconies. AC units in 3 windows. 4 aerials, a water tank, a roof hatch.
- **Tell:** a grey block with balconies, laundry and a forest of aerials.

### 4.9 What changes in later eras

| Building | Era 2, cartridge wars | Era 3, disc and 3D | Era 4, online | Era 5, pocket | Era 6, drought |
|---|---|---|---|---|---|
| Computer shop | video game section, cartridge shelves, bigger window, neon tubes | PC and console store, CD racks, glossy plastic sign | LAN and cable internet signs, security shutter | becomes a phone store: white glass front, LED sign, no awning | half empty, "REPAIR AND USED" sign, boarded half |
| Café | the same, with a small TV | an internet corner with 3 PCs | wifi sign, laptops on tables | chain-style, phone charging pillars | fewer tables, cheaper signs |
| Arcade | peak: neon everywhere, longer canopy, 20 cabinets | decline: fighting and racing cabinets, fewer bulbs | **closes**: becomes an internet café with "@" sign | a gym or a phone shop | empty, boarded |
| Bank | a cash machine in the wall | glossy sign, ATM lobby | LED board, dome camera | glass branch, tablet kiosks | queues, security guard |
| Municipality | a computer terminal in the window | a larger annex | e-forms kiosk | glass extension | long queue rope outside |
| School | a computer lab wing (new roof box) | a second lab, satellite dish | CCTV domes | solar panels | patched roof, fewer windows lit |
| Home | big dish in rich gardens | mini dishes on flats | cell mast near, white-blue light | solar panels, EV charger | patched roof, a bike in place of a car |

### 4.10 Vehicles and people (sizes, v)

| Item | Size W x D x H | Note |
|---|---|---|
| Hatchback | 14 x 30 x 10 | roof at 1.25 m. Four colours at once in view. |
| Saloon | 14 x 36 x 10 | |
| Van | 14 x 32 x 14 | |
| Minibus | 14 x 34 x 16 | |
| City bus | 16 x 64 x 19 | cream and moss or cream and rust, two-tone |
| Bicycle | 3 x 14 x 9 | era 1: racer and a chopper style |
| Moped | 4 x 14 x 10 | |
| Animal (adult) | 6 x 6 x 11 | today's plan: legs 3, torso 4, head 4, plus ears and snout. As a fine model: 18 x 18 x 33 f. |

---
## 5. Interior direction (from reference 4, with reference 5 as the detail bar)

### 5.1 Cutaway rules

| Rule | Spec |
|---|---|
| **View** | One fixed angle: yaw 45 degrees, from the south-east. The player sees the **north and west** walls. The south and east sides are open. |
| **Wall height** | 24 v (3 m). Civic and school rooms 28 v. |
| **Wall thickness** | 2 v. The top edge has a 1 v lighter cap in `plastic`, like reference 4's clean edge. |
| **Skirting** | 2 v high, `umber` or `wood-d`. A 1 v darker line where the wall meets the floor (baked corner shadow). |
| **Floor edge (the plinth)** | The floor is a slab 3 v thick. Its cut sides (south and east) show in `umber`, with a 1 v lighter top lip in `taupe`. The room looks like an object on a plinth. This matches the "cut model base" of the district map. |
| **Backdrop** | `void` (today 2A2422, which is close). Flat. |
| **Openings** | Windows and doors are real holes with a 2 v reveal and a 1 v frame on four sides. Doors and windows are only in the north and west walls. |
| **Window view** | An opaque plate 4 v behind the wall: day `sky` with a darker lower band (rooftops) and one tree plate; dusk `salmon` fading to `orange`; night `night` with 3 `butter` dots and a lamp glow. One emissive material, set by the clock. |
| **Inner walls** | Full height only for 2 tiles out from a back wall. Beyond that they drop to 8 v (waist height), like the bathroom partition in reference 4. |
| **Zones** | A room larger than 20 tiles has at least 2 zones, marked by floor material or level (section 5.3). |
| **Facing** | Furniture fronts face south or east: the sides the camera sees. Items on the north wall face south. Items on the west wall face east. |
| **Wall items** | At least 2 per 4 tiles of visible wall (poster, shelf, clock, notice, mirror, calendar). Hang them at mixed heights. None above the tallest item behind them. |
| **Room zoom** | Section 2.9: width + depth of 12 or less uses 48 px per metre. Up to 30 uses 24. |

### 5.2 Floors and walls

| Material | Where | Recipe | Colours |
|---|---|---|---|
| **Parquet blocks** | homes, bedroom | 8 x 8 v squares. Each has 4 planks of 2 v. Squares alternate plank direction. Tone steps by hash (plus or minus one ramp step). | `wood`, `wood-l` |
| **Plank floor** | cheap rooms | boards 2 v wide, 16 v long, joints offset | `wood`, `wood-d` |
| **Lino check** | shops, kitchens (today's shop) | 4 v squares | `putty`, `clay` |
| **Carpet** | homes, offices | 3 shades by hash. A 1 v fringe at edges. | era 1: `rust`, `orange`, `moss`, `wood-l` |
| **Arcade carpet** | arcade | `night` and `umber`. A repeating 4 v diamond in `pink` and `teal`, at 15 percent density. | |
| **Concrete** | garage, workshop (later) | `clay`/`steel` speckle, 3 hairline cracks, a stain plate | |
| **Bath and kitchen tile** | wet areas | 4 v squares in two tones, one step lower than the main floor (section 5.3) | era 1: `moss` or `mustard` with `plastic` |
| **Rug** | everywhere | 1 v raised. 2 v border in a contrasting colour. 1 v fringe. Sizes 16 x 16, 24 x 16, 32 x 24 v. | |
| **Floor wear** | shops, schools | R3 plates: lighter streaks 6 to 10 v long, along the 2:1 diagonal, at doors and counters. A worn runner from door to counter. | |
| **Wall: painted plaster** | most | 2-shade speckle (today's) | |
| **Wall: wallpaper** | homes | 2 v stripes in two close tones, or a 1 v dot every 4 v | era 1: `salmon`/`plastic`, `sand`/`plastic` |
| **Wall: panelling** | shops, bars | 2 v vertical planks up to 10 v, `wood`/`wood-d`, with a 1 v rail on top | |
| **Soft furniture shadow** | all | a plate under each item, 1 v larger than its footprint, 20 percent darker. Baked. | |

### 5.3 Split levels, stairs and rails (reference 5)

| Item | Spec |
|---|---|
| **Heights** | 0, +3 v (a step-up zone), +8 v (a platform, 1 m), and -3 v (sunken). Two levels up at most. |
| **Platform face** | The south and east faces of a raised area show a face the height of the step in `umber`, with a 1 v lighter lip on top. |
| **Stairs** | For +8 v: 4 steps, each 2 v rise and 3 v run. The stair is 12 v long (1.5 tiles, use 2 tiles in the map). For +3 v: one step, no stair. |
| **Rails** | Top rail 2 f at 7 v above the tread line. Posts every 8 f. `rust` or `steel` for shops and schools, `wood-d` for homes. A rail never hides a click target: rails stand on the sides of the stair, not across it. |
| **Pathing** | A per-tile height. Stair tiles are walkable and the feet follow the slope. Platform edges without stairs are blocked. |
| **Placement** | Raised areas go on the north and west sides (behind). Sunken areas go on the south and east (in front). The "tall at the back, low at the front" rule. |
| **Where they appear** | Café booth platform (+3 v with a brass rail). Arcade prize platform (+3 v). School and municipality entrances (outside, 4 steps). Shop repair loft (+8 v, optional, behind a rail). Sunken bathroom (-3 v) in homes. |

### 5.4 Prop kit

Sizes W x D x H in v. Fine items are 3 times denser (R2). Footprints in tiles are for pathing. Detail notes say what to paint so the item reads at 29 px.

**Bedroom and study.**

| Prop | Size (v) | Tiles | Detail |
|---|---|---|---|
| Single bed | 8 x 16 x 5, headboard 8 x 1 x 9 | 1 x 2 | quilt in 2 stripe colours, folded corner, pillow 5 x 4 x 2 with a dent, frame legs |
| Bedside table | 4 x 4 x 5 | shares a tile | drawer plate, lamp or flip clock on top |
| Wardrobe | 16 x 6 x 16 | 2 x 1 | two doors with a 2 f seam, handles, boxes on top |
| Chest of drawers | 8 x 5 x 8 | 1 | three drawer plates, one open a finger |
| Bookcase | 8 x 4 x 14 | 1 | 5 shelves, about 30 spines in 5 colours, a trophy or model |
| Desk | 16 x 8 x 6 | 2 x 1 | side drawers, a worn front edge in a lighter colour |
| Desk chair | 4 x 4 x 7 | walkable | swivel base, a cushion in a bright colour, pulled out 1 v |
| Rug | 16 x 16 x 1 | decal | |
| Floor lamp | 3 x 3 x 14 | 1 | shade `butter`, emissive at night |
| Wall shelf | 10 x 3 x 1 | wall | 3 books, a model, a plant |
| Curtains | 14 x 2 x 16 | wall | 2 pleats, `rust` or `salmon` |
| Radiator | 8 x 2 x 6 | wall | 5 fins |
| Mirror | 8 x 1 x 10 | wall | 2 f sky rim |

**Desk items.**

| Item | Size (v) | Detail |
|---|---|---|
| Typewriter | 5 x 5 x 3 | keys as a 2 f dot grid, paper sheet 3 x 1 x 3 |
| Portable TV | 10 x 9 x 9 | screen 6 x 5 emissive, antenna 2 rods in a V |
| Kit micro board | 8 x 6 x 2 | green board plate, 6 chips (2 x 1 x 1, `ink`), 2 wires |
| Beige micro and keyboard | 8 x 3 x 1 | rows of keys as a plate |
| Cassette recorder | 6 x 3 x 2 | two reels, one red button |
| Pocket calculator | 1 x 2 x 0.5 (3 x 6 x 1.5 f) | one red LED, emissive |
| Graph paper pad | 4 x 5 x 0.5 | plate with a hand-drawn grid and a sprite sketch |
| Magazine stack | 4 x 5 x 2 | 3 colours on the spines |
| Desk lamp | 3 x 3 x 5 | emissive shade, a point light |
| Mug | 1 x 1 x 1.5 | `orange` or `plastic`, a 1 f handle |
| Alarm clock | 3 x 2 x 3 | flip digits plate |
| Lava lamp | 2 x 2 x 7 | `pink` blobs, a faint glow |

**Living, kitchen, bath.**

| Prop | Size (v) | Tiles | Detail |
|---|---|---|---|
| Sofa | 16 x 7 x 7, back to 9 | 2 x 1 | 3 cushions, one crooked, a throw |
| Armchair | 8 x 7 x 7 | 1 | worn arms in a lighter colour |
| Coffee table | 10 x 6 x 4 | 1 x 1 | a magazine, a mug |
| TV on a stand | 12 x 8 x 17 | 1 x 1 | the family TV: 10 x 9 x 9 on a stand 8 high |
| Sideboard | 14 x 4 x 7 | 2 x 1 | a lamp, a photo, a radio |
| Dining table and 4 chairs | 12 x 8 x 6, chairs 4 x 4 x 7 | 2 x 1 | a plate with bread, a bowl |
| Fridge | 8 x 8 x 15 | 1 | round top, handle, magnets |
| Cooker | 8 x 8 x 7 | 1 | 4 dark plates, a kettle on one |
| Sink unit | 8 x 8 x 7 | 1 | tap, a mug in the sink |
| Bath | 14 x 6 x 5 | 2 x 1 | taps, a towel on the rim |
| Toilet | 4 x 6 x 7 | 1 | tank 4 x 2 x 6 |
| Basin | 5 x 4 x 7 | 1 | pedestal, a toothbrush mug |
| Doormat | 8 x 5 x 1 | decal | one colour, a 2 f border |
| Coat rack | 4 x 2 x 16 | wall | 3 hooks, a coat plate |

**Shop and café.**

| Prop | Size (v) | Tiles | Detail |
|---|---|---|---|
| Tall shelf (today's) | 8 x 4 x 16 | 1 | 3 levels, boxed tapes in clear bags (3 x 1 x 4, `plastic`), manuals with coloured spines, a hand-lettered price strip |
| Gondola (double side) | 16 x 6 x 10 | 2 x 1 | |
| Peg board | 16 x 1 x 14 | wall | tool or kit silhouettes (plate) |
| Glass display case | 16 x 6 x 7 | 2 x 1 | 2 f `sky` rim on the glass edges, items inside: 6 calculators (2 x 3 x 1), kit boxes |
| Counter segment | 8 x 8 x 7 | 1 | `wood` front, lighter top. A worn patch where the elbows lean. |
| Cash register | 4 x 4 x 4 | on the counter | key plate, a drawer, a bell (2 x 2 x 2) next to it |
| Magazine rack | 8 x 4 x 12 | 1 | 12 magazines in 3 colours |
| Parts-bin cabinet | 8 x 4 x 12 | 1 | 2 x 4 drawers, each with a yellow tag (`mustard` 2 x 1) |
| Workbench | 8 x 8 x 7 | 1 per tile | an anti-static mat plate, an oscilloscope, a soldering iron |
| Oscilloscope | 8 x 7 x 7 | on a bench | emissive green trace screen 5 x 4 |
| Demo table | 8 x 6 x 6 | 1 | a micro (existing CRT model 7 wide) and a price card |
| Stool | 3 x 3 x 6 | | |
| Round table | 6 x 6 x 6 | 1 | two-tone top, a sugar pot |
| Chair | 3 x 3 x 6 | | |
| Booth bench | 14 x 4 x 7 | 2 x 1 | vinyl `rust` or `orange`, a seam line |
| Café counter | 24 x 6 x 8 | 3 x 1 | with 4 fixed stools |
| Coffee urn | 3 x 3 x 5 | on the counter | `steel`, a tap, steam |
| Pie case | 6 x 4 x 6 | on the counter | glass rim, 3 pies |
| Jukebox | 8 x 4 x 12 | 1 | round top in 3 steps, emissive bands in `orange` and `pink` |
| Menu board | 12 x 1 x 8 | wall | dark with 6 chalk lines |
| Wall clock | 5 x 1 x 5 | wall | round, 2 hands (2 f) |
| Pendant lamp | 4 x 4 x 4 | ceiling light | `orange` globe |

**Arcade.**

| Prop | Size (v) | Tiles | Detail |
|---|---|---|---|
| Upright cabinet | 6 x 7 x 14 | 1 | marquee 6 x 2 x 2 lit; screen 4 x 3 emissive (2 colours); angled control panel with a stick and 2 buttons; coin door with a lit slot; side art in 2 stripe colours |
| Cocktail table cabinet | 8 x 8 x 6 | 1 | glass top, a lit screen |
| Pinball | 6 x 14 x 9 | 1 x 2 | a tilted playfield, back glass 6 x 1 x 5 lit |
| Change booth | 8 x 6 x 10 | 1 | a window, a coin tray |
| Bench | 12 x 4 x 5 | 2 x 1 | |
| Neon strip | along a wall | | a 1 v line of `pink` or `teal` at 14 v |

**Civic, school, bank.**

| Prop | Size (v) | Tiles | Detail |
|---|---|---|---|
| Service counter with glass | 24 x 4 x 8 | 3 x 1 | a grille window with a gap |
| Queue post and rope | 1 x 1 x 4, rope 1 f | | in a line of 4 |
| Ticket machine | 3 x 3 x 8 | 1 | a red button, a paper slot |
| Bench row | 12 x 4 x 5 | 2 x 1 | |
| Filing cabinet | 4 x 5 x 10 | 1 | 4 drawers, `moss`/`steel` |
| Typewriter desk | 12 x 6 x 6 | 2 x 1 | |
| Notice board | 10 x 1 x 8 | wall | 6 pinned cards (plate) |
| Flag stand | 2 x 2 x 20, flag 6 x 4 | 1 | |
| Blackboard | 20 x 1 x 10 | wall | `pine` green, 4 chalk lines (plate) |
| School desk and chair | 5 x 4 x 5 | 1 | |
| Teacher's desk | 10 x 6 x 6 | 2 x 1 | |
| Lockers | 4 x 4 x 14 each | 1 each | `denim` or `steel`, vents, a number plate |
| Lab terminal | 6 x 5 x 6 | 1 | era 1: a micro |
| Teller counter | 20 x 5 x 9 | 3 x 1 | brass rail, glass 20 x 1 x 6 |
| Vault door | 10 x 2 x 10 | wall | `steel`, 3 spokes, a dial |

**Clutter (1 to 4 v each, used in clusters).** Books (1 x 3 x 4), magazines, cassettes (1 x 2 x 3), mugs, plates, bottles, shoes (2 x 3 x 1), a clothes pile (6 x 6 x 3), a box (4 x 4 x 3), a plant pot (2 x 2 x 2), a framed photo (3 x 1 x 4), a radio (5 x 3 x 4), a cushion (3 x 3 x 1), paper balls (1 x 1 x 1), a skateboard (8 x 2 x 1), a boom box (10 x 3 x 5), dumbbells (4 x 2 x 2).

### 5.5 Layering front to back

Distance from the back walls (north and west) sets the maximum height of an item.

| Band | Distance from the back wall | Max height |
|---|---|---|
| 0 | touching a back wall | 18 v |
| 1 | 1 tile out | 12 v |
| 2 | 2 to 3 tiles out | 8 v |
| 3 | the two front rows and the two east columns | 5 v |

- Exception: thin tall items (footprint 3 x 3 v or less: a floor lamp, a coat rack) may stand anywhere.
- An item of height h hides about 1.2 x h tiles diagonally behind it. Never put a click target inside that shadow.
- Keep a clear lane of 1 tile to every click target (its approach tile).

**Fill (blocked tiles as a share of floor tiles).** Bedroom 35 to 45 percent. Shop 25 to 35. Café 40 to 50. Arcade 45 to 55. Civic 20 to 30. Classroom 30 to 40. The rest is walking space. This is also why rooms read as lived-in but not crammed.

### 5.6 Lived-in rules

1. **One clutter cluster per 4 floor tiles.** A cluster is 2 to 4 small props in one tile or on one surface.
2. Clusters go **beside** furniture, never in the middle of a walking lane.
3. **Every room has one thing out of place:** a drawer open, a chair pulled out, a sock.
4. **No mirrored pairs.** Two lamps never match. Rugs sit 1 v off true. Wall items hang at mixed heights.
5. Small props may be shifted up to 2 v off the tile centre. Big furniture sits against a wall with a gap of 0 or 1 v.
6. **A light source is visible** in every room (a lamp, a window, a screen).
7. **One story prop** per character background (section 5.8).
8. Surfaces are never empty. A desk top has 4 to 7 items. A shelf has a gap of 1 or 2 items so it looks used.
9. Count: a bedroom has 45 to 60 props in total, 20 of them clutter. A shop has 70 to 90.

### 5.7 Lighting

| Source | Rule |
|---|---|
| **Ambient** | Warm. About 60 percent of day level indoors. |
| **Window light** | One spot light per window, pointed into the room. At most 2 shadowed lights per room. Colour and strength by hour: 06:00 to 09:00 `butter`, 60 percent, low and long; 09:00 to 16:00 `cream`, 100 percent; 16:00 to 19:00 `orange`, 80 percent; 19:00 to 22:00 off (lamps only); 22:00 to 06:00 `night` lightened to 6070B0, 20 percent, cool. |
| **Window patch** | A painted decal on the floor: the window's shape projected along the sun, with the window bars as 2 f dark lines. 12 x 18 v at noon, up to 30 v long at 08:00 and 17:00. 4 hourly variants. 20 percent lighter than the floor. |
| **Lamps** | A desk or floor lamp is a point light, `butter`, range 4 tiles, no shadow, plus a soft floor pool (a lighter ellipse) and a faint cone up the wall (reference 5's lamp). A switch plate (1 f) by the door toggles the room light. |
| **Screens** | Emissive plates. TV in 1978 is black and white: two greys that flicker over 2 frames. A micro screen is green. Each has a small cool point light of range 2 tiles at low strength. |
| **Ceiling light** | One point light at 2.6 m (today's shop light). It is the only extra shadowed light. |
| **Night** | The room darkens to 40 percent. Lamp pools and screens carry the picture. Windows show `night`. |
| **Rain** | Window view goes `steel` grey. Drip plates on the glass (2 frames). Room ambient drops 10 percent. |

### 5.8 How a room tells the character's story

**Background props** (choose 3 to 5):

| Background | Props |
|---|---|
| Muscle head | dumbbells (2), a pull-up bar plate in the doorway, a protein tub (2 x 2 x 3), a sports poster, a gym bag |
| Nerd | two book stacks (5 x 4 x 5), parts trays on the desk, a chessboard (4 x 4 x 1), a pile of computer magazines, a slide rule |
| Art kid | an easel (8 x 6 x 14) with a canvas plate, paint tins, six pinned drawings (plates), a pencil cup |

**Family wealth.**
- Poor: a bare bulb (3 x 3 x 3), a patched quilt, a cracked pane (a 2 f line plate), a bucket in the corner.
- Average: the layout in 5.10.
- Rich: a second TV, a bigger rug, a solid desk, a hi-fi (10 x 4 x 6), a guitar leaning on the wall.

**State props** (a few of these change with the game):

| Game state | Prop |
|---|---|
| Debt | a pile of bills on the desk (3 x 4 x 2) with red-cornered plates. One more pile per major bill. |
| Sleep debt | a blanket on the floor, empty mugs (1 per night skipped, up to 6) |
| Junk food | a pizza box (5 x 5 x 1) on the floor |
| First game finished | a cassette on a shelf with a hand-written label plate |
| Published | a framed review on the wall (4 x 1 x 5) |
| Grounded or family rules | a note on the door (2 x 1 x 3) |
| Early success | a trophy, a second machine |

**The machine slot** (era 1 bedroom, the desk's right half). The design note says the start is "a weak laptop". That does not fit 1978, so the slot evolves: **S0** typewriter, graph paper and a pocket calculator (the player designs on paper); **S1** a kit board and a portable TV; **S2** a beige micro, the TV and a cassette recorder; **S3** a better micro and a second tape deck. See the open question in section 8.

**Species fit** (later): small species get a step stool and a lower desk (4 v). Big species duck at door frames and get a longer bed. Build a `size_class` (S, M, L) hook that scales bed length and desk height by 0.7, 1 and 1.3.

### 5.9 Map format extension

The current room map is one character per 1 m tile, top row north, walls on the north and west edges, with `D` for the door and `@` for the arrival tile. We keep all of that. Add:

```
N: <one character per tile along the north wall, west to east>
W: <one character per tile along the west wall, north to south>
heights:        (optional block, same size as the grid)
<rows of 0, 1, 2 or s>
```

Lines starting with `N:` or `W:` are wall strips and must not be read as map rows.

**Grid letters.** Global, so every room shares them.

| Char | Meaning | | Char | Meaning |
|---|---|---|---|---|
| `.` | floor | | `b` | bed (2 tiles in a column) |
| `@` | arrival tile (a doormat sits here) | | `n` | bedside table |
| `D` | door (walkable) | | `w` | wardrobe (2 tiles) |
| `S` | tall shelf | | `d` | desk (2 tiles) |
| `M` | magazine or tape rack | | `h` | desk chair (walkable) |
| `B` | parts-bin cabinet | | `r` | bookcase |
| `W` | workbench | | `c` | chest of drawers |
| `C` | demo computer table | | `l` | floor lamp |
| `G` | glass display case | | `P` | plant |
| `K` | counter | | `g` | rug or runner (walkable decal) |
| `k` | shopkeeper spot | | `^ > v <` | stairs rising toward north, east, south, west |

A run of the same letter in a row or a column is one object spanning those tiles (as building footprints already work).

**Wall strip letters** (separate from the grid):

| Char | Meaning | | Char | Meaning |
|---|---|---|---|---|
| `-` | plain wall | | `q` | peg board |
| `W` | window (a pair `WW` is one 12 v window; a single `W` is a small one) | | `m` | mirror |
| `p` | poster | | `n` | notice board |
| `s` | wall shelf | | `b` | blackboard |
| `c` | clock | | `a` | open doorway |
| `k` | calendar or card | | `D` | door (must match the grid) |

**Heights block:** `0` ground, `1` +3 v, `2` +8 v, `s` -3 v.

### 5.10 The 1978 bedroom (average family home)

6 x 5 tiles. Room zoom 48 px per metre (width + depth is 11). 30 tiles, 11 blocked (37 percent).

```
# Bedroom, 1978, average family home. 6 x 5 tiles. Top row is north.
# Walls stand on the north and west edges. The camera looks from the south-east.
bnwwdd
b...h.
r.gg..
D@gg.l
c....P
N: -kc-WW
W: pspDm
```

What is where:

| Tile (x, z) | Object | Notes |
|---|---|---|
| (0,0), (0,1) | bed | along the west wall, the head in the north-west corner. Band poster `p` above the head, a shelf `s` over the bed. |
| (1,0) | bedside table | calendar `k` above. A lava lamp, a flip clock, a glass. |
| (2,0), (3,0) | wardrobe | boxes on top. One door open a finger, a sleeve showing. |
| (4,0), (5,0) | desk | under the window `WW` (x 4 to 5). Sill at 9 v, desk top at 6 v. Curtains on both sides. |
| (4,1) | desk chair | pulled out 1 v |
| (0,2) | bookcase | against the west wall, facing east. A poster `p` above it. |
| (2,2), (3,2), (2,3), (3,3) | rug | 16 x 16 v, `rust` with a `butter` border. The player starts on it. |
| (0,3) | door | in the west wall. A coat rack by it. |
| (1,3) | arrival | doormat 8 x 5 |
| (0,4) | chest of drawers | mirror `m` above. A boom box on top. |
| (5,3) | floor lamp | `butter` shade |
| (5,4) | plant | small, 8 v tall (front band) |

**Dressing the desk (16 x 8 v, top at 6 v), state S0:**
- Left tile: a typewriter (5 x 5 x 3) with a sheet in it, 2 v from the left and 2 v from the back. A pencil cup.
- Right tile: a graph-paper pad (4 x 5) with a sprite sketch, a pocket calculator, a mug. There is no TV in the room until S1 (it is a hand-me-down portable).
- Back right: a desk lamp (3 x 3 x 5) with a point light.
- Under the desk: a waste bin (3 x 3 x 4) with 3 paper balls around it.

**State S1 and S2:** the TV (10 x 9 x 9) sits on the right tile. S1 adds a kit board in front of it. S2 replaces the board with a beige micro and puts a cassette recorder on the left tile.

**Floor and wall:** parquet blocks, `wood` and `wood-l`. Wallpaper `sand` and `plastic` stripes. Skirting `umber`. Floor clutter (4 clusters): slippers by the bed, a clothes pile beside the wardrobe, a magazine by the chair, a skateboard on its side near the rug.

**Wall items (7):** a band poster over the bed (west wall, z 0), a shelf with a model rocket and 3 books (west, z 1), a space poster above the bookcase (west, z 2), a mirror above the chest of drawers (west, z 4), a calendar over the bedside table (north, x 1), a clock above the wardrobe (north, x 2, at 18 v up), and a note on the door.

**Light:** window light on the desk and floor by day. A desk lamp pool on the desk and a rug pool from the floor lamp by night. One ceiling light at 2.6 m.

**Variants.** Bed: a bunk or a double for a richer roll. Floor: carpet for a poorer roll. Wallpaper: 3 patterns. Window view: a street, a garden, a brick wall. Background and wealth props from 5.8.

### 5.11 The 1978 computer shop

12 x 8 tiles. Room zoom 24 px per metre (width + depth 20). 96 tiles, 24 blocked (25 percent).

```
# Computer shop, 1978. 12 x 8 tiles. Top row is north.
# Walls stand on the north and west edges. The camera looks from the south-east.
SSSS.MMBBWWW
............
.C....C.....
....C....GG.
............
D@ggggggKKK.
........Kk..
P.......K..P
N: ppppcppssqqq
W: n-WW-D--
```

What is where:

| Tiles | Object | Notes |
|---|---|---|
| (0..3, 0) | 4 tall shelves | boxed tapes in clear bags on the top two levels, manuals below. A hand-lettered price strip on each level. Posters `p` above. |
| (4, 0) | a gap | a clock `c` above. The only empty wall slot: it rests the eye. |
| (5, 0), (6, 0) | magazine racks | 12 magazines each. Posters above. |
| (7, 0), (8, 0) | parts-bin cabinets | 2 x 4 drawers, yellow tags. Wall shelves `ss` above with sample boxes. |
| (9..11, 0) | workbench | an oscilloscope (green trace), a soldering iron (a wisp of smoke, an FX sprite), 4 parts trays, an anti-static mat, an open micro with its lid off. Peg board `qqq` above with tool silhouettes. An articulated lamp (3 x 3 x 10) with a pool. |
| (1,2), (6,2), (4,3) | 3 demo tables | staggered, so no two share a diagonal. Machines: an all-in-one with a built-in tape deck; a keyboard micro with a monitor; a grey desktop with a mono screen and two disk drives. A price card on each. |
| (9,3), (10,3) | glass display case | 6 calculators with red LEDs, kit boxes, chips in foam. |
| (8..10,5), (8,6), (8,7) | counter (an L) | the till and bell on the corner, a magazine stack, a pen cup. A worn patch where elbows lean. |
| (9,6) | the shopkeeper | inside the L. A different species from the player. An apron. |
| (2..7, 5) | runner | a worn path from the door to the counter. A lighter streak at the door. |
| (0,5) | door | in the west wall. A bell above (2 x 2 x 2). |
| (1,5) | arrival | doormat |
| (0,7), (11,7) | plants | small, in the front band |
| West wall z 2 to 3 | shop window `WW` | the shopfront's back. 2 tile wide display stand outside it. |
| West wall z 0 | notice board `n` | cards: "WANTED" and "FOR SALE" (placeholder text plates). |

**Floor and wall:** lino check `putty` and `clay` (4 v squares), plus the runner and wear plates. Walls: panelling to 10 v in `wood`/`wood-d` on the north wall behind the shelves, plaster `sand` above. The west wall is brick-coloured `brick` plaster. Ceiling tubes: two lit tubes (16 x 2 x 1) over the aisle, one flickering about every 30 s.

**Wall items (12):** 4 posters over the shelves, a clock, 2 over the racks, 2 wall shelves, a peg board, a notice board.

**Light:** the window by the door gives day light. The shop's ceiling light stays on during opening hours. The workbench lamp and screens glow. At closing time (18:00) the sign light goes off and the room darkens to the lamp and screen light.

**Story tells:** a hand-lettered "NO FOOD OR DRINKS" plate by the demo tables; the owner's chair behind the counter with a cushion; a cassette taped to the till labelled "local games" (the zip-lock tape start of the game).

### 5.12 Other 1978 interiors

| Room | Size (tiles) | Zones and layout | Tells |
|---|---|---|---|
| **Café** | 10 x 7 | A counter along the north wall (24 v, 4 stools). A raised booth platform (+3 v) on the west side with 3 booths (14 v benches, brass rail). Window tables along the east. A jukebox in the north-east corner. Floor: lino check in `orange` and `plastic`; the booth platform is carpet. | steam from the urn, pendant `orange` lamps, a chalk menu, a pie case |
| **Arcade** | 12 x 9 | Two rows of 6 upright cabinets back to back down the middle. 4 pinballs along the north wall. A cocktail table by the window. A change booth near the door. A +3 v prize platform in the north-east. Carpet `night` with the diamond pattern. Walls black with neon strips. | every screen glows, the floor reflects (a lighter plate under each cabinet), coloured bulbs |
| **Bank** | 10 x 8 | A teller counter across the north (20 v, glass and a brass rail). Queue posts. A vault door in the north wall. 2 desks on the east side. A potted palm. Floor: stone tile `putty`/`clay`. | the vault, the brass rail, a banker's green lamp |
| **Municipality office** | 12 x 8 | Counters with glass along the north. A ticket machine by the door. 3 bench rows facing the counters. Filing cabinets on the west wall. Notice boards. Floor: lino check in `moss` and `putty`. | a number display, a flag in the corner, a long queue rope |
| **Classroom** | 10 x 8 | A blackboard on the north wall, a teacher's desk, 4 rows of 5 school desks. A map poster. Floor: plank. | chalk dust on the floor in front of the board, a globe |
| **School corridor** | 14 x 4 | Lockers along the west wall. Notice boards. Floor: lino check in `clay` and `steel`. | a lost shoe, a poster for a dance |
| **School computer lab** | 8 x 6 | 4 lab terminals on a long table along the north wall. A teacher's desk by the door. | "NO FOOD" plate, one machine switched off, a locked cabinet |
| **Home living room and kitchen** | 8 x 7 | A sofa and the family TV on the north wall. A rug. A kitchen run on the west wall (fridge, cooker, sink). A dining table. Floor: carpet in the living area, lino in the kitchen (a step between). | a rotary phone on the sideboard, family photos, a note on the fridge (the home rules) |

### 5.13 Later eras: how rooms change

| Era | Workplace | Floor | Walls | Lights | Machine props |
|---|---|---|---|---|---|
| 1 Bedroom | the bedroom | parquet, carpet | wallpaper | window, lamp | typewriter, kit micro, TV, cassette |
| 2 Cartridge wars | a garage | concrete, with oil stains | brick, plywood | bare bulb, tubes | a console on the TV, cartridges, a joystick |
| 3 Disc and 3D | a rented office | carpet tile | partitions with fabric panels | ceiling tubes | PCs with CRTs, CD towers |
| 4 Online | an open-plan floor | grey carpet tile | glass partitions | panel lights | monitors, a server rack, cable trays |
| 5 Pocket | a tower floor | polished concrete, wood strips | glass walls with a view | LED strips | laptops, phones on stands, a coffee bar |
| 6 Drought | the same, worn | cracked tiles, patched | taped panels | half the tubes off | old machines kept alive, spares on shelves, a heater |
| 7 After | your campus | mixed | murals, plants | warm LEDs | mixed old and new, a repair bench |

---
## 6. Palette guidance per era

### 6.1 Rules

These come from the Refactoring UI colour rules and from pixel-art practice.

1. **Think in HSL, not hex.** Pick hue, then saturation, then lightness.
2. **Ramps hue-shift.** Going lighter, rotate toward yellow. Going darker, rotate toward blue or violet (at most 20 to 30 degrees). Raise saturation at both ends so shades do not look washed out.
3. **Tinted greys.** The era 1 greys lean warm (yellow or orange). No pure black or pure white. The darkest colour is `void` `1C1714`. The lightest is `cream` `F5F1EA`.
4. **70 percent muted, 30 percent accent** (by area on screen). Accents are small and sit on signs, awnings, clothes and doors.
5. **Each material has 2 or 3 steps.** A base, a shade and (sometimes) a light accent. A fourth only for skin, glass and metal.
6. **Signs and text pass 4.5 to 1** (3 to 1 for very large letters). Pair them as in section 4.4.
7. **Never rely on colour alone.** A kind of place is also a shape (pictogram, tower, awning pattern).
8. **Warm light, cool shadow.** Era 1 shadows lean blue-violet. Lamps are orange.
9. **A new colour must earn its place.** Add it only if no swatch in the era list can do the job.

### 6.2 How the limit is kept (the palette lock)

- Models store palette **indices**, not RGB. The renderer maps (index, light step) to a colour from the **current era's palette**. Light steps: lit, side, shade, shadow.
- The palette has **four time keyframes** (dawn, noon, dusk, night). The clock mixes between them. Lamps and screens are emissive and pick colours from the same list.
- Era change = a new palette. The models do not change. That is how arcades turn into internet cafés with new colours and the same shapes.
- The storyteller's chill (section 3.8) is a small shift on the keyframes.
- A **palette lint** script checks every model and plate against the current era's list and prints unknown colours.

### 6.3 Era 1 starter swatches (the bedroom era, about 1978)

On-screen budget: **60 colours per time keyframe** = 32 world swatches + 12 cast swatches + 16 shade tones. (The UI uses its own 16; the first 8 below are shared with the UI palette so the world and the beige windows match.)

**World swatches (32).**

| # | Name | Hex | Role |
|---|---|---|---|
| 1 | void | `1C1714` | deepest shadow, backdrop, outline floor |
| 2 | ink | `2B2420` | outlines on light, tyres, iron, door gaps |
| 3 | umber | `52463D` | room plinth, dark wood, dark carpet |
| 4 | taupe | `6B5F55` | concrete shade, building plinth |
| 5 | clay | `B8AE96` | stone, pavement joints |
| 6 | putty | `D4CAB0` | pavement, plaster |
| 7 | plastic | `E8E0CC` | trim, lit plaster, kerb stone |
| 8 | cream | `F5F1EA` | highlights, paper, letters |
| 9 | asphalt-d | `454750` | gutter, deep road, gravel roofs |
| 10 | asphalt | `5A5C62` | road |
| 11 | steel | `8E949C` | metal, drains, car trim |
| 12 | wood-d | `6A4A32` | dark wood, trunks |
| 13 | wood | `9A6A44` | furniture, floors |
| 14 | wood-l | `C49A6A` | light wood, floors |
| 15 | sand | `D2BC8C` | plaster, dirt paths, school brick |
| 16 | rust-d | `8A3A22` | deep brick, rust shade |
| 17 | rust | `A84E33` | roofs, awnings, carpets |
| 18 | brick | `C0684A` | brick walls |
| 19 | salmon | `E09A78` | wallpaper, doors, cloth |
| 20 | orange | `D8802E` | cafés, cars, lamp shades |
| 21 | mustard | `D9A93A` | brass, cars, plates |
| 22 | butter | `F2D888` | lit windows, lamp glow |
| 23 | grass | `7CA85A` | lawns |
| 24 | moss | `6B9148` | banks, hedge shade, carpets |
| 25 | leaf | `4E8A44` | tree and plant leaves |
| 26 | pine | `36663A` | deep leaves, blackboards |
| 27 | denim | `35618A` | civic, cloth |
| 28 | glass | `4E6E86` | window glass |
| 29 | sky | `9FC7D8` | sky, glass lights |
| 30 | night | `232B4A` | night sky, arcade walls |
| 31 | teal | `2E7E7A` | computer shop accent |
| 32 | pink | `D8607A` | arcade neon, a rare accent |

**Cast swatches (12).** Match `animal.rs`.

| Group | Hex |
|---|---|
| Fur | fox `E07A3F`, cream fur `F4E3C9`, rabbit fawn `C8B49A`, raccoon grey `8E8C92`, bear brown `7A5236`, dark fur `3A2A22` |
| Cloth | red `C84B4B`, blue `3E6EA8`, green `5A8F5A`, yellow `E6C04E`, violet `7A5CA6`, white `EEEEE8` |

Trousers use the world swatches `night`, `umber`, `ink` and `steel`.

**Shade tones (16).** For the swatches that have no darker partner in the list. Starter values by hand. **Measure them from the renderer** (render each swatch on a lit cube at noon, dusk and night, record the result) and replace these.

| Swatch | Shade | | Swatch | Shade |
|---|---|---|---|---|
| putty | `A89C94` | | orange | `A85A30` |
| plastic | `C0B4A6` | | mustard | `A8803A` |
| cream | `D0C8BE` | | salmon | `C07A68` |
| clay | `8C8078` | | butter | `C8A860` |
| steel | `626A7C` | | sky | `6F8FA8` |
| sand | `AE9470` | | glass | `3A4C68` |
| brick | `8E4A44` | | denim | `28426A` |
| teal | `1D4E5B` | | pink | `A84860` |

Ramp partners already in the list: asphalt to asphalt-d, wood to wood-d, wood-l to wood, rust to rust-d, grass to moss, moss to leaf, leaf to pine.

**Light colours (they come from the list above).**

| Source | Colour |
|---|---|
| Sun at noon / low sun | `FFF6E8` / `FFB070` (as in `clock.rs`) |
| Ambient at noon / night | `D8E4F0` / `5060A0` (as in `clock.rs`) |
| Sodium street lamp | `F2A850` glow, light `FFD28A` |
| Lit window | `butter` |
| TV (black and white) | `plastic` and `steel`, 2 frames |
| Micro screen | `grass` with a `cream` highlight row |
| Neon and arcade bulbs | `pink` with a `cream` core, `butter` bulbs |
| Moon through a window | `6070B0` |

### 6.4 Time keyframes

| Keyframe | Hours | Sky | Palette grade |
|---|---|---|---|
| Dawn | 05:30 to 07:30 | night to `E8A09A` to `sky` | multiply `FFD0B8`, value minus 10 percent |
| Noon | 09:00 to 16:00 | `sky` `9FC7D8` | none |
| Dusk | 17:30 to 19:30 | `E8A86A` | multiply `FFC890`, value minus 8 percent |
| Night | 21:00 to 04:30 | `1E2440` | multiply `6A7AB0`, value minus 35 percent. Emissives stay at full strength. |

The sky values match `clock.rs` today.

### 6.5 Growth by era

On-screen budgets are guides. Each era adds swatches and the shade tones for them.

| Era | Budget | Look | Starter additions |
|---|---|---|---|
| 1 Bedroom | 60 | limited, warm, faded film. Hue +4 degrees toward orange, saturation minus 8 percent. | the 32 + 12 + 16 above |
| 2 Cartridge wars | 96 | punchy: neon and plastic. Saturation plus 10 percent. | magenta `E0388A`, electric blue `2F6FE0`, cyan `38C8D8`, lime `A8D830`, violet `7A5CA6`, laminate white `F8F8F4`, plastic black `24222A`, signal red `F05A28` |
| 3 Disc and 3D | 128 | glossy, a little cooler | teal-grey `5E8A8C`, purple `5A3A8A`, chrome `C8D0D8`, mint `9AD8B8`, glass-blue `6AA8D8`, deep navy `14182E`, crimson `C02860`, warm white `FFF4D8` |
| 4 Online | 176 | clean and flat, cool greys | clean white `F2F4F6`, cool greys `3C4450`, `7A8694`, `B8C2CC`, brand blue `1E6FD8`, signal green `2EB872`, amber `F0A020`, LED white `E8F4FF` |
| 5 Pocket | 256 | rich, soft, pastel and dark mode | peach `F8C8B0`, lilac `C8B8F0`, aqua `A8E8E0`, butter-yellow `FFE88A`, dark greys `1E2028`, `2E323C`, `4A505C`, cool LED street light `E0F0FF` |
| 6 Silicon Drought | 256, graded | the same colours, worn: saturation minus 15 percent, value minus 8 percent on warm colours, a cool shift of 6 degrees | dust ochre `B89860`, rust brown `7A4A30`, soot `33302E`, faded blue `6C8298` |
| 7 After | 256 and more | warmth returns, freshness | fresh green `5CC060`, patina teal `4FA89A`, plus a warm light |

### 6.6 What changes first

If the designer wants era 1 to feel harder, cut the shade tones to 8 and the cast to 6 (a 46 colour budget) and keep the 4 time keyframes. Do not cut the world swatches: the street needs the ramps.

---

## 7. Build order and what not to do

### 7.1 Build order (biggest visual jump first)

Sizes: S = 1 to 2 days, M = 3 to 5 days, L = 1 to 2 weeks. This is engineer time, including the art Claude makes with the tools.

| # | Step | What the player sees | Size | Needs |
|---|---|---|---|---|
| 1 | **The gate: model preview tool, palette-indexed grids and the 3-part spike** (section 2.6) | Nothing yet. It stops us from making weeks of art at the wrong resolution. | M | none |
| 2 | **The look pass:** selective outline, palette-lock shader with 4 light steps, baked corner shadows, shadow tuning. Apply to today's street and shop. | The same world looks about twice as rich with no new models. | M | 1 |
| 3 | **Street skeleton:** 6 tile road, 1 v kerb step, markings and crossing, pavement slabs and wear plates, building relief (plinth, 2 v string courses, cornice, recessed W1 windows), parapets, roof clutter set for 1978 (aerials, chimneys) | A real street instead of boxes on a stripe. | M | 2 |
| 4 | **Storefront kit and the 7 buildings of 1978:** formula, sign slots, kind colours, pictograms, awnings, 12 pieces of street furniture, sodium lamps | Each place is recognisable at a glance. Signs and awnings add colour. | L | 3 |
| 5 | **Fine animals:** fox first (then rabbit, raccoon, bear), the 12 outfits, snapping, 6 poses; then pedestrians with schedules, 3 vehicle types, parked cars | The characters look like reference 5's. The street has people. | L | 1, 2 |
| 6 | **Rooms v1:** the bedroom and the shop with fine props, floor and wall recipes, window view and window light, lamp pools, lived-in dressing, the machine slot states S0 to S3 | The interiors look like reference 4 and 5. | L | 1, 2 |
| 7 | **Night:** 3 lit-window schedule groups, shutters at closing, arcade bulb chase, lamp pools on the street, moths | The city comes alive at dusk. | S | 4 |
| 8 | **Split levels, stairs, rails and height-map pathing** (café booths, school steps, bank steps) | The reference 5 floor plan. | M | 6 |
| 9 | **Era palettes and the chill grade:** the four keyframes, the palette lint script, then the first era change test | The palette grows with the eras and the world can turn cold. | S | 2 |
| 10 | **Life polish:** laundry, ivy, murals and billboards, posters, desire paths, smoke, birds, flags, rain puddles | The street feels lived in. | M | 4, 5 |
| 11 | **Rooms v2:** café, arcade, bank, municipality, school rooms, home living room | The rest of the era 1 places. | L | 6, 8 |
| 12 | **Era 2 kit** (video rental, neon, consoles) | The first era change. | L | 9, 11 |

**Top five (the biggest jumps):** 1 the gate, 2 the look pass, 3 the street skeleton with the storefront kit (steps 3 and 4), 4 fine animals (step 5), 5 the first two rooms (step 6).

### 7.2 What not to do (cost traps at this resolution)

| Trap | Why it costs | Do instead |
|---|---|---|
| Fine voxels for ground, walls and roofs | the street ground becomes 35 MB, one building 130 MB, with slow meshing | coarse shells, fine modules, plates |
| Features of 1 f width on a 2:1 diagonal | they break into dashes and flicker | 2 f minimum |
| One mesh per window or prop instance | thousands of meshes and draw calls | one mesh per type and variant. Clone the handle. |
| One entity per lit window | the same problem | one lit-pane mesh per building per schedule group, 3 shared materials |
| Dithering in the world | a screen-space pattern shimmers when the camera moves | flat steps. Dither only for the building fade and for UI. |
| Alpha glass | sorting errors and cost | opaque glass with a light rim and visible contents |
| Letters smaller than 3 x 5 v (or 7 x 11 f) | unreadable noise | pictogram first. Scribble lines for small text. |
| 8-direction sprites for animals | 112 frames per cast type, no sun, no shadow | fine voxels with painted views |
| A sprite-stack renderer | a second renderer, fake light | no |
| 1/32 m voxels | below a pixel. Noise, memory. | 1/24 m at the smallest |
| Hand-placing noise voxel by voxel | slow, unrepeatable | `speckle`, `wear`, `scatter` with seeds |
| Judging pixel art from code | the result is usually wrong | always render and look (preview tool) |
| Tall buildings next to walkable ground | they hide the street and the player | at most 3 floors there, fade, or set back 10 tiles |
| Lamps with shadows in the street | many shadow maps | street lamps without shadows. Rooms: 2 shadowed lights at most. |
| 30 unique pedestrians | 30 unique meshes | 12 outfits per species, shared meshes |
| Over-animating ambient life | cost for nothing | birds 2 frames, flags 2 frames, smoke 4 puffs |
| Too many saturated accents | the street becomes noise | the 70/30 rule. Two saturated colours per sign. |
| Palette creep | the era look blurs | new colours go through the lint list |
| Rooms larger than needed | empty tiles cost the same as full ones | the fill targets in 5.5 |
| Copying a reference | rights, and a look that is not ours | principles only. Invented names and layouts. |
| Weather as an afterthought | flat or wrong light | weather changes ambient, palette grade and ground, nothing else |

---

## 8. Assumptions and open questions

Each item says what this file assumes. The designer can overturn any.

1. **The starting machine.** `GAME_DESIGN.md` says you start with "a weak laptop that can only browse the web". A laptop does not fit 1978. This file assumes the bedroom starts with a typewriter, graph paper and a calculator (S0) and the machine arrives later (S1 to S3). **Question:** is that right, or should the first machine be a hand-me-down micro?
2. **Pigeons.** Pigeons are in the cast as people. Ambient pigeons in the street could be confused with them. This file assumes ambient birds are small brown sparrows and, at docks, gulls. **Question:** do you want ambient pigeons anyway?
3. **Pets.** No pets. Animals are people. A bird or an insect is ambient only.
4. **Traffic keeps right**, in a generic temperate city (kerbs, pillar post boxes, yellow edge lines). The reference's country is unclear. **Question:** left or right?
5. **Smoking props.** Not used in era 1 (no ashtrays, no cigarette machine). A sweets vending machine stands in. Add them if you want strict period accuracy.
6. **How limited is limited?** 60 colours per keyframe for era 1, against the UI's 16. Tunable (section 6.6).
7. **The fine unit** (1/24 m or 1/16 m) is decided by the spike. Every size in this file stays valid.
8. **Street zoom steps change** (section 2.9): 32 px per metre goes away. Far zoom uses coarse LOD only.
9. **Tall buildings.** This file limits them next to walkable ground and fades them. The alternative is no limit and a turn-the-camera habit.
10. **Sign names** ("MICROS", "THE CUP", "COIN-OP", "SAVINGS", "TOWN OFFICE", "HIGH") are placeholders. Rename freely. Do not reuse sign texts from the references.
11. **Era dates** are not fixed in `GAME_DESIGN.md`. This file only says era 1 is about 1978.
12. **Q and E** keep turning in quarter turns (four resting views), as in `gameplay.rs` today.
13. **Wall windows in rooms** show a flat painted view that changes with the clock, not an animated outdoors.

### Where files go

- Generators and Aseprite scripts: `art/` (like `art/clawd_pixel.lua`).
- Models and plates the game reads: `assets/models/` (text format v2).
- Room maps: `assets/maps/` (with the `N:`, `W:` and `heights:` extension).
- This file: `design/art/WORLD_ART_DIRECTION.md`.
