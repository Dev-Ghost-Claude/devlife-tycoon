//! The world camera: an isometric orthographic view, rendered at full resolution with soft
//! light (filmic tone curve, glow, soft shadows, smoothed edges).
//!
//! The world is rendered into an image the size of the window and shown on screen pixel
//! for pixel, rather than straight to the window: screenshots of a covered window come out
//! black on macOS, and the capture scripts need reliable shots.

use bevy::{
    anti_alias::smaa::Smaa,
    camera::{Hdr, RenderTarget, ScalingMode},
    core_pipeline::tonemapping::Tonemapping,
    light::{CascadeShadowConfig, CascadeShadowConfigBuilder, DirectionalLightShadowMap, PointLightShadowMap, ShadowFilteringMethod},
    post_process::bloom::Bloom,
    prelude::*,
    render::{
        Render, RenderApp, RenderSystems,
        mesh::allocator::MeshAllocatorSettings,
        render_resource::{Extent3d, PollType, TextureFormat},
        renderer::RenderDevice,
        slab_allocator::SlabAllocatorSettings,
    },
};

use crate::{
    Screen,
    outline::{Outline, outlined_camera},
    voxel::FINE,
};

/// The isometric view. With the true isometric pitch (35.26 degrees) and the screen's
/// vertical axis squashed by cos 30 degrees, voxel edges along the ground run at 2:1.
const PITCH: f32 = 0.615_479_7; // atan(1 / sqrt(2))
const VERTICAL: f32 = 0.866_025_4; // cos 30 degrees
/// Metres across the view at zoom 1.
const VIEW_WIDTH: f32 = 642.0 * std::f32::consts::FRAC_1_SQRT_2 / 24.0;
/// The capture scripts give click positions on a frame this many pixels across, whatever
/// the window size.
const SCRIPT_FRAME: Vec2 = Vec2::new(642.0, 362.0);
/// How fast zoom animations settle.
const ZOOM_EASE: f32 = 6.0;
/// The camera sees this far in front of and behind its position, in metres.
const DEPTH: f32 = 200.0;
/// Anything that rises at least this much above what is behind it gets an outline.
const OUTLINE_RISE: f32 = 1.5 * FINE;
/// How far back from its focus the camera sits, in metres.
const CAMERA_BACK: f32 = 40.0;
/// The tallest things in the town (the flats' roof), in metres: near the bottom of the view
/// they can stand much closer to the camera than the ground there.
const TALLEST: f32 = 12.0;
/// How far below zero the lowest ground goes (the road, a kerb down), in metres.
const LOWEST: f32 = 0.125;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // Never let the GPU fall behind: each frame waits for the previous one to finish on
        // the GPU. Normally the window's refresh paces frames, but a hidden or covered window
        // doesn't, and on 2026-10-13 an unpaced run queued up frames until it had used 4 GB
        // and hung the GPU, freezing the whole Mac.
        if let Some(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app.add_systems(Render, wait_for_gpu.in_set(RenderSystems::Cleanup));
            // Mesh buffers on the GPU grow 10% at a time rather than 50%, so less sits unused
            // (the scene is built once, so the extra copying while growing doesn't matter).
            render_app.insert_resource(MeshAllocatorSettings {
                slab_allocator_settings: SlabAllocatorSettings { growth_factor: 1.1, ..default() },
                ..default()
            });
        }
        // Only the shop lamp casts point-light shadows, and Bevy keeps one cube of them even
        // when none does: 1024 is 25 MB, 2048 was 100 MB.
        // The sun's map is fitted to the view each frame (`place_camera`), so 2048 is as sharp
        // in the bedroom as 4096 was over a fixed 90 m. Spot lights take a layer each at the
        // same size: in the bedroom 3 x 17 MB instead of 3 x 67 MB.
        app.insert_resource(DirectionalLightShadowMap { size: 2048 })
            .insert_resource(PointLightShadowMap { size: 1024 })
            .add_systems(OnEnter(Screen::Game), spawn_cameras)
            .add_systems(
                PostUpdate,
                (animate_zoom, fit_canvas, place_camera).chain().run_if(in_state(Screen::Game)).before(TransformSystems::Propagate),
            );
    }
}

/// `--no-outline`: render without the outline pass, for comparing the two looks.
#[derive(Resource)]
pub struct NoOutline;

/// Where the camera looks and from which side. Other systems steer it by changing these.
#[derive(Resource)]
pub struct CameraRig {
    /// Ground point at the centre of the view.
    pub focus: Vec3,
    /// Turn around the vertical axis, in radians. 45 degrees looks from the south-east.
    pub yaw: f32,
    /// Zoom to settle at. 1 is the normal view; larger shows more of the world.
    pub zoom: f32,
    /// The zoom the player sees right now; eases toward `zoom`. Set it directly to start
    /// an animation (e.g. lower than `zoom` to pull back from close in).
    pub apparent: f32,
}

impl CameraRig {
    pub fn new(focus: Vec3, yaw: f32, zoom: f32) -> Self {
        Self { focus, yaw, zoom, apparent: zoom }
    }
}

#[derive(Component)]
pub struct WorldCamera;

/// The image the world is rendered into.
#[derive(Resource)]
pub struct CanvasImage(pub Handle<Image>);

#[derive(Component)]
struct Canvas;

fn spawn_cameras(mut commands: Commands, mut images: ResMut<Assets<Image>>, window: Single<&Window>, no_outline: Option<Res<NoOutline>>) {
    let size = window.physical_size().max(UVec2::ONE);
    let mut canvas = Image::new_target_texture(size.x, size.y, TextureFormat::Bgra8UnormSrgb, None);
    // Only the GPU draws into it, so it needs no blank copy in main memory.
    canvas.data = None;
    let canvas = images.add(canvas);
    commands.insert_resource(CanvasImage(canvas.clone()));

    let mut camera = commands.spawn((
        Camera3d::default(),
        Camera { order: -1, clear_color: ClearColorConfig::Custom(Color::srgb_u8(0x9f, 0xc7, 0xd8)), ..default() },
        RenderTarget::Image(canvas.clone().into()),
        Projection::from(OrthographicProjection { near: -DEPTH, far: DEPTH, ..OrthographicProjection::default_3d() }),
        Hdr,
        // A filmic curve: bright lamps and windows roll off softly instead of clipping.
        Tonemapping::TonyMcMapface,
        Bloom { intensity: 0.12, ..Bloom::NATURAL },
        // Smoothed edges. (Multisampling would block the outline pass, which reads depth.)
        Msaa::Off,
        Smaa::default(),
        ShadowFilteringMethod::Gaussian,
        WorldCamera,
        DespawnOnExit(Screen::Game),
    ));
    if no_outline.is_none() {
        camera.insert(outlined_camera(2.0 * DEPTH));
    }
    commands.spawn((Sprite::from_image(canvas), Canvas, DespawnOnExit(Screen::Game)));
}

/// Blocks the render thread until the GPU has finished this frame (at most two seconds).
/// The memory guard quits the game if the GPU stays stuck.
fn wait_for_gpu(device: Res<RenderDevice>) {
    let finished = device.poll(PollType::Wait { submission_index: None, timeout: Some(std::time::Duration::from_secs(2)) }).is_ok();
    crate::memguard::gpu_waited(finished);
}

fn animate_zoom(time: Res<Time>, mut rig: ResMut<CameraRig>) {
    let ease = 1.0 - (-ZOOM_EASE * time.delta_secs()).exp();
    rig.apparent += (rig.zoom - rig.apparent) * ease;
    if (rig.apparent - rig.zoom).abs() < 0.002 {
        rig.apparent = rig.zoom;
    }
}

/// Keeps the canvas the size of the window, drawn pixel for pixel.
fn fit_canvas(window: Single<&Window>, canvas: Res<CanvasImage>, mut images: ResMut<Assets<Image>>, sprite: Single<&mut Sprite, With<Canvas>>) {
    let size = window.physical_size().max(UVec2::ONE);
    if images.get(&canvas.0).is_some_and(|image| image.size() != size)
        && let Some(mut image) = images.get_mut(&canvas.0)
    {
        image.resize(Extent3d { width: size.x, height: size.y, depth_or_array_layers: 1 });
        // Every screen-sized texture changes size with it: a big jump in memory, but not a runaway.
        crate::memguard::rebase();
    }
    let logical = Vec2::new(window.width(), window.height());
    let mut sprite = sprite.into_inner();
    if sprite.custom_size != Some(logical) {
        sprite.custom_size = Some(logical);
    }
}

fn place_camera(
    rig: Res<CameraRig>,
    window: Single<&Window>,
    camera: Single<(&mut Transform, &mut Projection, Option<&mut Outline>), With<WorldCamera>>,
    mut sun: Query<&mut CascadeShadowConfig, With<DirectionalLight>>,
) {
    let (mut transform, mut projection, outline) = camera.into_inner();
    let rotation = Quat::from_euler(EulerRot::YXZ, rig.yaw, -PITCH, 0.0);
    let size = window.physical_size().max(UVec2::ONE).as_vec2();
    let width = VIEW_WIDTH * rig.apparent;
    let height = width * size.y / size.x / VERTICAL;
    if let Projection::Orthographic(ortho) = &mut *projection {
        ortho.scaling_mode = ScalingMode::Fixed { width, height };
    }
    if let Some(mut outline) = outline {
        outline.fit(rotation, width / size.x, height / size.y, PITCH, OUTLINE_RISE);
    }
    transform.translation = rig.focus + rotation * Vec3::Z * CAMERA_BACK;
    transform.rotation = rotation;

    // The sun's shadow map covers only the depth the view can show. The thinner the slice
    // of world it covers, the sharper the shadows from the same size of map. Seen through a
    // point of the screen, something at height y is (focus height - y) / sin(pitch) deeper
    // than the focus; a point higher up the screen is cot(pitch) deeper per metre. So the
    // farthest thing in view is the street at the top edge, the nearest the tallest roof at
    // the bottom edge.
    let up_screen = height / 2.0 / PITCH.tan();
    let near = (CAMERA_BACK - up_screen - (TALLEST - rig.focus.y).max(0.0) / PITCH.sin()).max(0.1);
    let far = CAMERA_BACK + up_screen + (rig.focus.y + LOWEST) / PITCH.sin() + 1.0;
    for mut config in &mut sun {
        if config.bounds.first().is_none_or(|&f| (f - far).abs() > 0.05) || (config.minimum_distance - near).abs() > 0.05 {
            *config = CascadeShadowConfigBuilder { num_cascades: 1, minimum_distance: near, maximum_distance: far, first_cascade_far_bound: far, ..default() }.build();
        }
    }
}

/// Cursor picking: rays from the mouse (or from a script position) into the world.
#[derive(bevy::ecs::system::SystemParam)]
pub struct CursorPicker<'w, 's> {
    window: Single<'w, 's, &'static Window>,
    camera: Single<'w, 's, (&'static Camera, &'static GlobalTransform), With<WorldCamera>>,
}

impl CursorPicker<'_, '_> {
    /// The ray under the mouse cursor, if the cursor is over the window.
    pub fn ray(&self) -> Option<Ray3d> {
        let cursor = self.window.cursor_position()?;
        let (camera, transform) = *self.camera;
        // The canvas fills the window pixel for pixel; its pixels are physical ones.
        camera.viewport_to_world(transform, cursor * self.window.scale_factor()).ok()
    }

    /// The ray through a point given on the capture scripts' 642 x 362 frame.
    pub fn ray_at_canvas(&self, px: Vec2) -> Option<Ray3d> {
        let (camera, transform) = *self.camera;
        let size = camera.logical_viewport_size()?;
        camera.viewport_to_world(transform, px / SCRIPT_FRAME * size).ok()
    }
}

/// Where a ray meets a floor at height `y`.
pub fn ground_point(ray: Ray3d, y: f32) -> Option<Vec3> {
    let distance = ray.intersect_plane(Vec3::Y * y, InfinitePlane3d::new(Vec3::Y))?;
    Some(ray.get_point(distance))
}
