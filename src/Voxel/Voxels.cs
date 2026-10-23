namespace DevLife.Voxel;

public static class Voxels
{
    /// Edge length of one voxel in world units (metres): 8 voxels to the metre.
    public const float Voxel = 0.125f;

    /// Fine voxels for detailed models: 24 to the metre, one canvas pixel each at zoom 1.
    public const float Fine = 1f / 24f;
}
