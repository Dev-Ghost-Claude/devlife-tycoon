//! Voxel grids and the mesher that turns them into flat-shaded, vertex-coloured meshes.
//!
//! Grid axes: x runs left to right, y bottom to top, z front (0) to back. The front faces
//! world -Z, so a model built facing its own front can be turned with `Transform::looking_to`.

use bevy::{asset::RenderAssetUsages, mesh::Indices, prelude::*, render::render_resource::PrimitiveTopology};

/// Edge length of one voxel in world units (metres): 8 voxels to the metre.
pub const VOXEL: f32 = 0.125;

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

    /// Builds the mesh. `origin` is the grid point (in voxels) that lands on the mesh's local origin.
    pub fn mesh(&self, origin: Vec3) -> Mesh {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut colors = Vec::new();
        let mut indices = Vec::new();

        let s = self.size.as_ivec3();
        for z in 0..s.z {
            for y in 0..s.y {
                for x in 0..s.x {
                    let Some(color) = self.get(x, y, z) else { continue };
                    let linear = color.to_linear().to_f32_array();
                    for face in &FACES {
                        let n = face.normal;
                        if self.get(x + n.x, y + n.y, z + n.z).is_some() {
                            continue;
                        }
                        let base = positions.len() as u32;
                        for corner in face.corners {
                            // Grid z grows toward the back, which is world +Z; the front is world -Z.
                            let p = (Vec3::new(x as f32, y as f32, z as f32) + corner - origin) * VOXEL;
                            positions.push([p.x, p.y, p.z]);
                            normals.push([n.x as f32, n.y as f32, n.z as f32]);
                            colors.push(linear);
                        }
                        indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
                    }
                }
            }
        }

        // Kept in the main world too, so mouse clicks can be ray cast against the shapes.
        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
            .with_inserted_indices(Indices::U32(indices))
    }

    /// Mesh with the origin at the bottom centre of the grid: where a model stands.
    pub fn mesh_standing(&self) -> Mesh {
        let s = self.size.as_vec3();
        self.mesh(Vec3::new(s.x / 2.0, 0.0, s.z / 2.0))
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
