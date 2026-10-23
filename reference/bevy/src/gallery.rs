//! `--gallery`: every street model from `street_props::catalogue()` laid out on a plain
//! slab, far from the street, with the camera on it. For checking models without building
//! a street around them: `cargo run -- --gallery --capture target/gallery`.

use bevy::prelude::*;

use crate::{
    Screen,
    street_props::catalogue,
    voxel::{FINE, VoxelGrid},
    world::{LampGlow, VoxelMaterial},
};

/// Where the gallery is built: far east of everything else.
pub const GALLERY_ORIGIN: Vec3 = Vec3::new(5000.0, 0.0, 0.0);
/// The slab's size in fine voxels (16 m x 12.5 m); its centre is where the camera looks.
const SLAB: IVec2 = IVec2::new(384, 300);
/// Rows stay out of this band of z: the animal line-up stands in it.
const KEEP_CLEAR: IVec2 = IVec2::new(186, 222);
/// Space between models, in fine voxels.
const GAP: i32 = 10;

#[derive(Resource)]
pub struct GalleryMode;

pub struct GalleryPlugin;

impl Plugin for GalleryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Game), build_gallery.after(crate::world::build_areas).run_if(resource_exists::<GalleryMode>));
    }
}

/// The ground point the camera frames in gallery mode.
pub fn gallery_centre() -> Vec3 {
    GALLERY_ORIGIN + Vec3::new(SLAB.x as f32, 0.0, SLAB.y as f32) * FINE / 2.0
}

fn build_gallery(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, material: Res<VoxelMaterial>, glow: Res<LampGlow>) {
    let mut slab = VoxelGrid::new(SLAB.x as u32, 4, SLAB.y as u32);
    slab.fill_with(IVec3::ZERO, IVec3::new(SLAB.x - 1, 3, SLAB.y - 1), |p| {
        let check = (p.x / 24 + p.z / 24) % 2 == 0;
        Some(if p.y < 3 { Color::srgb_u8(0x6A, 0x62, 0x5C) } else if check { Color::srgb_u8(0xCF, 0xC8, 0xBC) } else { Color::srgb_u8(0xC4, 0xBD, 0xB0) })
    });
    let mut spawn = |grid: &VoxelGrid, at: Vec3, material: &Handle<StandardMaterial>| {
        commands.spawn((Mesh3d(meshes.add(grid.mesh_sized(Vec3::ZERO, FINE))), MeshMaterial3d(material.clone()), Transform::from_translation(at), DespawnOnExit(Screen::Game)));
    };
    spawn(&slab, GALLERY_ORIGIN - Vec3::Y * 4.0 * FINE, &material.0);

    // `GALLERY_ONLY=lamp,bench` shows only the models whose names contain one of the words.
    let only: Vec<String> = std::env::var("GALLERY_ONLY").map(|v| v.split(',').map(str::to_owned).collect()).unwrap_or_default();

    // Lay the models out in rows, west to east, with a gap between them. The first row
    // starts a little in from the corner, which the camera's frame would clip.
    let (mut x, mut z, mut row_depth) = (24, 8, 0);
    for prop in catalogue().into_iter().filter(|p| only.is_empty() || only.iter().any(|word| p.name.contains(word.as_str()))) {
        let size = prop.grid.size.as_ivec3();
        if x + size.x > SLAB.x - 8 {
            x = 8;
            z += row_depth + GAP;
            row_depth = 0;
        }
        if z < KEEP_CLEAR.y && z + size.z > KEEP_CLEAR.x {
            (x, z, row_depth) = (8, KEEP_CLEAR.y, 0);
        }
        let at = GALLERY_ORIGIN + IVec3::new(x, 0, z).as_vec3() * FINE;
        info!("gallery: {} at ({x}, {z}), {}x{}x{}", prop.name, size.x, size.y, size.z);
        spawn(&prop.grid, at, &material.0);
        if let Some(lit) = &prop.glow {
            spawn(lit, at, &glow.0);
        }
        x += size.x + GAP;
        row_depth = row_depth.max(size.z);
    }
}
