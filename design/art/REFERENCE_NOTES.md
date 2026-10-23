# Reference notes, Pinterest pass 1 (2026-10-12)

Ideas taken from public voxel and isometric pixel art. Nothing here is copied: no
images were saved, and every rule below is written in our own terms for our own game.

Searches: isometric voxel rooms, MagicaVoxel interiors, voxel animal characters,
voxel humanoid characters, 1970s bedrooms in isometric pixel art, voxel streets at
night, clothed fox characters.

## What makes the best pieces read instantly

### Rooms
1. **The room is a diorama.** Every strong room sits on a visible floor slab a few
   voxels thick, with a darker base edge. The walls show their thickness as a
   lighter cap along the top. The room reads as an object, not a hole in the dark.
2. **Three hues and one accent.** The cosiest rooms use a base of wood and cream
   plus two colours, then repeat one accent three or four times (a coral chair,
   coral pillow, coral box, coral guitar). The repetition ties the room together.
3. **Walls carry half the props.** Posters, a shelf with a trailing plant, a
   hung guitar, a corkboard, a calendar. Empty wall above the furniture line is the
   main thing that makes a room look unfinished.
4. **The floor carries clutter.** Loose papers, a teddy, sneakers, a box. Two or
   three small things break the grid and say someone lives here.
5. **Each floor plank gets its own tone.** Planks alternate between two or three
   close shades with darker seams. Brick and stone walls do the same per brick.
6. **Faces take three values.** Top faces are the lightest, the left face is mid,
   the right face is darkest. This alone makes small objects legible.

### Night
7. **The ambient goes dark and cool; the lights go warm and local.** A desk lamp,
   a window, a candle, a screen. Each throws a pool of light with a soft falloff,
   and the rest of the room falls to deep blue-violet.
8. **Emissive things glow.** Lit windows, screens and bulbs are full bright with a
   small halo. A screen's cool blue against the lamp's warm orange is the classic
   pairing.
9. **A light shaft sells the window.** One soft beam across the floor from the
   window does more than any amount of window detail.

### Characters
10. **A big head.** The most readable characters give the head about 40% of the
    height. Ours is about 30% plus ears.
11. **The fox reads from four things:** tall ears with dark tips, a white mask that
    sweeps from the cheeks up around the eyes, a dark nose at the bottom of the
    mask, and dark "socks" on the paws and feet. The big cream-tipped tail is the
    fifth, in profile.
12. **Eyes are the face.** Larger, set lower than you would expect, with one
    highlight voxel. Big eyes carry expression even at 30 px.
13. **One accent separates head from body.** A neckerchief or collar in a colour
    that contrasts the fur (red on an orange fox) stops the head merging into the
    shirt.
14. **Fabric has a slight per-voxel noise.** Shirts and trousers vary by a shade
    from voxel to voxel. It reads as cloth, not plastic.
15. **Blocky hands and shoes are fine.** Hands are a single block with a dark
    notch; shoes are chunky and darker than the trousers.

### Street
16. **Stacked, lived-in buildings.** Air-con boxes, laundry, water tanks, signs,
    awnings and balconies make each facade distinct.
17. **Every lit window is a different warmth.** Some amber, some white, some off.

## What this means for our build

- Bedroom: add a floor slab and wall caps; pick the room's palette as walnut and
  cream plus mustard and teal, with the fox's orange kept for the fox; hang props
  on both walls; scatter floor clutter; vary the planks.
- Night: dark cool ambient, a warm spotlight from the lamp, a cool screen glow, and
  a soft shaft from the window.
- Fox: bigger head (about 40%), bigger and lower eyes with a highlight, a white mask
  around the eyes, dark paws and feet, and a contrasting neckerchief or collar.
- Outline: the pixel-art pieces outline every object in a dark line; the rendered
  voxel pieces don't. To be decided (see the open questions).

## House cutaways (2026-10-15)

Looked at for the bedroom inside the house (style only, nothing copied):
- Two-storey isometric cutaways (image search, dreamstime): the roof stays on the back half; only the front slope and front walls go, so it still reads as a house. Floors show as thick cut bands.
- Brownstone in its street (Pinterest, AI-modified): only the corner facing the camera is cut out, floor by floor; the facade, stoop and roofline stay whole.
- Voxel "house doll" (Virginie Paulus, via Pinterest): stairs connect the floors; floor sections are thick wooden bands.
- Voxel rooms with white wall tops over a coloured core (Pinterest): the cut reads at a glance.
- Attic bedrooms (Pinterest): the roof slopes as walls, skylights, very cosy (an option the user did not pick).
