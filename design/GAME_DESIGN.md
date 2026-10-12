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

- **Fine pixels:** internal resolution about 640×360, scaled up sharp (3× to 1080p, 6× to 4K).
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

- You start with nothing but a weak laptop that can only browse the web.
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
