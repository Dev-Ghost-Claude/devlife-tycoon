# Voxel Tools notes (second reference for step 1b)

Source: github.com/Zylann/godot_voxel (MIT, C++ module, master branch, read 2026-10-16 via `gh api`, nothing cloned).
Our first reference stays `reference/bevy/src/voxel.rs`. This file only says what Voxel Tools adds.
Paths below are inside their repo. All recommendations keep the step 1b counts (20/30, 48, 65/192, 125/750/180) unless marked.

## 1. Their blocky mesher

Entry: `meshers/blocky/voxel_mesher_blocky.cpp`, function `generate_mesh<>` (about 440 lines) plus `build()`.

- **Input.** A flat byte/ushort array of model ids, one id per voxel, with a 1-voxel padded border (`PADDING = 1`) copied from the neighbouring chunks. Neighbours are read by adding precomputed index offsets (`side_neighbor_lut`, `edge_neighbor_lut`, `corner_neighbor_lut`), so there are no bounds checks in the hot loop.
- **Face culling.** Per voxel, per side, a face is kept if the neighbour is air, or the neighbour is "more transparent", or the neighbour does not cull. From `voxel_mesher_blocky.h` (Zylann, MIT):
  `return other_vt.empty || (other_vt.transparency_index > vt.transparency_index) || !other_vt.culls_neighbors;`
  If that is false, a baked table of side shapes decides (partial shapes, slabs, stairs). Their 1-voxel result: all 6 faces.
- **Per-model geometry.** Each model (cube, mesh, fluid) is baked once into `BakedModel` (`blocky_baked_library.h`): surfaces split per cube side so a side can be dropped when occluded, plus an "inside" part that is always kept. A mesher run just copies pre-built vertices, adds the voxel position, and appends. A model can have up to 2 materials; output is one surface per material.
- **Ambient occlusion.** The classic 0fps scheme (their code comment links 0fps.net 2013/07/03). For each face it counts solid neighbours on the 4 edges and 4 corners touching the face: both side neighbours solid gives 3, else 0 to 3. Vertex shade = `darkness/3 * level`, weighted by `1 - distance^2` to that corner, taken as the max of the four. Colour = `(1-shade) * model colour`. Reach is 1 voxel. Models can opt out (`contributes_to_ao`). Switch: `bake_occlusion`, `occlusion_darkness`.
- **Vertex sharing.** None. Every visible face writes its own 4 vertices (24 for a lone cube). Indices are just `index_offset + baked side indices`.
- **Diagonal choice.** Not flipped. They carry a TODO ("occlusion inconsistency caused by triangles orientation, not sure if worth it").
- **Transparency.** `transparency_index` (ordered integer) and `culls_neighbors`; same-index glass hides its own inner faces, opaque next to glass keeps the opaque face. Different material = different surface, so glass can have its own alpha material. Fluids have a separate path (`blocky_fluids_meshing_impl.h`).
- **Not relevant to us.** LOD skirts (`blocky_lod_skirts.h`), shadow occluder meshes, tint sampler, tangents, UVs, collision surface output, cutout sides.
- **Caching.** No mesh cache by content. Their caching is: the library is baked once (model shapes, side-pattern occlusion bitset), and output arrays live in a `thread_local` `Cache` that is `clear()`ed and reused, so there are no per-build allocations.

## 2. Their instancer (scattering grass and props)

Files: `terrain/instancing/voxel_instancer.cpp` (node), `voxel_instance_generator.cpp` (placement), `voxel_instance_library_multimesh_item.cpp`, doc `doc/source/instancing.md`.

Core idea: scatter is computed from the finished surface MESH of each chunk, not from the voxels.
1. Pick random triangles: `count = density * triangle_count` (`generate_random_points_from_triangles_fast`), a random barycentric point in each (two PCG streams, one for triangle choice, one for the rest). Slower variant weights by triangle area; another gives one point per triangle or per vertex.
2. Interpolate the surface normal at that point.
3. Filter: slope (min/max degrees with soft falloff), height range, optional noise, optional voxel-material mask.
4. Build a transform: up axis = lerp(surface normal, world up, `vertical_alignment`); random yaw (or fixed look axis); random scale in [min,max] with optional curve; push out by `offset_along_normal` so roots sink in.
5. One `MultiMesh` per chunk per item, created via RenderingServer; chunks unload with the terrain. Items persist or regenerate; scene items exist but are slow.

Small C# version on `MultiMesh` would need:
- Input: the `MeshData` arrays from our mesher (triangles + normals), or simply the top faces of the grid (cheaper and exact for blocky).
- A seeded RNG keyed by `VoxelGrid.Fingerprint`, so scatter is repeatable and cacheable.
- A filter (normal.y threshold, optional colour mask: grass only on green voxels) and the transform builder above.
- `MultiMesh { TransformFormat = Transform3D, Mesh = blade, InstanceCount = n }`, `SetInstanceTransform(i, t)` in a loop (or one bulk `Buffer`), one `MultiMeshInstance3D` per placed model; reuse one MultiMesh for identical fingerprints.
- Skip their LOD, persistence, collision, scene items, SDF snapping. Not needed at our scale.

## 3. Threading and mesh reuse worth copying later

- **Pure data step, then engine step.** The mesher (`build`) only fills plain arrays; building the Godot `Mesh` is a later step (`build_mesh` in `mesh_block_task.cpp`, done on the worker only if the engine allows). Copy this split: C# `MeshData` (Vector3[], normals, Color[], int[]), then `ArrayMesh.AddSurfaceFromArrays` on the main thread.
- Tasks copy the chunk plus its neighbours first (`copy_block_and_neighbors`), then mesh a private copy, so no locks are held while meshing. Settings are copied under a read lock at the start of `build`.
- `thread_local` scratch buffers (C# `[ThreadStatic]` lists, cleared not reallocated).
- Priority by distance to the viewer and cancel tokens (`MeshBlockTask::is_cancelled`): only needed if we stream; our town is built once.

## 4. Side by side

| Area | Voxel Tools blocky | Our Bevy mesher (to port) |
|---|---|---|
| Voxel data | model id per voxel, padded | `Option<Color>` per voxel, no padding, `Occupancy` for scene-wide neighbours |
| Culling | neighbour transparency/shape table | neighbour empty only; bottoms skipped unless moving part |
| Hidden pockets | not handled | flood fill from outside (`open_space`) drops sealed faces |
| Shading | 0fps AO: 8 neighbours, reach 1, 4 levels | 25x2 sky rays per corner, reach 4, soft, cached per (corner, normal) |
| Vertices | 4 per face, never shared | shared by (corner, face, colour): 65 not 128 for the slab |
| Diagonal | fixed | flipped to keep shading even |
| Colour | `gs * model colour` | linear colour * shade |
| Geometry | any baked model | cubes only |
| Output | arrays per material | one surface |
| Caching | baked library; reused scratch buffers | `MeshCache` by fingerprint |

## 5. Recommendations for step 1b

Adopt (all keep the test numbers):
1. **Keep the Bevy algorithm as the spec** (flood-fill sealed faces, vertex sharing, ray AO, diagonal flip). Voxel Tools cannot reach 65 vertices for the slab (it has no sharing: 128) and has no sealed-pocket culling, so it is not a drop-in for the checks.
2. **Mesher returns plain `MeshData` arrays; building `ArrayMesh` is a separate method.** Tests read counts from the arrays, headless, no Godot runtime. It is also the thread-safe shape later (section 3).
3. **Flat arrays in the hot loop.** Map each distinct colour (bit-equal) to a small id once per build; use it for the share key instead of a `[u32;3]` tuple. Use a flat `float[]` for the AO cache (corner point x 6 normals, NaN = unset) instead of `Dictionary<(Vector3I,Vector3I),float>`. Same results, far less garbage. Optionally pad a local bool/id array by 1 so culling needs no bounds checks.
4. **Reuse scratch lists** (`[ThreadStatic] List<>`, `Clear()`), as they do. Copy to arrays only at the end.
5. **Keep the 6-entry face table with a per-face neighbour offset**, like their LUTs. Winding: Godot is clockwise like Voxel Tools, so reuse the planned `[a,c,b,a,d,c]` / `[b,d,c,b,a,d]` swap and test it with the cross-product check.

Optional, later:
6. **Transparency (glass, windows):** add a per-voxel `transparencyIndex` (0 = opaque) and use their rule: keep a face if the neighbour is empty or has a higher index. Glass goes to a second surface with an alpha material. With only opaque voxels in the checks the counts do not move. Skip in 1b; it needs a `VoxelGrid` field, so ask first.

Skip, with reasons:
- Their 0fps AO as the main shader: cheaper, but a different look from the Bevy build. **Changes the look, not the counts: needs the Admin's OK** (only worth offering as a fast path for the huge ground mesh).
- Unshared vertices: **changes the test numbers (65 would become 128): needs the Admin's OK**; no reason to do it.
- Baked models, side-pattern tables, cutouts, fluids, LOD skirts, shadow occluders, tint, tangents, UVs, collision output, greedy meshing (they themselves rejected it): we have only cubes, one surface, and a camera that never sees bottoms.

## 6. Licence note

Voxel Tools is MIT, "Copyright (c) 2016-2024 Marc Gilleron". We have only read it and re-derived ideas (0fps AO is a public technique, culling rule is a one-liner), so nothing needs adding today. If an agent ports code closely (the AO neighbour counting loop, the face visibility rule, instancer transform maths), the NOTICE file must then carry their copyright line and the MIT permission text. Suggested wording for that day (the Admin edits NOTICE, not workers): "Parts of the voxel mesher/instancer are adapted from Voxel Tools (godot_voxel), Copyright (c) 2016-2024 Marc Gilleron, MIT licence." Snippets in this file are short and credited.

## 7. Open doubts

- Does the Admin want the ray AO kept exactly (pixel parity with Bevy), or is the cheaper 0fps look acceptable for large meshes?
- Glass/windows in the town: needs a transparency field in `VoxelGrid`; decide before step 4 (world), not in 1b.
- Picking (step 5): Bevy ray-cast the mesh; in Godot a grid DDA over `VoxelGrid` may be simpler than a trimesh collider. Not researched here.
