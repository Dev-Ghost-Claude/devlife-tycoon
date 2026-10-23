using Godot;

namespace DevLife.Voxel;

/// Which scene voxels are solid, over the box from Min to Min + Size - 1.
public sealed class Occupancy
{
    public Vector3I Min { get; }
    public Vector3I Size { get; }
    private readonly bool[] _cells;

    public Occupancy(Vector3I min, Vector3I max)
    {
        Min = min;
        Size = max - min + Vector3I.One;
        _cells = new bool[Size.X * Size.Y * Size.Z];
    }

    /// Index of scene voxel p in the cell array, or -1 when outside the box.
    private int IndexOf(Vector3I p)
    {
        Vector3I q = p - Min;
        if (q.X < 0 || q.Y < 0 || q.Z < 0 || q.X >= Size.X || q.Y >= Size.Y || q.Z >= Size.Z)
        {
            return -1;
        }
        return q.X + Size.X * (q.Y + Size.Y * q.Z);
    }

    /// Marks every filled voxel of grid, placed with its grid origin at scene voxel at.
    public void Add(VoxelGrid grid, Vector3I at)
    {
        Vector3I s = grid.Size;
        for (int z = 0; z < s.Z; z++)
        {
            for (int y = 0; y < s.Y; y++)
            {
                for (int x = 0; x < s.X; x++)
                {
                    if (grid.Get(x, y, z) is null)
                    {
                        continue;
                    }
                    int i = IndexOf(at + new Vector3I(x, y, z));
                    if (i >= 0)
                    {
                        _cells[i] = true;
                    }
                }
            }
        }
    }

    public bool Solid(Vector3I p)
    {
        int i = IndexOf(p);
        return i >= 0 && _cells[i];
    }
}
