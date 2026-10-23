using System;
using System.Collections.Generic;
using Godot;

namespace DevLife.Voxel;

/// Turns a VoxelGrid into triangle arrays. Ported from the Bevy build_mesh (reference/bevy/src/voxel.rs).
/// Winding is clockwise, as Godot expects for front faces.
public static class VoxelMesher
{
    private static readonly Vector3I NegY = new(0, -1, 0);

    /// Outward normal of a face, and its four corners as offsets in voxel units (in quad order).
    private sealed record Face(Vector3I Normal, Vector3I[] Corners);

    private static readonly Face[] Faces =
    {
        new Face(new Vector3I(1, 0, 0), new Vector3I[] { new(1, 0, 0), new(1, 1, 0), new(1, 1, 1), new(1, 0, 1) }),
        new Face(new Vector3I(-1, 0, 0), new Vector3I[] { new(0, 0, 1), new(0, 1, 1), new(0, 1, 0), new(0, 0, 0) }),
        new Face(new Vector3I(0, 1, 0), new Vector3I[] { new(0, 1, 0), new(0, 1, 1), new(1, 1, 1), new(1, 1, 0) }),
        new Face(new Vector3I(0, -1, 0), new Vector3I[] { new(0, 0, 0), new(1, 0, 0), new(1, 0, 1), new(0, 0, 1) }),
        new Face(new Vector3I(0, 0, 1), new Vector3I[] { new(1, 0, 1), new(1, 1, 1), new(0, 1, 1), new(0, 0, 1) }),
        new Face(new Vector3I(0, 0, -1), new Vector3I[] { new(0, 0, 0), new(0, 1, 0), new(1, 1, 0), new(1, 0, 0) }),
    };

    /// Mesh with the grid's own voxels as the only shading source. The origin is the grid point
    /// that lands on the mesh's local origin.
    public static MeshData Mesh(this VoxelGrid g, Vector3 origin)
    {
        return g.MeshSized(origin, Voxels.Voxel);
    }

    /// Like Mesh, with voxels of edge `size` metres.
    public static MeshData MeshSized(this VoxelGrid g, Vector3 origin, float size)
    {
        return g.MeshLit(origin, size, Filled(g));
    }

    /// Like MeshSized, with corner shading from everything `solid` reports, in this grid's coordinates.
    /// Faces pointing straight down are left out.
    public static MeshData MeshLit(this VoxelGrid g, Vector3 origin, float size, Func<Vector3I, bool> solid)
    {
        return Build(g, origin, size, solid, bottoms: false);
    }

    /// Like MeshSized, keeping the faces that point down (for parts that can swing and show their underside).
    public static MeshData MeshMoving(this VoxelGrid g, Vector3 origin, float size)
    {
        return Build(g, origin, size, Filled(g), bottoms: true);
    }

    /// Mesh with the origin at the bottom centre of the grid: where a model stands.
    public static MeshData MeshStanding(this VoxelGrid g)
    {
        Vector3 origin = new(g.Size.X / 2f, 0f, g.Size.Z / 2f);
        return g.Mesh(origin);
    }

    /// How far corner shading reaches, in voxels.
    private const float ShadeReach = 4f;
    /// How dark a fully enclosed corner gets (0 = no shading, 1 = black).
    private const float ShadeDepth = 0.62f;

    /// Number of quads split along the flipped diagonal in the last Build call (test hook).
    [ThreadStatic]
    private static int _lastFlippedCount;

    internal static int LastFlippedCount => _lastFlippedCount;

    /// Light reaching a mesh corner `at` (a grid point) on a face pointing along `n`: rays go
    /// out over the face's half of the sky and count what they hit, nearer hits counting more.
    /// This is the soft shading in creases and under furniture that makes voxel scenes look
    /// solid; it reaches a few voxels, like light bouncing in a real room.
    internal static float Ambient(Func<Vector3I, bool> solid, Vector3I at, Vector3I n)
    {
        var normal = new Vector3(n.X, n.Y, n.Z);
        Vector3 t1;
        Vector3 t2;
        if (Math.Abs(n.X) == 1)
        {
            t1 = new Vector3(0f, 1f, 0f);
            t2 = new Vector3(0f, 0f, 1f);
        }
        else if (Math.Abs(n.Y) == 1)
        {
            t1 = new Vector3(1f, 0f, 0f);
            t2 = new Vector3(0f, 0f, 1f);
        }
        else
        {
            t1 = new Vector3(1f, 0f, 0f);
            t2 = new Vector3(0f, 1f, 0f);
        }
        Vector3 start = new Vector3(at.X, at.Y, at.Z) + normal * 0.51f;
        float blocked = 0f;
        float total = 0f;
        for (int a = -2; a <= 2; a++)
        {
            for (int b = -2; b <= 2; b++)
            {
                for (int c = 1; c <= 2; c++)
                {
                    Vector3 dir = (t1 * (float)a + t2 * (float)b + normal * (float)c).Normalized();
                    // Rays near the face count less, like light arriving at a glancing angle.
                    float weight = dir.Dot(normal);
                    total += weight;
                    float t = 0.6f;
                    while (t <= ShadeReach)
                    {
                        Vector3 p = start + dir * t;
                        var cell = new Vector3I(Mathf.FloorToInt(p.X), Mathf.FloorToInt(p.Y), Mathf.FloorToInt(p.Z));
                        if (solid(cell))
                        {
                            blocked += weight * (1f - t / (ShadeReach + 1f));
                            break;
                        }
                        t += 0.8f;
                    }
                }
            }
        }
        return 1f - ShadeDepth * Math.Min(blocked / total * 1.8f, 1f);
    }

    internal static MeshData Build(VoxelGrid g, Vector3 origin, float size, Func<Vector3I, bool> solid, bool bottoms)
    {
        Func<Vector3I, bool> open = OpenSpace(g);
        var positions = new List<Vector3>();
        var normals = new List<Vector3>();
        var colors = new List<Color>();
        var indices = new List<int>();
        // A corner shared by neighbouring faces that look the same way and have the same colour is
        // stored once. The key holds the three float bits of the linear colour.
        var shared = new Dictionary<(Vector3I At, int Face, int PaintR, int PaintG, int PaintB), int>();
        // Corner shading, cached per (corner, face normal) like the Rust shade_cache.
        var shadeCache = new Dictionary<(Vector3I At, Vector3I N), float>();
        var shades = new float[4];
        var corners = new int[4];
        int flipped = 0;

        Vector3I s = g.Size;
        for (int z = 0; z < s.Z; z++)
        {
            for (int y = 0; y < s.Y; y++)
            {
                for (int x = 0; x < s.X; x++)
                {
                    if (g.Get(x, y, z) is not Color colour)
                    {
                        continue;
                    }
                    Color lin = colour.SrgbToLinear();
                    int paintR = BitConverter.SingleToInt32Bits(lin.R);
                    int paintG = BitConverter.SingleToInt32Bits(lin.G);
                    int paintB = BitConverter.SingleToInt32Bits(lin.B);
                    var v = new Vector3I(x, y, z);
                    for (int f = 0; f < Faces.Length; f++)
                    {
                        Face face = Faces[f];
                        Vector3I n = face.Normal;
                        if ((n == NegY && !bottoms) || !open(v + n))
                        {
                            continue;
                        }
                        for (int i = 0; i < 4; i++)
                        {
                            Vector3I at = v + face.Corners[i];
                            if (!shadeCache.TryGetValue((at, n), out float shade))
                            {
                                shade = Ambient(solid, at, n);
                                shadeCache[(at, n)] = shade;
                            }
                            shades[i] = shade;
                            var key = (at, f, paintR, paintG, paintB);
                            if (!shared.TryGetValue(key, out int index))
                            {
                                index = positions.Count;
                                // Grid z grows toward the back, which is world +Z.
                                positions.Add((new Vector3(at.X, at.Y, at.Z) - origin) * size);
                                normals.Add(new Vector3(n.X, n.Y, n.Z));
                                Color shaded = new Color(lin.R * shade, lin.G * shade, lin.B * shade, 1f);
                                // Godot stores vertex colours as 8 bits per channel, so they are stored as
                                // sRGB and the material uses vertex_color_is_srgb = true. The maths above
                                // (shading) is done in linear light.
                                colors.Add(shaded.LinearToSrgb());
                                shared.Add(key, index);
                            }
                            corners[i] = index;
                        }
                        // Split the quad along the diagonal that keeps the shading even.
                        int a = corners[0], b = corners[1], c = corners[2], d = corners[3];
                        if (shades[0] + shades[2] >= shades[1] + shades[3])
                        {
                            indices.AddRange(new[] { a, c, b, a, d, c });
                        }
                        else
                        {
                            indices.AddRange(new[] { b, d, c, b, a, d });
                            flipped++;
                        }
                    }
                }
            }
        }

        _lastFlippedCount = flipped;
        return new MeshData(positions.ToArray(), normals.ToArray(), colors.ToArray(), indices.ToArray());
    }

    /// Which empty cells connect to the space around the model; cells beyond the grid all do.
    /// A face looking into a sealed pocket can never be seen, so it is left out.
    internal static Func<Vector3I, bool> OpenSpace(VoxelGrid g)
    {
        Vector3I s = g.Size;
        // One cell of padding all round, so the flood can get around the whole model.
        Vector3I padded = s + new Vector3I(2, 2, 2);
        Vector3I start = new(-1, -1, -1);
        int Index(Vector3I q) => (q.X + 1) + padded.X * ((q.Y + 1) + padded.Y * (q.Z + 1));
        bool Outside(Vector3I q) => q.X < start.X || q.Y < start.Y || q.Z < start.Z
            || q.X > s.X || q.Y > s.Y || q.Z > s.Z;

        var reachable = new bool[padded.X * padded.Y * padded.Z];
        reachable[Index(start)] = true;
        var stack = new Stack<Vector3I>();
        stack.Push(start);
        while (stack.Count > 0)
        {
            Vector3I cell = stack.Pop();
            foreach (Face face in Faces)
            {
                Vector3I r = cell + face.Normal;
                if (Outside(r))
                {
                    continue;
                }
                int i = Index(r);
                if (!reachable[i] && g.Get(r.X, r.Y, r.Z) is null)
                {
                    reachable[i] = true;
                    stack.Push(r);
                }
            }
        }
        return p => Outside(p) || reachable[Index(p)];
    }

    private static Func<Vector3I, bool> Filled(VoxelGrid g)
    {
        return p => g.Get(p.X, p.Y, p.Z) is not null;
    }
}
