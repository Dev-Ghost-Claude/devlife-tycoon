#!/usr/bin/env python3
# Generates pc_opus_48.txt: 1980s handle-top home computer + mouse at 1/48 m voxels.
# Coordinates: x west->east, y bottom->top, z back(north, 0)->front(south).
import os

W, D, H = 52, 38, 41
G = {}

def box(x0, x1, y0, y1, z0, z1, c):
    for x in range(x0, x1 + 1):
        for y in range(y0, y1 + 1):
            for z in range(z0, z1 + 1):
                if c is None:
                    G.pop((x, y, z), None)
                else:
                    G[(x, y, z)] = c

def put(x, y, z, c):
    if c is None:
        G.pop((x, y, z), None)
    else:
        G[(x, y, z)] = c

PALETTE = [
    ("M", "#5c6064", "light charcoal: top faces and front edges"),
    ("m", "#45484c", "mid charcoal: monitor body, base deck"),
    ("b", "#333639", "dark charcoal: base sides"),
    ("n", "#1c1e20", "near black: recesses, vents, handle, cable"),
    ("S", "#6f8273", "dim grey-green CRT glass (unlit)"),
    ("g", "#c8cac4", "light grey key caps"),
    ("r", "#c4473b", "red keys"),
    ("G", "#4f9a4a", "green keys"),
    ("u", "#3f6ec0", "blue keys"),
    ("y", "#dcb23c", "yellow keys"),
    ("o", "#eba23a", "yellow-orange slot label and badge"),
    ("e", "#ff3a2a glow", "power LED"),
    ("w", "#8a8e92", "mouse shell"),
]

# ---------------- base (x 0..43, z 0..33, y 0..5) ----------------
BX0, BX1, BZ0, BZ1 = 0, 43, 0, 33
box(BX0 + 1, BX1 - 1, 0, 0, BZ0 + 1, BZ1 - 1, "n")          # recessed foot line
box(BX0, BX1, 1, 4, BZ0, BZ1, "b")                          # body
box(BX0, BX1, 5, 5, BZ0, BZ1, "m")                          # top deck
# light top edges: front, sides, back
for x in range(BX0, BX1 + 1):
    put(x, 5, BZ1, "M"); put(x, 5, BZ0, "M")
for z in range(BZ0, BZ1 + 1):
    put(BX0, 5, z, "M"); put(BX1, 5, z, "M")
# front face: chamfered top edge (step back one)
for x in range(BX0, BX1 + 1):
    put(x, 4, BZ1, "M")
    put(x, 5, BZ1, None)
    put(x, 5, BZ1 - 1, "M")
# side vent slots on both sides (recessed one voxel)
for z in range(4, 31, 2):
    for y in (2, 3):
        put(BX0, y, z, None); put(BX0 + 1, y, z, "n")
        put(BX1, y, z, None); put(BX1 - 1, y, z, "n")
# rear vent slots on the back face, right of the monitor
for x in range(30, 42, 2):
    for y in (2, 3):
        put(x, y, BZ0, None); put(x, y, BZ0 + 1, "n")

# ---------------- keyboard well and keys ----------------
KX0, KX1, KZ0, KZ1 = 2, 41, 17, 31
box(KX0, KX1, 5, 5, KZ0, KZ1, None)
box(KX0, KX1, 4, 4, KZ0, KZ1, "n")

def key(col, row, c, wcols=1, hrows=1):
    x0 = 3 + 3 * col
    z0 = 18 + 3 * row
    x1 = x0 + 3 * wcols - 2
    z1 = z0 + 3 * hrows - 2
    box(x0, x1, 5, 6, z0, min(z1, KZ1), c)

layout = [
    # row 0 (back, function row): colour strip in front of the slot
    ["r", "g", "g", "g", "g", "g", "g", "g", "g", "r", "y", "G", "u"],
    ["r", "g", "g", "g", "g", "g", "g", "g", "g", "g", "g", "g", "g"],
    ["G", "g", "g", "g", "g", "g", "g", "g", "g", "g", "r", "r", None],
    ["G", "G", "g", "g", "g", "g", "g", "g", "g", "g", "u", "g", None],
]
for row, cols in enumerate(layout):
    for col, c in enumerate(cols):
        if c:
            key(col, row, c)
# tall yellow return key, inverted-T blue arrows, red corner key
key(12, 2, "y", hrows=2)
key(0, 4, "G", wcols=2)
key(2, 4, "g", wcols=7)
key(9, 4, "u"); key(10, 4, "u"); key(11, 4, "u"); key(12, 4, "r")

# ---------------- slot housing (back right) ----------------
SX0, SX1, SZ0, SZ1 = 31, 41, 3, 14
box(SX0, SX1, 6, 9, SZ0, SZ1, "m")
box(SX0, SX1, 9, 9, SZ0, SZ1, "M")
box(SX0, SX1, 6, 9, SZ1, SZ1, "m")
for x in range(SX0, SX1 + 1):
    put(x, 9, SZ1, "M")
# cartridge slot opening in top
box(33, 39, 9, 9, 6, 8, None)
box(33, 39, 8, 8, 6, 8, "n")
# recessed label window on front face
box(32, 40, 6, 8, SZ1, SZ1, "n")
box(33, 39, 7, 8, SZ1, SZ1, None)
box(33, 39, 7, 8, SZ1 - 1, SZ1 - 1, "o")

# ---------------- monitor ----------------
# bezel (front frame) x 3..28, y 6..33, z 15..16; body inset behind; CRT hump at back
box(4, 27, 6, 32, 4, 14, "m")                  # body
box(7, 24, 8, 29, 1, 3, "m")                   # rear CRT hump
box(7, 24, 29, 29, 1, 3, "M")
box(3, 28, 6, 33, 15, 16, "m")                 # bezel
box(4, 27, 32, 32, 4, 14, "M")                 # body top
box(3, 28, 33, 33, 15, 16, "M")                # bezel top
# bezel bevel: round the four front corners, light the front perimeter
for y in range(6, 34):
    put(3, y, 16, "M"); put(28, y, 16, "M")
for x in range(3, 29):
    put(x, 33, 16, "M")
for (x, y) in ((3, 33), (28, 33)):
    put(x, y, 16, None)
put(3, 33, 15, "M"); put(28, 33, 15, "M")
# screen: two-step recess, glass two voxels behind the bezel face
box(6, 25, 12, 30, 16, 16, None)
box(6, 25, 12, 30, 15, 15, "n")
box(7, 24, 13, 29, 15, 15, None)
box(7, 24, 13, 29, 14, 14, "S")
for (x, y) in ((7, 13), (24, 13), (7, 29), (24, 29)):
    put(x, y, 14, "n")
# bottom lip under screen opening, light
for x in range(6, 26):
    put(x, 11, 16, "M")
# controls strip: badge, knobs, LED
box(6, 9, 8, 8, 16, 16, "o")
box(17, 18, 7, 8, 17, 17, "n")
box(20, 21, 7, 8, 17, 17, "n")
box(24, 25, 8, 8, 16, 16, "e")
# rear vent slots on the CRT hump
for y in range(18, 27, 2):
    for x in range(10, 22):
        put(x, y, 1, None); put(x, y, 2, "n")
# side vent grilles on the monitor body (both sides), recessed
for y in range(20, 31, 2):
    for z in range(6, 13):
        put(4, y, z, None); put(5, y, z, "n")
        put(27, y, z, None); put(26, y, z, "n")
# carry handle: hinge blocks, posts, bar
box(6, 9, 33, 34, 6, 11, "n")
box(22, 25, 33, 34, 6, 11, "n")
box(6, 9, 34, 34, 6, 11, "b")
box(22, 25, 34, 34, 6, 11, "b")
box(7, 8, 35, 38, 8, 9, "n")
box(23, 24, 35, 38, 8, 9, "n")
box(7, 24, 38, 39, 8, 9, "n")
box(7, 24, 40, 40, 8, 9, "b")

# ---------------- mouse (front right, on the desk) ----------------
MX0, MX1, MZ0, MZ1 = 45, 51, 30, 37
box(MX0 + 1, MX1 - 1, 0, 0, MZ0 + 1, MZ1 - 1, "n")          # sole
box(MX0, MX1, 1, 1, MZ0, MZ1, "w")
for (x, z) in ((MX0, MZ0), (MX1, MZ0), (MX0, MZ1), (MX1, MZ1)):
    put(x, 1, z, None)
box(MX0, MX1, 2, 2, MZ0 + 1, MZ1 - 1, "w")
box(MX0 + 1, MX1 - 1, 2, 2, MZ0, MZ1, "w")
box(MX0, MX1, 3, 3, MZ0 + 1, MZ1 - 1, "w")
box(MX0 + 1, MX1 - 1, 3, 3, MZ0, MZ1, "w")
for (x, z) in ((MX0, MZ0 + 1), (MX1, MZ0 + 1), (MX0, MZ1 - 1), (MX1, MZ1 - 1)):
    put(x, 3, z, None)
box(MX0 + 1, MX1 - 1, 4, 4, MZ0 + 1, MZ1 - 1, "w")
# buttons at the cable (north) end, split by a seam
box(MX0 + 1, MX1 - 1, 4, 4, MZ0 + 1, MZ0 + 3, "M")
box(48, 48, 4, 4, MZ0 + 1, MZ0 + 3, "n")
box(MX0 + 1, MX1 - 1, 3, 3, MZ0, MZ0, "M")
box(MX0 + 1, MX1 - 1, 4, 4, MZ0 + 4, MZ0 + 4, "n")
# cable: out of the mouse nose, curving back into the base side
cable = [(48, 2, 29), (48, 1, 29), (48, 0, 28), (48, 0, 27), (47, 0, 26), (47, 0, 25),
         (46, 0, 24), (45, 0, 23), (44, 0, 23), (44, 1, 22)]
for (x, y, z) in cable:
    put(x, y, z, "n")

# ---------------- write ----------------
out = []
out.append("# Player's home computer, opus_48 showcase: 1980s charcoal micro with a handle-top CRT on a")
out.append("# flat keyboard base, cartridge slot at back right, and a grey mouse on the desk. one voxel is 1/48 m.")
out.append("# Parody only: no logos or text.")
out.append("palette")
for k, v, note in PALETTE:
    out.append(f"{k} = {v}")
for y in range(H):
    out.append("layer")
    for z in range(D):
        out.append("".join(G.get((x, y, z), ".") for x in range(W)))
path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "pc_opus_48.txt")
with open(path, "w") as f:
    f.write("\n".join(out) + "\n")
used = {c for c in G.values()}
print("voxels", len(G), "colours used", len(used), sorted(used))
xs = [p[0] for p in G]; ys = [p[1] for p in G]; zs = [p[2] for p in G]
print("bounds x", min(xs), max(xs), "y", min(ys), max(ys), "z", min(zs), max(zs))
