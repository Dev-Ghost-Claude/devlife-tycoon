using System;
using DevLife.Voxel;
using Godot;

// Headless checks for glowing palette entries in text models. Run with:
// godot --headless --path . --script res://tests/VoxelTextTests.cs
public partial class VoxelTextTests : SceneTree
{
    private int _failures;

    // Two layers, 3 wide, 2 deep. The top row of a layer is z = 0 (the back), as in the room maps.
    // Glow voxels: (1,0,0), (0,1,0), (2,1,1). Plain voxels: (0,0,0), (2,0,0), (0,1,1).
    private static readonly string MixedModel = string.Join("\n",
        "# two layers, one plain colour and one glowing colour",
        "palette",
        "b = #d8cfb8",
        "g = #9dff9d glow",
        "layer",
        "bgb",
        "...",
        "layer",
        "g..",
        "b.g");

    private static readonly Color GlowColour = Color.FromHtml("#9dff9d");

    public override void _Initialize()
    {
        Run(TestPartsSplit);
        Run(TestParseModelAllVoxels);
        Run(TestNoGlowGivesNullGlow);
        Run(TestCrtComputer);
        Run(TestGlowKeywordIsFlexible);
        Run(TestGlowWithoutColourThrows);
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

    private static bool SameCells(VoxelGrid a, VoxelGrid b)
    {
        if (a.Size != b.Size)
        {
            return false;
        }
        for (int z = 0; z < a.Size.Z; z++)
        {
            for (int y = 0; y < a.Size.Y; y++)
            {
                for (int x = 0; x < a.Size.X; x++)
                {
                    if (a.Get(x, y, z) != b.Get(x, y, z))
                    {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    private void TestPartsSplit()
    {
        var (body, glow) = VoxelText.ParseModelParts(MixedModel);
        Check("parts_glow_present", glow is not null, "Glow was null");
        if (glow is null)
        {
            return;
        }

        Check("parts_same_size", body.Size == new Vector3I(3, 2, 2) && glow.Size == body.Size,
            $"body {body.Size}, glow {glow.Size}");
        Check("parts_body_count", CountFilled(body) == 3, $"body filled {CountFilled(body)}");
        Check("parts_glow_count", CountFilled(glow) == 3, $"glow filled {CountFilled(glow)}");

        Vector3I[] glowCells = { new(1, 0, 0), new(0, 1, 0), new(2, 1, 1) };
        Vector3I[] bodyCells = { new(0, 0, 0), new(2, 0, 0), new(0, 1, 1) };

        bool glowOnlyInGlow = true;
        foreach (Vector3I c in glowCells)
        {
            if (glow.Get(c.X, c.Y, c.Z) is null || body.Get(c.X, c.Y, c.Z) is not null)
            {
                glowOnlyInGlow = false;
            }
        }
        Check("parts_glow_voxels_only_in_glow", glowOnlyInGlow);

        bool bodyOnlyInBody = true;
        foreach (Vector3I c in bodyCells)
        {
            if (body.Get(c.X, c.Y, c.Z) is null || glow.Get(c.X, c.Y, c.Z) is not null)
            {
                bodyOnlyInBody = false;
            }
        }
        Check("parts_plain_voxels_only_in_body", bodyOnlyInBody);

        Check("parts_glow_keeps_colour", glow.Get(1, 0, 0) == GlowColour, $"got {glow.Get(1, 0, 0)}");
    }

    private void TestParseModelAllVoxels()
    {
        VoxelGrid all = VoxelText.ParseModel(MixedModel);
        var (body, glow) = VoxelText.ParseModelParts(MixedModel);
        Check("model_one_grid_same_size", glow is not null && all.Size == new Vector3I(3, 2, 2), $"size {all.Size}");
        Check("model_all_voxels_filled", CountFilled(all) == 6, $"filled {CountFilled(all)}");
        Check("model_equals_body_plus_glow", glow is not null && CountFilled(all) == CountFilled(body) + CountFilled(glow));

        bool glowAsPlainColour = true;
        if (glow is not null)
        {
            for (int z = 0; z < all.Size.Z; z++)
            {
                for (int y = 0; y < all.Size.Y; y++)
                {
                    for (int x = 0; x < all.Size.X; x++)
                    {
                        Color? expected = glow.Get(x, y, z) ?? body.Get(x, y, z);
                        if (all.Get(x, y, z) != expected)
                        {
                            glowAsPlainColour = false;
                        }
                    }
                }
            }
        }
        Check("model_glow_is_plain_colour", glowAsPlainColour);
    }

    private void TestNoGlowGivesNullGlow()
    {
        string plain = string.Join("\n",
            "palette",
            "b = #d8cfb8",
            "k = #6d675c",
            "layer",
            "bb.",
            ".k.",
            "layer",
            "..k",
            "b..");

        var (body, glow) = VoxelText.ParseModelParts(plain);
        Check("no_glow_returns_null", glow is null, "Glow was not null");
        Check("no_glow_body_matches_parse_model", SameCells(body, VoxelText.ParseModel(plain)));
    }

    private void TestCrtComputer()
    {
        string text = FileAccess.GetFileAsString("res://assets/models/crt_computer.txt");
        Check("crt_file_readable", text.Length > 0, "empty or missing file");
        if (text.Length == 0)
        {
            return;
        }

        VoxelGrid grid = VoxelText.ParseModel(text);
        Check("crt_size", grid.Size == new Vector3I(7, 6, 5), $"size {grid.Size}");
        Check("crt_filled", CountFilled(grid) == 97, $"filled {CountFilled(grid)}");

        var (body, glow) = VoxelText.ParseModelParts(text);
        Check("crt_parts_no_glow", glow is null, "Glow was not null");
        Check("crt_parts_body_matches", SameCells(body, grid));
    }

    private void TestGlowKeywordIsFlexible()
    {
        string[] variants =
        {
            "palette\na=#ffffff\nb=#9dff9d glow\nlayer\nab",
            "palette\na=#ffffff\nb  =  #9dff9d     GLOW   \nlayer\nab",
            "palette\na = #ffffff\nb = #9dff9d Glow\nlayer\nab",
        };

        for (int i = 0; i < variants.Length; i++)
        {
            var (body, glow) = VoxelText.ParseModelParts(variants[i]);
            bool ok = glow is not null
                && CountFilled(glow) == 1
                && glow.Get(1, 0, 0) == GlowColour
                && body.Get(0, 0, 0) == Color.FromHtml("#ffffff")
                && CountFilled(body) == 1;
            Check($"glow_keyword_variant_{i}", ok, $"variant {i} did not split as expected");
        }
    }

    private void TestGlowWithoutColourThrows()
    {
        bool threw = false;
        try
        {
            VoxelText.ParseModelParts("palette\nb = glow\nlayer\nb");
        }
        catch (FormatException)
        {
            threw = true;
        }
        Check("glow_without_colour_throws", threw, "no FormatException");
    }
}
