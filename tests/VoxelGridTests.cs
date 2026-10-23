using System;
using DevLife.Voxel;
using Godot;

// Headless checks for the voxel core. Run with:
// godot --headless --path . --script res://tests/VoxelGridTests.cs
public partial class VoxelGridTests : SceneTree
{
    private int _failures;

    public override void _Initialize()
    {
        Run(TestGetSet);
        Run(TestFill);
        Run(TestCropped);
        Run(TestOccupancy);
        Run(TestHash);
        Run(TestParseModel);
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

    private static int CountFilled(VoxelGrid g)
    {
        int n = 0;
        for (int z = 0; z < g.Size.Z; z++)
        {
            for (int y = 0; y < g.Size.Y; y++)
            {
                for (int x = 0; x < g.Size.X; x++)
                {
                    if (g.Get(x, y, z) is not null)
                    {
                        n++;
                    }
                }
            }
        }
        return n;
    }

    private void TestGetSet()
    {
        var g = new VoxelGrid(3, 4, 5);
        g.Set(1, 2, 3, Colors.Red);
        Check("get_set_inside", g.Get(1, 2, 3) == Colors.Red, "got " + g.Get(1, 2, 3));
        Check("get_empty_inside", g.Get(0, 0, 0) is null, "expected null");
        g.Set(0, 1, 0, Colors.Blue);
        Check("no_index_alias", g.Get(1, 0, 0) is null && g.Get(0, 1, 0) == Colors.Blue, "cells alias each other");
        Check(
            "get_out_of_bounds",
            g.Get(-1, 0, 0) is null && g.Get(3, 0, 0) is null && g.Get(0, -1, 0) is null
                && g.Get(0, 4, 0) is null && g.Get(0, 0, -1) is null && g.Get(0, 0, 5) is null,
            "expected null outside");
        int before = CountFilled(g);
        g.Set(-1, 0, 0, Colors.Red);
        g.Set(3, 0, 0, Colors.Red);
        g.Set(0, 4, 0, Colors.Red);
        g.Set(0, 0, 5, Colors.Red);
        Check("set_out_of_bounds_ignored", CountFilled(g) == before, "count changed to " + CountFilled(g));
        g.Set(1, 2, 3, null);
        Check("set_null_clears", g.Get(1, 2, 3) is null, "still set");
    }

    private void TestFill()
    {
        var g = new VoxelGrid(4, 4, 4);
        g.Fill(new Vector3I(0, 0, 0), new Vector3I(2, 1, 3), Colors.Green);
        Check("fill_count", CountFilled(g) == 24, "expected 24, got " + CountFilled(g));
        Check("fill_max_inclusive", g.Get(2, 1, 3) == Colors.Green && g.Get(3, 1, 3) is null, "max bound wrong");
        g.Clear(new Vector3I(1, 0, 0), new Vector3I(2, 1, 0));
        Check("clear_count", CountFilled(g) == 20, "expected 20, got " + CountFilled(g));
        g.FillWith(new Vector3I(0, 0, 0), new Vector3I(3, 3, 3), p => p.X == 0 ? (Color?)null : Colors.Red);
        Check("fill_with_count", CountFilled(g) == 48, "expected 48, got " + CountFilled(g));
    }

    private void TestCropped()
    {
        var g = new VoxelGrid(6, 6, 6);
        g.Set(1, 2, 3, Colors.Red);
        g.Set(4, 2, 0, Colors.Blue);
        var (c, off) = g.Cropped();
        Check("cropped_size", c.Size == new Vector3I(4, 1, 4), "got " + c.Size);
        Check("cropped_offset", off == new Vector3I(1, 2, 0), "got " + off);
        Check(
            "cropped_content",
            c.Get(0, 0, 3) == Colors.Red && c.Get(3, 0, 0) == Colors.Blue && CountFilled(c) == 2,
            "contents misplaced, filled " + CountFilled(c));

        var (e, eOff) = new VoxelGrid(3, 3, 3).Cropped();
        Check("cropped_empty", e.Size == Vector3I.Zero && eOff == Vector3I.Zero, "got size " + e.Size + " offset " + eOff);
    }

    private void TestOccupancy()
    {
        var piece = new VoxelGrid(2, 2, 2);
        piece.Set(0, 0, 0, Colors.Red);
        piece.Set(1, 0, 1, Colors.Red);
        piece.Set(0, 1, 0, Colors.Red);

        var occ = new Occupancy(new Vector3I(0, 0, 0), new Vector3I(9, 9, 9));
        Check("occupancy_size", occ.Size == new Vector3I(10, 10, 10), "got " + occ.Size);
        occ.Add(piece, new Vector3I(3, 4, 5));
        Check(
            "occupancy_add_solid",
            occ.Solid(new Vector3I(3, 4, 5)) && occ.Solid(new Vector3I(4, 4, 6)) && occ.Solid(new Vector3I(3, 5, 5)),
            "filled voxels not solid");
        Check("occupancy_add_empty_cell", !occ.Solid(new Vector3I(4, 4, 5)), "empty grid cell marked solid");
        Check(
            "occupancy_outside_not_solid",
            !occ.Solid(new Vector3I(-1, 0, 0)) && !occ.Solid(new Vector3I(10, 0, 0)),
            "outside box reported solid");
        occ.Add(piece, new Vector3I(9, 9, 9));
        Check("occupancy_add_clipped", occ.Solid(new Vector3I(9, 9, 9)), "clipped add lost the inside voxel");

        var neg = new Occupancy(new Vector3I(-2, 0, 0), new Vector3I(2, 3, 3));
        neg.Add(piece, new Vector3I(-2, 0, 0));
        Check(
            "occupancy_negative_min",
            neg.Size == new Vector3I(5, 4, 4)
                && neg.Solid(new Vector3I(-2, 0, 0))
                && neg.Solid(new Vector3I(-1, 0, 1))
                && !neg.Solid(new Vector3I(-2, 0, 1)),
            "negative origin wrong");
    }

    private void TestHash()
    {
        uint h1 = VoxelText.Hash(new Vector3I(1, 2, 3), 7u);
        Check("hash_1_2_3_seed7", h1 == 2909141854u, "expected 2909141854, got " + h1);
        uint h2 = VoxelText.Hash(new Vector3I(0, 0, 0), 23u);
        Check("hash_0_0_0_seed23", h2 == 1404512558u, "expected 1404512558, got " + h2);
        uint h3 = VoxelText.Hash(new Vector3I(-4, 5, -6), 99u);
        Check("hash_neg_seed99", h3 == 1477150849u, "expected 1477150849, got " + h3);
    }

    private void TestParseModel()
    {
        string text = FileAccess.GetFileAsString("res://assets/models/crt_computer.txt");
        Check("crt_file_read", text.Length > 0, "file empty or missing");
        VoxelGrid m = VoxelText.ParseModel(text);
        int filled = CountFilled(m);
        GD.Print("crt_computer size=" + m.Size + " filled=" + filled);
        Check("crt_not_empty", filled > 0, "no filled voxels");
        Check("crt_size", m.Size == new Vector3I(7, 6, 5), "expected (7, 6, 5), got " + m.Size);
        Check("crt_filled_count", filled == 97, "expected 97, got " + filled);
    }
}
