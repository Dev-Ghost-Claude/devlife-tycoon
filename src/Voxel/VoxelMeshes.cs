using Godot;
using Godot.Collections;

namespace DevLife.Voxel;

/// Turns MeshData into Godot meshes, and holds the shared material for voxel meshes.
public static class VoxelMeshes
{
    private static StandardMaterial3D? _material;

    /// One shared material for every voxel mesh, created on first use. Vertex colours are
    /// stored as sRGB (see VoxelMesher), so the material reads them as sRGB too.
    public static StandardMaterial3D Material
    {
        get
        {
            _material ??= new StandardMaterial3D
            {
                VertexColorUseAsAlbedo = true,
                VertexColorIsSrgb = true,
            };
            return _material;
        }
    }

    /// One triangle-list surface holding the arrays of `d`.
    public static ArrayMesh ToArrayMesh(this MeshData d)
    {
        var arrays = new Array();
        arrays.Resize((int)Mesh.ArrayType.Max);
        arrays[(int)Mesh.ArrayType.Vertex] = d.Positions;
        arrays[(int)Mesh.ArrayType.Normal] = d.Normals;
        arrays[(int)Mesh.ArrayType.Color] = d.Colors;
        arrays[(int)Mesh.ArrayType.Index] = d.Indices;

        var mesh = new ArrayMesh();
        mesh.AddSurfaceFromArrays(Mesh.PrimitiveType.Triangles, arrays);
        return mesh;
    }
}
