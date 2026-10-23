using Godot;

namespace DevLife.Voxel;

/// Plain triangle-list arrays produced by VoxelMesher. Colours are stored as sRGB (shaded in linear first), with alpha 1.
public sealed class MeshData
{
    public MeshData(Vector3[] positions, Vector3[] normals, Color[] colors, int[] indices)
    {
        Positions = positions;
        Normals = normals;
        Colors = colors;
        Indices = indices;
    }

    public Vector3[] Positions { get; }

    public Vector3[] Normals { get; }

    public Color[] Colors { get; }

    public int[] Indices { get; }
}
