using System;
using System.Collections.Generic;
using Godot;

namespace DevLife.Voxel;

/// Text models and the deterministic colour noise shared with the Bevy version.
public static class VoxelText
{
    /// Parses a small text model. Layers run bottom to top; inside a layer, rows run back
    /// (top row, like north on a map) to front, and characters left to right. '.' is empty;
    /// other characters come from the palette. '#' lines without '=' are comments.
    /// A palette colour may end with the word `glow` (any case); here glowing voxels are
    /// plain colours in the one grid. Use ParseModelParts to get them separately.
    public static VoxelGrid ParseModel(string text)
    {
        return Parse(text, separateGlow: false).Body;
    }

    /// Parses a model like ParseModel, but splits out the voxels whose palette entry ends with
    /// `glow`. Body holds every other voxel; Glow holds only the glowing ones, or is null when
    /// the palette has no glow entry. Both grids have the same size.
    public static (VoxelGrid Body, VoxelGrid? Glow) ParseModelParts(string text)
    {
        return Parse(text, separateGlow: true);
    }

    private static (VoxelGrid Body, VoxelGrid? Glow) Parse(string text, bool separateGlow)
    {
        var palette = new Dictionary<char, (Color Colour, bool Glow)>();
        var layers = new List<List<string>>();
        bool inPalette = false;
        bool hasGlowEntry = false;

        foreach (string raw in text.Split('\n'))
        {
            string line = raw.Trim();
            if (line.Length == 0 || (line.StartsWith('#') && !line.Contains('=')))
            {
                continue;
            }

            if (line == "palette")
            {
                inPalette = true;
            }
            else if (line == "layer")
            {
                inPalette = false;
                layers.Add(new List<string>());
            }
            else if (inPalette)
            {
                int eq = line.IndexOf('=');
                if (eq < 0)
                {
                    throw new FormatException($"palette line must be `k = #rrggbb`: {line}");
                }
                string keyText = line[..eq].Trim();
                if (keyText.Length == 0)
                {
                    throw new FormatException($"palette line has no key: {line}");
                }
                string value = line[(eq + 1)..].Trim();
                bool isGlow = value.EndsWith("glow", StringComparison.OrdinalIgnoreCase);
                if (isGlow)
                {
                    value = value[..^4].Trim();
                    if (value.Length == 0)
                    {
                        throw new FormatException($"palette line needs a colour before `glow`: {line}");
                    }
                    hasGlowEntry = true;
                }
                palette[keyText[0]] = (Color.FromHtml(value), isGlow);
            }
            else
            {
                if (layers.Count == 0)
                {
                    throw new FormatException("rows before the first `layer`");
                }
                layers[^1].Add(line);
            }
        }

        int depth = 0;
        int width = 0;
        foreach (List<string> rows in layers)
        {
            depth = Math.Max(depth, rows.Count);
            foreach (string row in rows)
            {
                width = Math.Max(width, row.Length);
            }
        }

        var body = new VoxelGrid(width, layers.Count, depth);
        VoxelGrid? glowGrid = separateGlow && hasGlowEntry ? new VoxelGrid(width, layers.Count, depth) : null;
        for (int y = 0; y < layers.Count; y++)
        {
            List<string> rows = layers[y];
            for (int row = 0; row < rows.Count; row++)
            {
                string r = rows[row];
                // Each layer reads as a top-down map, like the room maps: the top row is the back (north, low z),
                // the bottom row the front (south, high z, toward the default camera), columns west to east (+x).
                // Bevy flipped z here (depth - 1 - row), which mirrored every model left to right (fixed 2026-10-19).
                int z = row;
                for (int x = 0; x < r.Length; x++)
                {
                    char ch = r[x];
                    if (ch == '.')
                    {
                        continue;
                    }
                    if (!palette.TryGetValue(ch, out var entry))
                    {
                        throw new FormatException($"no palette entry for '{ch}'");
                    }
                    VoxelGrid target = entry.Glow && glowGrid is not null ? glowGrid : body;
                    target.Set(x, y, z, entry.Colour);
                }
            }
        }
        return (body, glowGrid);
    }

    /// A tiny deterministic hash for colour noise: same position, same result.
    public static uint Hash(Vector3I p, uint seed)
    {
        unchecked
        {
            uint h = ((uint)p.X * 0x8da6b343u)
                ^ ((uint)p.Y * 0xd8163841u)
                ^ ((uint)p.Z * 0xcb1ab31fu)
                ^ (seed * 0x165667b1u);
            h ^= h >> 13;
            h *= 0x5bd1e995u;
            return h ^ (h >> 15);
        }
    }

    /// Picks one of colours by position, for speckled surfaces like grass or brick.
    public static Color Speckle(Vector3I p, uint seed, Color[] colours)
    {
        return colours[(int)(Hash(p, seed) % (uint)colours.Length)];
    }
}
