# Which engine for DevLife Tycoon, built by AI agents? (2026-10-15)

## Why we are leaving Bevy

On 2026-10-13 and 2026-10-15 the Bevy build froze the Mac (Apple M5, macOS 27.0) and needed a forced restart. The cause was not memory, which was normal (about 0.7 GB):
- macOS logged six GPU crashes from bevy-game on 2026-10-14, each "BIF0 page fault": the GPU read memory at address 0 or other invalid addresses while running the game's draw commands.
- On 2026-10-15 the GPU stopped finishing frames. The game's watchdog killed it, but the graphics driver hung tearing it down, and WindowServer's watchdog fired.

Bevy 0.20's renderer (wgpu 30 on Metal), on a brand-new OS and GPU, issued invalid GPU memory reads, and no safety layer in our code can stop that. Bevy is young, and its Mac path gets far less real-world use than the big engines'.

## Verdict

**Godot 4 (4.7.2, the .NET edition with C#) suits this game and our way of working best.** Unity 6 is a close second. Unreal 5 is a poor fit.

## What matters when AI agents build everything

1. **Everything as text.** Agents read and write files. Scenes, resources, scripts and shaders should be plain, diffable text that can be edited without opening an editor.
2. **Verify without rendering.** After two freezes, as much checking as possible has to happen without the GPU: compiling, headless runs, tests, exports from the command line.
3. **Errors caught early.** Static types and a compiler catch an agent's mistakes before anything runs.
4. **Agent tooling.** MCP servers that let an agent inspect scenes, run the game and take screenshots.
5. **Stable on this Mac.** A mature Metal renderer that is heavily used on Apple Silicon.
6. **Fit for the game.** An orthographic isometric voxel diorama, procedural voxel models, soft lighting and an outline post-process. Nothing needs cutting-edge rendering.
7. **Cost, size, licence.**

## Comparison

| | Godot 4.7 | Unity 6 | Unreal 5 |
|---|---|---|---|
| Project files | All text: .tscn scenes, .tres resources, .gd/.cs scripts, .gdshader | Text (YAML), but scenes and prefabs are full of GUIDs and every asset has a .meta file; editable but brittle for agents | Blueprints and most assets are binary .uasset files; agents need plugins to touch them |
| Headless / CLI | `godot --headless`: import, run scripts and tests, export | Batch mode works, but most workflows assume the editor is open | Command-line builds exist; very heavy |
| Errors caught early | C#: compile-time types (`dotnet build`); GDScript: typed, checked on load | C#: compile-time types | C++: compile-time, but slow builds |
| Agent tooling (MCP) | Several community servers (e.g. GodotIQ, beckett-godot-mcp): scene tree, scripts, screenshots | **Official** MCP server (Unity AI open beta, May 2026), but tied to Unity Cloud and paid credits after a trial on the free tier | Several community servers for Blueprints; none official |
| Mac / Metal | Native Metal renderer on Apple Silicon since 4.4 | Mature Metal support, widely used | Works, but the Mac is a second-class platform |
| Install | ~190 MB (.NET edition) plus the .NET SDK | Several GB via Unity Hub, plus an account | Tens of GB |
| Licence / cost | MIT, free forever | Free tier with revenue limits; account required; a history of licence changes | Free until $1M revenue, then a 5% royalty |
| Fit for this game | Very good: orthographic camera, ArrayMesh for voxels, glow, tonemapping, SSAO, shadows, screen shaders for outlines; Voxel Tools add-on | Very good, huge asset store | Overkill for an isometric tycoon |

## Why Godot over Unity, for us

- **Agents can own the whole project.** Godot's text formats are simple enough that an agent can write a scene by hand. In Unity, agents usually work through the editor via MCP, because hand-editing scenes and prefabs is fragile.
- **Checks without the GPU.** With C#, `dotnet build` catches type errors and headless Godot can run logic and tests, so most work is verified before anything is drawn. That is the safety lesson of this week.
- **Small, free, no accounts.** The install is ~190 MB, the licence is MIT, and there's no Unity Cloud project or credits.
- **Your Voxel Tools link fits.** It is a Godot add-on for voxel meshing with shading and for scattering small details (grass tufts, flowers), the cure for the "golf course" lawns.

What Unity does better: an official MCP server, the largest asset store, and the most material for models to learn from. If Godot ever blocks us, Unity is the fallback.

## Risks with Godot and how we handle them

- **AI models sometimes mix up Godot 3 and 4 APIs.** We pin Godot 4.7 docs in prompts, use typed code, and compile on every change.
- **The MCP servers are community-made.** We vet each before connecting it, and work from files and the CLI first.
- **Any engine can still hit a driver bug on a brand-new OS.** We keep the safety rules: one run at a time, check `/Library/Logs/DiagnosticReports` for GPU events after every run, verify headless first, and do the first rendered run only with your OK.

## GDScript or C#?

- **C# (recommended):** a compiler catches agent mistakes without running the game, and it is fast enough to build the voxel models (meshing with corner shading over hundreds of thousands of voxels) at load. It needs the .NET edition (191.5 MB) and the .NET SDK.
- **GDScript:** Godot's own language. It's simpler, has the most examples and needs no extra SDK, but it's much slower for heavy meshing (we would bake meshes ahead of time), and more mistakes surface only when the code runs.

## Porting plan (outline)

1. Project skeleton: Godot 4.7, C#, a folder layout, the safety rules carried over.
2. Voxel pipeline: VoxelGrid and the mesher (corner shading, faces sealed inside models left out, shared corners), with meshes cached and shared.
3. Look: orthographic isometric camera, soft light, filmic tonemapping, glow, soft shadows, the outline pass.
4. World: street, houses (with the cut-open bedroom), shop, props, the four animals with expressions.
5. Gameplay: clock, areas, click-to-walk, actions, picking.
6. Capture and checks: headless tests; guarded screenshot runs only with your OK.

Each step is done by a worker agent at high effort: Haiku 5.5 for code, Sonnet 5.5 for art. Claude reviews the work, and hands it to a fresh agent with a summary when the worker's context reaches 85K for Haiku (prompts over 100K cost 5× as much) or 850K for Sonnet (updated 2026-10-16; was xhigh and 90k).

## Sources

- [Godot MCP explainer and server comparison](https://www.summerengine.com/blog/what-is-godot-mcp) (vendor blog; details to verify per repo)
- [GodotIQ MCP server](https://vibeindex.ai/mcp/salvo10f/godotiq), [beckett-godot-mcp](https://enterprisedna.co/directories/mcp/beckettlab-beckett-godot-mcp)
- [Unity AI open beta and its MCP server](https://gamedev.net/news/4376-unity-ai-open-beta-how-to-get-started-with-mcp/), [coverage of the May 2026 beta](https://app.cinevva.com/news/2026-05-04-unity-ai-open-beta)
- [Unreal MCP servers: ziggymar/unreal-mcp](https://mcpservers.org/servers/ziggymar/unreal-mcp), [remiphilippe/mcp-unreal](https://mcpservers.org/servers/remiphilippe/mcp-unreal)
- [Godot's native Metal backend (4.4 dev 1 announcement)](https://godotengine.org/article/dev-snapshot-godot-4-4-dev-1/)
- [Godot 4.7.2 stable release (GitHub)](https://github.com/godotengine/godot/releases/tag/4.7.2-stable)
- macOS diagnostic reports on this Mac: `gpuEvent-bevy-game-2026-10-14-*.ips`, `WindowServer-2026-10-15-082922.ips`
