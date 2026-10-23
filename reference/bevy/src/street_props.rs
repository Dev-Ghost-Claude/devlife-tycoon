//! 1978 street furniture and parked cars, in fine voxels (1/24 m, like the bedroom).
//!
//! Every model is built facing its front toward grid z = max (the room-style convention
//! used in `bedroom.rs`: grid x runs east, y up, z toward the viewer/south), standing on
//! y = 0, with its origin at the grid's corner. Street furniture faces the road with its
//! front; cars lie along x with their nose toward +x (the driver sits on the z = 0 side,
//! the left of a car facing east, since traffic drives on the right). `catalogue()` lists
//! them all for the gallery (`cargo run -- --gallery --capture target/gallery`).

use std::f32::consts::PI;

use bevy::prelude::*;

use crate::voxel::{VoxelGrid, hash};

/// One street model. `glow` holds the parts that light up at night (lamp heads, lit
/// panels), drawn with the glowing material on top of `grid`; same size and origin. The
/// lit voxels are only in `glow`, so the two never overlap.
pub struct Prop {
    pub name: &'static str,
    pub grid: VoxelGrid,
    pub glow: Option<VoxelGrid>,
}

fn c(hex: u32) -> Color {
    Color::srgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

// Shared swatches (the era-1 list in WORLD_ART_DIRECTION.md 6.3, plus a few tones for metal).
const INK: u32 = 0x2B2420;
const IRON: u32 = 0x33393A;
const STEEL: u32 = 0x8E949C;
const STEEL_LIGHT: u32 = 0xA9B1B8;
const STEEL_DARK: u32 = 0x626A7C;
const CHROME: u32 = 0xDCE3E8;
const HUBCAP: u32 = 0xEEF2F4;
const CONCRETE: u32 = 0xB8AE96;
const CONCRETE_DARK: u32 = 0x8C8078;
const CREAM: u32 = 0xF5F1EA;
const PLASTIC: u32 = 0xE8E0CC;
const POST_RED: u32 = 0xB5352B;
const POST_RED_DARK: u32 = 0x8A2A22;
const RUST: u32 = 0xA84E33;
const ORANGE: u32 = 0xD8802E;
const MUSTARD: u32 = 0xD9A93A;
const BUTTER: u32 = 0xF2D888;
const LEAF: u32 = 0x4E8A44;
const PINE: u32 = 0x36663A;
const PINK: u32 = 0xD8607A;
const DENIM: u32 = 0x35618A;
const DENIM_DARK: u32 = 0x28426A;
const TEAL: u32 = 0x2E7E7A;
const TEAK: u32 = 0x9A6A44;
const TEAK_DARK: u32 = 0x6A4A32;
/// The beacon's globe by day.
const BEACON_AMBER: u32 = 0xEDA83A;
/// Lit lenses and globes by day; at night the glow material lights them amber.
const LENS: u32 = 0xF0C070;
const GLASS: u32 = 0x2F4256;
const GLASS_STREAK: u32 = 0x88A2B6;
const GLASS_PALE: u32 = 0xA6C4CC;

/// Every street model, for the gallery and for the street builder.
pub fn catalogue() -> Vec<Prop> {
    let mut all = street_furniture();
    all.extend(street_details());
    all
}

/// First batch: street furniture and parked cars.
fn street_furniture() -> Vec<Prop> {
    // Ordered for the gallery's row packing (see `gallery.rs`): short things at the corners,
    // tall ones inside, where the camera's frame never cuts them off.
    vec![
        bollard(),
        parking_meter(),
        a_board(),
        planter(),
        street_lamp(),
        phone_booth(),
        bus_stop(),
        crossing_beacon(),
        post_box(),
        bench(),
        hatchback("hatchback-orange", ORANGE),
        hatchback("hatchback-teal", TEAL),
        saloon("saloon-cream", 0xE4DCC6),
        fire_hydrant(),
        litter_bin(),
        saloon("saloon-moss", 0x5E8A46),
        van("van-cream", 0xEFE8D8),
        van("van-denim", DENIM),
        newspaper_box(),
    ]
}

/// Second batch: the small things that make the street look lived in.
fn street_details() -> Vec<Prop> {
    vec![
        bottle_crates(),
        bicycle(),
        street_sign(),
        utility_pole(),
        washing_line(),
        window_flower_box(),
        builders_skip(),
        garden_shed(),
        picket_fence(),
        panel_fence(),
        trash_can(),
        rubbish_bags(),
        sleeping_cat(),
        pigeon("pigeon-standing", Pose::Stand),
        pigeon("pigeon-pecking", Pose::Peck),
        pigeon("pigeon-walking", Pose::Walk),
        leaf_patch("leaves-mustard", [MUSTARD, 0xE6BC52, 0xC8962E, RUST], 3),
        leaf_patch("leaves-rust", [RUST, 0xC0684A, 0x8A3A22, MUSTARD], 5),
        paper_litter(),
        plant_pot_tall(),
        plant_pot_round(),
        leaf_patch("leaves-brown", [TEAK_DARK, 0x8C5C3C, 0x52463D, RUST], 9),
    ]
}

// ---------------------------------------------------------------------------------------
// Drawing helpers. Ranges are inclusive, like `VoxelGrid::fill`.

/// An inclusive range of voxels along one axis.
type R = (i32, i32);

fn prop(name: &'static str, grid: VoxelGrid) -> Prop {
    Prop { name, grid, glow: None }
}

fn bx(g: &mut VoxelGrid, x: R, y: R, z: R, hex: u32) {
    g.fill(IVec3::new(x.0, y.0, z.0), IVec3::new(x.1, y.1, z.1), c(hex));
}

/// Paints the voxels of a box whose centres pass `inside` (centres are in grid units).
fn shape(g: &mut VoxelGrid, x: R, y: R, z: R, hex: u32, inside: impl Fn(Vec3) -> bool) {
    for zz in z.0..=z.1 {
        for yy in y.0..=y.1 {
            for xx in x.0..=x.1 {
                if inside(Vec3::new(xx as f32 + 0.5, yy as f32 + 0.5, zz as f32 + 0.5)) {
                    g.set(xx, yy, zz, Some(c(hex)));
                }
            }
        }
    }
}

/// The voxel range that can touch a circle of radius `r` around `centre`.
fn span(centre: f32, r: f32) -> R {
    ((centre - r).floor() as i32, (centre + r).ceil() as i32)
}

/// Upright cylinder around the point (cx, cz).
fn cyl(g: &mut VoxelGrid, cx: f32, cz: f32, r: f32, y: R, hex: u32) {
    shape(g, span(cx, r), y, span(cz, r), hex, |p| (p.x - cx).hypot(p.z - cz) <= r);
}

/// A disc standing across z (a wheel seen from the side), centred at (cx, cy).
fn disc_z(g: &mut VoxelGrid, cx: f32, cy: f32, r: f32, z: R, hex: u32) {
    shape(g, span(cx, r), span(cy, r), z, hex, |p| (p.x - cx).hypot(p.y - cy) <= r);
}

/// A disc standing across x (a headlight seen from the front), centred at (cy, cz).
fn disc_x(g: &mut VoxelGrid, cy: f32, cz: f32, r: f32, x: R, hex: u32) {
    shape(g, x, span(cy, r), span(cz, r), hex, |p| (p.y - cy).hypot(p.z - cz) <= r);
}

/// A ball (or squashed ball) of radii `r` around `at`.
fn ball(g: &mut VoxelGrid, at: Vec3, r: Vec3, hex: u32) {
    let lo = (at - r).floor().as_ivec3();
    let hi = (at + r).ceil().as_ivec3();
    shape(g, (lo.x, hi.x), (lo.y, hi.y), (lo.z, hi.z), hex, |p| ((p - at) / r).length() <= 1.0);
}

/// Recolours the voxels that already exist in a box.
fn tint(g: &mut VoxelGrid, x: R, y: R, z: R, hex: u32) {
    for zz in z.0..=z.1 {
        for yy in y.0..=y.1 {
            for xx in x.0..=x.1 {
                if g.get(xx, yy, zz).is_some() {
                    g.set(xx, yy, zz, Some(c(hex)));
                }
            }
        }
    }
}

/// Moves the voxels of a box into a new grid of the same size: the lit parts of a model.
fn lift(g: &mut VoxelGrid, x: R, y: R, z: R) -> VoxelGrid {
    let mut lit = VoxelGrid::new(g.size.x, g.size.y, g.size.z);
    for zz in z.0..=z.1 {
        for yy in y.0..=y.1 {
            for xx in x.0..=x.1 {
                if let Some(color) = g.get(xx, yy, zz) {
                    lit.set(xx, yy, zz, Some(color));
                    g.set(xx, yy, zz, None);
                }
            }
        }
    }
    lit
}

/// A hex colour scaled toward black (k < 1) for seams and shade variants.
fn shade(hex: u32, k: f32) -> u32 {
    let ch = |s: u32| (((hex >> s) & 0xFF) as f32 * k).min(255.0) as u32;
    ch(16) << 16 | ch(8) << 8 | ch(0)
}

/// Windscreens reflect the sky: mid blue on light bodies, near black on dark ones, so the
/// glass never melts into the paint.
fn windscreen(body: u32) -> u32 {
    if is_light(body) { 0x3E5A74 } else { 0x1F2B38 }
}

fn is_light(hex: u32) -> bool {
    let ch = |s: u32| ((hex >> s) & 0xFF) as f32;
    0.3 * ch(16) + 0.59 * ch(8) + 0.11 * ch(0) > 140.0
}

// ---------------------------------------------------------------------------------------
// Street furniture.

/// Sodium street lamp (9 x 78 x 38). A grey column on a concrete foot with a service hatch,
/// a curved arm reaching toward +z (over the road) and a cobra head: grey housing on top,
/// an amber lens band under it that is the glow part. The band's sides show from the camera.
fn street_lamp() -> Prop {
    let mut g = VoxelGrid::new(11, 78, 38);
    bx(&mut g, (1, 9), (0, 2), (1, 9), CONCRETE_DARK);
    bx(&mut g, (2, 8), (3, 9), (2, 8), CONCRETE);
    bx(&mut g, (2, 8), (10, 10), (2, 8), STEEL_DARK);
    bx(&mut g, (3, 7), (11, 58), (3, 7), STEEL);
    bx(&mut g, (4, 6), (59, 71), (4, 6), STEEL);
    bx(&mut g, (4, 6), (16, 27), (7, 7), STEEL_DARK);
    // The arm climbs three voxels while it leaves the column, then runs level into the head.
    for z in 4..=22 {
        let top = 74 + ((z - 4) / 4).min(3);
        bx(&mut g, (4, 6), (top - 2, top), (z, z), STEEL);
    }
    bx(&mut g, (1, 9), (75, 77), (22, 35), STEEL_LIGHT);
    bx(&mut g, (2, 8), (75, 77), (20, 21), STEEL_LIGHT);
    bx(&mut g, (2, 8), (75, 77), (36, 37), STEEL_LIGHT);
    bx(&mut g, (2, 8), (72, 74), (22, 35), LENS);
    bx(&mut g, (3, 7), (72, 74), (21, 21), LENS);
    bx(&mut g, (3, 7), (72, 74), (36, 36), LENS);
    let glow = lift(&mut g, (2, 8), (72, 74), (21, 36));
    Prop { name: "street-lamp", grid: g, glow: Some(glow) }
}

/// Park bench (42 x 21 x 15): three teak slats for the seat, three for the tilted back,
/// dark iron ends with armrests, and a small brass plate on the back.
fn bench() -> Prop {
    let mut g = VoxelGrid::new(42, 21, 15);
    let wood = [TEAK, 0xA67848, 0x8F6240];
    for (i, z) in [4, 8, 12].into_iter().enumerate() {
        bx(&mut g, (1, 40), (10, 11), (z, z + 2), wood[i]);
    }
    for (i, y) in [13, 16, 19].into_iter().enumerate() {
        let z = 3 - i as i32;
        bx(&mut g, (1, 40), (y, y + 1), (z, z + 1), wood[(i + 1) % 3]);
    }
    for x in [(4, 6), (35, 37)] {
        bx(&mut g, x, (0, 20), (0, 2), IRON);
        bx(&mut g, x, (0, 14), (12, 14), IRON);
        bx(&mut g, x, (8, 9), (0, 14), IRON);
        bx(&mut g, x, (15, 16), (3, 14), IRON);
    }
    bx(&mut g, (19, 22), (16, 17), (3, 3), MUSTARD);
    prop("bench", g)
}

/// Litter bin (12 x 21 x 12): a round green steel drum with steel hoops, a dark flap
/// opening on the front and a domed lid.
fn litter_bin() -> Prop {
    let green = 0x3F6455;
    let mut g = VoxelGrid::new(12, 21, 12);
    cyl(&mut g, 6.0, 6.0, 4.6, (0, 1), INK);
    cyl(&mut g, 6.0, 6.0, 5.4, (2, 15), green);
    cyl(&mut g, 6.0, 6.0, 5.9, (3, 3), STEEL);
    cyl(&mut g, 6.0, 6.0, 5.9, (14, 14), STEEL);
    cyl(&mut g, 6.0, 6.0, 6.0, (16, 17), shade(green, 0.8));
    cyl(&mut g, 6.0, 6.0, 5.0, (18, 18), shade(green, 0.8));
    cyl(&mut g, 6.0, 6.0, 3.0, (19, 19), shade(green, 0.8));
    cyl(&mut g, 6.0, 6.0, 1.5, (20, 20), STEEL);
    tint(&mut g, (3, 8), (7, 12), (9, 11), 0x17120F);
    prop("litter-bin", g)
}

/// Red pillar post box (18 x 36 x 18): a black foot, a fat red drum, a wider cap with a
/// domed top, a black mail slot under a hood and a cream collection plate below it.
fn post_box() -> Prop {
    let mut g = VoxelGrid::new(18, 36, 18);
    cyl(&mut g, 9.0, 9.0, 8.9, (0, 3), INK);
    cyl(&mut g, 9.0, 9.0, 8.2, (4, 29), POST_RED);
    cyl(&mut g, 9.0, 9.0, 8.9, (30, 31), POST_RED_DARK);
    cyl(&mut g, 9.0, 9.0, 7.6, (32, 33), POST_RED);
    cyl(&mut g, 9.0, 9.0, 5.6, (34, 34), POST_RED);
    cyl(&mut g, 9.0, 9.0, 3.2, (35, 35), POST_RED);
    bx(&mut g, (3, 14), (25, 26), (14, 17), POST_RED_DARK);
    tint(&mut g, (4, 13), (22, 24), (13, 17), INK);
    tint(&mut g, (6, 11), (13, 19), (14, 17), CREAM);
    tint(&mut g, (7, 10), (17, 17), (14, 17), INK);
    tint(&mut g, (7, 10), (15, 15), (14, 17), INK);
    prop("post-box", g)
}

/// Phone booth (24 x 60 x 24): red frame with a dome roof and cream sign bands. The front
/// (+z) and east (+x) panes are open so the dial phone on the west wall shows; the back
/// and west panes are solid pale glass.
fn phone_booth() -> Prop {
    let mut g = VoxelGrid::new(24, 60, 24);
    bx(&mut g, (0, 23), (0, 5), (0, 23), POST_RED_DARK);
    for (x, z) in [(0, 0), (21, 0), (0, 21), (21, 21)] {
        bx(&mut g, (x, x + 2), (6, 52), (z, z + 2), POST_RED);
    }
    // Horizontal rails and vertical mullions on all four faces.
    let rails = [(6, 7), (21, 22), (36, 37), (50, 52)];
    let panes_x = [(3, 7), (10, 13), (16, 20)];
    let panes_y = [(8, 20), (23, 35), (38, 49)];
    for (lo, hi) in [(0, 1), (22, 23)] {
        for r in rails {
            bx(&mut g, (3, 20), r, (lo, hi), POST_RED);
            bx(&mut g, (lo, hi), r, (3, 20), POST_RED);
        }
        for m in [(8, 9), (14, 15)] {
            bx(&mut g, m, (8, 49), (lo, hi), POST_RED);
            bx(&mut g, (lo, hi), (8, 49), m, POST_RED);
        }
    }
    for p in panes_x {
        for q in panes_y {
            bx(&mut g, p, q, (1, 1), GLASS_PALE);
            bx(&mut g, (1, 1), q, p, GLASS_PALE);
        }
    }
    // Cream sign band on every top rail, red dome above.
    bx(&mut g, (4, 19), (50, 52), (23, 23), PLASTIC);
    bx(&mut g, (4, 19), (50, 52), (0, 0), PLASTIC);
    bx(&mut g, (23, 23), (50, 52), (4, 19), PLASTIC);
    bx(&mut g, (0, 0), (50, 52), (4, 19), PLASTIC);
    bx(&mut g, (0, 23), (53, 54), (0, 23), POST_RED);
    bx(&mut g, (1, 22), (55, 56), (1, 22), POST_RED);
    bx(&mut g, (3, 20), (57, 58), (3, 20), POST_RED);
    bx(&mut g, (6, 17), (59, 59), (6, 17), POST_RED);
    // The phone on the west wall: black body, cream dial, handset, shelf.
    bx(&mut g, (2, 4), (27, 37), (6, 16), IRON);
    disc_x(&mut g, 32.0, 9.5, 2.8, (5, 5), CREAM);
    disc_x(&mut g, 32.0, 9.5, 1.0, (5, 5), INK);
    bx(&mut g, (5, 6), (28, 37), (13, 15), INK);
    bx(&mut g, (2, 5), (25, 26), (5, 17), STEEL_DARK);
    prop("phone-booth", g)
}

/// Bus stop (84 x 66 x 24): a pole with a red roundel sign and a timetable at the west end
/// and a shelter beside it: dark frame, pale glass back, teak bench, red roof edge and a
/// lit poster panel on the east end (the glow part).
fn bus_stop() -> Prop {
    let frame = 0x3D4C4A;
    let mut g = VoxelGrid::new(84, 66, 24);
    // Pole, roundel and timetable.
    bx(&mut g, (0, 5), (0, 1), (11, 16), CONCRETE_DARK);
    bx(&mut g, (1, 4), (2, 52), (12, 15), STEEL);
    disc_x(&mut g, 57.0, 14.0, 9.0, (2, 3), POST_RED);
    tint(&mut g, (2, 3), (55, 59), (5, 23), CREAM);
    bx(&mut g, (2, 3), (36, 46), (9, 19), DENIM);
    bx(&mut g, (2, 3), (38, 44), (10, 18), CREAM);
    bx(&mut g, (2, 3), (42, 42), (11, 17), INK);
    bx(&mut g, (2, 3), (40, 40), (11, 15), INK);
    // Shelter: back posts, front end posts, roof.
    for x in [12, 35, 58, 81] {
        bx(&mut g, (x, x + 2), (0, 50), (0, 1), frame);
    }
    for x in [12, 81] {
        bx(&mut g, (x, x + 2), (0, 50), (22, 23), frame);
    }
    bx(&mut g, (12, 83), (51, 53), (0, 23), frame);
    bx(&mut g, (12, 83), (51, 53), (21, 23), POST_RED);
    // Back wall: a kick plate, pale glass, a header rail.
    bx(&mut g, (15, 80), (0, 5), (0, 1), frame);
    bx(&mut g, (15, 80), (49, 50), (0, 1), frame);
    for x in [(15, 34), (38, 57), (61, 80)] {
        bx(&mut g, x, (6, 48), (1, 1), GLASS_PALE);
    }
    // West end: glass. East end: kick plate, lit poster, header.
    bx(&mut g, (13, 13), (6, 48), (2, 21), GLASS_PALE);
    bx(&mut g, (81, 83), (0, 9), (2, 21), frame);
    bx(&mut g, (81, 83), (40, 50), (2, 21), frame);
    for z in 2..=21 {
        for y in 10..=39 {
            let hex = if z == 2 || z == 21 || y == 10 || y == 39 {
                CREAM
            } else if (z as f32 - 11.5).hypot(y as f32 - 31.0) <= 5.0 {
                ORANGE
            } else if y >= 23 {
                0x8CC4DC
            } else if y >= 19 {
                BUTTER
            } else if y >= 15 {
                TEAL
            } else {
                DENIM
            };
            bx(&mut g, (81, 83), (y, y), (z, z), hex);
        }
    }
    let glow = lift(&mut g, (81, 83), (10, 39), (2, 21));
    // Bench along the back wall.
    bx(&mut g, (20, 72), (11, 13), (3, 9), TEAK);
    bx(&mut g, (20, 72), (12, 12), (9, 9), TEAK_DARK);
    for x in [22, 68] {
        bx(&mut g, (x, x + 2), (0, 10), (4, 8), IRON);
    }
    Prop { name: "bus-stop", grid: g, glow: Some(glow) }
}

/// Fire hydrant (9 x 18 x 9): a butter-yellow barrel on a flange with red caps: a dome, a
/// top nut, two side nozzles and a big front pumper nozzle.
fn fire_hydrant() -> Prop {
    let body = 0xE8B93C;
    let cap = 0xB03A2C;
    let mut g = VoxelGrid::new(9, 18, 9);
    cyl(&mut g, 4.5, 4.5, 4.4, (0, 1), shade(body, 0.7));
    cyl(&mut g, 4.5, 4.5, 3.6, (2, 12), body);
    cyl(&mut g, 4.5, 4.5, 3.9, (11, 11), shade(body, 0.8));
    cyl(&mut g, 4.5, 4.5, 3.9, (13, 14), cap);
    cyl(&mut g, 4.5, 4.5, 2.7, (15, 15), cap);
    cyl(&mut g, 4.5, 4.5, 1.6, (16, 17), cap);
    for (x, end) in [((0, 1), 0), ((7, 8), 8)] {
        disc_x(&mut g, 8.5, 4.5, 1.6, x, body);
        disc_x(&mut g, 8.5, 4.5, 1.6, (end, end), cap);
    }
    disc_z(&mut g, 4.5, 6.5, 2.4, (7, 8), body);
    disc_z(&mut g, 4.5, 6.5, 2.4, (8, 8), cap);
    prop("fire-hydrant", g)
}

/// Newspaper box (15 x 21 x 15): a blue box on a steel pedestal, a red header strip, a
/// window showing a stack of cream papers with a dark headline, a brass coin plate.
fn newspaper_box() -> Prop {
    let mut g = VoxelGrid::new(15, 21, 15);
    bx(&mut g, (4, 10), (0, 4), (4, 10), STEEL_DARK);
    bx(&mut g, (0, 14), (5, 18), (0, 14), DENIM);
    bx(&mut g, (0, 14), (19, 20), (0, 14), DENIM_DARK);
    bx(&mut g, (0, 14), (16, 18), (14, 14), POST_RED);
    bx(&mut g, (2, 12), (8, 15), (14, 14), GLASS);
    bx(&mut g, (3, 11), (9, 14), (14, 14), PLASTIC);
    bx(&mut g, (4, 10), (13, 13), (14, 14), INK);
    bx(&mut g, (4, 7), (10, 11), (14, 14), STEEL);
    bx(&mut g, (9, 12), (5, 7), (14, 14), MUSTARD);
    bx(&mut g, (10, 11), (6, 6), (14, 14), INK);
    bx(&mut g, (2, 6), (5, 6), (14, 14), STEEL);
    prop("newspaper-box", g)
}

/// Bollard (6 x 15 x 6): a round post banded black, white, black, white with a domed top.
fn bollard() -> Prop {
    let mut g = VoxelGrid::new(6, 15, 6);
    cyl(&mut g, 3.0, 3.0, 3.0, (0, 2), INK);
    cyl(&mut g, 3.0, 3.0, 3.0, (3, 8), CREAM);
    cyl(&mut g, 3.0, 3.0, 3.0, (9, 11), INK);
    cyl(&mut g, 3.0, 3.0, 3.0, (12, 13), CREAM);
    cyl(&mut g, 3.0, 3.0, 2.0, (14, 14), CREAM);
    prop("bollard", g)
}

/// Crossing beacon (9 x 69 x 9): a pole in eight black and white bands, a black collar
/// and a round amber globe (the glow part).
fn crossing_beacon() -> Prop {
    let mut g = VoxelGrid::new(9, 69, 9);
    for band in 0..8 {
        let hex = if band % 2 == 0 { INK } else { CREAM };
        cyl(&mut g, 4.5, 4.5, 2.75, (band * 7, band * 7 + 6), hex);
    }
    cyl(&mut g, 4.5, 4.5, 3.6, (56, 58), INK);
    cyl(&mut g, 4.5, 4.5, 2.75, (59, 59), INK);
    ball(&mut g, Vec3::new(4.5, 64.5, 4.5), Vec3::splat(4.4), BEACON_AMBER);
    let glow = lift(&mut g, (0, 8), (60, 68), (0, 8));
    Prop { name: "crossing-beacon", grid: g, glow: Some(glow) }
}

/// Planter (24 x 13 x 12): a terracotta trough with a pale rim, soil, a hump of leaves
/// and pink and mustard flower clusters.
fn planter() -> Prop {
    let terracotta = 0xB8603E;
    let mut g = VoxelGrid::new(24, 13, 12);
    bx(&mut g, (0, 23), (0, 6), (0, 11), terracotta);
    bx(&mut g, (0, 23), (6, 6), (0, 11), 0xD08A62);
    bx(&mut g, (1, 22), (6, 6), (1, 10), 0x4A3020);
    bx(&mut g, (0, 23), (1, 1), (0, 11), shade(terracotta, 0.85));
    for (x, y, z, r) in [(5.0, 8.5, 5.0, 4.0), (12.0, 9.0, 6.0, 4.6), (19.0, 8.5, 5.0, 4.0), (8.5, 8.0, 8.5, 3.6), (15.5, 8.0, 8.5, 3.6)] {
        ball(&mut g, Vec3::new(x, y, z), Vec3::new(r, r * 0.8, r), LEAF);
    }
    tint(&mut g, (0, 23), (7, 8), (0, 11), PINE);
    let flowers = [(3, 11, 4), (7, 12, 6), (11, 12, 3), (14, 12, 7), (18, 11, 4), (21, 11, 6), (10, 9, 9), (16, 9, 9)];
    for (i, (x, y, z)) in flowers.into_iter().enumerate() {
        bx(&mut g, (x, x + 1), (y, y + 1), (z, z + 1), if i % 2 == 0 { PINK } else { MUSTARD });
    }
    prop("planter", g)
}

/// Cafe A-board (15 x 24 x 9): two teak boards leaning together, a chalkboard on the front
/// with a chalk coffee cup and two lines of menu.
fn a_board() -> Prop {
    let mut g = VoxelGrid::new(15, 24, 9);
    let front = |y: i32| 8 - (y * 4) / 24;
    for y in 0..24 {
        bx(&mut g, (0, 14), (y, y), (front(y) - 1, front(y)), TEAK);
        let back = y / 8;
        bx(&mut g, (0, 14), (y, y), (back, back + 1), TEAK_DARK);
    }
    tint(&mut g, (2, 12), (5, 20), (4, 8), 0x2F3B38);
    let chalk = |g: &mut VoxelGrid, x: R, y: R| {
        for yy in y.0..=y.1 {
            bx(g, x, (yy, yy), (front(yy) - 1, front(yy)), PLASTIC);
        }
    };
    // A coffee cup with steam, then two lines of menu.
    chalk(&mut g, (4, 8), (11, 14));
    chalk(&mut g, (3, 9), (10, 10));
    chalk(&mut g, (9, 11), (14, 14));
    chalk(&mut g, (9, 11), (11, 11));
    chalk(&mut g, (11, 11), (12, 13));
    chalk(&mut g, (5, 5), (16, 18));
    chalk(&mut g, (7, 7), (16, 17));
    chalk(&mut g, (3, 11), (7, 7));
    chalk(&mut g, (3, 8), (5, 5));
    prop("a-board", g)
}

/// Parking meter (8 x 33 x 6): a slim steel pole and a green coin-meter head with a cream
/// dial window (red flag) and a coin plate with a slot.
fn parking_meter() -> Prop {
    let mut g = VoxelGrid::new(8, 33, 6);
    bx(&mut g, (1, 6), (0, 0), (0, 5), CONCRETE_DARK);
    bx(&mut g, (2, 5), (1, 23), (1, 4), STEEL);
    bx(&mut g, (0, 7), (24, 31), (0, 5), 0x3D6B54);
    bx(&mut g, (1, 6), (32, 32), (1, 4), 0x3D6B54);
    bx(&mut g, (2, 5), (24, 26), (5, 5), STEEL);
    bx(&mut g, (3, 4), (25, 25), (5, 5), INK);
    bx(&mut g, (2, 5), (27, 30), (5, 5), CREAM);
    bx(&mut g, (3, 4), (29, 29), (5, 5), POST_RED);
    prop("parking-meter", g)
}

// ---------------------------------------------------------------------------------------
// Cars. All lie along x with the nose toward +x; the body spans z 1..40 and the mirrors
// stick out to z 0 and 41, so every car is 42 wide (1.75 m).

/// The body of a car, built column by column from a side profile.
struct Hull {
    /// First and last body column.
    x: R,
    /// Body width at the belt line and below.
    z: R,
    /// Lowest body voxel.
    sill: i32,
    /// Top of the lower body; the greenhouse (windows and roof) starts above.
    belt: i32,
    /// Plan-view corner radius at the nose and tail.
    corner: f32,
    /// Ease between profile points with a cosine, for rounder shapes.
    smooth: bool,
    /// Profile points: column, height of that column's top voxel.
    keys: &'static [(i32, i32)],
}

impl Hull {
    fn top(&self, x: i32) -> i32 {
        let k = self.keys;
        let i = k.iter().rposition(|&(kx, _)| kx <= x).unwrap_or(0).min(k.len() - 2);
        let ((x0, y0), (x1, y1)) = (k[i], k[i + 1]);
        let t = ((x - x0) as f32 / (x1 - x0) as f32).clamp(0.0, 1.0);
        let t = if self.smooth { 0.5 - 0.5 * (t * PI).cos() } else { t };
        (y0 as f32 + (y1 - y0) as f32 * t).round() as i32
    }

    /// The z range of the body at column `x`, height `y`: rounded at the ends, tucked in
    /// above the belt line, with the top row chamfered.
    fn z_range(&self, x: i32, y: i32) -> R {
        let end = (x - self.x.0).min(self.x.1 - x) as f32;
        let mut inset = 0;
        if end < self.corner {
            inset = (self.corner - (self.corner * self.corner - (self.corner - end - 0.5).powi(2)).sqrt()).round() as i32;
        }
        if y > self.belt {
            inset += 2 + (y - self.belt - 1) / 4;
        }
        if y == self.top(x) {
            inset += 1;
        }
        (self.z.0 + inset, self.z.1 - inset)
    }

    fn build(&self, g: &mut VoxelGrid, hex: u32) {
        for x in self.x.0..=self.x.1 {
            for y in self.sill..=self.top(x) {
                let (z0, z1) = self.z_range(x, y);
                bx(g, (x, x), (y, y), (z0, z1), hex);
            }
        }
    }

    /// Paints the outer `layers` voxels of both sides over a rectangle of columns and rows.
    fn paint_sides(&self, g: &mut VoxelGrid, x: R, y: R, hex: u32, layers: i32) {
        for xx in x.0..=x.1 {
            for yy in y.0..=y.1 {
                let (z0, z1) = self.z_range(xx, yy);
                for k in 0..layers {
                    for z in [z0 + k, z1 - k] {
                        if g.get(xx, yy, z).is_some() {
                            g.set(xx, yy, z, Some(c(hex)));
                        }
                    }
                }
            }
        }
    }

    /// Side windows over columns `xs`, from row `y0` up to two below the roofline: dark
    /// glass with one light streak, following the slopes of the pillars.
    fn side_glass(&self, g: &mut VoxelGrid, xs: R, y0: i32) {
        for x in xs.0..=xs.1 {
            for y in y0..=self.top(x) - 2 {
                let d = (x - xs.0) - (y - y0);
                let hex = if d == 7 || d == 8 { GLASS_STREAK } else { GLASS };
                self.paint_sides(g, (x, x), (y, y), hex, 2);
            }
        }
    }

    /// Windscreen or rear window: the two top voxels of the sloping columns `xs`, kept
    /// clear of the pillars at the sides.
    fn screen(&self, g: &mut VoxelGrid, xs: R, glass: u32) {
        for x in xs.0..=xs.1 {
            let top = self.top(x);
            for y in top - 1..=top {
                let (z0, z1) = self.z_range(x, y);
                for z in z0 + 2..=z1 - 2 {
                    let d = (z - z0 - 2) - 2 * (x - xs.0);
                    g.set(x, y, z, Some(c(if d == 8 || d == 9 { GLASS_STREAK } else { glass })));
                }
            }
        }
    }
}

/// A wheel in an arch on both sides of the body. The arch is cut clean through the side
/// skin (its back wall painted dark), so the whole wheel shows: black tyre, steel rim, a
/// big chrome hubcap with a darker centre cap.
fn wheel(g: &mut VoxelGrid, cx: f32, cy: f32, r: f32, z: R) {
    for (outer, dir) in [(z.0 + 1, 1), (z.1 - 1, -1)] {
        let side = outer - dir;
        let wall = side + 6 * dir;
        for yy in 0..=(cy + r + 3.0) as i32 {
            for xx in span(cx, r + 2.0).0..=span(cx, r + 2.0).1 {
                if (xx as f32 + 0.5 - cx).hypot(yy as f32 + 0.5 - cy) > r + 1.4 {
                    continue;
                }
                for k in 0..6 {
                    g.set(xx, yy, side + k * dir, None);
                }
                if g.get(xx, yy, wall).is_some() {
                    g.set(xx, yy, wall, Some(c(0x13100F)));
                }
            }
        }
        let tyre_z = (outer.min(outer + 4 * dir), outer.max(outer + 4 * dir));
        disc_z(g, cx, cy, r, tyre_z, 0x3A3734);
        disc_z(g, cx, cy, r * 0.72, (outer, outer), STEEL);
        disc_z(g, cx, cy, r * 0.58, (outer, outer), HUBCAP);
        disc_z(g, cx, cy, r * 0.22, (outer, outer), STEEL_DARK);
    }
}

/// A round headlight facing +x or -x: a dark bezel around a bright lens, so it shows on
/// any body colour.
fn headlight(g: &mut VoxelGrid, x: i32, yc: f32, zc: f32, r: f32) {
    disc_x(g, yc, zc, r + 0.1, (x, x), 0x2B2926);
    disc_x(g, yc, zc, r - 1.0, (x, x), 0xFFF3C4);
}

/// Mirrors on both doors: a short dark arm and a small box.
fn mirrors(g: &mut VoxelGrid, x: R, y: i32, hex: u32) {
    for z in [0, 41] {
        bx(g, x, (y, y + 2), (z, z), hex);
    }
}

/// Small hatchback (94 x 34 x 42): short bonnet, a tall greenhouse with a steep hatch,
/// body-colour doors, chrome bumpers with a cream front plate and a yellow rear plate,
/// round headlights and red tail lights painted on.
fn hatchback(name: &'static str, body: u32) -> Prop {
    let mut g = VoxelGrid::new(94, 34, 42);
    let hull = Hull {
        x: (2, 91),
        z: (1, 40),
        sill: 3,
        belt: 21,
        corner: 6.0,
        smooth: false,
        keys: &[(2, 18), (4, 21), (8, 21), (22, 33), (56, 33), (70, 22), (73, 21), (91, 19)],
    };
    hull.build(&mut g, body);
    hull.side_glass(&mut g, (12, 39), 23);
    hull.side_glass(&mut g, (44, 67), 23);
    hull.screen(&mut g, (56, 70), windscreen(body));
    hull.screen(&mut g, (9, 22), windscreen(body));
    // Door seams, handles and mirrors.
    for x in [40, 63] {
        hull.paint_sides(&mut g, (x, x), (6, 21), shade(body, 0.7), 1);
    }
    hull.paint_sides(&mut g, (45, 49), (19, 19), CHROME, 1);
    mirrors(&mut g, (64, 65), 20, INK);
    wheel(&mut g, 16.5, 7.5, 7.2, (1, 40));
    wheel(&mut g, 74.5, 7.5, 7.2, (1, 40));
    // Front: chrome bumper and plate, grille, headlights.
    bx(&mut g, (92, 93), (6, 9), (3, 38), CHROME);
    bx(&mut g, (93, 93), (6, 9), (15, 26), CREAM);
    bx(&mut g, (93, 93), (7, 8), (3, 5), ORANGE);
    bx(&mut g, (93, 93), (7, 8), (36, 38), ORANGE);
    bx(&mut g, (91, 91), (11, 17), (13, 28), CHROME);
    bx(&mut g, (91, 91), (12, 16), (14, 27), INK);
    headlight(&mut g, 91, 15.5, 9.5, 3.5);
    headlight(&mut g, 91, 15.5, 31.5, 3.5);
    // Rear: chrome bumper, yellow plate, tail lights.
    bx(&mut g, (0, 1), (6, 9), (3, 38), CHROME);
    bx(&mut g, (0, 0), (6, 9), (15, 26), MUSTARD);
    for z in [(5, 10), (31, 36)] {
        bx(&mut g, (2, 2), (13, 18), z, 0xC02A24);
    }
    prop(name, g)
}

/// Rounded older saloon (108 x 34 x 42): long bonnet and boot, a smooth bubble of a
/// greenhouse, a chrome strip along the flank, big round headlights over a chrome grille,
/// chrome bumpers with overriders.
fn saloon(name: &'static str, body: u32) -> Prop {
    let mut g = VoxelGrid::new(108, 34, 42);
    let hull = Hull {
        x: (2, 105),
        z: (1, 40),
        sill: 3,
        belt: 21,
        corner: 8.0,
        smooth: true,
        keys: &[(2, 18), (5, 21), (26, 21), (42, 33), (70, 33), (80, 22), (84, 21), (105, 19)],
    };
    hull.build(&mut g, body);
    hull.side_glass(&mut g, (31, 55), 23);
    hull.side_glass(&mut g, (59, 76), 23);
    hull.screen(&mut g, (30, 41), windscreen(body));
    hull.screen(&mut g, (71, 80), windscreen(body));
    for x in [56, 78] {
        hull.paint_sides(&mut g, (x, x), (6, 21), shade(body, 0.7), 1);
    }
    hull.paint_sides(&mut g, (60, 64), (19, 19), CHROME, 1);
    hull.paint_sides(&mut g, (8, 101), (14, 14), CHROME, 1);
    mirrors(&mut g, (77, 78), 20, CHROME);
    wheel(&mut g, 22.5, 7.5, 7.2, (1, 40));
    wheel(&mut g, 84.5, 7.5, 7.2, (1, 40));
    // Front.
    bx(&mut g, (106, 107), (6, 9), (3, 38), CHROME);
    for z in [(8, 9), (32, 33)] {
        bx(&mut g, (106, 107), (10, 12), z, CHROME);
    }
    bx(&mut g, (107, 107), (6, 9), (15, 26), CREAM);
    bx(&mut g, (105, 105), (10, 17), (15, 26), CHROME);
    bx(&mut g, (105, 105), (11, 16), (16, 25), INK);
    for z in [19, 22] {
        bx(&mut g, (105, 105), (11, 16), (z, z), CHROME);
    }
    headlight(&mut g, 105, 14.5, 11.5, 3.6);
    headlight(&mut g, 105, 14.5, 29.5, 3.6);
    // Rear.
    bx(&mut g, (0, 1), (6, 9), (3, 38), CHROME);
    for z in [(8, 9), (32, 33)] {
        bx(&mut g, (0, 1), (10, 12), z, CHROME);
    }
    bx(&mut g, (0, 0), (6, 9), (15, 26), MUSTARD);
    for z in [(7, 13), (28, 34)] {
        bx(&mut g, (2, 2), (12, 17), z, 0xC02A24);
    }
    prop(name, g)
}

/// Delivery van (96 x 42 x 42): a tall box body with a flat roof, a short cab with a steep
/// windscreen, a coloured stripe and a logo plate down the side, rear doors with two small
/// windows, chrome mirrors on arms.
fn van(name: &'static str, body: u32) -> Prop {
    let accent = if is_light(body) { POST_RED } else { CREAM };
    let mut g = VoxelGrid::new(96, 42, 42);
    let hull = Hull {
        x: (2, 93),
        z: (1, 40),
        sill: 4,
        belt: 40,
        corner: 6.0,
        smooth: false,
        keys: &[(2, 38), (3, 41), (72, 41), (84, 27), (87, 26), (93, 24)],
    };
    hull.build(&mut g, body);
    hull.side_glass(&mut g, (64, 82), 29);
    hull.screen(&mut g, (73, 84), windscreen(body));
    // Stripe along the flank and across the back, logo plate with a sun.
    hull.paint_sides(&mut g, (3, 92), (19, 23), accent, 2);
    bx(&mut g, (2, 3), (19, 23), (5, 36), accent);
    hull.paint_sides(&mut g, (22, 47), (27, 37), accent, 2);
    hull.paint_sides(&mut g, (23, 46), (28, 36), PLASTIC, 2);
    for x in 24..=45 {
        for y in 29..=35 {
            if (x as f32 - 34.5).hypot(y as f32 - 32.0) <= 3.2 {
                hull.paint_sides(&mut g, (x, x), (y, y), MUSTARD, 2);
            }
        }
    }
    // Door seams, cab handle, rear doors with windows, mirrors.
    for x in [62, 52] {
        hull.paint_sides(&mut g, (x, x), (6, 39), shade(body, 0.7), 1);
    }
    hull.paint_sides(&mut g, (66, 69), (24, 24), CHROME, 1);
    bx(&mut g, (2, 3), (6, 39), (20, 21), shade(body, 0.7));
    for z in [(7, 17), (24, 34)] {
        bx(&mut g, (2, 3), (26, 35), z, GLASS);
    }
    for z in [0, 41] {
        bx(&mut g, (79, 80), (27, 31), (z, z), CHROME);
    }
    wheel(&mut g, 19.5, 8.5, 8.2, (1, 40));
    wheel(&mut g, 77.5, 8.5, 8.2, (1, 40));
    // Front: dark bumper, grille, headlights, plate.
    bx(&mut g, (94, 95), (6, 10), (3, 38), STEEL_DARK);
    bx(&mut g, (95, 95), (6, 10), (15, 26), CREAM);
    bx(&mut g, (93, 93), (13, 20), (14, 27), INK);
    bx(&mut g, (93, 93), (14, 19), (15, 26), CHROME);
    headlight(&mut g, 93, 17.0, 9.5, 3.6);
    headlight(&mut g, 93, 17.0, 31.5, 3.6);
    // Rear: bumper, plate, tail lights.
    bx(&mut g, (0, 1), (6, 10), (3, 38), STEEL_DARK);
    bx(&mut g, (0, 0), (6, 10), (15, 26), MUSTARD);
    for z in [(2, 4), (37, 39)] {
        bx(&mut g, (2, 2), (12, 24), z, 0xC02A24);
    }
    prop(name, g)
}

// ---------------------------------------------------------------------------------------
// Second batch: the small things that make a street look lived in.

/// A straight bar in the x-y plane from corner `a` to corner `b`, `t` voxels thick, over
/// the z range.
fn bar(g: &mut VoxelGrid, a: (i32, i32), b: (i32, i32), t: i32, z: R, hex: u32) {
    let n = (b.0 - a.0).abs().max((b.1 - a.1).abs()).max(1);
    for i in 0..=n {
        let k = i as f32 / n as f32;
        let x = (a.0 as f32 + (b.0 - a.0) as f32 * k).round() as i32;
        let y = (a.1 as f32 + (b.1 - a.1) as f32 * k).round() as i32;
        bx(g, (x, x + t - 1), (y, y + t - 1), z, hex);
    }
}

/// A ring standing across z (a tyre or hoop), centred at (cx, cy).
fn ring_z(g: &mut VoxelGrid, cx: f32, cy: f32, r_out: f32, r_in: f32, z: R, hex: u32) {
    shape(g, span(cx, r_out), span(cy, r_out), z, hex, |p| {
        let d = (p.x - cx).hypot(p.y - cy);
        d <= r_out && d >= r_in
    });
}

/// A 3 x 5 capital letter facing +z, with its lower-left voxel at (x, y).
fn glyph(g: &mut VoxelGrid, ch: char, x: i32, y: i32, z: i32, hex: u32) {
    let rows: [&str; 5] = match ch {
        'M' => ["X.X", "XXX", "XXX", "X.X", "X.X"],
        'A' => [".X.", "X.X", "XXX", "X.X", "X.X"],
        'P' => ["XX.", "X.X", "XX.", "X..", "X.."],
        'L' => ["X..", "X..", "X..", "X..", "XXX"],
        'E' => ["XXX", "X..", "XX.", "X..", "XXX"],
        'R' => ["XX.", "X.X", "XX.", "X.X", "X.X"],
        'O' => ["XXX", "X.X", "X.X", "X.X", "XXX"],
        'D' => ["XX.", "X.X", "X.X", "X.X", "XX."],
        _ => ["...", "...", "...", "...", "..."],
    };
    for (i, row) in rows.iter().enumerate() {
        for (j, cell) in row.chars().enumerate() {
            if cell == 'X' {
                g.set(x + j as i32, y + 4 - i as i32, z, Some(c(hex)));
            }
        }
    }
}

/// Bicycle (44 x 28 x 10): a blue diamond frame, two spoked wheels with black tyres, a
/// saddle, handlebars, a chainring with pedals and a kickstand. It stands on its own.
fn bicycle() -> Prop {
    let frame = 0x3E6EA8;
    let mut g = VoxelGrid::new(44, 28, 10);
    for cx in [9.5, 34.5] {
        ring_z(&mut g, cx, 8.5, 8.2, 6.3, (4, 5), 0x2B2926);
        let x = cx as i32;
        bx(&mut g, (x - 6, x + 5), (8, 8), (4, 4), 0xB8C2CC);
        bx(&mut g, (x, x), (2, 14), (4, 4), 0xB8C2CC);
        bar(&mut g, (x - 4, 4), (x + 4, 12), 1, (4, 4), 0xB8C2CC);
        bar(&mut g, (x - 4, 12), (x + 4, 4), 1, (4, 4), 0xB8C2CC);
        bx(&mut g, (x - 1, x), (8, 9), (4, 5), STEEL_DARK);
    }
    // Frame: seat tube, top tube, down tube, stays, fork, head tube.
    bar(&mut g, (17, 8), (14, 22), 2, (4, 5), frame);
    bar(&mut g, (15, 21), (31, 21), 2, (4, 5), frame);
    bar(&mut g, (19, 7), (31, 18), 2, (4, 5), frame);
    bar(&mut g, (10, 8), (15, 21), 1, (4, 5), frame);
    bar(&mut g, (10, 8), (19, 7), 1, (4, 5), frame);
    bar(&mut g, (31, 17), (34, 8), 2, (4, 5), frame);
    bx(&mut g, (30, 31), (17, 22), (4, 5), frame);
    // Saddle, handlebars, chainring, pedals, kickstand.
    bx(&mut g, (14, 15), (22, 24), (4, 5), STEEL);
    bx(&mut g, (11, 19), (25, 26), (3, 6), INK);
    bx(&mut g, (30, 31), (23, 26), (4, 5), STEEL);
    bx(&mut g, (30, 32), (26, 27), (1, 8), INK);
    bx(&mut g, (30, 32), (26, 27), (1, 2), MUSTARD);
    bx(&mut g, (30, 32), (26, 27), (7, 8), MUSTARD);
    disc_z(&mut g, 20.0, 7.5, 3.0, (6, 6), STEEL_DARK);
    bx(&mut g, (18, 21), (3, 4), (7, 8), INK);
    bx(&mut g, (18, 21), (10, 11), (1, 2), INK);
    bx(&mut g, (14, 15), (0, 6), (7, 8), STEEL);
    prop("bicycle", g)
}

/// Window flower box (24 x 14 x 10): a cream wooden box on a sill with soil, a hump of
/// leaves, red and mustard flowers and vines trailing over the front.
fn window_flower_box() -> Prop {
    let mut g = VoxelGrid::new(24, 14, 10);
    bx(&mut g, (0, 23), (0, 6), (0, 8), PLASTIC);
    bx(&mut g, (0, 23), (0, 1), (0, 8), shade(PLASTIC, 0.8));
    bx(&mut g, (0, 23), (3, 3), (8, 8), shade(PLASTIC, 0.85));
    bx(&mut g, (1, 22), (6, 6), (1, 7), 0x4A3020);
    for x in [3.0, 7.5, 12.0, 16.5, 21.0] {
        ball(&mut g, Vec3::new(x, 8.0, 4.5), Vec3::new(3.6, 2.8, 3.4), LEAF);
    }
    tint(&mut g, (0, 23), (7, 8), (0, 9), PINE);
    let flowers = [(2, 10, 4), (6, 11, 3), (10, 10, 5), (13, 11, 3), (17, 10, 5), (20, 11, 4), (8, 10, 6), (15, 10, 2)];
    for (i, (x, y, z)) in flowers.into_iter().enumerate() {
        bx(&mut g, (x, x + 1), (y, y + 1), (z, z + 1), if i % 2 == 0 { 0xD04030 } else { MUSTARD });
    }
    for (x, len) in [(2, 4), (6, 6), (10, 3), (14, 7), (18, 4), (21, 5)] {
        bx(&mut g, (x, x + 1), (6 - len, 6), (9, 9), if x % 8 < 4 { LEAF } else { PINE });
    }
    prop("window-flower-box", g)
}

/// Washing line (78 x 52 x 9): two grey T-posts about 3 m apart, two sagging lines, and on
/// the front one a blue shirt, a cream sheet, a pair of socks, a red towel and trousers.
fn washing_line() -> Prop {
    let post = 0x5E666E;
    let rope = 0xE8E0CC;
    let mut g = VoxelGrid::new(78, 52, 9);
    for x in [0, 75] {
        bx(&mut g, (x, x + 2), (0, 49), (3, 5), post);
        bx(&mut g, (x, x + 2), (50, 51), (0, 8), post);
        bx(&mut g, (x, x + 2), (0, 1), (2, 6), CONCRETE_DARK);
    }
    let line_y = |x: i32| 50 - (4.0 * (PI * (x - 1) as f32 / 76.0).sin()).round() as i32;
    for x in 1..=76 {
        bx(&mut g, (x, x), (line_y(x), line_y(x)), (7, 7), rope);
        bx(&mut g, (x, x), (line_y(x), line_y(x)), (1, 1), rope);
    }
    // Laundry hangs from the front line: columns follow the sag.
    let mut cloth = |x0: i32, w: i32, h: i32, z: i32, paint: &dyn Fn(i32, i32) -> Option<u32>| {
        for dx in 0..w {
            for dy in 0..h {
                if let Some(hex) = paint(dx, dy) {
                    g.set(x0 + dx, line_y(x0 + dx) - 1 - dy, z, Some(c(hex)));
                }
            }
        }
    };
    // Shirt: torso and two sleeves.
    cloth(7, 11, 12, 7, &|dx, dy| ((2..=8).contains(&dx) || dy < 6).then_some(if dy < 2 && (4..=6).contains(&dx) { CREAM } else { 0x5F8FB8 }));
    // Sheet: wavy, with a pale blue hem.
    cloth(20, 22, 16, 7, &|_, dy| Some(if dy >= 14 { 0x9FC7D8 } else { 0xF2EEE2 }));
    // Two socks, towel, trousers.
    cloth(45, 2, 7, 7, &|_, dy| Some(if dy < 2 { CREAM } else if dy >= 5 { RUST } else { MUSTARD }));
    cloth(49, 2, 7, 7, &|_, dy| Some(if dy < 2 { CREAM } else if dy >= 5 { RUST } else { TEAL }));
    cloth(53, 9, 13, 7, &|_, dy| Some(if (9..=10).contains(&dy) { CREAM } else { 0xC84B4B }));
    cloth(65, 9, 17, 7, &|dx, dy| (dy < 5 || dx < 4 || dx > 4).then_some(DENIM_DARK));
    // Fold the sheet: every other 6 columns sit one voxel behind the line.
    for x in 20..42 {
        if (x - 20) / 6 % 2 == 1 {
            for y in 0..52 {
                if let Some(col) = g.get(x, y, 7) {
                    if y < line_y(x) {
                        g.set(x, y, 7, None);
                        g.set(x, y, 6, Some(col));
                    }
                }
            }
        }
    }
    prop("washing-line", g)
}

/// Garden shed (52 x 51 x 40, about 2 x 1.5 m): vertical timber boards in two close tones,
/// a felt roof pitched along x with an overhang, a ledged door with cream trim and handle,
/// and a small cream-framed window beside it. The door faces +z.
fn garden_shed() -> Prop {
    let mut g = VoxelGrid::new(52, 52, 40);
    let board = |i: i32| [0x8B5E3C, 0x946543, 0x835838][(hash(IVec3::new(i, 0, 0), 7) % 3) as usize];
    // Height of the roof's underside at depth z: level at the eaves, pitching up to the ridge.
    let under = |z: i32| 38 + ((17.5 - (z as f32 - 19.5).abs()) * 0.6).round() as i32;
    for x in 2..=49 {
        for y in 1..=37 {
            for z in [2, 3, 36, 37] {
                g.set(x, y, z, Some(c(board(x / 3))));
            }
        }
    }
    for z in 2..=37 {
        for y in 1..under(z) {
            for x in [2, 3, 48, 49] {
                g.set(x, y, z, Some(c(board(z / 3 + 20))));
            }
        }
    }
    bx(&mut g, (2, 49), (0, 0), (2, 37), shade(0x8B5E3C, 0.55));
    // Roof: felt in strips, with a lighter drip edge along the eaves.
    for z in 0..40 {
        for x in 0..52 {
            let y = under(z);
            let felt = if (x / 6) % 2 == 0 { 0x373C3F } else { 0x40454A };
            bx(&mut g, (x, x), (y, y + 1), (z, z), felt);
        }
    }
    bx(&mut g, (0, 51), (under(0), under(0) + 1), (0, 0), 0x5A6064);
    bx(&mut g, (0, 51), (under(39), under(39) + 1), (39, 39), 0x5A6064);
    // Door: darker boards, ledges and a brace, cream trim, a handle.
    tint(&mut g, (30, 41), (1, 34), (36, 37), 0x6E4A30);
    for y in [4, 17, 30] {
        bx(&mut g, (30, 41), (y, y + 1), (37, 37), 0x7E573A);
    }
    bar(&mut g, (31, 5), (40, 29), 2, (37, 37), 0x8E6240);
    bx(&mut g, (29, 29), (1, 35), (37, 37), PLASTIC);
    bx(&mut g, (42, 42), (1, 35), (37, 37), PLASTIC);
    bx(&mut g, (29, 42), (35, 35), (37, 37), PLASTIC);
    bx(&mut g, (32, 34), (17, 18), (38, 38), IRON);
    bx(&mut g, (30, 30), (4, 5), (38, 38), IRON);
    bx(&mut g, (30, 30), (30, 31), (38, 38), IRON);
    // Window: cream frame, pale glass, a cross of bars, a sill.
    bx(&mut g, (8, 19), (16, 29), (36, 37), PLASTIC);
    bx(&mut g, (9, 18), (17, 28), (37, 37), GLASS_PALE);
    bx(&mut g, (13, 14), (17, 28), (37, 37), PLASTIC);
    bx(&mut g, (9, 18), (22, 23), (37, 37), PLASTIC);
    bx(&mut g, (7, 20), (15, 15), (37, 38), PLASTIC);
    prop("garden-shed", g)
}

/// White picket fence segment (24 x 27 x 4), tileable along x: a post at the left end and
/// four pointed pickets (3 wide, two voxels apart), held by two rails behind them.
fn picket_fence() -> Prop {
    let white = 0xF0ECE0;
    let mut g = VoxelGrid::new(24, 27, 4);
    bx(&mut g, (0, 23), (5, 6), (0, 1), shade(white, 0.88));
    bx(&mut g, (0, 23), (15, 16), (0, 1), shade(white, 0.88));
    bx(&mut g, (0, 3), (0, 25), (0, 3), white);
    bx(&mut g, (1, 2), (26, 26), (1, 2), white);
    for i in 0..4 {
        let x = 4 + i * 5;
        bx(&mut g, (x, x + 2), (0, 21), (2, 3), white);
        bx(&mut g, (x + 1, x + 1), (22, 22), (2, 3), white);
    }
    prop("picket-fence", g)
}

/// Wooden panel fence segment (24 x 37 x 4), tileable along x: a brown post at the left
/// end and a lap-board panel (each board a lighter face over a darker lower edge) under
/// a dark cap rail.
fn panel_fence() -> Prop {
    let mut g = VoxelGrid::new(24, 37, 4);
    bx(&mut g, (0, 2), (0, 36), (0, 3), TEAK_DARK);
    bx(&mut g, (0, 2), (36, 36), (0, 3), shade(TEAK_DARK, 0.75));
    bx(&mut g, (3, 23), (0, 1), (1, 2), shade(TEAK_DARK, 0.7));
    for k in 0..6 {
        let y = 2 + k * 5;
        let tone = [TEAK, 0xA67848, 0x8F6240][(k % 3) as usize];
        bx(&mut g, (3, 23), (y + 1, y + 4), (1, 2), tone);
        bx(&mut g, (3, 23), (y, y), (1, 2), shade(tone, 0.72));
    }
    bx(&mut g, (3, 23), (32, 34), (0, 3), TEAK_DARK);
    bx(&mut g, (3, 23), (35, 35), (0, 3), shade(TEAK_DARK, 0.8));
    prop("panel-fence", g)
}

/// Utility pole (64 x 171 x 8, about 7 m): a weathered wooden pole on a dark tarred foot,
/// a crossarm with two braces and four green glass insulators, one more insulator on
/// the tip, and a small mustard number plate. The wires are drawn elsewhere.
fn utility_pole() -> Prop {
    let wood = 0x7B6650;
    let mut g = VoxelGrid::new(64, 171, 8);
    cyl(&mut g, 32.0, 4.0, 3.0, (0, 9), 0x2E2620);
    cyl(&mut g, 32.0, 4.0, 2.8, (10, 163), wood);
    cyl(&mut g, 32.0, 4.0, 2.1, (164, 165), wood);
    bx(&mut g, (31, 33), (40, 45), (7, 7), MUSTARD);
    bx(&mut g, (2, 61), (152, 154), (2, 5), 0x6E5A46);
    bar(&mut g, (29, 140), (17, 152), 2, (3, 4), 0x6E5A46);
    bar(&mut g, (33, 140), (45, 152), 2, (3, 4), 0x6E5A46);
    let glass = 0x8CC4B0;
    for x in [8.5, 20.5, 43.5, 55.5] {
        cyl(&mut g, x, 4.0, 2.0, (155, 156), glass);
        cyl(&mut g, x, 4.0, 1.4, (157, 159), glass);
    }
    cyl(&mut g, 32.0, 4.0, 2.0, (166, 167), glass);
    cyl(&mut g, 32.0, 4.0, 1.4, (168, 170), glass);
    prop("utility-pole", g)
}

/// Galvanised trash can (14 x 22 x 12): a corrugated grey steel drum with side handles
/// and a domed lid with a knob, the 1970s kind. Name avoids "bin" so the street keeps
/// choosing the litter bin.
fn trash_can() -> Prop {
    let steel = 0x959CA4;
    let mut g = VoxelGrid::new(14, 22, 12);
    cyl(&mut g, 7.0, 6.0, 5.4, (0, 16), steel);
    for y in [1, 6, 11] {
        cyl(&mut g, 7.0, 6.0, 5.9, (y, y), shade(steel, 0.85));
    }
    cyl(&mut g, 7.0, 6.0, 5.9, (15, 15), shade(steel, 0.8));
    cyl(&mut g, 7.0, 6.0, 6.0, (16, 17), 0xAAB2BA);
    cyl(&mut g, 7.0, 6.0, 4.6, (18, 18), 0xAAB2BA);
    cyl(&mut g, 7.0, 6.0, 3.0, (19, 19), 0xAAB2BA);
    bx(&mut g, (5, 8), (20, 20), (5, 6), STEEL_DARK);
    bx(&mut g, (5, 5), (19, 20), (5, 6), STEEL_DARK);
    bx(&mut g, (8, 8), (19, 20), (5, 6), STEEL_DARK);
    for x in [0, 12] {
        bx(&mut g, (x, x + 1), (11, 13), (5, 6), STEEL_DARK);
    }
    prop("trash-can", g)
}

/// A pair of black rubbish bags (22 x 14 x 10): fat lumpy sacks, one upright and one
/// slumped against it, each tied off with a twisted neck and a pale sheen on the shoulder.
fn rubbish_bags() -> Prop {
    let black = 0x34323C;
    let sheen = 0x5A5868;
    let mut g = VoxelGrid::new(22, 14, 10);
    ball(&mut g, Vec3::new(6.0, 5.0, 5.0), Vec3::new(5.6, 5.0, 4.6), black);
    cyl(&mut g, 6.0, 5.0, 1.5, (9, 11), black);
    bx(&mut g, (5, 8), (12, 12), (4, 6), black);
    tint(&mut g, (2, 5), (7, 9), (2, 4), sheen);
    ball(&mut g, Vec3::new(15.5, 4.0, 5.5), Vec3::new(4.8, 4.0, 4.2), black);
    cyl(&mut g, 16.0, 5.5, 1.3, (7, 9), black);
    bx(&mut g, (16, 18), (10, 10), (5, 6), black);
    tint(&mut g, (12, 14), (5, 7), (3, 5), sheen);
    prop("rubbish-bags", g)
}

/// Builder's skip (62 x 38 x 30, about 2.5 m long): a yellow tub with sloped sides and
/// ends on two skids, red and white hazard boards on the ends, heaped with bricks, plaster,
/// concrete and planks, a broken wooden chair sticking out of the top.
fn builders_skip() -> Prop {
    let yellow = 0xE8B93C;
    let mut g = VoxelGrid::new(62, 38, 30);
    let xr = |y: i32| (9 - (8 * y) / 25, 52 + (8 * y) / 25);
    let zr = |y: i32| (6 - (5 * y) / 25, 23 + (5 * y) / 25);
    for y in 2..=25 {
        let ((x0, x1), (z0, z1)) = (xr(y), zr(y));
        for x in x0..=x1 {
            for z in z0..=z1 {
                if y <= 3 || x < x0 + 2 || x > x1 - 2 || z < z0 + 2 || z > z1 - 2 {
                    g.set(x, y, z, Some(c(yellow)));
                }
            }
        }
    }
    // Rim, skids, hazard boards on both ends.
    let ((x0, x1), (z0, z1)) = (xr(25), zr(25));
    for x in x0 - 1..=x1 + 1 {
        for z in z0 - 1..=z1 + 1 {
            if x < x0 + 1 || x > x1 - 1 || z < z0 + 1 || z > z1 - 1 {
                g.set(x, 26, z, Some(c(shade(yellow, 0.82))));
            }
        }
    }
    for z in [(8, 11), (18, 21)] {
        bx(&mut g, (11, 50), (0, 1), z, 0x3A3836);
    }
    for y in 12..=21 {
        for z in 6..=24 {
            let hex = if (y + z) / 3 % 2 == 0 { 0xC0392B } else { CREAM };
            for x in [xr(y).0, xr(y).0 + 1, xr(y).1 - 1, xr(y).1] {
                if g.get(x, y, z).is_some() {
                    g.set(x, y, z, Some(c(hex)));
                }
            }
        }
    }
    // The heap: chunky 3 x 2 x 3 lumps of rubble, highest in the middle.
    let rubble = [0xA8553A, 0xB8603E, 0xA8553A, 0xE8E0CC, 0x9A958C, 0x9A958C, TEAK, TEAK_DARK, 0x5A4636, 0xC8C0A8];
    for x in 3..=58 {
        for z in 3..=26 {
            let (u, v) = ((x as f32 - 30.5) / 28.0, (z as f32 - 14.5) / 12.0);
            let top = 29 - (7.0 * (u * u + v * v)).round() as i32;
            for y in 4..=top {
                let (ix, iz) = (xr(y.min(25)), zr(y.min(25)));
                if x >= ix.0 + 2 && x <= ix.1 - 2 && z >= iz.0 + 2 && z <= iz.1 - 2 {
                    let pick = hash(IVec3::new(x / 3, y / 2, z / 3), 11) as usize % rubble.len();
                    g.set(x, y, z, Some(c(rubble[pick])));
                }
            }
        }
    }
    // Planks and the broken chair poking out.
    bar(&mut g, (8, 25), (18, 34), 2, (6, 8), TEAK);
    bar(&mut g, (46, 26), (54, 31), 2, (19, 21), TEAK_DARK);
    let chair = 0xC49A6A;
    bx(&mut g, (30, 31), (24, 37), (12, 14), chair);
    bx(&mut g, (40, 41), (24, 33), (12, 14), chair);
    bx(&mut g, (30, 41), (28, 29), (12, 14), 0xA67848);
    bx(&mut g, (30, 36), (33, 34), (12, 14), 0xA67848);
    bx(&mut g, (22, 30), (26, 28), (9, 16), POST_RED);
    bar(&mut g, (44, 25), (51, 34), 2, (14, 15), chair);
    prop("builders-skip", g)
}

/// Crates and bottles (41 x 20 x 12): two slatted wooden crates side by side, one with
/// brown bottles under mustard caps and one with green bottles under red caps, a third on
/// top full of milk bottles with silver caps, and a loose milk bottle beside the pile.
fn bottle_crates() -> Prop {
    let mut g = VoxelGrid::new(41, 20, 12);
    // One crate, 18 wide, 12 deep and 10 tall, whose left edge is at `x0` and floor at `y0`.
    let mut crate_at = |x0: i32, y0: i32, bottle: u32, cap: u32| {
        let (wood, dark) = (0xA67848, 0x7A5236);
        bx(&mut g, (x0, x0 + 17), (y0, y0 + 1), (0, 11), dark);
        for i in 0..5 {
            for j in 0..3 {
                let (bx0, bz0) = (x0 + 2 + i * 3, 2 + j * 3);
                bx(&mut g, (bx0, bx0 + 1), (y0 + 2, y0 + 8), (bz0, bz0 + 1), bottle);
                bx(&mut g, (bx0, bx0 + 1), (y0 + 9, y0 + 9), (bz0, bz0 + 1), cap);
            }
        }
        for z in [(0, 1), (10, 11)] {
            for y in [(2, 4), (6, 8)] {
                bx(&mut g, (x0, x0 + 17), (y0 + y.0, y0 + y.1), z, wood);
            }
            bx(&mut g, (x0, x0 + 17), (y0 + 9, y0 + 9), z, wood);
        }
        for x in [(x0, x0 + 1), (x0 + 16, x0 + 17)] {
            for y in [(2, 4), (6, 8)] {
                bx(&mut g, x, (y0 + y.0, y0 + y.1), (2, 9), wood);
            }
            bx(&mut g, x, (y0 + 9, y0 + 9), (2, 9), wood);
        }
        for (cx, cz) in [(x0, 0), (x0 + 16, 0), (x0, 10), (x0 + 16, 10)] {
            bx(&mut g, (cx, cx + 1), (y0, y0 + 9), (cz, cz + 1), dark);
        }
    };
    crate_at(0, 0, 0x6A3A1E, MUSTARD);
    crate_at(19, 0, 0x3E8E6A, 0xC84B4B);
    crate_at(9, 10, 0xF0EEE8, 0xB8C2CC);
    bx(&mut g, (38, 39), (0, 7), (4, 5), 0xF0EEE8);
    bx(&mut g, (38, 39), (8, 8), (4, 5), 0xB8C2CC);
    prop("bottle-crates", g)
}

/// Street name sign (28 x 32 x 5): a cream plate with a black border on two short
/// wooden posts, lettered MAPLE over ROAD in a tiny 3 x 5 voxel font.
fn street_sign() -> Prop {
    let mut g = VoxelGrid::new(28, 32, 5);
    for x in [4, 21] {
        bx(&mut g, (x, x + 2), (0, 28), (1, 3), TEAK_DARK);
        bx(&mut g, (x, x + 2), (0, 0), (0, 4), CONCRETE_DARK);
    }
    bx(&mut g, (0, 27), (16, 31), (3, 4), INK);
    bx(&mut g, (1, 26), (17, 30), (3, 4), CREAM);
    for (i, ch) in "MAPLE".chars().enumerate() {
        glyph(&mut g, ch, 4 + 4 * i as i32, 25, 4, INK);
    }
    for (i, ch) in "ROAD".chars().enumerate() {
        glyph(&mut g, ch, 6 + 4 * i as i32, 19, 4, INK);
    }
    prop("street-sign", g)
}

/// Sleeping ginger cat (13 x 8 x 14): a curled loaf with tabby stripes, a cream muzzle
/// and paws, two ears, closed eyes, and the tail wrapped along its side. Faces +z.
fn sleeping_cat() -> Prop {
    let (ginger, stripe, cream) = (ORANGE, 0xB5601E, 0xF4E3C9);
    let mut g = VoxelGrid::new(13, 8, 14);
    ball(&mut g, Vec3::new(5.5, 3.5, 5.0), Vec3::new(4.5, 3.5, 5.0), ginger);
    ball(&mut g, Vec3::new(5.5, 3.2, 10.5), Vec3::new(3.3, 2.8, 2.8), ginger);
    for z in [1, 3, 5, 7] {
        tint(&mut g, (2, 9), (4, 6), (z, z), stripe);
    }
    for x in [(3, 4), (7, 8)] {
        bx(&mut g, x, (6, 7), (9, 10), ginger);
        bx(&mut g, x, (0, 1), (12, 13), cream);
        bx(&mut g, x, (3, 3), (13, 13), INK);
    }
    bx(&mut g, (4, 7), (1, 2), (13, 13), cream);
    bx(&mut g, (5, 6), (2, 2), (13, 13), PINK);
    for z in 0..=10 {
        let edge = 5.5 + 4.5 * (1.0 - ((z as f32 - 5.0) / 5.4).powi(2)).max(0.0).sqrt();
        let x = edge.round() as i32;
        bx(&mut g, (x, x + 1), (0, 1), (z, z), if z >= 9 { stripe } else { ginger });
    }
    prop("sleeping-cat", g)
}

#[derive(Clone, Copy, PartialEq)]
enum Pose {
    Stand,
    Peck,
    Walk,
}

/// A pigeon (6 x 11 x 12) facing +z, grey with a teal-purple neck, dark wing bars, an
/// orange eye and pink legs: standing, pecking at the ground, or mid-stride.
fn pigeon(name: &'static str, pose: Pose) -> Prop {
    let (grey, dark, neck, head) = (0x8E8C92, 0x6A6870, 0x6C8AA0, 0x9A98A2);
    let mut g = VoxelGrid::new(6, 11, 12);
    let pecking = pose == Pose::Peck;
    ball(&mut g, Vec3::new(3.0, 5.5, 5.0), Vec3::new(2.7, 2.6, 3.8), grey);
    bx(&mut g, (1, 4), (if pecking { 6 } else { 4 }, if pecking { 7 } else { 5 }), (0, 3), dark);
    ball(&mut g, Vec3::new(3.0, 5.2, 7.6), Vec3::new(2.3, 2.2, 1.6), 0xA9A7B2);
    for x in [0, 5] {
        tint(&mut g, (x, x), (4, 7), (2, 7), dark);
        tint(&mut g, (x, x), (5, 7), (3, 3), INK);
        tint(&mut g, (x, x), (5, 7), (5, 5), INK);
    }
    let (neck_at, head_at) = match pose {
        Pose::Stand => (Vec3::new(3.0, 8.5, 7.3), Vec3::new(3.0, 9.6, 8.8)),
        Pose::Peck => (Vec3::new(3.0, 6.4, 8.5), Vec3::new(3.0, 3.8, 10.0)),
        Pose::Walk => (Vec3::new(3.0, 8.3, 7.8), Vec3::new(3.0, 9.3, 9.4)),
    };
    ball(&mut g, neck_at, Vec3::new(2.0, 1.8, 1.7), neck);
    ball(&mut g, head_at, Vec3::new(1.7, 1.6, 1.7), head);
    let (hx, hy, hz) = (head_at.x as i32, head_at.y as i32, head_at.z as i32);
    let beak = 0xC9A27E;
    if pecking {
        bx(&mut g, (hx - 1, hx), (hy - 2, hy - 1), (hz + 1, hz + 1), beak);
    } else {
        bx(&mut g, (hx - 1, hx), (hy, hy), (hz + 2, hz + 2), beak);
    }
    for x in [hx - 2, hx + 1] {
        g.set(x, hy + 1, hz, Some(c(0xE07A3F)));
    }
    // Legs and toes; when walking the left leg strides forward.
    let legs = [(1, if pose == Pose::Walk { 7 } else { 5 }), (4, if pose == Pose::Walk { 3 } else { 5 })];
    for (x, z) in legs {
        bx(&mut g, (x, x), (0, 2), (z, z), PINK);
        bx(&mut g, (x, x), (0, 0), (z, z + 2), PINK);
    }
    prop(name, g)
}

/// A flat patch of fallen leaves (16 x 1 x 14): rounded leaf-sized tiles, thickest in the
/// middle, mostly in one colour with a few of its neighbour's.
fn leaf_patch(name: &'static str, colours: [u32; 4], seed: u32) -> Prop {
    let mut g = VoxelGrid::new(16, 1, 14);
    for gz in 0..7 {
        for gx in 0..8 {
            let d = (((gx * 2 + 1) as f32 - 8.0) / 8.0).hypot(((gz * 2 + 1) as f32 - 7.0) / 7.0);
            let h = hash(IVec3::new(gx, gz, 0), seed);
            if (h % 100) as f32 >= (1.45 - d).clamp(0.0, 1.0) * 100.0 {
                continue;
            }
            let (w, l) = [(2, 2), (3, 2), (2, 3)][((h >> 8) % 3) as usize];
            let (ox, oz) = (gx * 2 + ((h >> 12) % 2) as i32, gz * 2 + ((h >> 13) % 2) as i32);
            let pick = match (h >> 16) % 10 {
                0..=3 => 0,
                4..=6 => 1,
                7..=8 => 2,
                _ => 3,
            };
            bx(&mut g, (ox, ox + w - 1), (0, 0), (oz, oz + l - 1), colours[pick]);
        }
    }
    prop(name, g)
}

/// Paper litter (16 x 3 x 14): a puffed mustard crisp packet with a red band, two
/// crumpled newspaper sheets with dark headline lines, a red can on its side and a blue
/// sweet wrapper.
fn paper_litter() -> Prop {
    let mut g = VoxelGrid::new(16, 3, 14);
    bx(&mut g, (2, 6), (0, 1), (3, 6), MUSTARD);
    bx(&mut g, (3, 5), (1, 1), (4, 5), 0xE6BC52);
    bx(&mut g, (2, 6), (0, 0), (4, 5), 0xC84B4B);
    bx(&mut g, (8, 14), (0, 0), (8, 12), PLASTIC);
    bx(&mut g, (7, 11), (0, 0), (6, 9), 0xD2CCBE);
    bx(&mut g, (9, 13), (0, 0), (11, 11), INK);
    bx(&mut g, (9, 12), (0, 0), (9, 9), INK);
    bx(&mut g, (10, 13), (0, 1), (1, 2), 0xC84B4B);
    bx(&mut g, (9, 9), (0, 1), (1, 2), CHROME);
    bx(&mut g, (14, 14), (0, 1), (1, 2), CHROME);
    bx(&mut g, (1, 2), (0, 0), (10, 11), 0x4E86B8);
    prop("paper-litter", g)
}

/// Tall terracotta pot (12 x 34 x 12) for the front steps: a tapering pot with a lipped
/// rim holding a tiered green conifer on a short trunk.
fn plant_pot_tall() -> Prop {
    let terracotta = 0xC0684A;
    let mut g = VoxelGrid::new(12, 34, 12);
    cyl(&mut g, 6.0, 6.0, 3.6, (0, 1), terracotta);
    cyl(&mut g, 6.0, 6.0, 4.5, (2, 4), terracotta);
    cyl(&mut g, 6.0, 6.0, 5.2, (5, 8), terracotta);
    cyl(&mut g, 6.0, 6.0, 5.9, (9, 10), 0xD08A62);
    cyl(&mut g, 6.0, 6.0, 4.6, (10, 10), 0x4A3020);
    bx(&mut g, (5, 6), (11, 14), (5, 6), TEAK_DARK);
    for y in 12..=33 {
        let t = (y - 12) / 5;
        let r = 5.4 - t as f32 * 1.15 - ((y - 12) % 5) as f32 * 0.25;
        cyl(&mut g, 6.0, 6.0, r.max(0.7), (y, y), if t < 2 { PINE } else { LEAF });
    }
    prop("plant-pot-tall", g)
}

/// Round terracotta pot (16 x 22 x 16) for the front steps: a wide pot under a bushy dome
/// of leaves with red geranium flowers.
fn plant_pot_round() -> Prop {
    let terracotta = 0xC0684A;
    let mut g = VoxelGrid::new(16, 22, 16);
    cyl(&mut g, 8.0, 8.0, 4.2, (0, 1), terracotta);
    cyl(&mut g, 8.0, 8.0, 5.4, (2, 4), terracotta);
    cyl(&mut g, 8.0, 8.0, 6.3, (5, 7), terracotta);
    cyl(&mut g, 8.0, 8.0, 7.2, (8, 9), 0xD08A62);
    cyl(&mut g, 8.0, 8.0, 6.0, (9, 9), 0x4A3020);
    ball(&mut g, Vec3::new(8.0, 14.0, 8.0), Vec3::new(7.4, 6.0, 7.4), LEAF);
    tint(&mut g, (0, 15), (10, 12), (0, 15), PINE);
    for (x, y, z) in [(5, 18, 6), (9, 19, 9), (11, 17, 5), (4, 16, 10), (8, 19, 5), (12, 16, 9)] {
        bx(&mut g, (x, x + 1), (y, y + 1), (z, z + 1), 0xD04030);
    }
    prop("plant-pot-round", g)
}
