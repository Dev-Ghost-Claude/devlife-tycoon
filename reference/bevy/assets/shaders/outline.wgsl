// The pixel-art linework, drawn from depth and normals. See src/outline.rs.
//
// - Silhouettes: a pixel with something well behind it is the edge of whatever is in
//   front, and gets the dark outline.
// - Creases, where two faces meet at an angle: convex edges (the rim of a table top) get
//   a lighter line, concave edges (where a bed meets the wall) a darker one. This is what
//   outlines objects pushed flush against a wall, where depth alone sees no edge.
// - Every line is one pixel wide, inside the object, and faces narrower than two pixels
//   (single voxel steps) get no line, so small detail doesn't turn into noise.

@group(0) @binding(0) var screen: texture_2d<f32>;
@group(0) @binding(1) var depth: texture_2d<f32>;
@group(0) @binding(2) var normals: texture_2d<f32>;

struct Outline {
    ink: vec4<f32>,
    right: vec4<f32>,
    up: vec4<f32>,
    forward: vec4<f32>,
    depth_range: f32,
    threshold: f32,
    highlight: f32,
    shade: f32,
}
@group(0) @binding(3) var<uniform> outline: Outline;

// Picks one side of each crease, top faces first, so creases are one pixel wide.
const BIAS = vec3(0.3, 2.0, 0.7);

var<private> last: vec2<i32>;

fn at(p: vec2<i32>) -> vec2<i32> {
    return clamp(p, vec2(0), last);
}

// Distance from the camera in metres. The projection is orthographic with reversed depth:
// 1 at the near plane, 0 at the far plane, linear in between.
fn distance_at(p: vec2<i32>) -> f32 {
    return (1.0 - textureLoad(depth, at(p), 0).r) * outline.depth_range;
}

fn normal_at(p: vec2<i32>) -> vec3<f32> {
    return normalize(textureLoad(normals, at(p), 0).xyz * 2.0 - 1.0);
}

@fragment
fn fragment(@builtin(position) position: vec4<f32>, @location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let p = vec2<i32>(position.xy);
    last = vec2<i32>(textureDimensions(screen)) - 1;
    let colour = textureLoad(screen, p, 0);
    let d = distance_at(p);
    // The empty background has no edges of its own.
    if d >= outline.depth_range - 0.001 {
        return colour;
    }
    let n = normal_at(p);

    var edge = false;
    var convex = false;
    var concave = false;
    let offsets = array<vec2<i32>, 4>(vec2(1, 0), vec2(-1, 0), vec2(0, 1), vec2(0, -1));
    for (var i = 0; i < 4; i++) {
        let o = offsets[i];
        let behind = distance_at(p + o) - d;
        if behind > outline.threshold {
            edge = true;
            continue;
        }
        let m = normal_at(p + o);
        // Same way up: no crease. Both faces must be at least two pixels across.
        if dot(n, m) > 0.5 || dot(m, normal_at(p + 2 * o)) < 0.9 || dot(n, normal_at(p - o)) < 0.9 {
            continue;
        }
        if dot(n - m, BIAS) <= 0.0 {
            continue;
        }
        // Where the neighbour sits relative to this pixel's face: behind it is a convex
        // edge, in front of it a concave one. (Screen y runs down, world up runs up.)
        let step = outline.right.xyz * f32(o.x) - outline.up.xyz * f32(o.y) + outline.forward.xyz * behind;
        let side = dot(step, n);
        if side < -0.004 {
            convex = true;
        } else if side > 0.004 {
            concave = true;
        }
    }

    if edge {
        return vec4(mix(colour.rgb * 0.3, outline.ink.rgb, outline.ink.a), colour.a);
    }
    if concave {
        return vec4(colour.rgb * (1.0 - outline.shade), colour.a);
    }
    if convex {
        return vec4(min(colour.rgb * (1.0 + outline.highlight), vec3(1.0)), colour.a);
    }
    return colour;
}
