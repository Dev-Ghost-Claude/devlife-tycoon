//! The 1978 bedroom: the first room built at fine detail (24 voxels to the metre, one
//! canvas pixel each at zoom 1).
//!
//! Art rules for this room (see design/art/REFERENCE_NOTES.md):
//! - It is a diorama: the floor is a thick slab with a dark base, the walls show a cream cut.
//! - Palette: walnut wood, sage walls and cream, with mustard as the accent repeated around
//!   the room (curtains, cushion, blanket, records) and teal second (rug, poster, books).
//!   Orange belongs to the fox, so nothing else in the room is orange.
//! - Walls carry half the props; the floor carries a little clutter.
//! - Every model reads from its silhouette first; the outline pass draws the linework.

use std::collections::HashMap;

use bevy::{light::NotShadowCaster, prelude::*};

use crate::{
    Screen,
    fine_animal::Expression,
    house::{CUT, HOME, STAIRS_BOTTOM, STAIRS_TOP, STONE, UPSTAIRS},
    voxel::{FINE, Occupancy, VoxelGrid, hash},
    world::{Action, Area, AreaMap, Clickable, Lamp, OnlyIn},
};

/// Where the bedroom sits in the house on the street, from the north-west corner of the
/// house's footprint: upstairs (the floor slab sits on the ground floor's walls), filling
/// the footprint. The house's walls stand just outside it and are the room's walls.
pub const IN_HOUSE: Vec3 = Vec3::new(0.0, 3.0 + SLAB as f32 * FINE, 0.0);
/// Fine voxels per metre (and per map tile).
const T: i32 = 24;
/// Wall height in voxels: up to the roof, 6 m above the street.
const H: i32 = 64;
/// Height of the cut-down front walls (south and east), in voxels: enough to read as
/// walls, low enough to see over.
const STUB: i32 = 6;
/// Floor slab thickness in voxels.
const SLAB: i32 = 8;
/// The landing between the bedroom and the house's front wall, front to back in voxels
/// (the house is a tile deeper than the room): a low partition, then the staircase.
const LANDING: i32 = 24;
/// The staircase's width, front to back: the landing less the partition and the front
/// wall's inner layer.
const STAIR_WIDTH: i32 = 18;

const MAP: &str = include_str!("../assets/maps/bedroom.txt");

fn c(hex: u32) -> Color {
    Color::srgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

// The room's palette.
const WALNUT: u32 = 0x8C5C3C;
const TEAK: u32 = 0x9A6A44;
const TEAK_DARK: u32 = 0x6E4A30;
const CREAM: u32 = 0xEFE8D8;
const PAPER_WHITE: u32 = 0xF4EFE4;
const MUSTARD: u32 = 0xD8A84A;
const MUSTARD_DARK: u32 = 0xB88A36;
const TEAL: u32 = 0x2F6F6E;
const TEAL_DARK: u32 = 0x1F4A4A;
const BRASS: u32 = 0xD8C08A;
const CHOCOLATE: u32 = 0x4A3020;
const IRON: u32 = 0x3A3836;
const BRICK: u32 = 0x9A4E3A;

/// A model waiting to be placed in the room.
struct Piece {
    grid: VoxelGrid,
    /// Room voxel where the grid's origin goes.
    at: IVec3,
    material: Handle<StandardMaterial>,
    click: Option<IVec2>,
    casts_shadow: bool,
}

/// Collects the room's models first, so each can shade its neighbours' corners (the floor
/// darkens under the bed, the wall behind the shelf), then spawns them all.
struct Room {
    /// The room's origin in the world.
    origin: Vec3,
    pieces: Vec<Piece>,
}

impl Room {
    fn add(&mut self, grid: VoxelGrid, at: IVec3, material: &Handle<StandardMaterial>) -> &mut Piece {
        // Some pieces are drawn on a room-sized grid (the window pane, the moon):
        // keep only the part they fill, 20 MB each otherwise.
        let (grid, offset) = grid.cropped();
        self.pieces.push(Piece { grid, at: at + offset, material: material.clone(), click: None, casts_shadow: true });
        self.pieces.last_mut().unwrap()
    }

    fn spawn(self, commands: &mut Commands, meshes: &mut Assets<Mesh>) {
        let min = self.pieces.iter().map(|p| p.at).reduce(IVec3::min).unwrap_or_default();
        let max = self.pieces.iter().map(|p| p.at + p.grid.size.as_ivec3()).reduce(IVec3::max).unwrap_or_default();
        let mut solid = Occupancy::new(min - IVec3::ONE, max + IVec3::ONE);
        for piece in &self.pieces {
            solid.add(&piece.grid, piece.at);
        }
        for piece in self.pieces {
            let at = piece.at;
            let mesh = piece.grid.mesh_lit(Vec3::ZERO, FINE, &|p| solid.solid(p + at));
            let mut entity = commands.spawn((
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(piece.material),
                Transform::from_translation(self.origin + metres(at)),
                OnlyIn(Area::Bedroom),
                DespawnOnExit(Screen::Game),
            ));
            if let Some(tile) = piece.click {
                entity.insert(Clickable { tile, area: Area::Bedroom });
            }
            if !piece.casts_shadow {
                entity.insert(NotShadowCaster);
            }
        }
    }
}

impl Piece {
    fn clickable(&mut self, x: i32, z: i32) -> &mut Self {
        self.click = Some(IVec2::new(x, z));
        self
    }
}

/// A room voxel position.
fn v(x: i32, y: i32, z: i32) -> IVec3 {
    IVec3::new(x, y, z)
}

/// Metres for a room voxel position.
fn metres(p: IVec3) -> Vec3 {
    p.as_vec3() * FINE
}

/// A light that comes on at night (the clock scales it by darkness). `at` and `target`
/// are relative to the room.
fn night_light(commands: &mut Commands, origin: Vec3, light: impl Bundle, at: Vec3, target: Vec3, strength: f32) {
    commands.spawn((
        light,
        Transform::from_translation(origin + at).looking_at(origin + target, Vec3::Y),
        Lamp { strength },
        OnlyIn(Area::Bedroom),
        DespawnOnExit(Screen::Game),
    ));
}

pub fn build(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    material: &Handle<StandardMaterial>,
    glow: &Handle<StandardMaterial>,
    glass: &Handle<StandardMaterial>,
    origin: Vec3,
) -> AreaMap {
    let rows: Vec<Vec<char>> = MAP.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).map(|l| l.chars().collect()).collect();
    let size = IVec2::new(rows[0].len() as i32, rows.len() as i32);
    let tile = |x: i32, z: i32| rows.get(z as usize).and_then(|r| r.get(x as usize)).copied().unwrap_or('.');
    let (w, d) = (size.x * T, size.y * T);
    // Screens show their picture as painted, whatever the light.
    let screen = materials.add(StandardMaterial { base_color: Color::WHITE, unlit: true, ..default() });
    let mut room = Room { origin, pieces: Vec::new() };

    // The way out is through the doorway in the front-west corner, onto the stairs on the
    // landing (where the house has its stairwell: the `E` tile must be on the bottom row of
    // the map's first column).
    let door_z = (0..size.y).find(|&z| tile(0, z) == 'E').unwrap_or(size.y - 1);
    room.add(slab(w, d), v(-3, -SLAB, -3), material);
    let (walls, pane, moon) = walls(w, d, door_z * T);
    room.add(walls, v(-3, 0, -3), material);
    // The glass lets the moon through: it casts no shadow.
    room.add(pane, v(-3, 0, -3), glass).casts_shadow = false;
    room.add(moon, v(-3, 0, -3), glow).casts_shadow = false;
    room.add(stairs(), v(STAIRS_TOP * T, -SLAB - UPSTAIRS * 3, d + 3), material).clickable(0, door_z);
    room.add(handrail(), v(STAIRS_TOP * T, -SLAB - UPSTAIRS * 3, d + 3), material);
    room.add(banister(), v(STAIRS_BOTTOM * T, 0, d + 3), material);

    let mut actions = HashMap::new();
    let mut walkable = vec![false; (size.x * size.y) as usize];
    let mut arrival = IVec2::new(1, door_z);
    for z in 0..size.y {
        for x in 0..size.x {
            let ch = tile(x, z);
            walkable[(z * size.x + x) as usize] = matches!(ch, '.' | '@' | 'E' | 'r');
            if ch == '@' {
                arrival = IVec2::new(x, z);
            }
        }
    }
    actions.insert(IVec2::new(0, door_z), Action::Exit);
    let mut say = |tiles: &[(i32, i32)], line: &'static str, face: Expression| {
        for &(x, z) in tiles {
            actions.insert(IVec2::new(x, z), Action::Say(line, face));
        }
    };

    // Along the north wall, west to east: bed, bedside crate, bookshelf, desk, records.
    room.add(bed(), v(3, 0, 3), material).clickable(0, 0);
    say(&[(0, 0), (0, 1), (0, 2)], "Your bed. It still smells of last summer.", Expression::Tired);

    room.add(milk_crate(c(0xC8A878), 10, 10, 10), v(29, 0, 3), material).clickable(1, 0);
    room.add(alarm_clock(), v(31, 10, 5), material);
    say(&[(1, 0)], "The alarm clock. Set for seven. You have never once got up at seven.", Expression::Puzzled);

    room.add(bookshelf(), v(41, 0, 1), material).clickable(2, 0);
    say(&[(2, 0)], "Comics, a road atlas, and one computer magazine you have read nine times.", Expression::Happy);

    let desk_at = v(74, 0, 1);
    room.add(desk(), desk_at, material).clickable(3, 0);
    let (tv, picture) = television();
    let tv_at = desk_at + v(9, 19, 1);
    room.add(tv, tv_at, material).clickable(3, 0);
    room.add(picture, tv_at, &screen);
    room.add(keyboard(), desk_at + v(8, 19, 10), material);
    room.add(cassette_recorder(), desk_at + v(24, 19, 3), material);
    room.add(graph_pad(), desk_at + v(24, 19, 9), material);
    say(&[(3, 0), (4, 0)], "The computer, plugged into the old TV. READY. One hour a day, your parents said.", Expression::Angry);

    let (lamp, bulb) = desk_lamp();
    let lamp_at = desk_at + v(1, 19, 2);
    room.add(lamp, lamp_at, material);
    room.add(bulb, lamp_at, glow);
    let bulb_at = metres(lamp_at + v(3, 8, 3));
    // The lamp throws a pool of light onto the desk, and a little warmth into the room.
    night_light(
        commands,
        origin,
        SpotLight { intensity: 0.0, range: 4.0, color: c(0xFFC878), outer_angle: 1.05, inner_angle: 0.55, shadow_maps_enabled: true, ..default() },
        bulb_at,
        metres(desk_at + v(16, 19, 14)),
        150_000.0,
    );
    night_light(commands, origin, PointLight { intensity: 0.0, range: 5.0, color: c(0xFFC878), ..default() }, bulb_at + metres(v(0, 10, 10)), Vec3::ZERO, 30_000.0);
    // The TV's glow, cool against the lamp.
    night_light(commands, origin, PointLight { intensity: 0.0, range: 1.6, color: c(0x9CE8C8), ..default() }, metres(tv_at + v(5, 5, 14)), Vec3::ZERO, 4_000.0);
    // Moonlight through the window: the frame and curtains print their shadow on the floor.
    night_light(
        commands,
        origin,
        // A narrow cone that covers the window and no more, so no light spills over the wall.
        SpotLight { intensity: 0.0, range: 14.0, color: c(0x7F9CFF), outer_angle: 0.15, inner_angle: 0.1, shadow_maps_enabled: true, ..default() },
        metres(v(88, 150, -110)),
        metres(v(88, 0, 42)),
        8_000_000.0,
    );

    room.add(chair(), v(84, 0, 18), material);
    room.add(record_crate(), v(112, 0, 2), material);
    room.add(record_player(), v(112, 11, 2), material);
    room.add(record_sleeve(c(TEAL), c(MUSTARD)), v(125, 0, 6), material);
    say(&[(5, 0)], "Your records. Side two of the best one is worn grey.", Expression::Happy);

    room.add(rug(), v(32, 0, 52), material);
    room.add(plant(), v(128, 0, 20), material).clickable(5, 1);
    say(&[(5, 1)], "Your mother's plant. You have been told to water it. You have not.", Expression::Surprised);

    // Floor clutter: magazines by the bed, clothes and sneakers at its foot.
    room.add(magazines(), v(27, 0, 27), material);
    room.add(clothes_heap(), v(3, 0, 75), material).clickable(0, 3);
    say(&[(0, 3)], "Yesterday's clothes. And the day before's.", Expression::Melancholy);
    say(&[(1, 1)], "Magazines. The one on top has a computer on the cover.", Expression::Happy);
    room.add(sneakers(), v(8, 0, 58), material);

    room.spawn(commands, meshes);
    // Framed a little closer than the street: the room is the subject, the street around it
    // the setting.
    AreaMap::room(origin, size, walkable, actions, arrival, 0.8)
}

/// The floor: a slab under the room and its walls, walnut planks on top. Planks run east to
/// west, one tone per plank length, with greyed seams and an occasional grain line. Its rim
/// is the house's outside: render, over a stone band where the string course runs.
fn slab(w: i32, d: i32) -> VoxelGrid {
    let (gw, gd) = (w + 6, d + LANDING + 6);
    let tones = [c(WALNUT), c(0x845638), c(0x94643F)];
    let seam = c(0x5A463A);
    let mut g = VoxelGrid::new(gw as u32, SLAB as u32, gd as u32);
    g.fill_with(IVec3::ZERO, IVec3::new(gw - 1, SLAB - 1, gd - 1), |p| {
        let rim = p.x < 3 || p.z < 3 || p.x >= gw - 3 || p.z >= gd - 3;
        // The opening over the staircase, along the landing.
        let (x, z) = (p.x - 3, p.z - 3);
        if (STAIRS_TOP * T..STAIRS_BOTTOM * T).contains(&x) && (d + 3..d + STAIR_WIDTH + 3).contains(&z) {
            return None;
        }
        if rim {
            return Some(if p.y < 3 { c(STONE) } else { c(HOME.render) });
        }
        Some(if p.y == SLAB - 1 {
            let (x, z) = (p.x - 3, p.z - 3);
            let row = z.div_euclid(6);
            let along = x + (row * 17).rem_euclid(48);
            if z.rem_euclid(6) == 5 || along.rem_euclid(48) == 0 {
                seam
            } else {
                let tone = tones[(hash(IVec3::new(along.div_euclid(48), row, 0), 3) % 3) as usize];
                // A grain line along some planks, broken so it doesn't read as a seam.
                let grain = z.rem_euclid(6) == 2 + (hash(IVec3::new(along.div_euclid(48), row, 1), 5) % 3) as i32
                    && hash(IVec3::new(along.div_euclid(5), row, 2), 7) % 3 == 0;
                if grain { tone.mix(&Color::BLACK, 0.08) } else { tone }
            }
        } else {
            c(0x4A3428)
        })
    });
    g
}

/// The house's upstairs walls around the room, cut open like a dollhouse: the north and
/// west walls full height (2.6 m), the south and east ones cut down low so the room shows.
/// 3 voxels thick: the house's render outside, wallpaper and skirting inside, plaster where
/// they are cut. On them: a window with mustard curtains, a corkboard, a pennant, a poster,
/// a calendar and a light switch. Returns the walls, the window glass
/// (tinted by the time of day, with the view outside painted in) and the moon in that view,
/// as separate shapes.
fn walls(w: i32, d: i32, door_z: i32) -> (VoxelGrid, VoxelGrid, VoxelGrid) {
    // Grid space: room voxel (x, z) is grid (x + 3, z + 3); the room side of the north and
    // west walls is grid 2, and things hung on them sit at grid 3.
    let (gw, gd) = (w + 6, d + LANDING + 6);
    let mut g = VoxelGrid::new(gw as u32, H as u32, gd as u32);
    let paper = |along: i32, y: i32| -> Color {
        if y < 4 {
            return if y == 3 { c(0x7A5536) } else { c(0x5E3F28) };
        }
        if y >= H - 2 {
            return c(CUT);
        }
        if along % 12 == 0 {
            c(0x85915F)
        } else if (along / 6) % 2 == 0 {
            c(0x9EAA7A)
        } else {
            c(0x94A070)
        }
    };
    g.fill_with(IVec3::ZERO, IVec3::new(gw - 1, H - 1, gd - 1), |p| {
        let full = p.z <= 2 || p.x <= 2;
        // The partition between the bedroom and the landing, with the doorway at its west end.
        let partition = (d + 3..d + 6).contains(&p.z) && p.x >= 3 + T;
        let cut_down = p.x >= gw - 3 || p.z >= gd - 3 || partition;
        let height = if full { H } else if cut_down { STUB } else { return None };
        if p.y >= height {
            return None;
        }
        let outside = !partition && (p.x == 0 || p.z == 0 || p.x == gw - 1 || p.z == gd - 1);
        let along = if p.z <= 2 || p.z >= gd - 3 { p.x } else { p.z };
        // Where a wall is cut (the tops of the low walls, the ends of the full ones above
        // them) it shows white all through.
        let cap = if full { 2 } else { 1 };
        let end = full && p.y >= STUB && (p.x == gw - 1 || p.z == gd - 1);
        Some(if p.y >= height - cap || end {
            c(CUT)
        } else if outside {
            c(HOME.render)
        } else {
            paper(along, p.y)
        })
    });

    // Window over the desk: frame, recessed glass, cross mullions, a sill.
    let frame = c(CREAM);
    let (wx0, wx1, wy0, wy1) = (81, 104, 26, 55);
    g.fill(IVec3::new(wx0, wy0, 0), IVec3::new(wx1, wy1, 2), frame);
    g.clear(IVec3::new(wx0 + 2, wy0 + 2, 0), IVec3::new(wx1 - 2, wy1 - 2, 2));
    // The view out of the window, painted into the glass (the glass material tints it with
    // the time of day): the neighbour's roof and chimney, a tree, a sagging telephone wire,
    // the sky paling toward the horizon. The moon is its own glowing shape.
    let mut pane = VoxelGrid::new(gw as u32, H as u32, gd as u32);
    let mut moon = VoxelGrid::new(gw as u32, H as u32, gd as u32);
    let (vx, vy) = (wx0 + 2, wy0 + 2);
    pane.fill_with(IVec3::new(vx, vy, 1), IVec3::new(wx1 - 2, wy1 - 2, 1), |p| {
        let (x, y) = (p.x - vx, p.y - vy);
        let roof_top = 9 - (x - 6).abs();
        let tree = Vec2::new(x as f32 - 16.0, (y as f32 - 13.0) * 1.15).length() <= 5.2;
        let wire = y == 21 - ((x - 9) * (x - 9)) / 30;
        Some(if wire {
            c(0x3A3430)
        } else if tree {
            if (x + y) % 3 == 0 || y > 15 { c(0x5E8E4C) } else { c(0x4A7A3E) }
        } else if x == 16 && y < 9 {
            c(0x6A4A32)
        } else if (9..=10).contains(&x) && (5..=10).contains(&y) {
            if y == 10 { c(0xD8CCB4) } else { c(BRICK) }
        } else if y <= roof_top && y >= 3 {
            if (roof_top - y) % 2 == 0 { c(0xA4523C) } else { c(0x94493A) }
        } else if y < 3 {
            if (3..=5).contains(&x) && y >= 1 { c(0x3A5468) } else { c(0xE8DCC4) }
        } else if y < 13 {
            c(0xC4DCE6)
        } else if y < 19 {
            c(0xAACDE0)
        } else {
            c(0x96C0D8)
        })
    });
    moon.fill(IVec3::new(vx + 3, vy + 20, 1), IVec3::new(vx + 4, vy + 21, 1), c(0xF4EED8));
    pane.clear(IVec3::new(vx + 3, vy + 20, 1), IVec3::new(vx + 4, vy + 21, 1));
    g.fill(IVec3::new(92, wy0 + 2, 2), IVec3::new(93, wy1 - 2, 2), frame);
    g.fill(IVec3::new(wx0 + 2, 40, 2), IVec3::new(wx1 - 2, 41, 2), frame);
    g.fill(IVec3::new(wx0 - 1, wy0 - 1, 3), IVec3::new(wx1 + 1, wy0, 4), frame);
    // Curtains hang in front of the wall, with fold lines every two voxels, on a rod.
    let curtain = |p: IVec3| Some(if p.x % 2 == 0 { c(MUSTARD) } else { c(MUSTARD_DARK) });
    g.fill_with(IVec3::new(75, 21, 3), IVec3::new(80, 58, 3), curtain);
    g.fill_with(IVec3::new(105, 21, 3), IVec3::new(110, 58, 3), curtain);
    g.fill(IVec3::new(73, 59, 3), IVec3::new(112, 59, 3), c(TEAK_DARK));

    // Corkboard by the shelf: a flowchart of index cards joined by lines, red pins.
    let (cx0, cx1, cy0, cy1) = (63, 74, 29, 46);
    g.fill(IVec3::new(cx0, cy0, 3), IVec3::new(cx1, cy1, 3), c(TEAK));
    g.fill_with(IVec3::new(cx0 + 1, cy0 + 1, 3), IVec3::new(cx1 - 1, cy1 - 1, 3), |p| {
        Some(if hash(p, 9) % 5 == 0 { c(0xA07A4C) } else { c(0xB89060) })
    });
    let ink = c(0x3A3430);
    g.fill(IVec3::new(68, 34, 3), IVec3::new(68, 41, 3), ink);
    g.fill(IVec3::new(66, 37, 3), IVec3::new(71, 37, 3), ink);
    for (x, y) in [(66, 41), (66, 33), (70, 37)] {
        g.fill(IVec3::new(x, y, 3), IVec3::new(x + 3, y + 2, 3), c(PAPER_WHITE));
        g.set(x + 1, y + 2, 3, Some(c(0xC8443A)));
    }

    // A felt pennant above the bed, pointing east: teal with a cream band.
    for x in 0..18 {
        let half = 4 - x * 4 / 18;
        for y in (37 - half)..=(37 + half) {
            let band = (y == 37 || y == 36) && x < 14;
            g.set(9 + x, y, 3, Some(if band { c(CREAM) } else { c(TEAL) }));
        }
    }
    g.fill(IVec3::new(8, 32, 3), IVec3::new(8, 42, 3), c(TEAK_DARK));

    // A small poster above the records: a mustard planet with a cream ring on teal.
    let (qx0, qx1, qy0, qy1) = (114, 127, 30, 47);
    g.fill_with(IVec3::new(qx0, qy0, 3), IVec3::new(qx1, qy1, 3), |p| {
        let (dx, dy) = (p.x - 120, p.y - 40);
        Some(if p.x == qx0 || p.x == qx1 || p.y == qy0 || p.y == qy1 {
            c(CREAM)
        } else if dy == 0 && dx.abs() <= 5 || (dy == 1 || dy == -1) && dx.abs() == 5 {
            c(CREAM)
        } else if dx * dx + dy * dy <= 10 {
            c(MUSTARD)
        } else if p.y <= qy0 + 3 {
            if p.x % 2 == 0 { c(CREAM) } else { c(TEAL_DARK) }
        } else if hash(p, 4) % 13 == 0 {
            c(PAPER_WHITE)
        } else {
            c(TEAL_DARK)
        })
    });

    // West wall. Calendar: white page, red header, a grid of days.
    let (kz0, kz1) = (59, 68);
    g.fill(IVec3::new(3, 30, kz0), IVec3::new(3, 44, kz1), c(PAPER_WHITE));
    g.fill(IVec3::new(3, 41, kz0), IVec3::new(3, 44, kz1), c(0xC8443A));
    g.fill_with(IVec3::new(3, 31, kz0 + 1), IVec3::new(3, 39, kz1 - 1), |p| Some(if p.z % 2 == 1 && p.y % 2 == 1 { c(0x8A8070) } else { c(PAPER_WHITE) }));

    // Poster: a sunset over stripes, in the room's colours, with a title band.
    let (pz0, pz1, py0, py1) = (75, 92, 22, 47);
    g.fill_with(IVec3::new(3, py0, pz0), IVec3::new(3, py1, pz1), |p| {
        let (dz, dy) = (p.z - 83, p.y - 38);
        Some(if p.y <= py0 + 2 {
            if p.z % 3 == 0 { c(TEAL_DARK) } else { c(CREAM) }
        } else if dz * dz + dy * dy <= 25 {
            c(MUSTARD)
        } else if p.y <= 33 {
            if (p.y / 2) % 2 == 0 { c(0x9A3B34) } else { c(MUSTARD_DARK) }
        } else {
            c(TEAL_DARK)
        })
    });

    // A light switch by the top of the stairs.
    let dz0 = door_z + 5;
    g.fill(IVec3::new(3, 26, dz0 - 2), IVec3::new(3, 29, dz0), c(CREAM));
    g.set(3, 27, dz0 - 1, Some(c(0x8A8070)));
    (g, pane, moon)
}

/// The staircase along the landing, from the doorway down to the hall: fifteen steps going
/// east, each 5 voxels down and 6 along, with a red runner on the treads and cream risers
/// (they face the camera). The grid's origin is the bottom of the stairs' west end, on the
/// ground floor; its top is the landing floor.
fn stairs() -> VoxelGrid {
    let (len, height) = ((STAIRS_BOTTOM - STAIRS_TOP) * T, SLAB + UPSTAIRS * 3);
    let mut g = VoxelGrid::new(len as u32, height as u32, STAIR_WIDTH as u32);
    for i in 0..(height / 5 - 1) {
        let (x0, top) = (6 * i, height - 1 - 5 * (i + 1));
        g.fill_with(IVec3::new(x0, 0, 0), IVec3::new(x0 + 5, top, STAIR_WIDTH - 1), |p| {
            Some(if p.y == top {
                match p.z {
                    3 | 14 => c(MUSTARD),
                    4..=13 => c(0x9A3B34),
                    _ => c(WALNUT),
                }
            } else if p.x == x0 + 5 {
                c(CREAM)
            } else {
                c(0x4A3428)
            })
        });
    }
    g
}

/// The handrail down the staircase's north side, on white balusters, from a newel post at
/// the top: it shows the flight going down even where the steps are out of sight. Same
/// origin as [`stairs`].
fn handrail() -> VoxelGrid {
    let (len, height) = ((STAIRS_BOTTOM - STAIRS_TOP) * T, SLAB + UPSTAIRS * 3);
    // The top tread is 5 below the floor; the rail stands 20 above the treads and drops 5
    // for every 6 along, like the steps.
    let rail = |x: i32| height - 1 - 5 + 20 - 5 * x / 6;
    let mut g = VoxelGrid::new(len as u32, (rail(0) + 6) as u32, 2);
    for x in (3..len).step_by(3) {
        let tread = height - 1 - 5 * (x / 6 + 1);
        g.fill(IVec3::new(x, tread + 1, 0), IVec3::new(x, rail(x) - 1, 0), c(PAPER_WHITE));
    }
    for x in 0..len {
        g.fill(IVec3::new(x, rail(x), 0), IVec3::new(x, rail(x) + 1, 1), c(TEAK_DARK));
    }
    g.fill(IVec3::new(0, height, 0), IVec3::new(1, rail(0) + 4, 1), c(TEAK_DARK));
    g.fill(IVec3::new(0, rail(0) + 5, 0), IVec3::new(1, rail(0) + 5, 1), c(TEAK));
    g
}

/// The balustrade across the far end of the opening over the stairs: white balusters, a
/// teak handrail and a newel post.
fn banister() -> VoxelGrid {
    let len = STAIR_WIDTH;
    let mut g = VoxelGrid::new(2, 24, len as u32);
    for z in (2..len - 1).step_by(3) {
        g.fill(IVec3::new(0, 0, z), IVec3::new(0, 18, z), c(PAPER_WHITE));
    }
    g.fill(IVec3::new(0, 19, 0), IVec3::new(1, 20, len - 1), c(TEAK_DARK));
    g.fill(IVec3::new(0, 0, 0), IVec3::new(1, 22, 1), c(TEAK_DARK));
    g.fill(IVec3::new(0, 23, 0), IVec3::new(1, 23, 1), c(TEAK));
    g
}

/// A single bed against the north wall: headboard, frame, mattress, pillow, sheet fold and
/// a harvest-gold plaid blanket that drapes over the sides.
fn bed() -> VoxelGrid {
    let (w, l) = (24, 50);
    let mut g = VoxelGrid::new(w as u32, 25, l as u32);
    let wood = c(TEAK);
    for (x, z) in [(0, 3), (w - 3, 3), (0, l - 3), (w - 3, l - 3)] {
        g.fill(IVec3::new(x, 0, z), IVec3::new(x + 2, 2, z + 2), c(TEAK_DARK));
    }
    g.fill(IVec3::new(0, 3, 3), IVec3::new(w - 1, 6, l - 1), wood);
    g.fill(IVec3::new(1, 7, 3), IVec3::new(w - 2, 10, l - 2), c(0xEDE6D6));
    // Headboard with a lighter inset panel and rounded top corners.
    g.fill(IVec3::new(0, 3, 0), IVec3::new(w - 1, 24, 2), c(0x7A5236));
    g.fill(IVec3::new(3, 9, 2), IVec3::new(w - 4, 21, 2), wood);
    g.clear(IVec3::new(0, 23, 0), IVec3::new(1, 24, 2));
    g.clear(IVec3::new(w - 2, 23, 0), IVec3::new(w - 1, 24, 2));
    // Pillow, with a shaded underside.
    g.fill(IVec3::new(3, 11, 4), IVec3::new(w - 4, 14, 12), c(PAPER_WHITE));
    g.fill(IVec3::new(3, 11, 4), IVec3::new(w - 4, 11, 12), c(0xDCD3C2));
    // Blanket: mustard plaid with brown bands crossing every eight voxels.
    let plaid = |p: IVec3| {
        let along = if p.y == 11 { p.x } else { p.y + 2 };
        let (a, b) = (along.rem_euclid(8) < 2, p.z.rem_euclid(8) < 2);
        Some(match (a, b) {
            (true, true) => c(0x5A3A22),
            (true, false) | (false, true) => c(0x8A6234),
            _ => c(0xC8963A),
        })
    };
    g.fill_with(IVec3::new(0, 11, 16), IVec3::new(w - 1, 11, l - 1), plaid);
    g.fill_with(IVec3::new(0, 6, 16), IVec3::new(0, 10, l - 1), plaid);
    g.fill_with(IVec3::new(w - 1, 6, 16), IVec3::new(w - 1, 10, l - 1), plaid);
    g.fill(IVec3::new(0, 11, 14), IVec3::new(w - 1, 12, 16), c(PAPER_WHITE));
    g
}

/// A slatted wooden crate, open toward the room (+z), used as a bedside table.
fn milk_crate(wood: Color, w: i32, h: i32, d: i32) -> VoxelGrid {
    let dark = wood.mix(&Color::BLACK, 0.3);
    let mut g = VoxelGrid::new(w as u32, h as u32, d as u32);
    g.fill_with(IVec3::ZERO, IVec3::new(w - 1, h - 1, d - 1), |p| {
        let side = p.x == 0 || p.x == w - 1;
        let back = p.z == 0;
        let top = p.y == h - 1;
        let bottom = p.y == 0;
        // Slats: gaps every third row on the sides and back.
        let slat = p.y % 3 != 1 || p.y == 0 || p.y == h - 1;
        if top || bottom || (side || back) && slat {
            Some(if p.y % 3 == 2 && !top { wood.mix(&Color::WHITE, 0.08) } else { wood })
        } else if side || back {
            None
        } else if p.y == 1 {
            Some(dark)
        } else {
            None
        }
    });
    g
}

/// A twin-bell alarm clock: dark body, cream face with hands, brass bells and feet.
fn alarm_clock() -> VoxelGrid {
    let mut g = VoxelGrid::new(6, 7, 3);
    g.fill(IVec3::new(1, 1, 0), IVec3::new(4, 5, 2), c(0x8A2E2E));
    g.fill(IVec3::new(1, 2, 2), IVec3::new(4, 4, 2), c(PAPER_WHITE));
    g.set(2, 3, 2, Some(c(0x2A2420)));
    g.set(2, 4, 2, Some(c(0x2A2420)));
    g.set(3, 3, 2, Some(c(0x2A2420)));
    g.fill(IVec3::new(0, 5, 1), IVec3::new(1, 6, 1), c(BRASS));
    g.fill(IVec3::new(4, 5, 1), IVec3::new(5, 6, 1), c(BRASS));
    g.set(1, 0, 1, Some(c(BRASS)));
    g.set(4, 0, 1, Some(c(BRASS)));
    g
}

/// A bookshelf against the north wall: frame, back panel, four shelves of books of varied
/// width, height and colour, a globe on top.
fn bookshelf() -> VoxelGrid {
    let (w, h, d) = (19, 41, 8);
    let wood = c(TEAK);
    let mut g = VoxelGrid::new(w as u32, (h + 7) as u32, d as u32);
    g.fill(IVec3::new(0, 0, 0), IVec3::new(w - 1, h - 1, 0), c(TEAK_DARK));
    g.fill(IVec3::new(0, 0, 0), IVec3::new(0, h - 1, d - 1), wood);
    g.fill(IVec3::new(w - 1, 0, 0), IVec3::new(w - 1, h - 1, d - 1), wood);
    g.fill(IVec3::new(0, h - 1, 0), IVec3::new(w - 1, h - 1, d - 1), wood);
    g.fill(IVec3::new(0, 0, 0), IVec3::new(w - 1, 1, d - 1), wood);
    let colours = [0x9A3B34, 0x2F5E6E, 0xD8A84A, 0x5A7A4A, 0xE8E0CC, 0x5A4A6A, 0x2E3F5A];
    for (level, y0) in [2, 12, 22, 32].into_iter().enumerate() {
        g.fill(IVec3::new(1, y0 + 8, 0), IVec3::new(w - 2, y0 + 8, d - 1), wood);
        let mut x = 1;
        let mut i = 0u32;
        while x < w - 2 {
            let r = hash(IVec3::new(x, level as i32, 7), 11 + i);
            let bw = 2 + (r % 2) as i32;
            let bh = 5 + (r / 3 % 3) as i32;
            if r % 9 == 0 {
                x += 2; // a gap
                i += 1;
                continue;
            }
            let col = c(colours[(r / 7 % colours.len() as u32) as usize]);
            let x1 = (x + bw - 1).min(w - 2);
            g.fill(IVec3::new(x, y0, 2), IVec3::new(x1, y0 + bh, d - 2), col);
            // A title band near the top of some spines.
            if r % 3 == 0 {
                g.fill(IVec3::new(x, y0 + bh - 2, d - 2), IVec3::new(x1, y0 + bh - 2, d - 2), c(0xEDE3C8));
            }
            x = x1 + 1;
            i += 1;
        }
    }
    // A small globe on top: blue sea, green land, on a dark stand.
    g.fill(IVec3::new(13, h, 3), IVec3::new(15, h, 5), c(0x4A3E34));
    g.fill_with(IVec3::new(11, h + 1, 1), IVec3::new(17, h + 6, 7), |p| {
        let q = p.as_vec3() - Vec3::new(14.0, h as f32 + 3.5, 4.0);
        (q.length() <= 3.2).then(|| if hash(p, 5) % 3 == 0 { c(0x5A8A5A) } else { c(0x4A7EA8) })
    });
    g
}

/// The desk under the window: a two-voxel top with a grain line, legs on the left, a
/// drawer pedestal on the right.
fn desk() -> VoxelGrid {
    let (w, h, d) = (34, 19, 14);
    let wood = c(TEAK);
    let dark = c(TEAK_DARK);
    let mut g = VoxelGrid::new(w as u32, h as u32, d as u32);
    g.fill_with(IVec3::new(0, h - 2, 0), IVec3::new(w - 1, h - 1, d - 1), |p| {
        let grain = p.y == h - 1 && (p.z == 4 && p.x % 11 < 7 || p.z == 9 && (p.x + 5) % 13 < 8);
        Some(if grain { wood.mix(&Color::BLACK, 0.08) } else { wood })
    });
    for z in [0, d - 2] {
        g.fill(IVec3::new(0, 0, z), IVec3::new(1, h - 3, z + 1), wood);
    }
    g.fill(IVec3::new(24, 0, 0), IVec3::new(w - 1, h - 3, d - 1), wood);
    // Two drawers: dark gaps around them, brass handles.
    g.fill(IVec3::new(24, 8, d - 1), IVec3::new(w - 1, 8, d - 1), dark);
    g.fill(IVec3::new(24, 0, d - 1), IVec3::new(24, h - 3, d - 1), dark);
    for y in [12, 4] {
        g.fill(IVec3::new(28, y, d - 1), IVec3::new(29, y, d - 1), c(BRASS));
    }
    g
}

/// A portable TV with a wood-grain case, two knobs and rabbit-ear antennas, used as the
/// computer's screen. The picture is a separate shape drawn with the screen material.
fn television() -> (VoxelGrid, VoxelGrid) {
    let (w, h, d) = (13, 11, 8);
    let case = c(0x6A4A32);
    let mut g = VoxelGrid::new(w as u32, 17, d as u32);
    g.fill_with(IVec3::new(0, 0, 0), IVec3::new(w - 1, h - 1, d - 1), |p| {
        Some(if p.y % 3 == 1 && p.z < d - 1 { case.mix(&Color::BLACK, 0.1) } else { case })
    });
    // The front: a dark bezel around the screen, a control panel on the right.
    g.fill(IVec3::new(1, 1, d - 1), IVec3::new(w - 2, h - 2, d - 1), c(0x2A2622));
    g.fill(IVec3::new(9, 1, d - 1), IVec3::new(w - 2, h - 2, d - 1), c(0x8A8478));
    g.set(10, 7, d - 1, Some(c(0xD8D2C4)));
    g.set(10, 4, d - 1, Some(c(0xD8D2C4)));
    g.fill(IVec3::new(9, 1, d - 1), IVec3::new(w - 2, 1, d - 1), c(0x5A5650));
    // Rabbit ears, out of a small base on top.
    g.fill(IVec3::new(5, h, 3), IVec3::new(7, h, 5), c(IRON));
    for i in 0..6 {
        g.set(5 - i * 2 / 3, h + 1 + i, 4, Some(c(0xC8C8C0)));
        g.set(7 + i * 2 / 3, h + 1 + i, 4, Some(c(0xC8C8C0)));
    }
    // The picture: green text on black, a few lines and a block cursor.
    let mut picture = VoxelGrid::new(w as u32, 17, d as u32);
    picture.fill(IVec3::new(2, 2, d - 1), IVec3::new(7, 8, d - 1), c(0x0E2A1C));
    for (y, len) in [(7, 5), (5, 3), (3, 4)] {
        picture.fill(IVec3::new(2, y, d - 1), IVec3::new(1 + len, y, d - 1), c(0x7CE08A));
    }
    picture.set(6, 3, d - 1, Some(c(0xC8F8C8)));
    // The picture takes the bezel's place.
    g.clear(IVec3::new(2, 2, d - 1), IVec3::new(7, 8, d - 1));
    (g, picture)
}

/// The computer itself: a one-piece beige keyboard unit with a dark label strip and a red
/// power light.
fn keyboard() -> VoxelGrid {
    let (w, d) = (15, 4);
    let mut g = VoxelGrid::new(w as u32, 3, d as u32);
    g.fill(IVec3::new(0, 0, 0), IVec3::new(w - 1, 1, d - 1), c(0xE6DABE));
    g.fill(IVec3::new(0, 2, 0), IVec3::new(w - 1, 2, 0), c(0xE6DABE));
    g.fill(IVec3::new(1, 2, 0), IVec3::new(5, 2, 0), c(0x4A4440));
    g.fill_with(IVec3::new(1, 1, 1), IVec3::new(w - 2, 1, d - 1), |p| {
        Some(if (p.x + p.z) % 2 == 0 { c(0xF6EFDF) } else { c(0x9E927A) })
    });
    g.fill(IVec3::new(4, 1, d - 1), IVec3::new(10, 1, d - 1), c(0xF6EFDF));
    g.set(w - 2, 2, 0, Some(c(0xE04A3A)));
    g
}

/// A portable cassette recorder for saving programs: dark body, silver key row, a window
/// onto the tape.
fn cassette_recorder() -> VoxelGrid {
    let (w, d) = (8, 6);
    let mut g = VoxelGrid::new(w as u32, 3, d as u32);
    g.fill(IVec3::new(0, 0, 0), IVec3::new(w - 1, 2, d - 1), c(IRON));
    g.fill(IVec3::new(1, 2, 1), IVec3::new(w - 2, 2, 3), c(0x1E1C1A));
    g.fill(IVec3::new(2, 2, 2), IVec3::new(5, 2, 2), c(0x6A4A32));
    g.fill_with(IVec3::new(1, 2, d - 2), IVec3::new(w - 2, 2, d - 1), |p| Some(if p.x % 2 == 0 { c(0xC8C8C0) } else { c(0x8A8A84) }));
    g
}

/// A pad of graph paper with a pencil.
fn graph_pad() -> VoxelGrid {
    let mut g = VoxelGrid::new(9, 2, 5);
    g.fill_with(IVec3::new(0, 0, 0), IVec3::new(6, 0, 4), |p| Some(if p.x % 2 == 1 || p.z % 2 == 1 { c(0xC8D8E0) } else { c(PAPER_WHITE) }));
    g.fill(IVec3::new(8, 0, 0), IVec3::new(8, 0, 3), c(MUSTARD));
    g.set(8, 0, 4, Some(c(0x3A3430)));
    g
}

/// A chocolate-brown 1970s desk lamp. The bulb is a separate shape drawn with the glow
/// material.
fn desk_lamp() -> (VoxelGrid, VoxelGrid) {
    let mut g = VoxelGrid::new(7, 12, 7);
    let shade = c(CHOCOLATE);
    g.fill(IVec3::new(1, 0, 1), IVec3::new(5, 0, 5), shade);
    g.fill(IVec3::new(3, 1, 3), IVec3::new(3, 7, 3), c(BRASS));
    g.fill(IVec3::new(0, 8, 0), IVec3::new(6, 8, 6), shade);
    g.fill(IVec3::new(1, 9, 1), IVec3::new(5, 9, 5), shade);
    g.fill(IVec3::new(2, 10, 2), IVec3::new(4, 10, 4), shade);
    g.fill(IVec3::new(0, 8, 6), IVec3::new(6, 8, 6), c(BRASS));
    g.clear(IVec3::new(1, 8, 1), IVec3::new(5, 8, 5));
    let mut bulb = VoxelGrid::new(7, 12, 7);
    bulb.fill(IVec3::new(2, 7, 2), IVec3::new(4, 8, 4), c(0xFFE7A8));
    (g, bulb)
}

/// A wooden chair facing the desk, its slatted back toward the room, a mustard cushion.
fn chair() -> VoxelGrid {
    let wood = c(TEAK);
    let mut g = VoxelGrid::new(12, 25, 12);
    for (x, z) in [(0, 0), (11, 0), (0, 11), (11, 11)] {
        g.fill(IVec3::new(x, 0, z), IVec3::new(x, 9, z), c(TEAK_DARK));
    }
    g.fill(IVec3::new(0, 10, 0), IVec3::new(11, 10, 11), wood);
    g.fill(IVec3::new(1, 11, 1), IVec3::new(10, 11, 10), c(MUSTARD));
    g.fill_with(IVec3::new(0, 12, 11), IVec3::new(11, 24, 11), |p| (p.y >= 22 || p.x % 3 == 0 || p.x == 11).then_some(wood));
    g
}

/// A crate of LPs: the sleeves show as thin coloured slices between the slats.
fn record_crate() -> VoxelGrid {
    let (w, h, d) = (12, 11, 11);
    let mut g = milk_crate(c(0xB89868), w, h, d);
    let sleeves = [0x9A3B34, MUSTARD, TEAL, 0xE8E0CC, 0x2E3F5A, 0x5A7A4A, 0x3A3430];
    for x in 1..w - 1 {
        let col = c(sleeves[(hash(IVec3::new(x, 0, 3), 2) % sleeves.len() as u32) as usize]);
        let top = h - 2 - (hash(IVec3::new(x, 1, 3), 2) % 2) as i32;
        g.fill(IVec3::new(x, 1, 1), IVec3::new(x, top, d - 2), col);
    }
    g
}

/// A record player on top of the crate: walnut plinth, black platter with a mustard
/// label, a silver tonearm.
fn record_player() -> VoxelGrid {
    let (w, d) = (12, 11);
    let mut g = VoxelGrid::new(w as u32, 4, d as u32);
    g.fill(IVec3::new(0, 0, 0), IVec3::new(w - 1, 1, d - 1), c(0x5A3A26));
    g.fill(IVec3::new(0, 1, d - 1), IVec3::new(w - 1, 1, d - 1), c(0x7A5236));
    g.fill_with(IVec3::new(1, 2, 1), IVec3::new(8, 2, 8), |p| {
        let q = Vec2::new(p.x as f32 - 4.5, p.z as f32 - 4.5);
        let r = q.length();
        (r <= 4.2).then(|| if r <= 1.2 { c(MUSTARD) } else if (r * 2.0) as i32 % 2 == 0 { c(0x1E1C1A) } else { c(0x2E2C2A) })
    });
    g.fill(IVec3::new(10, 2, 1), IVec3::new(10, 3, 1), c(0xC8C8C0));
    g.fill(IVec3::new(10, 3, 2), IVec3::new(10, 3, 6), c(0xC8C8C0));
    g.set(9, 3, 7, Some(c(0xC8C8C0)));
    g
}

/// One LP sleeve lying on the floor.
fn record_sleeve(back: Color, front: Color) -> VoxelGrid {
    let mut g = VoxelGrid::new(12, 1, 12);
    g.fill_with(IVec3::ZERO, IVec3::new(11, 0, 11), |p| {
        let (dx, dz) = (p.x - 6, p.z - 6);
        Some(if dx * dx + dz * dz <= 10 { front } else { back })
    });
    g
}

/// A teal rug with a dark border, a mustard inner line and a large cream-and-mustard
/// medallion.
fn rug() -> VoxelGrid {
    let (w, d) = (76, 44);
    let mut g = VoxelGrid::new(w as u32, 1, d as u32);
    g.fill_with(IVec3::ZERO, IVec3::new(w - 1, 0, d - 1), |p| {
        let edge = p.x.min(p.z).min(w - 1 - p.x).min(d - 1 - p.z);
        let (dx, dz) = ((p.x * 2 - (w - 1)).abs() / 2, (p.z * 2 - (d - 1)).abs() / 2);
        // A diamond, stretched along the rug.
        let diamond = dx / 2 + dz;
        Some(if edge < 3 {
            c(TEAL_DARK)
        } else if edge == 5 {
            c(MUSTARD)
        } else if diamond < 5 {
            c(0x9A3B34)
        } else if diamond < 9 {
            c(CREAM)
        } else if diamond < 13 {
            c(MUSTARD)
        } else if diamond == 15 {
            c(TEAL_DARK)
        } else {
            c(TEAL)
        })
    });
    g
}

/// A potted plant: a cream glazed pot with a teal band, and leaf clusters lit from above.
fn plant() -> VoxelGrid {
    let mut g = VoxelGrid::new(14, 24, 14);
    g.fill(IVec3::new(3, 0, 3), IVec3::new(10, 6, 10), c(CREAM));
    g.fill(IVec3::new(3, 2, 3), IVec3::new(10, 3, 10), c(TEAL));
    g.fill(IVec3::new(4, 6, 4), IVec3::new(9, 6, 9), c(0x4A3428));
    let blobs = [(7.0, 12.0, 7.0, 4.5), (3.5, 15.0, 5.0, 3.2), (10.5, 16.0, 9.0, 3.2), (7.0, 19.0, 6.0, 3.4), (4.0, 18.5, 10.0, 2.6)];
    g.fill_with(IVec3::new(0, 7, 0), IVec3::new(13, 23, 13), |p| {
        let q = p.as_vec3();
        blobs.iter().find(|(x, y, z, r)| (q - Vec3::new(*x, *y, *z)).length() <= *r).map(|(_, y, _, _)| {
            if q.y > *y { c(0x6FA05A) } else { c(0x4E8044) }
        })
    });
    g
}

/// A stack of magazines, slightly askew, with one open on the floor beside it.
fn magazines() -> VoxelGrid {
    let mut g = VoxelGrid::new(22, 3, 14);
    let covers = [(0, 0, c(0x9A3B34)), (1, 1, c(PAPER_WHITE)), (0, 2, c(TEAL))];
    for (shift, y, col) in covers {
        g.fill(IVec3::new(shift, y, shift), IVec3::new(shift + 8, y, shift + 11), col);
        g.fill(IVec3::new(shift, y, shift + 11), IVec3::new(shift + 8, y, shift + 11), col.mix(&Color::BLACK, 0.2));
    }
    // The cover on top: a computer, in two pixels.
    g.fill(IVec3::new(3, 2, 4), IVec3::new(5, 2, 6), c(0xE6DABE));
    g.fill(IVec3::new(4, 2, 4), IVec3::new(4, 2, 5), c(0x1E3A2A));
    // The open one: two white pages and a spine.
    g.fill(IVec3::new(12, 0, 2), IVec3::new(21, 0, 9), c(PAPER_WHITE));
    g.fill(IVec3::new(16, 0, 2), IVec3::new(17, 0, 9), c(0xC8C0B0));
    g.fill(IVec3::new(13, 0, 4), IVec3::new(15, 0, 6), c(MUSTARD));
    g
}

/// Yesterday's clothes: a heap of jeans, a cream shirt and a mustard jumper.
fn clothes_heap() -> VoxelGrid {
    let mut g = VoxelGrid::new(20, 8, 18);
    let blobs = [
        (6.0, 1.0, 8.0, 5.5, c(0x3E5A8A)),
        (12.0, 1.5, 6.0, 4.5, c(0xEDE6D2)),
        (11.0, 2.5, 12.0, 4.5, c(MUSTARD)),
        (5.0, 3.5, 10.0, 3.5, c(0x34507A)),
    ];
    g.fill_with(IVec3::ZERO, IVec3::new(19, 7, 17), |p| {
        let q = p.as_vec3() + Vec3::splat(0.5);
        // Flattened blobs: clothes slump.
        blobs.iter().rev().find(|(x, y, z, r, _)| {
            let d = (q - Vec3::new(*x, *y, *z)) * Vec3::new(1.0, 1.8, 1.0);
            d.length() <= *r
        }).map(|(_, _, _, _, col)| *col)
    });
    // Jeans seams and a leg trailing out.
    g.fill(IVec3::new(0, 0, 5), IVec3::new(2, 1, 8), c(0x3E5A8A));
    g
}

/// A pair of white sneakers with a red stripe, kicked off by the bed.
fn sneakers() -> VoxelGrid {
    let mut g = VoxelGrid::new(12, 3, 9);
    for (x, z) in [(0, 0), (6, 2)] {
        g.fill(IVec3::new(x, 0, z), IVec3::new(x + 3, 0, z + 6), c(0xC8C0B0));
        g.fill(IVec3::new(x, 1, z), IVec3::new(x + 3, 2, z + 4), c(PAPER_WHITE));
        g.fill(IVec3::new(x, 1, z + 5), IVec3::new(x + 3, 1, z + 6), c(PAPER_WHITE));
        g.fill(IVec3::new(x, 1, z + 2), IVec3::new(x, 1, z + 4), c(0xA8343A));
        g.fill(IVec3::new(x + 3, 1, z + 2), IVec3::new(x + 3, 1, z + 4), c(0xA8343A));
        g.fill(IVec3::new(x + 1, 2, z + 1), IVec3::new(x + 2, 2, z + 2), c(0x3A3430));
    }
    g
}
