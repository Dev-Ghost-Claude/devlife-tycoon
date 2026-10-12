//! Renders the 3D world into a small canvas and shows it scaled up with sharp pixels.
//!
//! The camera snaps to whole canvas pixels so the world never shimmers as it moves; the
//! leftover fraction moves the canvas on screen instead, so motion still looks smooth.
//!
//! Zoom works the same way. The world is only ever rendered at a zoom where a voxel is a
//! whole number of pixels; zoom animations scale the finished picture on screen instead,
//! so nothing jitters while the view pulls in or out.

use bevy::{
    camera::{RenderTarget, ScalingMode},
    core_pipeline::tonemapping::Tonemapping,
    image::ImageSampler,
    prelude::*,
    render::render_resource::TextureFormat,
};

use crate::Screen;

/// Visible canvas size in pixels; the art is drawn for this.
pub const CANVAS: UVec2 = UVec2::new(640, 360);
/// One spare pixel on every side, so the sub-pixel shift never shows an empty edge.
const MARGIN: u32 = 1;
/// Pixels per metre at zoom 1: a voxel (1/8 m) is 3 pixels.
const PIXELS_PER_METRE: f32 = 24.0;
/// Camera pitch below the horizon. 30 degrees gives the classic isometric look.
const PITCH: f32 = 30.0_f32.to_radians();
/// How fast zoom animations settle.
const ZOOM_EASE: f32 = 6.0;

pub struct PixelCameraPlugin;

impl Plugin for PixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Game), spawn_cameras).add_systems(
            PostUpdate,
            (animate_zoom, place_camera, fit_canvas).chain().run_if(in_state(Screen::Game)).before(TransformSystems::Propagate),
        );
    }
}

/// Where the camera looks and from which side. Other systems steer it by changing these.
#[derive(Resource)]
pub struct CameraRig {
    /// Ground point at the centre of the view.
    pub focus: Vec3,
    /// Turn around the vertical axis, in radians. 45 degrees looks from the south-east.
    pub yaw: f32,
    /// Zoom to settle at. 1 is the normal view; larger shows more of the world. Use values
    /// where a voxel is a whole number of pixels: 0.75 (4 px), 1.0 (3 px), 1.5 (2 px).
    pub zoom: f32,
    /// The zoom the player sees right now; eases toward `zoom`. Set it directly to start
    /// an animation (e.g. lower than `zoom` to pull back from close in).
    pub apparent: f32,
    /// The last zoom the view settled at.
    settled: f32,
}

impl CameraRig {
    pub fn new(focus: Vec3, yaw: f32, zoom: f32) -> Self {
        Self { focus, yaw, zoom, apparent: zoom, settled: zoom }
    }

    /// The zoom actually rendered: always a settled, whole-pixel zoom.
    fn render_zoom(&self) -> f32 {
        // Pulling back renders the wider final view and shrinks the scale-up toward it;
        // pushing in keeps the old view and enlarges it until the new one takes over.
        if self.zoom >= self.apparent { self.zoom } else { self.settled }
    }
}

#[derive(Component)]
pub struct WorldCamera;

/// The small image the world is rendered into.
#[derive(Resource)]
pub struct CanvasImage(pub Handle<Image>);

#[derive(Component)]
struct Canvas;

/// How the canvas sits on screen this frame, for cursor picking.
#[derive(Resource, Default)]
struct CanvasPlacement {
    /// Screen pixels per canvas pixel, including any zoom animation.
    scale: f32,
    /// Sub-pixel offset left over after snapping, in canvas pixels.
    remainder: Vec2,
}

fn spawn_cameras(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = CANVAS + UVec2::splat(2 * MARGIN);
    let mut canvas = Image::new_target_texture(size.x, size.y, TextureFormat::Bgra8UnormSrgb, None);
    canvas.sampler = ImageSampler::nearest();
    let canvas = images.add(canvas);
    commands.insert_resource(CanvasImage(canvas.clone()));

    commands.init_resource::<CanvasPlacement>();
    commands.spawn((
        Camera3d::default(),
        Camera { order: -1, clear_color: ClearColorConfig::Custom(Color::srgb_u8(0x9f, 0xc7, 0xd8)), ..default() },
        RenderTarget::Image(canvas.clone().into()),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: size.x as f32 / PIXELS_PER_METRE, height: size.y as f32 / PIXELS_PER_METRE },
            near: -200.0,
            far: 200.0,
            ..OrthographicProjection::default_3d()
        }),
        Msaa::Off,
        // Keep the art's colours exactly as painted.
        Tonemapping::None,
        WorldCamera,
        DespawnOnExit(Screen::Game),
    ));
    commands.spawn((Sprite::from_image(canvas), Canvas, DespawnOnExit(Screen::Game)));
}

fn animate_zoom(time: Res<Time>, mut rig: ResMut<CameraRig>) {
    let ease = 1.0 - (-ZOOM_EASE * time.delta_secs()).exp();
    rig.apparent += (rig.zoom - rig.apparent) * ease;
    if (rig.apparent - rig.zoom).abs() < 0.002 {
        rig.apparent = rig.zoom;
        rig.settled = rig.zoom;
    }
}

fn place_camera(
    rig: Res<CameraRig>,
    mut placement: ResMut<CanvasPlacement>,
    camera: Single<(&mut Transform, &mut Projection), With<WorldCamera>>,
) {
    let (mut transform, mut projection) = camera.into_inner();
    let rotation = Quat::from_euler(EulerRot::YXZ, rig.yaw, -PITCH, 0.0);
    let (right, up, back) = (rotation * Vec3::X, rotation * Vec3::Y, rotation * Vec3::Z);

    let size = (CANVAS + UVec2::splat(2 * MARGIN)).as_vec2();
    let texel = rig.render_zoom() / PIXELS_PER_METRE;
    if let Projection::Orthographic(ortho) = &mut *projection {
        ortho.scaling_mode = ScalingMode::Fixed { width: size.x * texel, height: size.y * texel };
    }

    // Snap the focus to whole pixels along the screen's axes.
    let (a, b) = (rig.focus.dot(right), rig.focus.dot(up));
    let (sa, sb) = ((a / texel).round() * texel, (b / texel).round() * texel);
    let snapped = rig.focus + (sa - a) * right + (sb - b) * up;
    placement.remainder = Vec2::new(a - sa, b - sb) / texel;

    // Close enough that the sun's shadow cascade covers everything in view.
    transform.translation = snapped + back * 40.0;
    transform.rotation = rotation;
}

/// Scales the canvas by the largest whole number that fits the window (times any zoom
/// animation), and applies the sub-pixel remainder so camera motion stays smooth.
fn fit_canvas(
    window: Single<&Window>,
    rig: Res<CameraRig>,
    mut placement: ResMut<CanvasPlacement>,
    canvas: Single<&mut Transform, With<Canvas>>,
) {
    let fit = (window.width() / CANVAS.x as f32).min(window.height() / CANVAS.y as f32);
    let scale = fit.floor().max(1.0) * rig.render_zoom() / rig.apparent;
    placement.scale = scale;
    let mut transform = canvas.into_inner();
    transform.scale = Vec3::new(scale, scale, 1.0);
    transform.translation = (-placement.remainder * scale).extend(0.0);
}

/// Cursor picking: rays from the mouse (or from a canvas pixel) into the world.
#[derive(bevy::ecs::system::SystemParam)]
pub struct CursorPicker<'w, 's> {
    window: Single<'w, 's, &'static Window>,
    camera: Single<'w, 's, (&'static Camera, &'static GlobalTransform), With<WorldCamera>>,
    placement: Res<'w, CanvasPlacement>,
}

impl CursorPicker<'_, '_> {
    /// The ray under the mouse cursor, if the cursor is over the window.
    pub fn ray(&self) -> Option<Ray3d> {
        let cursor = self.window.cursor_position()?;
        let scale = self.placement.scale.max(1e-3);
        let size = (CANVAS + UVec2::splat(2 * MARGIN)).as_vec2();
        // Window pixels (y down) relative to the window centre, then into canvas pixels.
        let centred = cursor - Vec2::new(self.window.width(), self.window.height()) / 2.0;
        let shift = Vec2::new(-self.placement.remainder.x, self.placement.remainder.y) * scale;
        self.ray_at_canvas((centred - shift) / scale + size / 2.0)
    }

    /// The ray through a pixel of the canvas (as saved in the capture shots).
    pub fn ray_at_canvas(&self, px: Vec2) -> Option<Ray3d> {
        let (camera, transform) = *self.camera;
        camera.viewport_to_world(transform, px).ok()
    }
}

/// Where a ray meets the ground (y = 0).
pub fn ground_point(ray: Ray3d) -> Option<Vec3> {
    let distance = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Vec3::Y))?;
    Some(ray.get_point(distance))
}
