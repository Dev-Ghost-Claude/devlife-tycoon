//! Your family's house, 1978: the first building in the new street style (after monogon's
//! Voxel Noir and Medieval Town packs). A two-storey white-rendered house on a brick
//! plinth; a rust-tiled gable roof with a chimney and a TV aerial; a bay window, a porch
//! canopy over a salmon door, sills under every window.
//!
//! The shell, windows and roof are coarse voxels (1/8 m), the scale where roof steps read
//! as chunky voxel tiles. Small things (the aerial, the door number, the drainpipe) are
//! fine voxels (1/24 m).

use bevy::prelude::*;

use crate::voxel::VoxelGrid;

fn c(hex: u32) -> Color {
    Color::srgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

const BRICK: u32 = 0x9A4E3A;
const BRICK_DARK: u32 = 0x8A4434;
pub(crate) const STONE: u32 = 0xD8CCB4;
const GUTTER: u32 = 0x4A4A4E;
/// Plaster under the roof, seen where the loft is cut away.
const CEILING: u32 = 0xEADFCB;
/// Where the house is cut open around your bedroom: the section reads at a glance.
pub(crate) const CUT: u32 = 0xF7F2E8;
/// Your house's stairs run up along the inside of the front wall, from this many metres
/// east of the west wall (the bottom, in the hall) to `STAIRS_TOP` (the landing outside
/// your bedroom's doorway). The bedroom draws the steps; the house leaves room for them.
pub const STAIRS_BOTTOM: i32 = 5;
pub const STAIRS_TOP: i32 = 1;
const GLASS: u32 = 0x3A5468;
const GLASS_STREAK: u32 = 0x6A8AA0;

/// How one house looks: wall render, roof tiles, door, window trim and curtains, and two
/// shape choices. Neighbours mix these so a row of houses never repeats.
#[derive(Clone, Copy)]
pub struct HouseStyle {
    pub render: u32,
    pub roof: u32,
    pub roof_dark: u32,
    pub ridge: u32,
    pub door: u32,
    pub trim: u32,
    pub curtain: u32,
    /// A bay window on the ground floor beside the door.
    pub bay: bool,
    /// The chimney at the east end of the ridge (else the west).
    pub chimney_east: bool,
}

/// Your family's house: white render, rust roof, a salmon door.
pub const HOME: HouseStyle = HouseStyle {
    render: 0xF0E6D2,
    roof: 0xA4523C,
    roof_dark: 0x94493A,
    ridge: 0x6E3A2C,
    door: 0xC8705E,
    trim: 0x46617E,
    curtain: 0xD8A84A,
    bay: true,
    chimney_east: true,
};

/// The neighbours: pale blue under slate, butter under brown tiles, sage under dark red.
pub const NEIGHBOURS: [HouseStyle; 3] = [
    HouseStyle { render: 0xC8D8E0, roof: 0x5A6068, roof_dark: 0x4E545C, ridge: 0x3A3E44, door: 0x2F6F6E, trim: 0xF4EFE4, curtain: 0xC86A5A, bay: false, chimney_east: false },
    HouseStyle { render: 0xEED9A0, roof: 0x7A5236, roof_dark: 0x6E4A30, ridge: 0x4A3020, door: 0x3E5A8A, trim: 0x6E4A30, curtain: 0x5A7A4A, bay: true, chimney_east: false },
    HouseStyle { render: 0xB8C8A8, roof: 0x8A3A34, roof_dark: 0x7A3430, ridge: 0x5A2824, door: 0xF4EFE4, trim: 0x4A5A44, curtain: 0xE8C060, bay: false, chimney_east: true },
];

/// Wall height in coarse voxels (two floors of 3 m).
const WALL: i32 = 48;
/// Where the upper floor starts, in coarse voxels.
pub const UPSTAIRS: i32 = WALL / 2;
/// Grid margin around the footprint: the roof overhangs 1 voxel at the gables and 2 at the
/// eaves.
const MARGIN: IVec3 = IVec3::new(1, 0, 2);

pub struct House {
    /// The shell in coarse voxels. Grid (0, 0, 0) sits at `shell_at`, in metres from the
    /// footprint's north-west corner.
    pub shell: VoxelGrid,
    /// Window glass that lights up at night; same grid layout as `shell`.
    pub glass: VoxelGrid,
    pub shell_at: Vec3,
    /// Small parts in fine voxels, each with its position in metres from the footprint's
    /// north-west corner, and whether it goes when the house is cut open around your
    /// bedroom.
    pub fine: Vec<(VoxelGrid, Vec3, bool)>,
    /// Your house only: the roof as seen while you are in your bedroom, cut open at the
    /// corner facing the camera (see [`cut_open_roof`]). Same layout as `shell`.
    pub cut_open: Option<VoxelGrid>,
}

/// Paints into the shell and glass in footprint coordinates (x east, z south, both from
/// the north-west corner), so the margin is handled in one place.
struct Shell {
    g: VoxelGrid,
    glass: VoxelGrid,
}

impl Shell {
    fn set(&mut self, x: i32, y: i32, z: i32, colour: Option<Color>) {
        self.g.set(x + MARGIN.x, y, z + MARGIN.z, colour);
    }

    fn fill(&mut self, min: IVec3, max: IVec3, colour: Color) {
        self.g.fill(min + MARGIN, max + MARGIN, colour);
    }

    fn fill_with(&mut self, min: IVec3, max: IVec3, mut paint: impl FnMut(IVec3) -> Option<Color>) {
        self.g.fill_with(min + MARGIN, max + MARGIN, |p| paint(p - MARGIN));
    }

    fn glass(&mut self, x: i32, y: i32, z: i32, colour: Color) {
        self.g.set(x + MARGIN.x, y, z + MARGIN.z, None);
        self.glass.set(x + MARGIN.x, y, z + MARGIN.z, Some(colour));
    }
}

/// Which wall a window sits in.
#[derive(Clone, Copy)]
enum Face {
    South,
    North,
    West,
    East,
}

/// The house on a footprint of `tiles` (8 coarse voxels each), its door in the south wall
/// `door_tile` tiles from the west edge. `grow` puts the walls that many voxels outside the
/// footprint on every side: your house's, so the bedroom upstairs fills the footprint
/// exactly and the house's walls are the room's walls.
pub fn house(tiles: IVec2, door_tile: i32, style: HouseStyle, grow: i32) -> House {
    let (w, d) = (tiles.x * 8 + 2 * grow, tiles.y * 8 + 2 * grow);
    // From wall coordinates (which start at the outside of the west and north walls) to
    // metres from the footprint's north-west corner.
    let to_footprint = Vec3::new(grow as f32, 0.0, grow as f32) / 8.0;
    let rise = (d + 3) / 2;
    let height = WALL + rise + 14;
    let size = IVec3::new(w, height, d) + 2 * MARGIN;
    let mut s = Shell { g: VoxelGrid::new(size.x as u32, size.y as u32, size.z as u32), glass: VoxelGrid::new(size.x as u32, size.y as u32, size.z as u32) };

    // Walls: a brick plinth with a stone coping, white render above, a stone string course
    // between the floors that stands one voxel proud.
    s.fill_with(IVec3::ZERO, IVec3::new(w - 1, WALL - 1, d - 1), |p| {
        Some(match p.y {
            0..=4 => {
                if p.y % 2 == 0 { c(BRICK) } else { c(BRICK_DARK) }
            }
            5 => c(STONE),
            _ => c(style.render),
        })
    });
    s.fill_with(IVec3::new(-1, 24, -1), IVec3::new(w, 24, d), |p| {
        let outside = p.x == -1 || p.x == w || p.z == -1 || p.z == d;
        // Inside stays solid wall: a hollow layer here gave every house 2,500 faces facing
        // into a sealed gap, where nobody can see them.
        Some(if outside { c(STONE) } else { c(style.render) })
    });

    // The roof: a gable along the house, tile courses in two tones, a dark ridge, gutters
    // along the eaves. Under it, the gable walls are rendered like the walls below.
    for k in 0..=rise {
        let (z0, z1) = (-2 + k, d + 1 - k);
        if z0 > z1 {
            break;
        }
        let y = WALL + k;
        let top = k == rise || z0 + 1 >= z1;
        s.fill_with(IVec3::new(-1, y, z0), IVec3::new(w, y, z1), |p| {
            let edge = p.z == z0 || p.z == z1;
            if !edge && !top {
                // Inside the roof: the gable ends show render, the rest is hidden.
                return Some(c(style.render));
            }
            Some(if top {
                c(style.ridge)
            } else if k == 0 {
                c(GUTTER)
            } else if (k / 2) % 2 == 0 {
                c(style.roof)
            } else {
                c(style.roof_dark)
            })
        });
        // The gable overhang: only the roof surface sticks out past the end walls.
        if !top {
            s.fill_with(IVec3::new(-1, y, z0 + 1), IVec3::new(-1, y, z1 - 1), |_| None);
            s.fill_with(IVec3::new(w, y, z0 + 1), IVec3::new(w, y, z1 - 1), |_| None);
        }
    }

    // The chimney on the east end of the ridge: brick, a stone cap, two pots.
    let (cx, cz) = (if style.chimney_east { w - 12 } else { 6 }, d / 2 - 3);
    let chimney_top = WALL + rise + 8;
    s.fill_with(IVec3::new(cx, WALL - 6, cz), IVec3::new(cx + 5, chimney_top, cz + 5), |p| Some(if p.y % 2 == 0 { c(BRICK) } else { c(BRICK_DARK) }));
    s.fill(IVec3::new(cx - 1, chimney_top, cz - 1), IVec3::new(cx + 6, chimney_top, cz + 6), c(STONE));
    for px in [cx + 1, cx + 3] {
        s.fill(IVec3::new(px, chimney_top + 1, cz + 2), IVec3::new(px + 1, chimney_top + 3, cz + 3), c(0xB8603E));
    }

    // The front (south). West of the door, a bay window on the ground floor.
    let door_x = door_tile * 8 + grow;
    let bay = (3, 13);
    if style.bay {
    s.fill(IVec3::new(bay.0, 0, d), IVec3::new(bay.1, 5, d + 2), c(BRICK));
    s.fill(IVec3::new(bay.0, 5, d), IVec3::new(bay.1, 5, d + 2), c(STONE));
    s.fill(IVec3::new(bay.0, 6, d), IVec3::new(bay.1, 19, d + 2), c(style.render));
    s.fill(IVec3::new(bay.0 - 1, 20, d), IVec3::new(bay.1 + 1, 20, d + 3), c(GUTTER));
    // Its front: three panes in a trim frame, the middle one widest.
    for x in bay.0 + 1..bay.1 {
        for y in 7..19 {
            let frame = x == bay.0 + 1 || x == bay.1 - 1 || y == 7 || y == 18 || x == bay.0 + 3 || x == bay.1 - 3;
            if frame {
                s.set(x, y, d + 2, Some(c(style.trim)));
            } else {
                window_pane(&mut s, x, y, d + 2, false, (x - bay.0) + (y - 7) == 9, style.curtain);
            }
        }
    }
    } else {
        window(&mut s, Face::South, 4, 7, 9, 12, w, d, true, style);
    }
    // The door: a stone frame, a salmon door with two raised panels, a brass knob and a
    // letter slot, a fanlight above, a step and a porch canopy.
    s.fill(IVec3::new(door_x - 1, 0, d - 1), IVec3::new(door_x + 8, 19, d - 1), c(STONE));
    s.fill_with(IVec3::new(door_x, 0, d - 1), IVec3::new(door_x + 7, 14, d - 1), |_| None);
    s.fill_with(IVec3::new(door_x, 0, d - 2), IVec3::new(door_x + 7, 14, d - 2), |p| {
        let panel = (p.x == door_x + 2 || p.x == door_x + 5) && (2..13).contains(&p.y);
        Some(if panel { c(style.door).mix(&Color::WHITE, 0.15) } else { c(style.door) })
    });
    s.set(door_x + 6, 7, d - 1, Some(c(0xE8C86A)));
    s.fill(IVec3::new(door_x + 2, 10, d - 1), IVec3::new(door_x + 5, 10, d - 1), c(0x3A3430));
    for x in door_x..door_x + 8 {
        for y in 15..18 {
            s.set(x, y, d - 1, None);
            s.glass(x, y, d - 2, c(GLASS));
        }
    }
    s.fill(IVec3::new(door_x - 1, 0, d), IVec3::new(door_x + 8, 0, d + 3), c(STONE));
    s.fill(IVec3::new(door_x - 2, 20, d), IVec3::new(door_x + 9, 21, d + 4), c(style.trim));
    s.fill(IVec3::new(door_x - 2, 21, d), IVec3::new(door_x + 9, 21, d + 4), c(GUTTER));
    // East of the door on the ground floor, and both upper windows: framed, sills.
    window(&mut s, Face::South, 29, 7, 10, 12, w, d, true, style);
    window(&mut s, Face::South, 5, 29, 8, 11, w, d, true, style);
    window(&mut s, Face::South, 31, 29, 8, 11, w, d, false, style);
    window(&mut s, Face::South, door_x + 1, 31, 5, 7, w, d, false, style);
    // Back and sides: plainer windows, so every view of the house is finished.
    for (x, y) in [(6, 8), (32, 8), (6, 30), (32, 30)] {
        window(&mut s, Face::North, x, y, 8, 10, w, d, false, style);
    }
    for face in [Face::West, Face::East] {
        window(&mut s, face, 20, 8, 8, 10, w, d, false, style);
        window(&mut s, face, 20, 30, 8, 10, w, d, matches!(face, Face::East), style);
    }

    // Fine parts. The TV aerial on the chimney: a mast, three cross bars of falling
    // length, a dipole at the top.
    let mut aerial = VoxelGrid::new(16, 40, 3);
    aerial.fill(IVec3::new(7, 0, 1), IVec3::new(8, 39, 1), c(0x9A9C9E));
    for (y, half) in [(26, 7), (31, 5), (36, 3)] {
        aerial.fill(IVec3::new(8 - half, y, 1), IVec3::new(7 + half, y, 1), c(0xB8BABC));
    }
    aerial.fill(IVec3::new(5, 39, 0), IVec3::new(10, 39, 2), c(0xB8BABC));
    let chimney_metres = Vec3::new((cx + 1) as f32, (chimney_top + 1) as f32, (cz + 2) as f32) / 8.0;
    let mut fine = vec![(aerial, chimney_metres + Vec3::new(0.2, 0.0, 0.0) - to_footprint, false)];
    // A drainpipe down the front corner, with a hopper at the gutter.
    let mut pipe = VoxelGrid::new(3, (WALL * 3) as u32 + 2, 3);
    pipe.fill(IVec3::new(0, 0, 0), IVec3::new(1, WALL * 3 - 1, 1), c(GUTTER));
    pipe.fill(IVec3::new(0, WALL * 3 - 4, 0), IVec3::new(2, WALL * 3 + 1, 2), c(GUTTER));
    fine.push((pipe, Vec3::new(w as f32 / 8.0 - 0.2, 0.0, d as f32 / 8.0) - to_footprint, true));
    // The house number beside the door: a white plate with a dark 7.
    let mut number = VoxelGrid::new(5, 6, 1);
    number.fill(IVec3::ZERO, IVec3::new(4, 5, 0), c(0xF4EFE4));
    number.fill(IVec3::new(1, 4, 0), IVec3::new(3, 4, 0), c(0x2A2420));
    for (x, y) in [(3, 3), (2, 2), (2, 1)] {
        number.set(x, y, 0, Some(c(0x2A2420)));
    }
    fine.push((number, Vec3::new((door_x + 9) as f32 / 8.0 + 0.05, 1.5, d as f32 / 8.0) - to_footprint, false));

    // Your house: the staircase up to the landing, along the inside of the front wall (the
    // wall and its door and windows stay whole in front of it).
    if grow > 0 {
        let (z0, z1) = (grow + (tiles.y - 1) * 8 + 1, grow + tiles.y * 8 - 2);
        s.fill_with(IVec3::new(grow + STAIRS_TOP * 8, 0, z0), IVec3::new(grow + STAIRS_BOTTOM * 8 - 1, UPSTAIRS - 1, z1), |_| None);
    }
    let cut_open = (grow > 0).then(|| cut_open_roof(&s.g, d, rise, IVec2::new(cx, cz), style.render));

    House { shell: s.g, glass: s.glass, shell_at: -MARGIN.as_vec3() / 8.0 - to_footprint, fine, cut_open }
}

/// The roof of your house while you are in your bedroom, cut open at the corner facing the
/// camera, like a dollhouse: the back slope stays (with plaster under it, so it reads as
/// the room's sloping ceiling), and so do the west gable, the ridge and the chimney; the
/// front slope, the east gable and the loft between are gone. The back slope is steeper
/// than the camera looks down, so it hides nothing in the room. Where the front slope was,
/// the cut shows white. The walls below are left to the bedroom, which has its own.
fn cut_open_roof(full: &VoxelGrid, d: i32, rise: i32, chimney: IVec2, render: u32) -> VoxelGrid {
    let ridge = (d - 1) / 2;
    let size = full.size.as_ivec3();
    let filled = |x: i32, y: i32, z: i32| full.get(x + MARGIN.x, y, z + MARGIN.z).is_some();
    let mut g = VoxelGrid::new(full.size.x, full.size.y, full.size.z);
    for gz in 0..size.z {
        for gy in WALL..size.y {
            for gx in 0..size.x {
                let Some(colour) = full.get(gx, gy, gz) else { continue };
                let (x, y, z) = (gx - MARGIN.x, gy, gz - MARGIN.z);
                // This height's course of the back slope, and whether it is the ridge.
                let k = y - WALL;
                let (z0, z1) = (-2 + k, d + 1 - k);
                let top = k == rise || z0 + 1 >= z1;
                let in_chimney = (chimney.x - 1..=chimney.x + 6).contains(&x) && (chimney.y - 1..=chimney.y + 6).contains(&z);
                let keep = if in_chimney {
                    // The stack above the ridge; below it was inside the loft.
                    (y >= WALL + rise - 2).then_some(colour)
                } else if top {
                    Some(colour)
                } else if z <= ridge && z <= (z0 + 1).max(0) {
                    Some(if z == ridge && filled(x, y, z + 1) {
                        c(CUT)
                    } else if z == z0 || k == 0 {
                        colour
                    } else {
                        c(CEILING)
                    })
                } else if x == 0 {
                    // The west gable stands whole; its edge under the front slope is cut.
                    Some(if z > ridge && colour != c(render) { c(CUT) } else { colour })
                } else {
                    None
                };
                g.set(gx, gy, gz, keep);
            }
        }
    }
    g
}

/// One pane voxel: curtain fabric at the sides, glass (with a light streak) elsewhere.
fn window_pane(s: &mut Shell, x: i32, y: i32, z: i32, curtain: bool, streak: bool, curtain_colour: u32) {
    if curtain {
        s.set(x, y, z, Some(c(curtain_colour)));
    } else {
        s.glass(x, y, z, if streak { c(GLASS_STREAK) } else { c(GLASS) });
    }
}

/// A framed window `wide` x `tall` with its lower-left corner at (`along`, `y`) on `face`
/// (along runs west to east on the south and north faces, north to south on the sides):
/// a trim frame, glass recessed one voxel with curtains at the sides when `lit`, a cross
/// bar, and a stone sill that stands out under it. Unlit windows stay dark glass at night.
#[allow(clippy::too_many_arguments)]
fn window(s: &mut Shell, face: Face, along: i32, y: i32, wide: i32, tall: i32, w: i32, d: i32, lit: bool, style: HouseStyle) {
    // Position on the face: (x, z) of the outer wall voxel, and the step inward and outward.
    let at = |a: i32, depth: i32| -> (i32, i32) {
        match face {
            Face::South => (a, d - 1 - depth),
            Face::North => (a, depth),
            Face::West => (depth, a),
            Face::East => (w - 1 - depth, a),
        }
    };
    let mid = along + wide / 2;
    for a in along..along + wide {
        for yy in y..y + tall {
            let frame = a == along || a == along + wide - 1 || yy == y || yy == y + tall - 1;
            let (ox, oz) = at(a, 0);
            if frame {
                s.set(ox, yy, oz, Some(c(style.trim)));
                continue;
            }
            s.set(ox, yy, oz, None);
            let (ix, iz) = at(a, 1);
            let bar = a == mid || yy == y + tall / 2;
            if bar {
                s.set(ix, yy, iz, Some(c(style.trim)));
            } else if lit {
                window_pane(s, ix, yy, iz, a <= along + 1 || a >= along + wide - 2, (a - along) + (yy - y) == wide / 2 + 1, style.curtain);
            } else {
                let streak = (a - along) + (yy - y) == wide / 2 + 1;
                s.set(ix, yy, iz, Some(if streak { c(GLASS_STREAK) } else { c(GLASS) }));
            }
        }
    }
    // The sill, one voxel proud.
    for a in along - 1..=along + wide {
        let (sx, sz) = at(a, -1);
        s.set(sx, y - 1, sz, Some(c(STONE)));
        let (wx, wz) = at(a, 0);
        s.set(wx, y - 1, wz, Some(c(STONE)));
    }
}
