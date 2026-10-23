//! The animals, in fine voxels (24 to the metre), in the simple style of the voxel packs the
//! user picked: a block head, two dark dot eyes, a small muzzle, the species' own ears and
//! tail. Few features, each one bold: a face this small reads as cute when it is simple and
//! as uncanny when it is detailed.
//!
//! Everyone shares one body plan (legs, torso, arms); the species picks the fur colours,
//! the head and the tail. Parts are separate entities so legs and arms swing from their
//! joints (see `animal::animate_gait`).

use bevy::prelude::*;

use crate::{
    animal::{Animal, Fur, Gait, Limb, Outfit, Species, UpperBody},
    voxel::{FINE, VoxelGrid},
};

/// Hip height in fine voxels.
const HIP: f32 = 9.0;
/// Head size in fine voxels: wide, tall, deep.
const HEAD: IVec3 = IVec3::new(12, 10, 10);

fn c(hex: u32) -> Color {
    Color::srgb_u8((hex >> 16) as u8, (hex >> 8) as u8, hex as u8)
}

const EYE: u32 = 0x1E1614;
/// The patch of the face that expressions repaint: both eyes and both brows.
const FACE_MIN: IVec2 = IVec2::new(1, 3);
const FACE_MAX: IVec2 = IVec2::new(10, 8);

/// A face, made only of eyes and brows (no mouth: a mouth on a face this small is what
/// tipped the earlier look into the uncanny).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Expression {
    #[default]
    Neutral,
    Blink,
    Happy,
    Tired,
    Angry,
    Surprised,
    Puzzled,
    Melancholy,
}

impl Expression {
    pub const ALL: [Expression; 8] = [
        Expression::Neutral,
        Expression::Blink,
        Expression::Happy,
        Expression::Tired,
        Expression::Angry,
        Expression::Surprised,
        Expression::Puzzled,
        Expression::Melancholy,
    ];

    fn index(self) -> usize {
        Self::ALL.iter().position(|e| *e == self).unwrap_or(0)
    }
}

/// What a character's face is doing: a resting face (set by mood), a passing reaction to
/// something that just happened, and blinking.
#[derive(Component)]
pub struct Face {
    pub resting: Expression,
    /// A reaction and the seconds it has left.
    pub reaction: Option<(Expression, f32)>,
    pub blinks: bool,
    next_blink: f32,
    closed: f32,
    shown: Expression,
    part: Entity,
    meshes: Vec<Handle<Mesh>>,
}

impl Face {
    fn new(part: Entity, meshes: Vec<Handle<Mesh>>) -> Self {
        Self { resting: Expression::Neutral, reaction: None, blinks: true, next_blink: 2.0, closed: 0.0, shown: Expression::Neutral, part, meshes }
    }

    /// Show `expression` for `seconds`, then settle back to the resting face.
    pub fn react(&mut self, expression: Expression, seconds: f32) {
        self.reaction = Some((expression, seconds));
    }
}

/// Runs reactions down, blinks now and then, and swaps the face mesh when it changes.
pub fn animate_faces(time: Res<Time>, mut faces: Query<&mut Face>, mut parts: Query<&mut Mesh3d>) {
    let dt = time.delta_secs();
    for mut face in &mut faces {
        if let Some((_, left)) = &mut face.reaction {
            *left -= dt;
            if *left <= 0.0 {
                face.reaction = None;
            }
        }
        face.next_blink -= dt;
        if face.closed > 0.0 {
            face.closed -= dt;
        } else if face.blinks && face.next_blink <= 0.0 {
            face.closed = 0.12;
            // Every 2.5 to 5.5 seconds, varied by when it last blinked.
            face.next_blink = 2.5 + (time.elapsed_secs() * 7.31).fract() * 3.0;
        }
        let current = face.reaction.map(|(e, _)| e).unwrap_or(face.resting);
        // Eyes already closed or narrowed don't blink.
        let blinking = face.closed > 0.0 && !matches!(current, Expression::Happy | Expression::Tired);
        let want = if blinking { Expression::Blink } else { current };
        if want != face.shown {
            face.shown = want;
            if let Ok(mut mesh) = parts.get_mut(face.part) {
                mesh.0 = face.meshes[want.index()].clone();
            }
        }
    }
}

/// The eye-and-brow patch for one expression, painted over the head's own colours in
/// `background`. Each eye is two voxels wide; `o` counts from its outer column toward the
/// nose, so one description draws both eyes mirrored.
fn face(background: &VoxelGrid, front: i32, expression: Expression, eye: Color, brow: Color, lid: Color) -> VoxelGrid {
    let mut g = background.clone();
    for (outer, step) in [(2, 1), (9, -1)] {
        let mut put = |o: i32, y: i32, colour: Color| g.set(outer + o * step, y, front, Some(colour));
        match expression {
            Expression::Neutral => {
                for (o, y) in [(0, 4), (1, 4), (0, 5), (1, 5)] {
                    put(o, y, eye);
                }
            }
            Expression::Blink => {
                put(0, 4, eye);
                put(1, 4, eye);
            }
            // Upturned arcs.
            Expression::Happy => {
                for (o, y) in [(-1, 4), (0, 5), (1, 5), (2, 4)] {
                    put(o, y, eye);
                }
            }
            // Heavy lids over a narrow eye.
            Expression::Tired => {
                put(0, 4, eye);
                put(1, 4, eye);
                put(0, 5, lid);
                put(1, 5, lid);
            }
            // Brows dropping toward the nose.
            Expression::Angry => {
                for (o, y) in [(0, 4), (1, 4), (0, 5), (1, 5)] {
                    put(o, y, eye);
                }
                for (o, y) in [(-1, 7), (0, 7), (1, 6), (2, 6)] {
                    put(o, y, brow);
                }
            }
            // Tall eyes, brows raised high.
            Expression::Surprised => {
                for (o, y) in [(0, 4), (1, 4), (0, 5), (1, 5), (0, 6), (1, 6)] {
                    put(o, y, eye);
                }
                for o in -1..=1 {
                    put(o, 8, brow);
                }
            }
            // Melancholy: eyes cast down, brows lifted at the nose.
            Expression::Melancholy => {
                for (o, y) in [(0, 3), (1, 3), (0, 4), (1, 4)] {
                    put(o, y, eye);
                }
                for (o, y) in [(-1, 6), (0, 6), (1, 7), (2, 7)] {
                    put(o, y, brow);
                }
            }
            Expression::Puzzled => {}
        }
    }
    // Puzzled is lopsided: one brow arched high, the other low and flat, one eye narrowed.
    if expression == Expression::Puzzled {
        for (x, y) in [(2, 4), (3, 4), (2, 5), (3, 5), (8, 4), (9, 4)] {
            g.set(x, y, front, Some(eye));
        }
        for (x, y) in [(1, 7), (2, 8), (3, 8), (8, 6), (9, 6), (10, 6)] {
            g.set(x, y, front, Some(brow));
        }
    }
    g
}

/// Spawns an animal standing at `at`, facing `facing`. Returns the root entity.
pub fn spawn_animal(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    material: &Handle<StandardMaterial>,
    species: Species,
    outfit: &Outfit,
    at: Vec3,
    facing: Vec3,
) -> Entity {
    let fur = species.fur();
    let mut part = |grid: VoxelGrid, origin: Vec3| (Mesh3d(meshes.add(grid.mesh_moving(origin, FINE))), MeshMaterial3d(material.clone()));
    let v = FINE;
    let shirt_dark = outfit.shirt.mix(&Color::BLACK, 0.18);
    // Collar, cuffs and the sneaker stripe: a ringer tee's trim, red on a light shirt.
    let s = outfit.shirt.to_srgba();
    let trim = if s.red + s.green + s.blue > 2.1 { c(0xA8343A) } else { outfit.shirt.mix(&Color::WHITE, 0.35) };

    // Legs: 4 x 9 x 5. Trousers, then shoes: a sole, a striped upper, a toe that pokes
    // forward one voxel.
    let mut leg = VoxelGrid::new(4, 9, 5);
    leg.fill(IVec3::new(0, 2, 1), IVec3::new(3, 8, 4), outfit.pants);
    leg.fill(IVec3::new(0, 2, 1), IVec3::new(3, 2, 4), outfit.pants.mix(&Color::BLACK, 0.15));
    leg.fill(IVec3::new(0, 0, 0), IVec3::new(3, 0, 4), outfit.shoes.mix(&Color::BLACK, 0.35));
    leg.fill(IVec3::new(0, 1, 0), IVec3::new(3, 1, 4), outfit.shoes);
    leg.fill(IVec3::new(0, 1, 2), IVec3::new(0, 1, 3), trim);
    leg.fill(IVec3::new(3, 1, 2), IVec3::new(3, 1, 3), trim);

    // Torso: 10 x 11 x 6. Belt with a buckle, the shirt with a collar and a fur V.
    let mut torso = VoxelGrid::new(10, 11, 6);
    torso.fill(IVec3::ZERO, IVec3::new(9, 10, 5), outfit.shirt);
    torso.fill(IVec3::new(0, 0, 0), IVec3::new(9, 0, 5), outfit.pants);
    torso.fill(IVec3::new(0, 1, 0), IVec3::new(9, 1, 5), c(0x4A3228));
    torso.fill(IVec3::new(4, 1, 0), IVec3::new(5, 1, 0), c(0xD8B04A));
    torso.fill(IVec3::new(0, 10, 0), IVec3::new(9, 10, 5), trim);
    torso.fill(IVec3::new(4, 8, 0), IVec3::new(5, 10, 0), fur.light);
    torso.set(3, 10, 0, Some(fur.light));
    torso.set(6, 10, 0, Some(fur.light));
    // A seam down each side gives the shirt a front and a back.
    torso.fill(IVec3::new(0, 2, 2), IVec3::new(0, 9, 2), shirt_dark);
    torso.fill(IVec3::new(9, 2, 2), IVec3::new(9, 9, 2), shirt_dark);

    // Arms: 3 x 10 x 3. Sleeve with a cuff, fur forearm, darker paw.
    let mut arm = VoxelGrid::new(3, 10, 3);
    arm.fill(IVec3::new(0, 5, 0), IVec3::new(2, 9, 2), outfit.shirt);
    arm.fill(IVec3::new(0, 5, 0), IVec3::new(2, 5, 2), trim);
    arm.fill(IVec3::new(0, 2, 0), IVec3::new(2, 4, 2), fur.main);
    arm.fill(IVec3::new(0, 0, 0), IVec3::new(2, 1, 2), fur.paw);

    let (head, head_origin, faces) = head(species, &fur);
    let (tail, tail_origin, tail_at) = tail(species, &fur);

    let root = commands
        .spawn((Transform::from_translation(at).looking_to(facing, Vec3::Y), Visibility::default(), Animal, Gait::default()))
        .id();
    let limb = |sign| Limb { owner: root, sign };
    let left_leg = commands.spawn((part(leg.clone(), Vec3::new(2.0, 9.0, 2.5)), Transform::from_xyz(-2.5 * v, HIP * v, 0.0), limb(1.0))).id();
    let right_leg = commands.spawn((part(leg, Vec3::new(2.0, 9.0, 2.5)), Transform::from_xyz(2.5 * v, HIP * v, 0.0), limb(-1.0))).id();
    let upper = commands
        .spawn((Transform::from_xyz(0.0, HIP * v, 0.0), Visibility::default(), UpperBody { owner: root, base: HIP * v, bob: v }))
        .id();
    let torso = commands.spawn((part(torso, Vec3::new(5.0, 0.0, 3.0)), Transform::default())).id();
    let left_arm = commands.spawn((part(arm.clone(), Vec3::new(1.5, 10.0, 1.5)), Transform::from_xyz(-6.5 * v, 10.0 * v, 0.0), limb(-1.0))).id();
    let right_arm = commands.spawn((part(arm, Vec3::new(1.5, 10.0, 1.5)), Transform::from_xyz(6.5 * v, 10.0 * v, 0.0), limb(1.0))).id();
    let head = commands.spawn((part(head, head_origin), Transform::from_xyz(0.0, 11.0 * v, 0.0))).id();
    let tail = commands.spawn((part(tail, tail_origin), Transform::from_translation(tail_at * v))).id();

    let faces: Vec<Handle<Mesh>> = faces.iter().map(|f| meshes.add(f.mesh_sized(head_origin, FINE))).collect();
    let face = commands.spawn((Mesh3d(faces[0].clone()), MeshMaterial3d(material.clone()), Transform::default())).id();
    commands.entity(head).add_child(face);
    commands.entity(root).insert(Face::new(face, faces));

    commands.entity(upper).add_children(&[torso, left_arm, right_arm, head, tail]);
    commands.entity(root).add_children(&[left_leg, right_leg, upper]);
    root
}

/// The shared head block: softened back corners and a light chin. The face is at grid
/// z = `front`; anything in front of it (a muzzle) uses z < `front`. `extra_height` leaves
/// room above the head for ears.
fn head_block(fur: &Fur, front: i32, extra_height: i32) -> VoxelGrid {
    let IVec3 { x: w, y: h, z: d } = HEAD;
    let mut g = VoxelGrid::new(w as u32, (h + extra_height) as u32, (d + front) as u32);
    g.fill_with(IVec3::new(0, 0, front), IVec3::new(w - 1, h - 1, front + d - 1), |p| {
        // Soften the top and back edges by trimming voxels that sit on two outer faces.
        let on_x = p.x == 0 || p.x == w - 1;
        let on_top = p.y == h - 1;
        let on_back = p.z == front + d - 1;
        let on_bottom = p.y == 0;
        let trimmed = (on_x && on_top) || (on_x && on_back) || (on_top && on_back) || (on_bottom && on_back);
        (!trimmed).then_some(fur.main)
    });
    // A light chin across the bottom of the face, wrapping a little round the sides.
    g.fill(IVec3::new(0, 0, front), IVec3::new(w - 1, 2, front + 2), fur.light);
    g
}

/// Pointed ears leaning outward, two voxels deep: rows of (height above the head, width),
/// `inner` inside below the tip, `tip` on the top `tip_rows` rows.
fn pointed_ears(g: &mut VoxelGrid, front: i32, rows: &[(i32, i32)], outer: Color, inner: Color, tip: Color, tip_rows: i32) {
    let IVec3 { x: w, y: h, .. } = HEAD;
    let top = rows.iter().map(|r| r.0).max().unwrap_or(0);
    for (x0, left) in [(0, true), (w - 4, false)] {
        for &(dy, width) in rows {
            for i in 0..width {
                let x = if left { x0 + i } else { x0 + 3 - i };
                for z in [front + 3, front + 4] {
                    let is_tip = dy > top - tip_rows;
                    let is_inner = z == front + 3 && i > 0 && i < width - 1 && !is_tip;
                    let col = if is_tip { tip } else if is_inner { inner } else { outer };
                    g.set(x, h + dy, z, Some(col));
                }
            }
        }
    }
}

/// Each species' head (without its eyes and brows), the grid point that sits on the neck,
/// and the face layer for every expression (in `Expression::ALL` order).
fn head(species: Species, fur: &Fur) -> (VoxelGrid, Vec3, Vec<VoxelGrid>) {
    let IVec3 { x: w, y: h, z: d } = HEAD;
    // The bear's muzzle sticks out two voxels; everyone else's one.
    let front = if species == Species::Bear { 2 } else { 1 };
    let mut g = match species {
        Species::Fox => {
            let mut g = head_block(fur, front, 6);
            // A cream muzzle one voxel proud of the face, a dark nose on top.
            g.fill(IVec3::new(4, 1, 0), IVec3::new(7, 3, 0), fur.light);
            g.fill(IVec3::new(5, 3, 0), IVec3::new(6, 3, 0), fur.nose);
            // Tall pointed ears with dark tips.
            pointed_ears(&mut g, front, &[(0, 4), (1, 4), (2, 3), (3, 3), (4, 2), (5, 1)], fur.main, fur.light, fur.dark, 2);
            g
        }
        Species::Rabbit => {
            let mut g = head_block(fur, front, 13);
            // A small light muzzle, a pink nose.
            g.fill(IVec3::new(5, 1, 0), IVec3::new(6, 2, 0), fur.light);
            g.fill(IVec3::new(5, 3, 0), IVec3::new(6, 3, 0), fur.nose);
            // Long upright ears, three wide with pink insides, rounded at the top.
            for x0 in [2, 7] {
                g.fill_with(IVec3::new(x0, h, front + 4), IVec3::new(x0 + 2, h + 12, front + 5), |p| {
                    let corner = p.y == h + 12 && p.x != x0 + 1;
                    let inner = p.z == front + 4 && p.x == x0 + 1 && p.y < h + 11;
                    (!corner).then_some(if inner { fur.accent } else { fur.main })
                });
            }
            g
        }
        Species::Raccoon => {
            let mut g = head_block(fur, front, 4);
            // The mask: a dark band across the eyes with a grey bridge, a light brow above.
            g.fill(IVec3::new(0, 3, front), IVec3::new(w - 1, 6, front + 1), fur.accent);
            g.fill(IVec3::new(5, 3, front), IVec3::new(6, 6, front), fur.main);
            g.fill(IVec3::new(1, 7, front), IVec3::new(4, 7, front), fur.light);
            g.fill(IVec3::new(7, 7, front), IVec3::new(10, 7, front), fur.light);
            // A light muzzle with a black nose.
            g.fill(IVec3::new(4, 0, 0), IVec3::new(7, 2, 0), fur.light);
            g.fill(IVec3::new(5, 2, 0), IVec3::new(6, 2, 0), fur.nose);
            // Small ears: light rims, dark middles.
            pointed_ears(&mut g, front, &[(0, 4), (1, 3), (2, 2), (3, 1)], fur.light, fur.dark, fur.light, 1);
            g
        }
        Species::Bear => {
            let mut g = head_block(fur, front, 3);
            // A broad light muzzle two voxels proud, a big dark nose on top.
            g.fill(IVec3::new(3, 0, 0), IVec3::new(8, 3, 1), fur.light);
            g.fill(IVec3::new(5, 2, 0), IVec3::new(6, 3, 0), fur.nose);
            // Round ears on the top corners, dark inside.
            for x0 in [0, w - 4] {
                g.fill(IVec3::new(x0, h, front + 3), IVec3::new(x0 + 3, h + 2, front + 4), fur.main);
                g.clear(IVec3::new(x0, h + 2, front + 3), IVec3::new(x0, h + 2, front + 4));
                g.clear(IVec3::new(x0 + 3, h + 2, front + 3), IVec3::new(x0 + 3, h + 2, front + 4));
                g.fill(IVec3::new(x0 + 1, h, front + 3), IVec3::new(x0 + 2, h + 1, front + 3), fur.dark);
            }
            g
        }
    };
    // Lift the eye-and-brow patch out of the head: each expression repaints all of it, so
    // swapping expressions is swapping one small mesh.
    let mut background = VoxelGrid::new(g.size.x, g.size.y, g.size.z);
    for y in FACE_MIN.y..=FACE_MAX.y {
        for x in FACE_MIN.x..=FACE_MAX.x {
            background.set(x, y, front, g.get(x, y, front));
            g.set(x, y, front, None);
        }
    }
    let eye = if species == Species::Raccoon { c(0x0A0808) } else { c(EYE) };
    let lid = fur.main.mix(&Color::BLACK, 0.25);
    let faces = Expression::ALL.iter().map(|e| face(&background, front, *e, eye, fur.dark, lid)).collect();
    (g, Vec3::new(w as f32 / 2.0, 0.0, front as f32 + d as f32 / 2.0), faces)
}

/// Each species' tail: the grid, the grid point at the attachment, and where that sits on
/// the upper body (in fine voxels).
fn tail(species: Species, fur: &Fur) -> (VoxelGrid, Vec3, Vec3) {
    let at = Vec3::new(0.0, 2.0, 3.0);
    match species {
        Species::Fox => {
            // Bushy and curving up, with a cream tip.
            let (w, h, l) = (7, 11, 14);
            let mut g = VoxelGrid::new(w as u32, h as u32, l as u32);
            g.fill_with(IVec3::ZERO, IVec3::new(w - 1, h - 1, l - 1), |p| {
                let t = p.z as f32 / (l - 1) as f32;
                let centre = Vec2::new(3.0, 2.0 + t * t * 6.0);
                let radius = 1.6 + (t * std::f32::consts::PI).sin() * 2.0;
                let q = Vec2::new(p.x as f32, p.y as f32);
                ((q - centre).length() <= radius).then(|| if p.z >= l - 4 { fur.light } else { fur.main })
            });
            (g, Vec3::new(3.5, 0.0, 0.0), at)
        }
        Species::Rabbit => {
            // A round cotton tail.
            let mut g = VoxelGrid::new(5, 5, 4);
            g.fill_with(IVec3::ZERO, IVec3::new(4, 4, 3), |p| {
                let q = p.as_vec3() - Vec3::new(2.0, 2.0, 1.5);
                (q.length() <= 2.4).then_some(fur.light)
            });
            (g, Vec3::new(2.5, 0.0, 0.0), at)
        }
        Species::Raccoon => {
            // Long and bushy with dark rings.
            let (w, h, l) = (5, 6, 14);
            let mut g = VoxelGrid::new(w as u32, h as u32, l as u32);
            g.fill_with(IVec3::ZERO, IVec3::new(w - 1, h - 1, l - 1), |p| {
                let t = p.z as f32 / (l - 1) as f32;
                let centre = Vec2::new(2.0, 1.5 + t * 2.5);
                let radius = 1.6 + (t * std::f32::consts::PI).sin() * 0.9;
                let q = Vec2::new(p.x as f32, p.y as f32);
                ((q - centre).length() <= radius).then(|| if (p.z / 3) % 2 == 1 { fur.dark } else { fur.main })
            });
            (g, Vec3::new(2.5, 0.0, 0.0), at)
        }
        Species::Bear => {
            // A short round stub.
            let mut g = VoxelGrid::new(4, 4, 2);
            g.fill(IVec3::ZERO, IVec3::new(3, 3, 1), fur.main);
            (g, Vec3::new(2.0, 0.0, 0.0), at)
        }
    }
}

/// `--gallery`: the four species in a row at the front of the gallery slab, for checking
/// them side by side.
pub fn gallery_lineup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, material: Res<crate::world::VoxelMaterial>) {
    let outfit = Outfit { shirt: c(0x5F8FB8), pants: c(0x2E3F5A), shoes: c(0xEEEAE0) };
    for (i, species) in Species::ALL.into_iter().enumerate() {
        let at = crate::gallery::GALLERY_ORIGIN + Vec3::new(1.0 + i as f32 * 1.2, 0.0, 8.2);
        let animal = spawn_animal(&mut commands, &mut meshes, &material.0, species, &outfit, at, Vec3::new(0.4, 0.0, 1.0));
        commands.entity(animal).insert(DespawnOnExit(crate::Screen::Game));
    }
    // Every expression, on a row of foxes facing the camera.
    for (i, expression) in Expression::ALL.into_iter().enumerate() {
        let at = crate::gallery::GALLERY_ORIGIN + Vec3::new(6.0 + i as f32 * 1.1, 0.0, 8.2);
        let fox = spawn_animal(&mut commands, &mut meshes, &material.0, Species::Fox, &outfit, at, Vec3::new(1.0, 0.0, 1.0));
        commands.entity(fox).insert((GalleryFace(expression), DespawnOnExit(crate::Screen::Game)));
    }
}

/// `--gallery`: holds a fox's face on one expression, without blinking.
#[derive(Component)]
pub struct GalleryFace(Expression);

pub fn hold_gallery_faces(mut faces: Query<(&GalleryFace, &mut Face)>) {
    for (hold, mut face) in &mut faces {
        face.resting = hold.0;
        face.blinks = false;
    }
}
