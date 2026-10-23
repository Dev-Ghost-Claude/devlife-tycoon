//! The pixel-art outline. After the world is drawn, a post-process reads the depth and
//! normals of every canvas pixel and draws:
//!
//! - a dark line on the edge of anything that stands in front of something farther back
//!   (the silhouette of every object, character and building), and
//! - a light line along convex edges, where a top face meets a side face, the way pixel
//!   artists catch the light on an edge.
//!
//! Both are one canvas pixel wide and drawn inside the object, so nothing grows or blurs.

use bevy::{
    core_pipeline::{
        prepass::{DepthPrepass, NormalPrepass, ViewPrepassTextures},
        schedule::Core3d,
        Core3dSystems, FullscreenShader,
    },
    prelude::*,
    render::{
        extract_component::{ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin},
        render_resource::{
            binding_types::{texture_2d, uniform_buffer},
            *,
        },
        renderer::{RenderContext, ViewQuery},
        view::ViewTarget,
        RenderApp, RenderStartup,
    },
};

const SHADER: &str = "shaders/outline.wgsl";

pub struct OutlinePlugin;

impl Plugin for OutlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ExtractComponentPlugin::<Outline>::default(), UniformComponentPlugin::<Outline>::default()));
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_systems(RenderStartup, init_pipeline)
            .add_systems(Core3d, draw_outline.in_set(Core3dSystems::PostProcess));
    }
}

/// Put on a 3D camera (with `DepthPrepass` and `NormalPrepass`) to outline its view.
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
#[extract_app(RenderApp)]
pub struct Outline {
    /// The outline's colour (linear RGB). Alpha is how far an edge pixel moves toward it
    /// from its own darkened colour, so outlines keep a hint of the object's hue.
    pub ink: Vec4,
    /// One canvas pixel to the right, in world metres.
    pub right: Vec4,
    /// One canvas pixel up, in world metres.
    pub up: Vec4,
    /// The direction the camera looks.
    pub forward: Vec4,
    /// Far minus near of the orthographic projection, to turn depth into metres.
    pub depth_range: f32,
    /// How much farther a neighbour must be, in metres, before this pixel is an edge.
    /// Must exceed the steepest flat surface's step per pixel (the ground), or flat
    /// ground would be outlined everywhere.
    pub threshold: f32,
    /// How much lighter a convex edge gets (0 turns it off).
    pub highlight: f32,
    /// How much darker a concave edge (an inside corner) gets (0 turns it off).
    pub shade: f32,
}

impl Outline {
    /// The usual look, for a camera whose projection spans `depth_range` metres.
    pub fn new(depth_range: f32) -> Self {
        Self {
            ink: Vec4::new(0.012, 0.008, 0.016, 0.55),
            right: Vec4::X,
            up: Vec4::Y,
            forward: Vec4::NEG_Z,
            depth_range,
            threshold: 0.15,
            highlight: 0.22,
            shade: 0.38,
        }
    }

    /// Fits the outline to the view: `rotation` is the camera's, and one canvas pixel is
    /// `texel` metres wide and `texel_up` metres tall. Anything that rises more than
    /// `min_rise` metres above what is behind it is outlined.
    pub fn fit(&mut self, rotation: Quat, texel: f32, texel_up: f32, pitch: f32, min_rise: f32) {
        self.right = (rotation * Vec3::X * texel).extend(0.0);
        self.up = (rotation * Vec3::Y * texel_up).extend(0.0);
        self.forward = (rotation * Vec3::NEG_Z).extend(0.0);
        // The ground gets this much deeper from one pixel row to the next.
        let ground_step = texel_up / pitch.tan();
        self.threshold = ground_step + min_rise / pitch.sin();
    }
}

/// The view's colour texture can be any of these (HDR, or either 8-bit order), so the pass
/// is built for each and picks the one that matches.
const FORMATS: [TextureFormat; 3] = [TextureFormat::Rgba16Float, TextureFormat::Rgba8UnormSrgb, TextureFormat::Bgra8UnormSrgb];

#[derive(Resource)]
struct OutlinePipeline {
    layout: BindGroupLayoutDescriptor,
    pipelines: [CachedRenderPipelineId; 3],
}

fn init_pipeline(mut commands: Commands, asset_server: Res<AssetServer>, fullscreen: Res<FullscreenShader>, cache: Res<PipelineCache>) {
    let layout = BindGroupLayoutDescriptor::new(
        "outline_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: false }),
                texture_2d(TextureSampleType::Float { filterable: false }),
                texture_2d(TextureSampleType::Float { filterable: false }),
                uniform_buffer::<Outline>(true),
            ),
        ),
    );
    let shader = asset_server.load(SHADER);
    let pipelines = FORMATS.map(|format| {
        cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some("outline_pipeline".into()),
            layout: vec![layout.clone()],
            vertex: fullscreen.to_vertex_state(),
            fragment: Some(FragmentState {
                shader: shader.clone(),
                targets: vec![Some(ColorTargetState { format, blend: None, write_mask: ColorWrites::ALL })],
                ..default()
            }),
            ..default()
        })
    });
    commands.insert_resource(OutlinePipeline { layout, pipelines });
}

fn draw_outline(
    view: ViewQuery<(&ViewTarget, &ViewPrepassTextures, &DynamicUniformIndex<Outline>)>,
    outline: Option<Res<OutlinePipeline>>,
    cache: Res<PipelineCache>,
    uniforms: Res<ComponentUniforms<Outline>>,
    mut ctx: RenderContext,
) {
    let Some(outline) = outline else { return };
    let (target, prepass, index) = view.into_inner();
    let Some(slot) = FORMATS.iter().position(|f| *f == target.main_texture_format()) else {
        return;
    };
    let (Some(pipeline), Some(depth), Some(normals), Some(settings)) =
        (cache.get_render_pipeline(outline.pipelines[slot]), prepass.depth_only_view(), prepass.normal_view(), uniforms.uniforms().binding())
    else {
        return;
    };

    let post = target.post_process_write();
    let bind_group = ctx.render_device().create_bind_group(
        Some("outline_bind_group"),
        &cache.get_bind_group_layout(&outline.layout),
        &BindGroupEntries::sequential((post.source, depth, normals, settings)),
    );
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some("outline"),
        color_attachments: &[Some(RenderPassColorAttachment {
            view: post.destination,
            depth_slice: None,
            resolve_target: None,
            ops: Operations::default(),
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, &bind_group, &[index.index()]);
    pass.draw(0..3, 0..1);
}

/// The components a camera needs for outlines.
pub fn outlined_camera(depth_range: f32) -> impl Bundle {
    (DepthPrepass, NormalPrepass, Outline::new(depth_range))
}
