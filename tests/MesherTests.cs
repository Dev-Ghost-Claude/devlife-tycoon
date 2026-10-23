using System;
using DevLife.Voxel;
using Godot;

// Headless checks for the voxel mesher. Run with:
// godot --headless --path . --script res://tests/MesherTests.cs
public partial class MesherTests : SceneTree
{
    private static readonly Color Paint = new("#3b3b3b");
    private static readonly Color Other = new("#e0a030");

    private int _failures;

    public override void _Initialize()
    {
        Run(TestSingleVoxel);
        Run(TestTwoVoxelsSameColour);
        Run(TestTwoVoxelsTwoColours);
        Run(TestSlab);
        Run(TestSolidBox);
        Run(TestHollowBox);
        Run(TestHollowBoxWithHole);
        Run(TestMovingKeepsBottoms);
        Run(TestWinding);
        Run(TestStanding);
        Run(TestAlphaIsOne);
        Run(TestLoneVoxelRoundTrip);
        Run(TestAmbientBlockOnFloor);
        Run(TestBlockOnFloorMesh);
        Run(TestBlockOnFloorShaded);
        Run(TestAmbientLOfThree);
        Run(TestLOfThreeMesh);
        Run(TestMeshLitFloor);
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

    private void CheckCounts(string name, MeshData mesh, int vertices, int indices)
    {
        Check(name, mesh.Positions.Length == vertices && mesh.Indices.Length == indices,
            "vertices " + mesh.Positions.Length + " (want " + vertices + "), indices "
            + mesh.Indices.Length + " (want " + indices + ")");
    }

    private static int CountBadWinding(MeshData mesh)
    {
        int bad = 0;
        for (int t = 0; t < mesh.Indices.Length; t += 3)
        {
            int a = mesh.Indices[t];
            int b = mesh.Indices[t + 1];
            int c = mesh.Indices[t + 2];
            Vector3 pa = mesh.Positions[a];
            Vector3 cross = (mesh.Positions[b] - pa).Cross(mesh.Positions[c] - pa);
            if (!(cross.Dot(mesh.Normals[a]) < 0f))
            {
                bad++;
            }
        }
        return bad;
    }

    private static VoxelGrid Solid(int x, int y, int z)
    {
        var g = new VoxelGrid(x, y, z);
        g.Fill(Vector3I.Zero, new Vector3I(x - 1, y - 1, z - 1), Paint);
        return g;
    }

    private void TestSingleVoxel()
    {
        var g = new VoxelGrid(1, 1, 1);
        g.Set(0, 0, 0, Paint);
        CheckCounts("TestSingleVoxel", g.Mesh(Vector3.Zero), 20, 30);
    }

    private void TestTwoVoxelsSameColour()
    {
        var g = new VoxelGrid(2, 1, 1);
        g.Set(0, 0, 0, Paint);
        g.Set(1, 0, 0, Paint);
        CheckCounts("TestTwoVoxelsSameColour", g.Mesh(Vector3.Zero), 26, 48);
    }

    private void TestTwoVoxelsTwoColours()
    {
        var g = new VoxelGrid(2, 1, 1);
        g.Set(0, 0, 0, Paint);
        g.Set(1, 0, 0, Other);
        CheckCounts("TestTwoVoxelsTwoColours", g.Mesh(Vector3.Zero), 32, 48);
    }

    private void TestSlab()
    {
        var g = new VoxelGrid(4, 1, 4);
        g.Fill(Vector3I.Zero, new Vector3I(3, 0, 3), Paint);
        CheckCounts("TestSlab", g.Mesh(Vector3.Zero), 65, 192);
    }

    private void TestSolidBox()
    {
        CheckCounts("TestSolidBox", Solid(5, 5, 5).Mesh(Vector3.Zero), 180, 750);
    }

    private void TestHollowBox()
    {
        var g = Solid(5, 5, 5);
        g.Clear(new Vector3I(1, 1, 1), new Vector3I(3, 3, 3));
        CheckCounts("TestHollowBox", g.Mesh(Vector3.Zero), 180, 750);
    }

    private void TestHollowBoxWithHole()
    {
        var g = Solid(5, 5, 5);
        g.Clear(new Vector3I(1, 1, 1), new Vector3I(3, 3, 3));
        g.Clear(new Vector3I(2, 2, 4), new Vector3I(2, 2, 4));
        CheckCounts("TestHollowBoxWithHole", g.Mesh(Vector3.Zero), 272, 1026);
    }

    private void TestMovingKeepsBottoms()
    {
        var g = new VoxelGrid(1, 1, 1);
        g.Set(0, 0, 0, Paint);
        CheckCounts("TestMovingKeepsBottoms", g.MeshMoving(Vector3.Zero, Voxels.Voxel), 24, 36);
    }

    private void TestWinding()
    {
        var single = new VoxelGrid(1, 1, 1);
        single.Set(0, 0, 0, Paint);
        int bad = CountBadWinding(single.Mesh(Vector3.Zero));
        int badSlab = CountBadWinding(Solid(4, 1, 4).Mesh(Vector3.Zero));
        Check("TestWinding", bad == 0 && badSlab == 0, "bad triangles: single " + bad + ", slab " + badSlab);
    }

    private void TestStanding()
    {
        var g = new VoxelGrid(1, 1, 1);
        g.Set(0, 0, 0, Paint);
        MeshData mesh = g.MeshStanding();
        const float tol = 1e-5f;
        float minX = float.MaxValue, minY = float.MaxValue, minZ = float.MaxValue;
        float maxX = float.MinValue, maxY = float.MinValue, maxZ = float.MinValue;
        foreach (Vector3 p in mesh.Positions)
        {
            minX = Math.Min(minX, p.X);
            minY = Math.Min(minY, p.Y);
            minZ = Math.Min(minZ, p.Z);
            maxX = Math.Max(maxX, p.X);
            maxY = Math.Max(maxY, p.Y);
            maxZ = Math.Max(maxZ, p.Z);
        }
        bool ok = Math.Abs(minX + 0.0625f) < tol && Math.Abs(maxX - 0.0625f) < tol
            && Math.Abs(minZ + 0.0625f) < tol && Math.Abs(maxZ - 0.0625f) < tol
            && Math.Abs(minY) < tol && Math.Abs(maxY - 0.125f) < tol;
        Check("TestStanding", ok, "x " + minX + ".." + maxX + ", y " + minY + ".." + maxY
            + ", z " + minZ + ".." + maxZ);
    }

    private void TestAlphaIsOne()
    {
        var g = new VoxelGrid(2, 1, 1);
        g.Set(0, 0, 0, Paint);
        g.Set(1, 0, 0, Other);
        MeshData mesh = g.Mesh(Vector3.Zero);
        bool ok = true;
        foreach (Color c in mesh.Colors)
        {
            if (c.A != 1f)
            {
                ok = false;
            }
        }
        Check("TestAlphaIsOne", ok && mesh.Colors.Length > 0, "some vertex colour has alpha != 1");
    }

    private const float ShadeTol = 1e-4f;

    private static bool Near(float got, float want)
    {
        return Math.Abs(got - want) < ShadeTol;
    }

    private void CheckShade(string name, float got, float want)
    {
        Check(name, Near(got, want), "got " + got.ToString("F6") + ", want " + want.ToString("F6"));
    }

    private static Func<Vector3I, bool> OwnVoxels(VoxelGrid g)
    {
        return p => g.Get(p.X, p.Y, p.Z) is not null;
    }

    private static VoxelGrid BlockOnFloor()
    {
        var g = new VoxelGrid(3, 2, 3);
        g.Fill(new Vector3I(0, 0, 0), new Vector3I(2, 0, 2), Paint);
        g.Set(1, 1, 1, Paint);
        return g;
    }

    private static VoxelGrid LOfThree()
    {
        var g = new VoxelGrid(2, 2, 1);
        g.Set(0, 0, 0, Paint);
        g.Set(1, 0, 0, Paint);
        g.Set(0, 1, 0, Paint);
        return g;
    }

    private void TestLoneVoxelRoundTrip()
    {
        var g = new VoxelGrid(1, 1, 1);
        g.Set(0, 0, 0, Paint);
        MeshData mesh = g.Mesh(Vector3.Zero);
        bool ok = mesh.Colors.Length > 0;
        foreach (Color c in mesh.Colors)
        {
            if (!Near(c.R, Paint.R) || !Near(c.G, Paint.G) || !Near(c.B, Paint.B))
            {
                ok = false;
            }
        }
        Check("TestLoneVoxelRoundTrip", ok, "a vertex colour differs from #3b3b3b beyond 1e-4");
    }

    private void TestAmbientBlockOnFloor()
    {
        VoxelGrid g = BlockOnFloor();
        Func<Vector3I, bool> solid = OwnVoxels(g);
        CheckShade("TestAmbientBlockOnFloor_side", VoxelMesher.Ambient(solid, new Vector3I(2, 1, 1), new Vector3I(1, 0, 0)), 0.657041f);
        CheckShade("TestAmbientBlockOnFloor_top", VoxelMesher.Ambient(solid, new Vector3I(2, 2, 1), new Vector3I(1, 0, 0)), 1f);
        CheckShade("TestAmbientBlockOnFloor_front", VoxelMesher.Ambient(solid, new Vector3I(1, 1, 0), new Vector3I(0, 1, 0)), 1f);
        CheckShade("TestAmbientBlockOnFloor_above", VoxelMesher.Ambient(solid, new Vector3I(1, 2, 1), new Vector3I(0, 1, 0)), 1f);
    }

    private void TestBlockOnFloorMesh()
    {
        MeshData mesh = BlockOnFloor().Mesh(Vector3.Zero);
        CheckCounts("TestBlockOnFloorMesh", mesh, 68, 150);
        int flipped = VoxelMesher.LastFlippedCount;
        Check("TestBlockOnFloorFlipped", flipped == 4, "flipped quads " + flipped + " (want 4)");
    }

    private void TestBlockOnFloorShaded()
    {
        MeshData mesh = BlockOnFloor().Mesh(Vector3.Zero);
        bool darker = false;
        foreach (Color c in mesh.Colors)
        {
            if (c.R < Paint.R - 0.01f)
            {
                darker = true;
            }
        }
        Check("TestBlockOnFloorShaded", darker, "no vertex colour darker than #3b3b3b");
    }

    private void TestAmbientLOfThree()
    {
        VoxelGrid g = LOfThree();
        Func<Vector3I, bool> solid = OwnVoxels(g);
        CheckShade("TestAmbientLOfThree_inner", VoxelMesher.Ambient(solid, new Vector3I(1, 1, 0), new Vector3I(0, 1, 0)), 0.798883f);
        CheckShade("TestAmbientLOfThree_outer", VoxelMesher.Ambient(solid, new Vector3I(2, 1, 0), new Vector3I(0, 1, 0)), 1f);
        CheckShade("TestAmbientLOfThree_side", VoxelMesher.Ambient(solid, new Vector3I(1, 1, 1), new Vector3I(1, 0, 0)), 0.858158f);
    }

    private void TestLOfThreeMesh()
    {
        MeshData mesh = LOfThree().Mesh(Vector3.Zero);
        CheckCounts("TestLOfThreeMesh", mesh, 38, 72);
        int flipped = VoxelMesher.LastFlippedCount;
        Check("TestLOfThreeFlipped", flipped == 2, "flipped quads " + flipped + " (want 2)");
    }

    private void TestMeshLitFloor()
    {
        var g = new VoxelGrid(1, 1, 1);
        g.Set(0, 0, 0, Paint);
        Func<Vector3I, bool> own = OwnVoxels(g);
        Func<Vector3I, bool> withFloor = p => own(p) || p.Y == -1;
        CheckShade("TestMeshLitFloor", VoxelMesher.Ambient(withFloor, new Vector3I(1, 0, 0), new Vector3I(1, 0, 0)), 0.628559f);
    }
}
