using System.Collections.Generic;
using Godot;

namespace DevLife.Voxel;

/// Meshes built so far while building a scene, by what they were built from: a model placed
/// many times (lamps, bins, neighbours' houses, aerials) is meshed and stored only once.
/// Only for meshes shaded by their own voxels (VoxelGrid.MeshSized), as in the Rust MeshCache.
public sealed class MeshCache
{
    private readonly Dictionary<ulong, ArrayMesh> _meshes = new();

    /// The cached mesh for this grid, origin and size, built on first request.
    public ArrayMesh Sized(VoxelGrid grid, Vector3 origin, float size)
    {
        ulong key = grid.Fingerprint(origin, size);
        if (_meshes.TryGetValue(key, out ArrayMesh? cached))
        {
            return cached;
        }
        ArrayMesh mesh = grid.MeshSized(origin, size).ToArrayMesh();
        _meshes[key] = mesh;
        return mesh;
    }

    /// How many distinct meshes are stored.
    public int Count => _meshes.Count;
}
