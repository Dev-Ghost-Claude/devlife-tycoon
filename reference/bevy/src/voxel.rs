//! Voxel grids and the mesher that turns them into flat-shaded, vertex-coloured meshes.
//!
//! Grid axes: x runs left to right, y bottom to top, z front (0) to back. The front faces
//! world -Z, so a model built facing its own front can be turned with `Transform::looking_to`.

use std::collections::HashMap;

use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology};

/// Edge length of one voxel in world units (metres): 8 voxels to the metre.
pub const VOXEL: f32 = 0.125;
/// Fine voxels for detailed models: 24 to the metre, one canvas pixel each at zoom 1.
pub const FINE: f32 = 1.0 / 24.0;

#[derive(Clone)]
pub struct VoxelGrid {
    pub size: UVec3,
    cells: Vec<Option<Color>>,
}

impl VoxelGrid {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { size: UVec3::new(x, y, z), cells: vec![None; (x * y * z) as usize] }
    }

    fn index(&self, x: i32, y: i32, z: i32) -> Option<usize> {
        let s = self.size.as_ivec3();
        if x < 0 || y < 0 || z < 0 || x >= s.x || y >= s.y || z >= s.z {
            return None;
        }
        Some((x + s.x * (y + s.y * z)) as usize)
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<Color> {
        self.index(x, y, z).and_then(|i| self.cells[i])
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, color: Option<Color>) {
        if let Some(i) = self.index(x, y, z) {
            self.cells[i] = color;
        }
    }

    /// Fills the box from `min` to `max`, both inclusive.
    pub fn fill(&mut self, min: IVec3, max: IVec3, color: Color) {
        self.fill_with(min, max, |_| Some(color));
    }

    /// Clears the box from `min` to `max`, both inclusive.
    pub fn clear(&mut self, min: IVec3, max: IVec3) {
        self.fill_with(min, max, |_| None);
    }

    /// Sets each cell of the box to whatever `paint` returns for its position.
    pub fn fill_with(&mut self, min: IVec3, max: IVec3, mut paint: impl FnMut(IVec3) -> Option<Color>) {
        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    let p = IVec3::new(x, y, z);
                    self.set(x, y, z, paint(p));
                }
            }
        }
    }

    /// The smallest grid holding all the filled voxels, and where its corner sat in this
    /// one. For models drawn on a big canvas (a door painted in room coordinates).
    pub fn cropped(&self) -> (VoxelGrid, IVec3) {
        let s = self.size.as_ivec3();
        let (mut min, mut max) = (s, IVec3::splat(-1));
        for z in 0..s.z {
            for y in 0..s.y {
                for x in 0..s.x {
                    if self.get(x, y, z).is_some() {
                        let p = IVec3::new(x, y, z);
                        (min, max) = (min.min(p), max.max(p));
                    }
                }
            }
        }
        if max.x < 0 {
            return (VoxelGrid::new(0, 0, 0), IVec3::ZERO);
        }
        let size = (max - min + IVec3::ONE).as_uvec3();
        let mut g = VoxelGrid::new(size.x, size.y, size.z);
        g.fill_with(IVec3::ZERO, size.as_ivec3() - IVec3::ONE, |p| self.get(p.x + min.x, p.y + min.y, p.z + min.z));
        (g, min)
    }

    /// Builds the mesh. `origin` is the grid point (in voxels) that lands on the mesh's local origin.
    pub fn mesh(&self, origin: Vec3) -> Mesh {
        self.mesh_sized(origin, VOXEL)
    }

    /// Like [`Self::mesh`], with voxels of edge `size` metres. Corners are shaded by the
    /// model's own voxels only.
    pub fn mesh_sized(&self, origin: Vec3, size: f32) -> Mesh {
        self.mesh_lit(origin, size, &|p| self.get(p.x, p.y, p.z).is_some())
    }

    /// Like [`Self::mesh_sized`], with corner shading from everything `solid` reports, in
    /// this grid's coordinates (so a chair can darken the floor under it).
    ///
    /// Faces pointing straight down are left out: the camera always looks down, so it can
    /// never see them (they were half of the ground's mesh). Moving parts that can tilt use
    /// [`Self::mesh_moving`].
    pub fn mesh_lit(&self, origin: Vec3, size: f32, solid: &dyn Fn(IVec3) -> bool) -> Mesh {
        self.build_mesh(origin, size, solid, false)
    }

    /// Like [`Self::mesh_sized`], keeping the faces that point down: for parts that swing
    /// and can show their underside (an animal's legs and arms).
    pub fn mesh_moving(&self, origin: Vec3, size: f32) -> Mesh {
        self.build_mesh(origin, size, &|p| self.get(p.x, p.y, p.z).is_some(), true)
    }

    fn build_mesh(&self, origin: Vec3, size: f32, solid: &dyn Fn(IVec3) -> bool, bottoms: bool) -> Mesh {
        let open = self.open_space();
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();
        let mut shade_cache: HashMap<(IVec3, IVec3), f32> = HashMap::new();
        // A corner shared by neighbouring faces that look the same way and have the same
        // colour is stored once: it has the same shading too. About half the vertices.
        let mut shared: HashMap<(IVec3, usize, [u32; 3]), u32> = HashMap::new();

        let s = self.size.as_ivec3();
        for z in 0..s.z {
            for y in 0..s.y {
                for x in 0..s.x {
                    let Some(color) = self.get(x, y, z) else { continue };
                    let linear = color.to_linear();
                    let paint = [linear.red, linear.green, linear.blue].map(f32::to_bits);
                    let v = IVec3::new(x, y, z);
                    for (f, face) in FACES.iter().enumerate() {
                        let n = face.normal;
                        if (n == IVec3::NEG_Y && !bottoms) || !open(v + n) {
                            continue;
                        }
                        let mut shades = [1.0; 4];
                        let mut corners = [0; 4];
                        for (i, corner) in face.corners.iter().enumerate() {
                            let at = v + corner.as_ivec3();
                            let shade = *shade_cache.entry((at, n)).or_insert_with(|| ambient(solid, at, n));
                            shades[i] = shade;
                            corners[i] = *shared.entry((at, f, paint)).or_insert_with(|| {
                                // Grid z grows toward the back, which is world +Z; the front is world -Z.
                                let p = (at.as_vec3() - origin) * size;
                                positions.push([p.x, p.y, p.z]);
                                normals.push([n.x as f32, n.y as f32, n.z as f32]);
                                colors.push(LinearRgba::rgb(linear.red * shade, linear.green * shade, linear.blue * shade).to_f32_array());
                                positions.len() as u32 - 1
                            });
                        }
                        // Split the quad along the diagonal that keeps the shading even.
                        let [a, b, c, d] = corners;
                        if shades[0] + shades[2] >= shades[1] + shades[3] {
                            indices.extend_from_slice(&[a, b, c, a, c, d]);
                        } else {
                            indices.extend_from_slice(&[b, c, d, b, d, a]);
                        }
                    }
                }
            }
        }

        // Kept in main memory too (about 50 MB for the whole town, packed): clicks are ray
        // cast against the shapes. Keeping the copy only for clickable shapes saved that, but
        // with both kinds in the scene Bevy 0.20 sometimes drew the kept ones into the depth
        // pass only, leaving sky-coloured holes (2026-10-14/15).
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_indices(Indices::U32(indices));
        pack(&mut mesh);
        mesh
    }

    /// Which empty cells connect to the space around the model; cells beyond the grid all
    /// do. A face looking into a sealed pocket (a gap inside a wall, a hollow in a tree's
    /// crown) can never be seen, so it is left out.
    fn open_space(&self) -> impl Fn(IVec3) -> bool + use<> {
        let s = self.size.as_ivec3();
        // One cell of padding all round, so the flood can get around the whole model.
        let padded = s + IVec3::splat(2);
        let index = move |q: IVec3| (q.x + 1 + padded.x * (q.y + 1 + padded.y * (q.z + 1))) as usize;
        let mut open = vec![false; (padded.x * padded.y * padded.z) as usize];
        let start = IVec3::splat(-1);
        open[index(start)] = true;
        let mut stack = vec![start];
        while let Some(q) = stack.pop() {
            for face in &FACES {
                let r = q + face.normal;
                if r.cmplt(start).any() || r.cmpgt(s).any() {
                    continue;
                }
                let i = index(r);
                if !open[i] && self.get(r.x, r.y, r.z).is_none() {
                    open[i] = true;
                    stack.push(r);
                }
            }
        }
        move |q| q.cmplt(start).any() || q.cmpgt(s).any() || open[index(q)]
    }

    /// Mesh with the origin at the bottom centre of the grid: where a model stands.
    pub fn mesh_standing(&self) -> Mesh {
        let s = self.size.as_vec3();
        self.mesh(Vec3::new(s.x / 2.0, 0.0, s.z / 2.0))
    }

    /// A fingerprint of the voxels plus how they would be meshed, for [`MeshCache`].
    fn fingerprint(&self, origin: Vec3, size: f32) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        self.size.hash(&mut h);
        [origin.x, origin.y, origin.z, size].map(f32::to_bits).hash(&mut h);
        for cell in &self.cells {
            match cell {
                None => 0u8.hash(&mut h),
                Some(colour) => colour.to_linear().to_f32_array().map(f32::to_bits).hash(&mut h),
            }
        }
        h.finish()
    }
}

/// Meshes built so far while building a scene, by what they were built from: a model placed
/// many times (lamps, bins, neighbours' houses, aerials) is meshed and stored only once.
/// Only for meshes shaded by their own voxels ([`VoxelGrid::mesh_sized`]).
#[derive(Default)]
pub struct MeshCache(HashMap<u64, Handle<Mesh>>);

impl MeshCache {
    pub fn sized(&mut self, meshes: &mut Assets<Mesh>, grid: &VoxelGrid, origin: Vec3, size: f32) -> Handle<Mesh> {
        self.0.entry(grid.fingerprint(origin, size)).or_insert_with(|| meshes.add(grid.mesh_sized(origin, size))).clone()
    }
}

/// Packs a mesh for the GPU: normals in 4 bytes instead of 12 and colours as half floats
/// (40 bytes a vertex down to 24). Bevy's shaders unpack them; click ray casts read only
/// positions. Packed here, as the mesh is made, so nothing changes it afterwards.
///
/// Positions stay plain floats, as the click ray casts read them. Indices stay 32-bit: 16-bit ones would save another 9 MB, but with both sizes in the
/// scene Bevy 0.20 drew some meshes into the depth pass only (holes showing the sky).
fn pack(mesh: &mut Mesh) {
    use bevy::mesh::{AttributeQuantization, MeshAttributeCompressionFlags, MeshCompressionArgs};
    let args = MeshCompressionArgs {
        compress_indices: false,
        quantize_attributes: vec![(Mesh::ATTRIBUTE_COLOR.id, AttributeQuantization::Float16)].into(),
        compress_attributes: MeshAttributeCompressionFlags::COMPRESS_NORMAL,
    };
    if let Err(errors) = mesh.compress_mesh(&args) {
        warn!("couldn't pack a voxel mesh: {errors:?}");
    }
}

struct Face {
    normal: IVec3,
    /// Counter-clockwise when seen from outside.
    corners: [Vec3; 4],
}

const FACES: [Face; 6] = [
    Face {
        normal: IVec3::X,
        corners: [Vec3::new(1., 0., 0.), Vec3::new(1., 1., 0.), Vec3::new(1., 1., 1.), Vec3::new(1., 0., 1.)],
    },
    Face {
        normal: IVec3::NEG_X,
        corners: [Vec3::new(0., 0., 1.), Vec3::new(0., 1., 1.), Vec3::new(0., 1., 0.), Vec3::new(0., 0., 0.)],
    },
    Face {
        normal: IVec3::Y,
        corners: [Vec3::new(0., 1., 0.), Vec3::new(0., 1., 1.), Vec3::new(1., 1., 1.), Vec3::new(1., 1., 0.)],
    },
    Face {
        normal: IVec3::NEG_Y,
        corners: [Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.), Vec3::new(1., 0., 1.), Vec3::new(0., 0., 1.)],
    },
    Face {
        normal: IVec3::Z,
        corners: [Vec3::new(1., 0., 1.), Vec3::new(1., 1., 1.), Vec3::new(0., 1., 1.), Vec3::new(0., 0., 1.)],
    },
    Face {
        normal: IVec3::NEG_Z,
        corners: [Vec3::new(0., 0., 0.), Vec3::new(0., 1., 0.), Vec3::new(1., 1., 0.), Vec3::new(1., 0., 0.)],
    },
];

/// How far corner shading reaches, in voxels.
const SHADE_REACH: f32 = 4.0;
/// How dark a fully enclosed corner gets (0 = no shading, 1 = black).
const SHADE_DEPTH: f32 = 0.62;

/// Light reaching a mesh corner `at` (a grid point) on a face pointing along `n`: rays go
/// out over the face's half of the sky and count what they hit, nearer hits counting more.
/// This is the soft shading in creases and under furniture that makes voxel scenes look
/// solid; it reaches a few voxels, like light bouncing in a real room.
fn ambient(solid: &dyn Fn(IVec3) -> bool, at: IVec3, n: IVec3) -> f32 {
    let normal = n.as_vec3();
    let (t1, t2) = match n.abs() {
        IVec3 { x: 1, .. } => (Vec3::Y, Vec3::Z),
        IVec3 { y: 1, .. } => (Vec3::X, Vec3::Z),
        _ => (Vec3::X, Vec3::Y),
    };
    let start = at.as_vec3() + normal * 0.51;
    let mut blocked = 0.0;
    let mut total = 0.0;
    for a in -2..=2 {
        for b in -2..=2 {
            for c in 1..=2 {
                let dir = (t1 * a as f32 + t2 * b as f32 + normal * c as f32).normalize();
                // Rays near the face count less, like light arriving at a glancing angle.
                let weight = dir.dot(normal);
                total += weight;
                let mut t = 0.6;
                while t <= SHADE_REACH {
                    let p = (start + dir * t).floor().as_ivec3();
                    if solid(p) {
                        blocked += weight * (1.0 - t / (SHADE_REACH + 1.0));
                        break;
                    }
                    t += 0.8;
                }
            }
        }
    }
    1.0 - SHADE_DEPTH * (blocked / total * 1.8).min(1.0)
}

/// Which voxels are solid across a whole scene, so models placed next to each other shade
/// each other's corners. Positions are scene voxels.
pub struct Occupancy {
    min: IVec3,
    size: IVec3,
    cells: Vec<bool>,
}

impl Occupancy {
    pub fn new(min: IVec3, max: IVec3) -> Self {
        let size = max - min + IVec3::ONE;
        Self { min, size, cells: vec![false; (size.x * size.y * size.z) as usize] }
    }

    fn index(&self, p: IVec3) -> Option<usize> {
        let q = p - self.min;
        (q.cmpge(IVec3::ZERO).all() && q.cmplt(self.size).all()).then(|| (q.x + self.size.x * (q.y + self.size.y * q.z)) as usize)
    }

    /// Marks every filled voxel of `grid`, placed with its grid origin at scene voxel `at`.
    pub fn add(&mut self, grid: &VoxelGrid, at: IVec3) {
        let s = grid.size.as_ivec3();
        for z in 0..s.z {
            for y in 0..s.y {
                for x in 0..s.x {
                    if grid.get(x, y, z).is_some()
                        && let Some(i) = self.index(at + IVec3::new(x, y, z))
                    {
                        self.cells[i] = true;
                    }
                }
            }
        }
    }

    pub fn solid(&self, p: IVec3) -> bool {
        self.index(p).is_some_and(|i| self.cells[i])
    }
}

/// Parses a small text model. Layers run bottom to top; inside a layer, rows run back (top
/// row, like north on a map) to front, and characters left to right. `.` is empty; other characters come from the palette.
///
/// ```text
/// palette
/// a = #3b3b3b
/// layer
/// .a.
/// layer
/// .a.
/// ```
pub fn parse_model(text: &str) -> VoxelGrid {
    let mut palette = std::collections::HashMap::new();
    let mut layers: Vec<Vec<&str>> = Vec::new();
    let mut in_palette = false;
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') && !line.contains('=') {
            continue;
        }
        match line {
            "palette" => in_palette = true,
            "layer" => {
                in_palette = false;
                layers.push(Vec::new());
            }
            _ if in_palette => {
                let (key, hex) = line.split_once('=').expect("palette line `k = #rrggbb`");
                let key = key.trim().chars().next().expect("palette key");
                palette.insert(key, Srgba::hex(hex.trim()).expect("palette colour").into());
            }
            _ => layers.last_mut().expect("rows before the first `layer`").push(line),
        }
    }
    let depth = layers.iter().map(Vec::len).max().unwrap_or(0) as u32;
    let width = layers.iter().flatten().map(|r| r.chars().count()).max().unwrap_or(0) as u32;
    let mut grid = VoxelGrid::new(width, layers.len() as u32, depth);
    for (y, rows) in layers.iter().enumerate() {
        for (z, row) in rows.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                if ch != '.' {
                    let color = *palette.get(&ch).unwrap_or_else(|| panic!("no palette entry for {ch:?}"));
                    let z = depth as i32 - 1 - z as i32;
                    grid.set(x as i32, y as i32, z, Some(color));
                }
            }
        }
    }
    grid
}

/// A tiny deterministic hash for colour noise: same position, same result.
pub fn hash(p: IVec3, seed: u32) -> u32 {
    let mut h = (p.x as u32).wrapping_mul(0x8da6_b343)
        ^ (p.y as u32).wrapping_mul(0xd816_3841)
        ^ (p.z as u32).wrapping_mul(0xcb1a_b31f)
        ^ seed.wrapping_mul(0x1656_67b1);
    h ^= h >> 13;
    h = h.wrapping_mul(0x5bd1_e995);
    h ^ (h >> 15)
}

/// Picks one of `colors` by position, for speckled surfaces like grass or brick.
pub fn speckle(p: IVec3, seed: u32, colors: &[Color]) -> Color {
    colors[(hash(p, seed) % colors.len() as u32) as usize]
}
