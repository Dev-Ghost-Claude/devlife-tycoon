using System;
using DevLife.Voxel;
using Godot;

// Headless checks for building Godot meshes, the voxel material and the mesh cache. Run with:
// godot --headless --path . --script res://tests/MeshBuildTests.cs
public partial class MeshBuildTests : SceneTree
{
    private static readonly Color Paint = new("#3b3b3b");
    private static readonly Color Other = new("#e0a030");

    private int _failures;

    public override void _Initialize()
    {
        Run(TestSingleVoxelMesh);
        Run(TestSlabMesh);
        Run(TestSlabSurfaceArrays);
        Run(TestMaterialFlags);
        Run(TestMaterialIsShared);
        Run(TestCacheSequence);
        Run(TestFingerprintEqual);
        Run(TestFingerprintChanged);
        Quit(_failures == 0 ? 0 : 1);
    }

    private void Run(Action test)
    {
        try
        {
            test();
        }
        catch (Exception e)
        {
            Check(test.Method.Name + "_threw", false, e.GetType().Name + ": " + e.Message);
        }
    }

    private void Check(string name, bool ok, string detail = "")
    {
        if (ok)
        {
            GD.Print("PASS " + name);
        }
        else
        {
            _failures++;
            GD.Print("FAIL " + name + ": " + detail);
        }
    }

    private static VoxelGrid SingleVoxel(Color colour)
    {
        var g = new VoxelGrid(1, 1, 1);
        g.Set(0, 0, 0, colour);
        return g;
    }

    private static VoxelGrid Slab()
    {
        var g = new VoxelGrid(4, 1, 4);
        g.Fill(Vector3I.Zero, new Vector3I(3, 0, 3), Paint);
        return g;
    }

    private void CheckSurface(string name, ArrayMesh mesh, int vertices, int indices)
    {
        int surfaces = mesh.GetSurfaceCount();
        int vertexLen = mesh.SurfaceGetArrayLen(0);
        int indexLen = mesh.SurfaceGetArrayIndexLen(0);
        Check(name, surfaces == 1 && vertexLen == vertices && indexLen == indices,
            "surfaces " + surfaces + ", vertices " + vertexLen + " (want " + vertices
            + "), indices " + indexLen + " (want " + indices + ")");
    }

    private void TestSingleVoxelMesh()
    {
        ArrayMesh mesh = SingleVoxel(Paint).Mesh(Vector3.Zero).ToArrayMesh();
        CheckSurface("TestSingleVoxelMesh", mesh, 20, 30);
    }

    private void TestSlabMesh()
    {
        ArrayMesh mesh = Slab().Mesh(Vector3.Zero).ToArrayMesh();
        CheckSurface("TestSlabMesh", mesh, 65, 192);
    }

    // Fallback check through the surface arrays themselves, in case the lengths above are not
    // reported under the headless dummy renderer.
    private void TestSlabSurfaceArrays()
    {
        ArrayMesh mesh = Slab().Mesh(Vector3.Zero).ToArrayMesh();
        Godot.Collections.Array arrays = mesh.SurfaceGetArrays(0);
        int vertexCount = ((Vector3[])arrays[(int)Mesh.ArrayType.Vertex]).Length;
        int indexCount = ((int[])arrays[(int)Mesh.ArrayType.Index]).Length;
        Check("TestSlabSurfaceArrays", vertexCount == 65 && indexCount == 192,
            "vertices " + vertexCount + " (want 65), indices " + indexCount + " (want 192)");
    }

    private void TestMaterialFlags()
    {
        StandardMaterial3D material = VoxelMeshes.Material;
        Check("TestMaterialFlags", material.VertexColorUseAsAlbedo && material.VertexColorIsSrgb,
            "UseAsAlbedo " + material.VertexColorUseAsAlbedo + ", IsSrgb " + material.VertexColorIsSrgb);
    }

    private void TestMaterialIsShared()
    {
        Check("TestMaterialIsShared", ReferenceEquals(VoxelMeshes.Material, VoxelMeshes.Material));
    }

    // One cache, used in order: the same voxels twice, a changed colour, then the first grid
    // again at another origin.
    private void TestCacheSequence()
    {
        var cache = new MeshCache();
        ArrayMesh first = cache.Sized(SingleVoxel(Paint), Vector3.Zero, Voxels.Voxel);
        ArrayMesh same = cache.Sized(SingleVoxel(Paint), Vector3.Zero, Voxels.Voxel);
        Check("TestCacheSameVoxels", ReferenceEquals(first, same) && cache.Count == 1,
            "same instance " + ReferenceEquals(first, same) + ", Count " + cache.Count + " (want 1)");

        ArrayMesh changed = cache.Sized(SingleVoxel(Other), Vector3.Zero, Voxels.Voxel);
        Check("TestCacheChangedColour", !ReferenceEquals(first, changed) && cache.Count == 2,
            "different instance " + !ReferenceEquals(first, changed) + ", Count " + cache.Count + " (want 2)");

        ArrayMesh moved = cache.Sized(SingleVoxel(Paint), new Vector3(1f, 0f, 0f), Voxels.Voxel);
        Check("TestCacheOtherOrigin", !ReferenceEquals(first, moved) && cache.Count == 3,
            "different instance " + !ReferenceEquals(first, moved) + ", Count " + cache.Count + " (want 3)");
    }

    private void TestFingerprintEqual()
    {
        ulong a = SingleVoxel(Paint).Fingerprint(Vector3.Zero, Voxels.Voxel);
        ulong b = SingleVoxel(Paint).Fingerprint(Vector3.Zero, Voxels.Voxel);
        Check("TestFingerprintEqual", a == b, a + " vs " + b);
    }

    private void TestFingerprintChanged()
    {
        var changed = SingleVoxel(Paint);
        changed.Set(0, 0, 0, Other);
        ulong a = SingleVoxel(Paint).Fingerprint(Vector3.Zero, Voxels.Voxel);
        ulong b = changed.Fingerprint(Vector3.Zero, Voxels.Voxel);
        Check("TestFingerprintChanged", a != b, a + " vs " + b);
    }
}
