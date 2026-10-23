using System;
using Godot;

namespace DevLife.Voxel;

/// A dense box of optional colours. Axes: x left to right, y bottom to top, z front (0) to back.
public sealed class VoxelGrid
{
    public Vector3I Size { get; }
    private readonly Color?[] _cells;

    public VoxelGrid(int x, int y, int z)
    {
        Size = new Vector3I(x, y, z);
        _cells = new Color?[x * y * z];
    }

    /// Index of (x, y, z) in the cell array, or -1 when outside the grid.
    private int IndexOf(int x, int y, int z)
    {
        if (x < 0 || y < 0 || z < 0 || x >= Size.X || y >= Size.Y || z >= Size.Z)
        {
            return -1;
        }
        return x + Size.X * (y + Size.Y * z);
    }

    /// The colour at (x, y, z), or null when empty or outside the grid.
    public Color? Get(int x, int y, int z)
    {
        int i = IndexOf(x, y, z);
        return i < 0 ? null : _cells[i];
    }

    /// Sets a cell. Positions outside the grid are ignored.
    public void Set(int x, int y, int z, Color? color)
    {
        int i = IndexOf(x, y, z);
        if (i >= 0)
        {
            _cells[i] = color;
        }
    }

    /// Fills the box from min to max, both inclusive.
    public void Fill(Vector3I min, Vector3I max, Color colour)
    {
        FillWith(min, max, _ => colour);
    }

    /// Clears the box from min to max, both inclusive.
    public void Clear(Vector3I min, Vector3I max)
    {
        FillWith(min, max, _ => null);
    }

    /// Sets each cell of the box to whatever paint returns for its position.
    public void FillWith(Vector3I min, Vector3I max, Func<Vector3I, Color?> paint)
    {
        for (int z = min.Z; z <= max.Z; z++)
        {
            for (int y = min.Y; y <= max.Y; y++)
            {
                for (int x = min.X; x <= max.X; x++)
                {
                    Set(x, y, z, paint(new Vector3I(x, y, z)));
                }
            }
        }
    }

    /// FNV-1a hash of the size, origin, voxel size and every cell (null as one marker byte,
    /// a colour as a marker byte then its R, G, B, A float bits): the key for mesh caching.
    public ulong Fingerprint(Vector3 origin, float size)
    {
        unchecked
        {
            ulong h = 14695981039346656037UL;
            h = FnvWord(h, (uint)Size.X);
            h = FnvWord(h, (uint)Size.Y);
            h = FnvWord(h, (uint)Size.Z);
            h = FnvWord(h, BitConverter.SingleToUInt32Bits(origin.X));
            h = FnvWord(h, BitConverter.SingleToUInt32Bits(origin.Y));
            h = FnvWord(h, BitConverter.SingleToUInt32Bits(origin.Z));
            h = FnvWord(h, BitConverter.SingleToUInt32Bits(size));
            foreach (Color? cell in _cells)
            {
                if (cell is not Color c)
                {
                    h = FnvByte(h, 0);
                    continue;
                }
                h = FnvByte(h, 1);
                h = FnvWord(h, BitConverter.SingleToUInt32Bits(c.R));
                h = FnvWord(h, BitConverter.SingleToUInt32Bits(c.G));
                h = FnvWord(h, BitConverter.SingleToUInt32Bits(c.B));
                h = FnvWord(h, BitConverter.SingleToUInt32Bits(c.A));
            }
            return h;
        }
    }

    private static ulong FnvWord(ulong h, uint word)
    {
        for (int i = 0; i < 4; i++)
        {
            h = FnvByte(h, (byte)(word >> (8 * i)));
        }
        return h;
    }

    private static ulong FnvByte(ulong h, byte b)
    {
        unchecked
        {
            return (h ^ b) * 1099511628211UL;
        }
    }

    /// The smallest grid holding all the filled voxels, and where its corner sat in this one.
    /// An empty 0x0x0 grid with a zero offset when nothing is filled.
    public (VoxelGrid Grid, Vector3I Offset) Cropped()
    {
        int minX = Size.X, minY = Size.Y, minZ = Size.Z;
        int maxX = -1, maxY = -1, maxZ = -1;
        for (int z = 0; z < Size.Z; z++)
        {
            for (int y = 0; y < Size.Y; y++)
            {
                for (int x = 0; x < Size.X; x++)
                {
                    if (Get(x, y, z) is null)
                    {
                        continue;
                    }
                    minX = Math.Min(minX, x);
                    minY = Math.Min(minY, y);
                    minZ = Math.Min(minZ, z);
                    maxX = Math.Max(maxX, x);
                    maxY = Math.Max(maxY, y);
                    maxZ = Math.Max(maxZ, z);
                }
            }
        }
        if (maxX < 0)
        {
            return (new VoxelGrid(0, 0, 0), Vector3I.Zero);
        }

        var grid = new VoxelGrid(maxX - minX + 1, maxY - minY + 1, maxZ - minZ + 1);
        for (int z = 0; z < grid.Size.Z; z++)
        {
            for (int y = 0; y < grid.Size.Y; y++)
            {
                for (int x = 0; x < grid.Size.X; x++)
                {
                    grid.Set(x, y, z, Get(minX + x, minY + y, minZ + z));
                }
            }
        }
        return (grid, new Vector3I(minX, minY, minZ));
    }
}
