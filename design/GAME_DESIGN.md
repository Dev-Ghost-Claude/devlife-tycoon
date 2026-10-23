# Game design

The user is the designer; Claude builds everything (code, engine, art). This file holds the user's decisions. Each section says when it was decided. Open questions are at the end.

## The pitch (2026-10-11)

A game-dev tycoon with extreme, in-depth intricacies. You play an average joe who "woke up one day and chose violence" and now wants to make games. It starts with nothing and climbs, slowly and painfully, to a studio with capital-market presence.

**Tone:** like Dark Souls: punishing, yet rewarding.

## The world (quiz 2026-10-11)

- **A world map:** many cities and countries, each with its own market, tastes, laws and ratings boards. You live in one fully built home city; other cities are smaller but walkable when you travel (trade shows, publisher HQs, outsourcing studios). You can relocate your studio later. (Build order: the home city first.)
- **Workplaces grow** bedroom → garage → rented office → tower floors → your own campus. You place desks, servers and rooms yourself; layout affects noise, morale and secrets.
- **City people live on schedules** (commute, lunch, bars). You meet them by being in the right place at the right time; that is how you meet friends and future hires.
- **Day, night and weather matter** for gameplay and mood (shops close at night, rain empties the streets, a heatwave melts the server room) and change the pixel-art lighting.
- **City districts:** suburbs (family home, high school, corner computer shop, arcade), campus town (college, dorms, cafés, bars, game-dev club), downtown (publisher towers, banks, law firms, newspapers, stock exchange), tech park and docks (hardware makers, office space, warehouses, server farms, shady deals at night).
- **Transport by era and money:** walk, then bike, bus, car; travel takes clock time, so where you live and work matters.
- **Cities change visibly across eras:** arcades close, internet cafés open, the computer shop becomes a phone store, the docks become a tech park.

## Interface (quiz 2026-10-12)

Based on the Sonnet designer's UI proposal (design/ui/UI_PROPOSAL.md and the mockups in design/ui/mockups/).

- **Title screen:** the cassette tape deck first (your life is a tape); later the bedroom at night becomes the title, with the deck as a prop on the desk.
- **Saving:** manual save slots, with an optional Ironman box at new-life setup.
- **Lives:** one life in progress at a time. A new life starts when the old one ends or you walk away from it (it enters the Hall of fame as "walked away").
- **The Game Master in menus:** whispers only. One quiet line on the title, pause and post-mortem screens; it never touches buttons or Settings.
- **No redeal:** your character and home are dealt once; the seed is shown so friends can compare deals.
- **Reading stops the clock:** every reading screen (character sheet, journal, computer, phone, the paper) pauses time; only actions cost time.
- **Scene picks** show sure costs (time, money, stamina) and requirements only, never odds. Danger shows as omens in the world.
- **HUD numbers:** left to Claude, scaled to how much UI there is. Starting point: money as a number, everything else as bars and icons with exact numbers on hover and in the character sheet.
- **Typefaces:** custom-made pixel fonts (Claude draws them; the user: "now you have the exact tools you can make your customs as well"), at least per era for headings, logos and the clock; open-licence fonts are the fallback. Source Sans 3 stays for long body text until the custom faces exist.
- **Detail target (2026-10-12):** the user shared an isometric pixel-art interior (multi-level floor, display cases, dense shelving, worn floors, warm lamps, ~35-pixel hand-shaded characters) as "the level of detail i want in the game wrt items, animals, world design".

## The district map (2026-10-12)

Inspired by a tabletop-diorama terrain map the user shared (its layout and readability, not its painterly art). Discussed with Fable.

- **One compact home district:** home, the era office, cafés, the computer shop, the arcade, the municipality office (licences and permits), a bank branch and a bus stop, all walkable at street level. College on the hill and downtown across the river are trips. This folds the earlier four-district plan into a home district plus trip destinations.
- **The map is the city itself, zoomed out:** a farthest wheel-zoom of the real voxel world, generated automatically at low detail from the same text maps (buildings become boxes with roof colours, trees become dots). Never a separately painted map.
- **Layout:** one strong diagonal (a river) and two or three big landmarks (hill, coast), lots of quiet ground. Which side of the river you live on decides commute times.
- **Miniature feel without blur:** a steeper camera at map zoom, the city on a cut model base (earth cross-section at the edges), crisp shadows, colours that fade toward the top. No tilt-shift blur: it fights pixel art.
- **Plaques:** places are marked by small standing pixel plaques in the scene (one size, one style, coloured pins by kind of place), about ten on screen at most, filtered by zoom. Their style changes with the eras (typed card → laminated → printout → phone pin); the buildings themselves change in the model (arcade → internet café → phone store).
- **Clicking a place:** hover shows name, travel time and cost; a confirm click travels there by the era's transport with the clock running (via bus stop or station, where encounters can happen). No free teleport.
- **Discovery:** big public landmarks (school, bank, municipality, station) are labelled from the start; small, new or shady places appear once you find them on foot or hear of them.

## The street rebuild (quiz 2026-10-13)

- **Town flavour: a fictional blend.** Our own town, taking the most charming bits from several countries (red pillar boxes, porches, diners, terraces). Cars drive on the right.
- **First rebuilt block: the home house only.** The computer shop and the flats keep their old models until their turn (they get the new lighting).
- **A main street:** two lanes, parking on both sides, wide pavements, a zebra crossing, parked 1970s cars (the art direction's 16-tile section).
- **All four animals now** (fox, rabbit, raccoon, bear) in the dot-eye style, used on the street and in rooms alike.

## More outside detail (user, 2026-10-13)

"Can you add more details to the outside environment?" Three parts, all wanted:
- **The street itself:** denser street life (flower boxes, washing lines, litter and leaves, signs, bikes, birds, worn paths, back gardens, telephone poles and wires, a varied skyline of background houses).
- **Around the room:** inside, the room no longer floats in a dark void: the house is shown around it as a cutaway (walls down to the garden, the ground floor below, the garden, fence and neighbours around).
- **Through the window:** the bedroom window shows the outside (sky, the neighbour's roof, a tree, a street lamp), changing with the time of day.

## Your bedroom inside the house (user, 2026-10-15)

The user flagged the bedroom as "graphically bad": it overlapped the house's walls and read as a deck on a stump (the whole upper storey and roof were hidden). The upstairs placement had been Claude's call (two-storey street houses; a teen in 1978 lives upstairs with the parents below; the window looks over the rooftops), never asked; the user kept it after hearing why.
- **Upstairs, cut open like a dollhouse:** your house's walls stand one voxel outside its 6 x 6 tile footprint, so the bedroom and landing fill it exactly and the house's walls are the room's walls (render outside, wallpaper inside). The south and east upstairs walls are cut down low.
- **The roof stays, cut at the front corner** (references below): the back slope, the west gable, the ridge and the chimney with its aerial stay; the front slope, the east gable and the loft go. Seen from our camera the back slope is edge-on (it is exactly as steep as the view), so what shows is the gable, the ridge line and the chimney.
- **Bold cut edges:** every cut face (wall tops, wall ends, the roof's section) is white.
- **The top of the stairs** replaces the door to nowhere. The house is 6 x 6 tiles and the room 6 x 5, so a 1 m landing runs along the front: a low partition with a doorway in the bedroom's front-west corner, the stairwell at the landing's west end (steps with a red runner going down into a well in the ground floor, a banister on its east side), the old red runner rug along the rest.
- **Later:** paths to the other houses (the back row has none).
- **Room hue (user, 2026-10-19):** warm mustard and teak. Ochre and mustard walls, teak and walnut furniture, cream accents, with shadows tinted toward the hue. The computer's cyan screen and red tape deck are the accents that pop. The bedroom art is made by Opus 5.5 (the user's choice, after its computer won), prop by prop at fine voxels and checked on the voxel desk.
- **The desk lamp (user, 2026-10-19):** the player can switch it on and off. Off, the computer's screen is the only light at night; on, the anglepoise throws a warm amber pool over the desk against the cool cyan screen. The art keeps the lamp's glowing bulb as a separate overlay model, so both looks exist.
- References (2026-10-15, Pinterest and image search): a two-storey isometric house cutaway with the roof kept (dreamstime), a brownstone cut at the corner in its street (Pinterest), a voxel "house doll" with stairs between floors (Virginie Paulus, via Pinterest), voxel rooms with white cut wall tops, attic bedrooms with sloped ceilings.

## Facial expressions (quiz 2026-10-13)

- **Eyes and brows only, no mouth** (a mouth risks the uncanny look the user rejected). Expressions change the eye shapes (dots, arcs, lines, wide eyes) and add small brows.
- **The set:** neutral with a blink (always), happy, tired/sleepy, angry/stressed, surprised/worried, puzzled, melancholy.
- **Mood + moments:** the character's mood and stamina set the resting face; events (a line of dialogue, news, a sale, a bad review) flash a reaction for a few seconds, then it settles back.

## Look and controls (2026-10-11)

- Isometric. Built in 3D from voxels, rendered at low resolution and scaled up with sharp pixels, so it reads as pixel art.
- Characters are anthropomorphic animals (in the spirit of Zootopia; never Disney's own characters or designs).
- **Animals (quiz 2026-10-11):**
  - **Species affects body and mind** (the user's choice): size, speed and senses (a mouse needs a tiny desk, a giraffe hits door frames, night animals prefer late shifts), and species also nudges talents. Claude's guard rails: every species gets strengths and weaknesses, nudges are small, individuals can override them, and species never stand in for real-world groups.
  - **The cast:** urban regulars (foxes, raccoons, rabbits, cats, dogs, pigeons), big animals (bears, elephants, giraffes, rhinos), small animals (mice, hamsters, squirrels), and exotic and odd ones (otters, red pandas, owls, capybaras, axolotls).
  - **Your character is random**, dealt like your starting home.
  - **Art style:** between chunky-cute (big heads, few voxels) and taller, more realistic proportions.
- Clawd is not a game character (Anthropic's mascot). He stays only on the "Made with Claude" splash as a credit.
- **World state:** the character roams the city; the camera follows and the view can turn. Click to walk.
- **Room state:** walking through a door zooms the camera out to frame the whole room, at one fixed angle. The player clicks objects; the character walks over and carries out the command.
- Claude makes all art (pixel-desk, the crafting apps, voxel models as text files).

## Art style and philosophy (quiz 2026-10-11)

- **Reference pass 2 (user, 2026-10-16):** after the first voxel-desk renders of the old CRT computer came out flat and dull, scouts gathered new leads (`design/art/REFERENCE_LEADS.md`). The user adopted four ideas:
  - **The screen is the hero.** A big dark screen with one bright element that glows (bloom) and spills light onto the desk and the character.
  - **The 1978 computer as three boxes.** The case, a keyboard slab in a different colour, and a CRT or TV with a back hump. Add a curly cable and a cassette or floppy drive. Built at fine 1/24 m voxels.
  - **One hue per room.** Each room in one colour family at 3 or 4 values plus one accent, with shadows tinted toward that hue, never grey or black.
  - **Time-of-day backdrops.** Pale by day, salmon at dusk, deep violet at night, with the window and screen glow changing and the geometry unchanged.
  **Outline:** the outline stays, but its ink is now a dark shade of the colour it borders instead of near-black. Creases that the corner shading already darkens get no line. This replaces the near-black ink of 2026-10-13.
- **The player's 1978 computer (user, 2026-10-19):** chosen from six variants (three by Sonnet 5.5, three by Opus 5.5, made independently from the same brief): `assets/models/computer_1978.txt`. It is a chunky, toy-like set: a warm-grey case with a light-grey monitor on top (dark plinth, back hump), a blue keyboard slab with a light key block, and a red cassette deck as the single accent. The hero is a cyan cassette-tape icon glowing on the dark screen, which ties the machine to the title-screen tape. Grid 24×15×19 fine voxels, 7 colours. Every detail is at least two voxels wide, because one-voxel patterns break up at the game's zoom.
- **New player's computer and the fidelity bar (user, 2026-10-22):** the user picked a reference from a sheet of 1980s voxel computers: a dark charcoal machine with a carry-handle CRT on a wide, flat keyboard base, coloured key clusters (red, green, blue, yellow) and a cartridge slot at the back right. The user asked for a mouse to be added. It's a parody, with no logos. Five builds were made from one brief: Opus 5.5 and Sonnet 5.5, each at 1/24 m and at 1/48 m, plus an Opus build made as Blockbench boxes. They were compared in a 3D viewer. **Opus at 1/48 m "wins by a landslide"** (52×38×41, about 18.5k voxels, 13 colours, with the power LED as the only glow). It has real key caps with gaps, hinge blocks on the handle, vent slots, and a domed mouse with a cable. The user: "I need this level of fidelity from you." That level is now the bar for models. The winning file is kept in `design/art/candidates/pc_opus_48/` and is not in `assets/models/` yet.
- **The whole game moves to 1/48 m voxels (user, 2026-10-22).** This replaces the fine 1/24 m voxels of 2026-10-13 for every model. Measured on the same design, 1/48 m has 7.5× the voxels and 4.4× the exposed faces, while the agent cost per model stays about the same, because the files are script-generated. The plan:
  1. A greedy mesher, with a headless memory and mesh benchmark.
  2. One render test, with the user's OK, to see how it reads at street zoom. This is not a gate. If it reads as smooth 3D rather than cubes, the user is fine with that: "i usually like Low-Poly design stuff".
  3. Rebuild the existing 16 models at 1/48 m, starting with the bedroom.

  **This waits until the home-hub design and philosophy are pinned down first** (user's order).
- **Voxel tools (user, 2026-10-22):** Goxel, Blockbench, MagicaVoxel and the pxl pixel-art skill were tried and removed, because none of them played a part in the winning build. Models stay as text files written by agents (a generator script is fine) and are checked with the voxel desk. The voxel desk keeps its `.vox` and `.bbmodel` export commands.

- **Recognisable, no artefacts (user, 2026-10-12):** "we are choosing to chase designs that are instantly recognizable and dont have artefacting", and "needs more voxel". So: real voxel geometry drawn by the engine (no sprite stacking, which smears and stripes at in-between angles), more voxels (about one voxel per screen pixel at the default zoom, three times the first build's density), every object readable from its silhouette alone (exaggerate key shapes, few colours per object, consistent light, no noise), and the camera rests only on the four clean isometric angles.
- **Reference pass decisions (user, 2026-10-12):** a dark 1-pixel outline on everything (characters, furniture, buildings), for the crisp hand-drawn pixel-art read; animal heads stay at about 30% of body height (grown-up proportions, not chibi), with the face made readable through bigger eyes, a clear mask and contrast instead; rooms float as dioramas on a thick slab with lighter wall caps. Notes from the references are in design/art/REFERENCE_NOTES.md.
- **Full-resolution soft voxels (user, 2026-10-13; replaces the pixel-art canvas):** after the bedroom pass the user found the faces and rooms "uncanny valley... something's missing". From an itch.io survey they chose to imitate monogon's *Voxel House Interior* and franco's *Interior House & Office* (rooms), monogon's *Voxel Noir* and *Medieval Town* (street and buildings), and SonaSar's *Voxel Animals* (faces: a block head, two dot eyes, ears). Decisions: render at full resolution (no 640×360 canvas), keep the fine 1/24 m voxels (each now several screen pixels, so the cubes read as cubes), soft lighting (corner shading, warm bounce, soft shadows, glow), and decide the outline from a with/without test. Style only: we build our own models and use none of their files. Earlier pixel-canvas notes below are superseded where they conflict. **Locked 2026-10-13** after the bedroom test ("This is it"), with the outlines kept (user picked the outlined version over the soft one).
- **Fine pixels (superseded 2026-10-13):** internal resolution about 640×360, scaled up sharp (3× to 1080p, 6× to 4K).
- **The palette grows with the eras:** the bedroom era looks limited like old home computers; each era opens more colours until the modern era is rich. The game's look tells the history.
- **Cozy surface, dark undertow:** warm, inviting and cute; the cruelty lives in the systems. Visuals turn subtly colder when the storyteller turns on you.
- **UI changes with the eras:** beige home-computer windows early, then CRT green, glossy 2000s, flat modern; the in-game computer and phone match the era.
- **Flat colours + outlines:** clean flat faces, soft shadows in corners, a thin dark outline around silhouettes (Into the Breach clarity).
- **Live lights, pixel-snapped:** the sun moves with the clock; lamps and monitors glow at night; everything stays crisp on the pixel grid.
- **Camera: smooth but pixel-stable**, gliding and turning without pixels swimming (the A Short Hike trick).
- **References:** *A Short Hike* (3D rendered low-res as pixel art, closest to our tech), *Into the Breach* (crisp isometric pixel art), *Habbo Hotel* (isometric rooms full of furniture and characters).

## Life stages (2026-10-11)

The game follows one life: high school → college (a full 4 years) → job or destitution → success or game over → middle age (the tycoon part: more managing people than making games) → late game. College is an era of its own.

## High school (2026-10-11)

- Choices are both free actions in the world (click the fire alarm, sneak into the computer lab, pick a lunch table) and scene-based picks of what to *do* at key moments (confirmed 2026-10-11).
- Choices are environment-based, like *Surviving High School* (EA Mobile / Centerscore, iOS), not three-answer dialogue menus. That game ran each day as a planner of 1–2 hour blocks and switched between scenarios, trivia, dialogue and button mini-games; choices shaped each episode's outcome.
- Earlier reference: the Java game *High School Hook Ups* (2010, in the style of Gameloft's *Nights* games): conversation scenes where you pick one of three answers to win people over, errands for friends, small mini-games (a basketball power gauge, a maths test), and a reputation to build by graduation. Its known weaknesses (PocketGamer, 2/5): one-dimensional cast, repetitive loop. Keep the style, fix those.
- Content line (Claude's rule): high-school characters are minors, so high school has friendships, rivalries and reputation but no sexual content. Romance belongs to college age and later.
- **Daily rhythm (quiz 2026-10-11):** school hours run as planner blocks with scenes and picks; after school, evenings and weekends you roam the world freely.
- **Home rules:** each family is different. The random starting home rolls its own rules (curfew, computer time, grade demands); breaking them means being grounded, losing the computer, or losing trust.
- **Social standing:** one school-wide reputation (not per clique).
- **High school is a short chapter**: a handful of key episodes that ease you into the game, then on to college.
- **Trouble leaves a permanent record** (detention, suspension, expulsion) that follows you to college applications and early job interviews.
- **Computers in high school:** the school lab (good machines, limited hours, a teacher watching; sneaking in after hours is an event) and the computer shop's demo machines (until the owner chases you out).
- **Teachers are background**, not characters.
- **Exams** are quick quiz mini-games: your knowledge meter and your own play decide the grade. Cheating is possible; getting caught goes on your record.
- **Learn, then apply (the user's example):** a rare random event where friends ask you to sneak into the school at night. While they do their thing, you get the chance to mess with the school's tech, a place to apply what you studied.

## College (quiz 2026-10-11)

- **Admission:** high-school grades, family money and scholarships open your options: community college, state university, or an elite school with better contacts and harsher costs. Skipping college is allowed.
- **Major matters** (computer science, art, music, business, engineering...): it shapes which knowledge you gain fastest. You can switch, at a cost in time and money.
- **Tuition** can come from family (if your starting home allows; wasting it costs bond), student loans (debt that follows you and colours how the bank sees you), scholarships (lost if grades drop), and your own jobs (the odd-job mini-games, at the cost of stamina and grades).
- **Campus life:** classes and exams (knowledge, grades, learn-then-apply), clubs (a game-dev club, a computer club, sports: skills and lifelong contacts), social life and parties (friendships, romance now that you are adults, reputation, lost sleep), and side projects (your first real games, made in dorm rooms at 3 a.m.).

## Early game: from nothing (2026-10-11)

- You start with nothing but an old 1978 home computer that can only read teletext-style text pages (changed 2026-10-12; a laptop with a browser could not exist in 1978).
- **Starting home is random.** Living with your parents is a strong start: the family's reach gives better connections to the job market. In a dorm you must find work yourself, through ads and the like.
- **Background is the player's choice** (muscle head, nerd, art kid...): it sets the direction you take the game. The user's example: a muscle head finds C++ unnatural, and "memory leaks will kill you".
- **Odd jobs are mini-games.** They take time and energy and pay money.
  - Jobs (era-accurate, quiz 2026-10-11): fast food and delivery (easy to get, low pay, heavy on stamina), computer shop and repair (teaches hardware, early access to new machines, the owner may sell your zip-lock tapes), arcade attendant (watch what players love, spot trends, fix cabinets), tutoring and office temp (teaching builds knowledge; typing builds speed and discipline).
  - Results come from the player's own play, tilted by stats (e.g. shaky hands at low stamina).
  - Jobs also give skills and knowledge, contacts, industry tips before they hit the news, and equipment and perks (discounts, a hand-me-down computer, after-hours access to machines).
- **After college ("job or destitution")** any career is open, inside or outside games. Inside: junior or QA tester at an existing studio, with contacts and future rivals; leaving with their ideas risks a lawsuit.
- **Knowledge meter**, separate from skills: studying builds knowledge, and knowledge is a barrier to entry.
- **Learning:** by doing, by studying (books, online courses, college), and both. Learning a concept and then getting to apply it gives a huge bonus; the catch is finding the chance to apply it. Skills fade without use.
- Skills form an RPG-style skill tree, deep and complex.
- Hardware is bought in the city and matters.

## Body (2026-10-11)

- Stamina comes back from eating (better food, more stamina) and sleep (better sleep, even more).
- Skipping sleep buys progress, but if stamina is already low it can severely damage the project.
- Overwork can put you in hospital, with bills you may not be able to afford.

## Time (2026-10-11)

Days with a clock; time can be paused. The length of a full successful playthrough is not decided yet.

## Failure and the storyteller (2026-10-11)

- The higher you climb, the greater the risk of game over. A late-game giant (Activision-sized) whose big investment flops can face bankruptcy, and it cannot escape by pumping out shovelware or taking contract work: the game must not let the player trick the system.
- **Name (2026-10-12): the storyteller is called the Game Master in the game.** The setup screen reads "Choose a Game Master" and "They are your narrators, they weave your story. Choose carefully." Descriptions use the same voice.
- **Storyteller quiz (2026-10-11):**
  - Each storyteller has a **theme** (e.g. life drama, market chaos, rivals and politics), plus a **separate difficulty** slider.
  - Storytellers are **animal narrator characters**. They introduce you to each era, drop subtle hints, and can turn sinister through cryptic, half-incomprehensible dialog boxes: a warning that the tide is slowly turning against you.
  - **Locked in**: the storyteller can never be changed during a playthrough.
  - **The poison trail shows itself** through subtle omens a sharp player can read, and after a game over through a **post-mortem replay** that reveals the trail but never all the details.
  - **First storyteller to build: rivals & politics.** Life drama and market chaos come later, one at a time.
  - **It remembers you across playthroughs**: "Back again?" Updated in the endings quiz: it also silently notes your habits so it can mess with you in the next run.
  - **Its sinister messages mix decodable riddles with pure mood**: some lines can be cracked with knowledge and attention, others are only tone, timing and glitches.
  - **Blind luck is separate** and the storyteller cannot touch it, so a sleeper hit can save you even when the storyteller is out to get you.
- **The player chooses the storyteller** at the start. Each one leans toward different content (one brings more life drama, another more business), which also lets the project build content one storyteller at a time.
- **An AI storyteller, like RimWorld's.** It tracks your success and silently lays a poison trail, waiting for you to get complacent, then poisons you slowly until it is too late to notice.
- Prudence is not enough: luck still turns, and you must take risks. A sleeper hit or a cult hit can carry you through.

## First release (2026-10-11)

Both options, the player's choice: a type-in listing sent to a magazine (small pay, name in print, first reputation), or a tape in a zip-lock bag on the counter of the local computer shop (split with the owner, a handful of copies).

## Working title (2026-10-11)

**DevLife Tycoon** (user's suggestion, not final). The folder stays `bevy-game` until the user decides. Unchecked against existing games and trademarks.

## Staff and man-management (quiz 2026-10-11)

- Candidates come from era-accurate job ads (classifieds → magazines → job boards → social feeds), head-hunters and poaching, referrals from staff, classmates and family (pre-loaded loyalty and conflicts of interest), and college fairs and interns (cheap raw talent that may leave once trained).
- Employees are lighter than the player: skills, traits and morale, without a full personal life.
- Roles are fluid in a tiny team; specialists and departments form as you grow (code, art, design, audio, QA, marketing, finance, legal).
- Loyalty and leaving are driven by pay, equity and bonuses (shares vest over years), credit and recognition (stolen credit is a lifelong grudge), crunch and workload (remembered; burnout and resignations), and creative freedom and culture fit.
- You appoint leads and managers, who run teams their way, build their own loyalties, and can protect you, sabotage you, or leave with half the team.
- Firing and layoffs cost everything real: severance, possible wrongful-dismissal lawsuits, a fired dev who may hold it against you, press stories, and the morale of everyone who stays.
- The industry changes over the eras: unions arrive (crunch and layoffs get harder and costlier), a remote-work era (cheaper and wider hiring, harder culture and secrets), outsourcing and contractors (cheaper, riskier), and studio culture emerges from your decisions (crunch house, hacker den, corporate), attracting and repelling certain hires.

## People (2026-10-11)

- Until the mid game it is you and a fellow college friend. The partnership can grow, or break apart over differences.
- **Several classmates**, not one fixed partner. Each can become your partner, a rival, or, if the friendship is good, someone who later sits in a seat of power and can help you.
- Aging: the body changes over a lifetime (less stamina, health risks) and experience offsets it.
- Personal life (friends, relationships, family) is in, and competes with work for your time. The user asked Claude to design it. Claude's sketch: bonds that grow with time and fade with neglect; people need you at the worst moments; strong bonds pay back (stamina, money, support); broken bonds can create rivals or press stories; the storyteller sets how often. The user: it needs "an extensive dictionary or perhaps something completely new". Approach chosen 2026-10-11: systems plus written pieces. Relationships run on needs and opinions (like The Sims or RimWorld); Claude writes events as building blocks that combine into many variations. No in-game AI model.
- Burnout, head-hunters (poaching and being poached), grudges, and extreme displays of loyalty to a company all exist.

## Making games (2026-10-11)

- The deep version: real development phases, not three sliders. Built one feature at a time.
- Optimization is a real skill and becomes a necessity when the resource crisis comes.
- **Mini-games early on**, when (a) you get a chance to improve the game's quality, or (b) you must give it your all to save the game.
- **Bugs are hunted interactively.** Sometimes you can keep one as a feature.
- **Hardware limits, like real history:** early eras demand extreme optimization. As machines get stronger you can get lazy, but customers may punish it. The player can choose to polish more and risk missing a window of opportunity; players who planned ahead and polished are rewarded.
- **Deadlines** are set by you early on, in the middle by the people you must pay back, and later by publishers and investors.
- **Playtesting:** hand builds to friends, family, strangers at the café, or an internet forum.
- **Feature costs (approved 2026-10-11):** every feature costs knowledge (the gate), work (person-days per discipline, shrunk by skill), a share of the machine's budget (memory, processor, disk), risk (bugs grow with difficulty above skill; new tech is riskiest), money (tools, licences, dev kits, hardware) and upkeep (e.g. servers). The payoff shifts with history: a feature starts as a wow bonus, then becomes expected (no bonus, but missing it is punished).
- **Legends (the user's twist):** once in a blue moon you can hire a "god"-tier developer, in the spirit of someone like John Carmack. They are fictional animal characters inspired by such archetypes, never real people.

## Critics (approved 2026-10-11)

- Critics are animals with tastes and careers: fanzine → magazine → TV → video → social media, rising or falling. A critic can grow famous with you, or by hating you.
- Critics remember how you treated them.
- Scores are honest but tilted: taste applied to what is really in the game, plus mood, plus a hidden bias from your history with them.
- You can court critics (preview trips, gifts); exposure is a scandal for both.
- The aggregator era moves power from single critics to the average; publishers tie bonuses to it.
- Critics can turn developer and become studio rivals.

## Business (2026-10-11)

- All of it: publishers and contracts, royalties, investors, crowdfunding, platform fees, marketing, review scores, piracy, rivals.
- Most of it stays out of the way until you are your own company of 8 to 10 people, with at least a one-person marketing team.
- Pitching to publishers in their towers is late game.
- **Publishers (quiz 2026-10-11):**
  - You can land a deal four ways: publishers find you after a hit, you pitch in their towers (late game), through an agent (a middleman animal with a cut and loyalties of their own), or in trade-show back rooms.
  - Contracts start simple (advance + royalty) and become fully negotiable later: recoup, milestones, IP ownership, sequel rights, marketing spend, kill fees.
  - A missed milestone does whatever the contract says: grace period, frozen payments, cancellation, or the publisher taking your IP. Bad contracts bite.
  - Publishers are run by animal executives who remember how you treated them, and take that with them when they change companies.
- Trade shows: early on, you only get random chances to show a game, when it is a huge hit, a cult classic or a sleeper hit.
- Finance ladder: with more success you open special bank accounts, get licences and permits, negotiate higher credit limits, look at loans, and finally establish a capital-market presence.
- **Finance (quiz 2026-10-11):**
  - **Legal form matters, with real liability**: sole trader → partnership (with your friend) → limited company → public company. As a sole trader, business debts are personal (a flop can cost your car or flat). Incorporating protects you but brings licences, permits, paperwork and fees.
  - **Bank managers are characters**: your history with them sets credit limits and loan terms. Better banks open their doors only as you grow.
  - **Other money:** family and friends (cheap, but bonds take the hit if you lose it), angels and venture capital (money for equity and a say), era-gated crowdfunding and grants with strings attached, and loan sharks (fast cash; missing a payment is very Dark Souls).
  - **Taxes are real**: taxes by era and company form, an accountant you can hire (who remembers how you treated them), random audits, and the option to cook the books (big savings, prison-level risk).
  - **Crime has real endings**: fraud, espionage or a DMCA fight gone wrong can end in court or prison. That is game over, or a fall from grace you claw back from, depending on the crime and your lawyer.
  - **Going public means everything**: the share price reacts to news and rumours, a board can fire you from your own company, activist investors and hostile takeovers are real, and a rival can buy up your shares.

## Marketing and players (quiz 2026-10-11)

- **Channels, arriving era by era:** print (magazine ads, covers, shop posters), TV and radio, trade shows and press tours (ties to critics), and online: forums, then streamers and influencers, then social feeds and trailers, including parody analogues of 4chan, Reddit and X.
- **Hype sets expectations:** it boosts launch sales but raises the bar; underdelivering brings a worse backlash than staying quiet.
- **Audience segments with tastes** (kids, hardcore fans, casual players, collectors...), drifting by era, plus nostalgia for older genres.
- **Piracy is era-accurate** (tape copying, cracked disks, downloads). You choose protection; heavy DRM angers paying customers, and piracy can also spread your name.
- **Communities form and can turn:** fan clubs, forums and modders defend you, extend sales with mods, or turn on you with review bombs and boycotts; the parody boards amplify everything.
- **Live ops, everything with consequences** (online era onward): patches, DLC, expansions, microtransactions, loot boxes. Big money, real backlash, and era-accurate regulation (loot-box laws).
- **Streamers and influencers are characters**, like critics: tastes, audiences, careers and long memories. One stream can make a sleeper hit or bury you.
- **Full price control:** launch price, sales, bundles, regional prices, price wars with rivals. Platform holders and retailers push back.

## Rivals

The rival system is the game's centrepiece. Its design is private and is not published with this repository.

## Timeline (approved 2026-10-11)

Fixed eras, living outcomes. Parody names, never real companies.
1. **Bedroom era:** home computers, cassettes, one-person games. The player starts here.
2. **Cartridge wars:** consoles; a crash can happen if the market floods with junk (simulated, not scripted).
3. **Disc and 3D era:** budgets explode, teams must grow.
4. **Online era:** servers, patches, live games; piracy changes shape.
5. **Pocket era:** phones, free-to-play, ad money.
6. **The Silicon Drought:** the resource crisis and the second half's turning point. Chips get scarce, new hardware stalls, part prices spike, players' machines must last; optimization is king. It never comes on a fixed date; warning signs come first.
7. **After the drought:** open; whoever survived shapes it.

Rules: dates drift within windows; platform-war winners depend on the games released (yours and rivals'); you can bend history (invent a genre early and it exists early; rivals copy it). A lifetime from a teenager around 1978 spans the whole timeline.

## Platforms and hardware (quiz 2026-10-11)

- **Platform makers are parody companies** run by animal execs who remember you: they court you, lock you out, or offer exclusivity deals.
- **Real gatekeeping for consoles:** dev kits need approval, money and NDAs; games must pass certification (fail and you are delayed). Early home computers need none of this.
- **Ports cost and differ:** each platform is its own target with its own limits; porting costs time and money, and a bad port hurts your name. Exclusivity deals pay you to stay on one platform.
- **The Silicon Drought hits everything:** players' machines (stall or get pricey; optimized games win), your studio's hardware (scarce and expensive), platform makers (launches slip, makers go bust or merge), and prices and the market (wallets shrink; cheaper and older-style games get a second life).
- **AI backlash (the user's addition):** in the drought era the studio can choose to use AI tools or not, and people get angry over any form of AI use.
- **Studio machines matter:** better workstations, build servers and test devices speed up work and catch bugs; old gear slows everyone and frustrates staff.
- **Old games come back:** remasters and remakes, retro compilations (or licensing to platform makers' collections), and lost-IP fights (if a publisher owns your old IP, a rival may remake it first).
- **Late game, you can become a platform holder** (your own console, store or handheld): huge risk, huge reward, and new reasons for every rival to hate you.

## Endings and legacy (quiz 2026-10-11)

- **Winning is legacy, judged at the end:** no single win screen. Your life is judged by the games you made, the people you shaped and the rivals you outlasted, with many kinds of greatness (beloved auteur, industry titan, cult legend).
- **Death of old age: continue as an heir**, a child, protégé or partner who inherits the studio, like a dynasty game.
- **Retirement:** you can retire and pick a successor (protégé, partner or the board), then watch from a distance whether they keep the studio or wreck it; that counts toward your legacy.
- **Carry-over between runs:** memory, no power. No stat boosts carry over; a hall of fame records past lives. But the storyteller silently notes your habits to target you next run, and blind luck (independent of the storyteller) grants small favours across runs for good deeds, enough to sometimes overturn the storyteller.

## Open questions

- Length of a full playthrough.
- All planned quiz rounds are done (2026-10-11). Next: build the foundation, one feature at a time.

## Build log

- **2026-10-11, foundation:** splash → a test street (text map `assets/maps/street.txt`) rendered as voxels into a 640×360 canvas, scaled up sharp and pixel-snapped. A random animal (fox, rabbit, raccoon or bear, random clothes) walks where you click (A* pathfinding), Q/E turns the street view, a 24-minute day/night cycle with a moving sun, shadows, street lamps that glow at night, T fast-forwards time. Walking into the computer shop's door fades into its interior (`assets/maps/computer_shop.txt`), framed at one fixed angle with the zoom-out; shelves, demo computers, the counter and the shopkeeper respond to clicks with a line of text. Not yet built from the art-style decisions: outlines, the era-growing palette, era-styled UI beyond the beige bedroom-era panels.
- **2026-10-11, first playtest fixes** (user feedback: shop entry jittery, characters too big, view too close, no signs on enterable places): zoom now only renders at whole-pixel levels and animates by scaling the finished picture, so entering a room pulls back without jitter; the mouse wheel steps the street between three crisp zooms (voxel = 4, 3 or 2 px; starts at 3); animals rebuilt smaller (about 1.4 m tall, 0.75 m wide; doors are 2.25 m); bobbing gold gems mark every door that leads somewhere; clicks hit real shapes (a building walks you to its door; shop objects respond directly) and the cursor becomes a hand over usable things.
- **2026-10-11, playtest verdict:** "much better": the zoom is smooth, the gold gems are easy to spot, character sizes are good. Session ended there. Next candidates: outlines (agreed art style), the era-growing palette, window glow at night, then the simulation core (clock, stamina, people, relationships) headless and testable.
- **2026-10-12/13, bedroom fidelity pass:** pixel-perfect isometric camera (true 35.26° pitch, screen squashed by cos 30°, so a 1/24 m voxel is exactly one pixel and every edge steps 2:1); the 1978 bedroom built at fine detail (`src/bedroom.rs`, `src/fine_animal.rs`). After a Pinterest reference pass (notes in `design/art/REFERENCE_NOTES.md`): a pixel-art outline pass on the whole 3D view (`src/outline.rs`, `assets/shaders/outline.wgsl`) that draws dark silhouettes, dark inside corners and light convex edges from depth and normals, skipping single-voxel steps; hard-edged shadows (2×2 hardware filtering, 4096 sun map); the room as a diorama on a thick slab; the palette rule (walnut, sage and cream, mustard accent, teal second, orange only on the fox); lived-in 1978 props (record player on a crate of LPs, alarm clock, corkboard flowchart, pennant, TV used as the computer's screen, keyboard unit, cassette recorder, graph pad, magazines, clothes heap, sneakers); the fox's face (white mask, bigger eyes with a glint, cream-lined dark-tipped ears, dark paws, ringer-tee trim); night lighting (desk-lamp spotlight, TV glow, moonlight through the window printing the frame on the floor). Renders in `design/art/renders/`.
- **2026-10-13, soft voxel look:** the 640×360 pixel canvas is gone: the world renders at full window resolution (`src/camera.rs`) with HDR, a filmic tone curve (TonyMcMapface), light glow (bloom), SMAA edge smoothing and soft shadows. Corner shading is baked into every voxel mesh (`src/voxel.rs`: rays from each corner count nearby voxels; scene-wide in the bedroom via `Occupancy`, so furniture darkens the floor and walls around it). Warmer daylight. Animal faces simplified to the voxel-pack style (block head, two dot eyes, a small muzzle, dark-tipped ears); the proof fox wears a sky-blue tee. `--no-outline` renders without the outline pass for comparison. Renders: `design/art/renders/softlit-*`, `outline-compare.png`, `fox-closeup.png` (the earlier pixel-canvas renders are kept as `pixel-*`).
- **2026-10-13, street rebuild (first block) and faces:** a new main street (`assets/maps/street.txt`, `src/street.rs`): pavements with slabs and pale kerb stones, the road one coarse voxel lower (tiles carry heights, so feet step down the kerb), gutters, lane edges, a dashed centre line, a zebra crossing with beacons, a manhole and drains. Your house rebuilt in the new style (`src/house.rs`): white render on a brick plinth, a rust-tiled gable roof with a chimney and TV aerial, a bay window, a salmon door with a fanlight and porch canopy, sills, curtains, windows lit warm at night; a front garden with a lawn, a low brick wall, gate pillars and hedges. The shop and flats keep the old model for now. Street furniture and 1970s parked cars by a Sonnet designer in fine voxels (`src/street_props.rs`), checked in a new `--gallery` mode (`src/gallery.rs`); glowing parts keep their painted colour (amber beacons, orange sodium lamps). All four animals rebuilt in fine voxels with dot-eye faces and used everywhere (`src/fine_animal.rs`). Facial expressions: an eye-and-brow layer per expression swapped at runtime, blinking, a resting face (sleepy after 22:00 until a mood system exists) and reactions to what the player clicks (each line in a room now carries an expression). Capture runs keep the game window hidden.
- **2026-10-13/14, the freeze:** a capture run froze the user's Mac (GPU hang, WindowServer watchdog). Cause: capture runs hid their window, so nothing paced frames to the GPU; with the heavier street the queue of unfinished frames grew about 13 MB a frame until the GPU hung (4.4 GB). Fixes: the render thread now waits for each frame to finish on the GPU (`camera.rs`), downward faces are left out of meshes (the camera can never see them; a third less mesh data), captures are paced, run one at a time (a lock) and quit themselves if stuck (a watchdog). Memory now stays flat at about 1.5 GB. `DEVLIFE_MEMLOG=1` logs memory per build stage and frame.
- **2026-10-14, memory diet and the memory cap:** the user saw memory spike; three agents audited it and a plan went through three phases. Memory went from 1.53 GB steady (1.56 GB peak) to about 0.62 GB (0.66 GB peak), with no visible change (checked pixel by pixel against before shots). What it took: meshes go to the GPU without a copy kept in main memory (only clickable things keep one, for click ray casts); identical models are meshed once and shared (`MeshCache`); corners shared by neighbouring faces are stored once and faces sealed inside models are left out (1.4M vertices, was 3.7M); normals and colours are packed (24 bytes a vertex, was 40); the bedroom's room-sized grids are cropped; the mesh buffers grow 10% at a time; the lamp-shadow cube is 1024; the sun's shadow map is 2048 and fitted to the view each frame (as sharp as 4096 over a fixed 90 m). On macOS 27 the system allocator kept 260 MB of freed memory counted against the game, so the game restarts itself once at launch with `MallocLargeCache=0`. **The memory cap** (`src/memguard.rs`): a watchdog thread checks memory four times a second; over 1.5 GB the game uses smaller shadow maps and slower frames; over 2.5 GB, or 400 MB of growth in 2 s once loaded and settled, or a GPU stuck for 6 s, it quits on the spot and says why in `target/memguard.log`. Frames slow right down while the window is covered. Capture runs quit straight out once every shot is saved (shutting down once hung for 30 s); `tools/guard.py` is the outer guard for capture runs. Bevy 0.20 pitfalls found on the way (worked around, see `voxel.rs` and `world.rs`): packed positions lose their bounds on GPU-only meshes; mixing 16-bit and 32-bit indices, or changing a mesh after Bevy has recorded its arrival, can leave a mesh drawn into the depth pass only (sky-coloured holes). Still open: on 2 of 14 street runs, memory rose in the first second after loading, outside the GPU and the heap. One run levelled off 190 MB up (830 MB); the other was still rising at 1.39 GB when the runaway rule stopped it (that rule now waits 5 s after loading; the 2.5 GB hard cap still applies). Likely the system compiling shaders it had not cached, not confirmed.
- **2026-10-15, your house cut open:** the bedroom now sits inside the house as decided above ("Your bedroom inside the house"): walls one voxel outside the footprint, a cut-open roof (`house.rs` `cut_open_roof`), white cut faces, a landing with the stairwell. Fixing the sky-coloured holes for good: every voxel mesh keeps its copy in main memory again (keeping it only for clickable ones left some drawn in the depth pass only); with packing and sharing that copy is ~50 MB, steady ~640 MB.
- **2026-10-15, leaving Bevy for Godot:** the Bevy build froze the Mac twice (GPU page faults in Bevy 0.20's Metal rendering; see `design/ENGINE_REPORT.md`). The user chose Godot 4 with C#. The Bevy version was backed up and the backup verified, and the old folder went to the Trash. The project now lives in `Engineering/devlife-tycoon`, with the Bevy code kept under `reference/bevy/` as the porting reference. Godot 4.7.2 .NET (notarized, official release) and .NET 10 are installed; a headless C# smoke test passed. The port is done step by step by Haiku 5.5 agents at xhigh, reviewed by Claude, verified headless first; rendered runs only with the user's OK.
- **2026-10-16, the Godot mesher begins:** Voxel Tools (Zylann's MIT voxel module for Godot) was studied as a second reference (`design/VOXEL_TOOLS_NOTES.md`). It is not a drop-in: it never shares corners and has no sealed-pocket culling, so the Bevy mesher stays the spec, and we take its structure (plain mesh data first, the Godot mesh after) and its scatter idea for later. A Fable review of the port plan added stronger mesher tests (a hollow and a punctured box), a tiny first rendered run before the look work, and moved the voxel desk (a CPU-only model viewer for the art agents) to right after corner shading. Step 1b's core (`src/Voxel/VoxelMesher.cs`) passes all its checks headless. **Decision (user):** vertex colours are stored as sRGB (`vertex_color_is_srgb = true`), because Godot keeps them at 8 bits per channel and linear storage bands in dark shadows; the shading maths is unchanged from Bevy.
- **2026-10-16 to 10-19, the voxel desk and the 1978 bedroom:** step 1b (the mesher) is complete and tested headless (84 checks). A CPU-only "voxel desk" (`~/claude-mods/voxel-desk/tool`, no GPU, no Godot) now draws voxel models and whole scenes to PNG. It uses the old camera, a warm key light, a cool sky, tinted shadows, tinted-ink outlines, contact shadows, glow with bloom and spill, and day, dusk and night backdrops. It has 19 self-tests. The model parser no longer mirrors models: the top row of a layer is the back (north), like the room maps. The player's computer was chosen from six variants (`computer_1978.txt`). The bedroom is built in warm mustard and teak, with ten props, a dollhouse shell, a landing with the top of the stairs, and wall items (`assets/scenes/bedroom_1978.txt`, plus a lamp-on version). Known limits of the desk: one point light, so the lamp's warm pool waits for Godot; the stairwell shows only two steps.
