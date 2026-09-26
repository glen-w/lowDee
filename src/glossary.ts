// Plain-language names for the whistle, the gestures, and the tune types
// a session plays. Bodies stay short enough to read between attempts.

export const GLOSSARY_SECTIONS = [
  "The whistle",
  "Notes and breath",
  "Ornaments",
  "Kinds of tune",
  "On the page",
  "While you play",
] as const;

export type GlossarySection = (typeof GLOSSARY_SECTIONS)[number];

export interface GlossaryEntry {
  id: string;
  term: string;
  section: GlossarySection;
  body: string;
  aka?: string[];
}

const INTRO: Record<GlossarySection, string> = {
  "The whistle": "The tube in your hands, and the name you give it.",
  "Notes and breath": "How a note starts, stays joined, and jumps the octave.",
  Ornaments: "Short gestures on a note you already meant. Cut, tap, and roll come first.",
  "Kinds of tune": "The shapes a session plays. The first pack is one slow air.",
  "On the page": "Pictures and words for after you have heard the phrase.",
  "While you play": "What the screen is doing during a sitting.",
};

function entry(
  id: string,
  term: string,
  section: GlossarySection,
  body: string,
  aka?: string[],
): GlossaryEntry {
  return aka ? { id, term, section, body, aka } : { id, term, section, body };
}

const GLOSSARY: GlossaryEntry[] = [
  entry(
    "low-whistle",
    "Low whistle",
    "The whistle",
    "A whistle an octave below the small tin whistle. The tube is longer, the stretch is wider, and it speaks on a slower breath.",
  ),
  entry(
    "low-d",
    "Low D",
    "The whistle",
    "This whistle. Six holes, home note D, an octave below the high D. The first note you learn is that low D, all holes covered.",
  ),
  entry(
    "high-d",
    "High D",
    "The whistle",
    "The small D whistle, an octave above this one. The fingerings match. The breath is quicker and the reach is smaller. This tube still wants its own breath and its own stretch.",
    ["tin whistle"],
  ),
  entry(
    "tin-whistle",
    "Tin whistle",
    "The whistle",
    "The small whistle, usually metal, usually a high D. Also the everyday name for that instrument even when it is not tin.",
    ["pennywhistle", "feadóg", "feadog"],
  ),
  entry(
    "whistle-key",
    "Whistle key",
    "The whistle",
    "The letter of the home note, all holes covered. This companion is the low D. A whistle in C, or a low F, is a different tube.",
  ),
  entry(
    "fipple",
    "Fipple",
    "The whistle",
    "The mouthpiece. Breath goes through a slot and splits on an edge, and that edge makes the note. The lips rest around it and seal.",
    ["mouthpiece"],
  ),
  entry(
    "windway",
    "Windway",
    "The whistle",
    "The narrow slot in the mouthpiece that leads the breath to the window. Moisture beads there. A shake of the mouthpiece clears it.",
  ),
  entry(
    "window",
    "Window",
    "The whistle",
    "The opening just past the mouthpiece, where the breath meets an edge and becomes a note. The six finger holes are numbered from here, 1 nearest the window and 6 nearest the bell.",
    ["voicing"],
  ),
  entry(
    "tone-hole",
    "Tone hole",
    "The whistle",
    "One of the six finger holes along the tube. Covering and opening them changes the note. Hole 1 is nearest the mouth. Hole 6 is nearest the bell.",
    ["finger hole"],
  ),
  entry(
    "bell",
    "Bell",
    "The whistle",
    "The open end of the tube, farthest from the mouth.",
  ),
  entry(
    "pad",
    "Pad",
    "The whistle",
    "The flat of the fingertip over a hole. Laid flat, it seals. Perched on the point, it leaks.",
  ),
  entry(
    "top-hand",
    "Top hand",
    "The whistle",
    "The hand nearer the mouth, covering holes 1, 2, and 3. For most players that is the left hand.",
  ),
  entry(
    "bottom-hand",
    "Bottom hand",
    "The whistle",
    "The hand nearer the bell, covering holes 4, 5, and 6. For most players that is the right hand.",
  ),
  entry(
    "tuning-slide",
    "Tuning slide",
    "The whistle",
    "A joint on some whistles that lengthens the tube a little, so the pitch can sit with other players. The app still listens to the note you hold.",
  ),
  entry(
    "this-horn",
    "This horn",
    "The whistle",
    "Your name for the whistle in your hand. It stays on this device, together with the pitch of its low D once that note will hold.",
    ["nickname"],
  ),

  entry(
    "home-note",
    "Home note",
    "Notes and breath",
    "All six holes covered. On this whistle that is low D, the note the tube is named for.",
    ["bell note", "bottom D"],
  ),
  entry(
    "staircase",
    "Staircase",
    "Notes and breath",
    "The notes in order, one finger lifted at a time: D with every hole covered, then E, F♯, G, A, B, then C♯ with every hole open. The first air uses D through B. The high D uses the same closed holes as the low one, with more breath.",
  ),
  entry(
    "seal",
    "Seal",
    "Notes and breath",
    "A closed hole with the pad flat and no air at the edge. Every hole the fingering marks closed needs that cover. On low D, that is all six.",
  ),
  entry(
    "leak",
    "Leak",
    "Notes and breath",
    "Air escaping past a finger on a hole that should be closed. The note drops, splits, or squeaks. On a low D the wide stretch makes the lower holes the usual place.",
  ),
  entry(
    "squeak",
    "Squeak",
    "Notes and breath",
    "A high chirp. A leak will do it, or a breath too hard for the fingering. Cover the holes fully and soften.",
    ["chirp"],
  ),
  entry(
    "condensation",
    "Condensation",
    "Notes and breath",
    "Moisture from the breath in the windway. The note gurgles or stops. Shake it out of the mouthpiece, then play on.",
    ["water", "clog"],
  ),
  entry(
    "octave",
    "Octave",
    "Notes and breath",
    "The same letter, higher or lower. On this whistle the fingering stays put and the breath does the lifting. The hole picture stays the same, with a mark for the higher one.",
    ["register"],
  ),
  entry(
    "first-octave",
    "First octave",
    "Notes and breath",
    "The lower notes, from low D up to C♯. A gentler breath keeps you here.",
    ["low octave"],
  ),
  entry(
    "second-octave",
    "Second octave",
    "Notes and breath",
    "The same fingerings, sounding an octave higher, from the high D upward. The first pack asks for that high D. The picture is the same holes, with a ring to say you are up there.",
    ["high octave"],
  ),
  entry(
    "break",
    "Break",
    "Notes and breath",
    "The jump from the first octave to the second on the same fingering, when the breath crosses a threshold. The lesson is to choose that moment, and to come back down.",
    ["octave break"],
  ),
  entry(
    "overblowing",
    "Overblowing",
    "Notes and breath",
    "Breath strong enough to push the note into the octave above. It arrives by accident at first. Later it is how you ask for the high note.",
    ["overblow"],
  ),
  entry(
    "breath",
    "Breath",
    "Notes and breath",
    "What you send through the fipple. Softer breath holds the low octave. A little more, on purpose, finds the high one. The tune called an air is a different word.",
  ),
  entry(
    "on-the-breath",
    "On the breath",
    "Notes and breath",
    "The notes of a phrase joined into one line. The tongue starts the phrase. The notes after that stay connected until the phrase ends.",
    ["legato"],
  ),
  entry(
    "tongue",
    "Tongue",
    "Notes and breath",
    "A light touch of the tongue that starts a note. Here it starts the phrase, then rests.",
    ["tonguing"],
  ),
  entry(
    "pulse",
    "Pulse",
    "Notes and breath",
    "The beat you can clap. An air holds it loosely. A dance tune holds the pulse of that dance.",
    ["beat"],
  ),
  entry(
    "phrase",
    "Phrase",
    "Notes and breath",
    "A musical sentence you can hear and copy in one breath. When you try it, you start again from its beginning.",
  ),
  entry(
    "chunk",
    "Chunk",
    "Notes and breath",
    "One phrase of the tune, practiced before the next. The May Morning Dew is four: Opening, Home again, Second strain, Close.",
  ),
  entry(
    "half-step",
    "Half step",
    "Notes and breath",
    "The smaller distance between two neighboring notes, as from F to F♯, or from C to C♯.",
    ["semitone"],
  ),
  entry(
    "sharp",
    "Sharp",
    "Notes and breath",
    "A note one half step higher than the plain letter. F♯ sits one half step above F. The sign is ♯.",
  ),
  entry(
    "natural",
    "Natural",
    "Notes and breath",
    "The plain letter, one half step below the sharp of that name. C natural sits one half step below C♯. The sign is ♮.",
  ),
  entry(
    "accidental",
    "Accidental",
    "Notes and breath",
    "A note sitting outside the scale the tune was using. On a D whistle the common ones are C natural and F natural.",
  ),
  entry(
    "c-sharp",
    "C♯",
    "Notes and breath",
    "Every hole open. One half step below D. In a D major tune it is the ordinary C.",
    ["C sharp"],
  ),
  entry(
    "c-natural",
    "C natural",
    "Notes and breath",
    "One half step below C♯. It sits outside the plain D scale, so it needs its own fingering. In this pack the picture is hole 1 open, holes 2 and 3 covered, and the lower three open. Another whistle may want a different picture.",
    ["C nat"],
  ),
  entry(
    "f-sharp",
    "F♯",
    "Notes and breath",
    "The ordinary F on a D whistle: holes 1 through 4 covered, holes 5 and 6 open.",
    ["F sharp"],
  ),
  entry(
    "f-natural",
    "F natural",
    "Notes and breath",
    "One half step below F♯. It sits outside the plain D scale. It comes in a later pack.",
  ),
  entry(
    "half-hole",
    "Half-hole",
    "Notes and breath",
    "A hole only partly covered, so the pitch sits between two full fingerings. Used for some accidentals, and for gliding into a note.",
    ["half hole"],
  ),
  entry(
    "cross-fingering",
    "Cross-fingering",
    "Notes and breath",
    "A fingering that closes a hole out of staircase order, to reach an accidental. C natural is often one of these. The picture that speaks can differ from whistle to whistle.",
    ["cross fingering"],
  ),
  entry(
    "key",
    "Key",
    "Notes and breath",
    "The home note a tune rests on, and the scale around it. This whistle’s home is D. A tune in D can be major, mixolydian, or dorian.",
  ),
  entry(
    "mode",
    "Mode",
    "Notes and breath",
    "The color of a scale around a home note. Major, dorian, and mixolydian are the ones Irish tunes use most.",
  ),
  entry(
    "dorian",
    "Dorian",
    "Notes and breath",
    "A minor-colored scale with a bright sixth step. As I Roved Out is in A dorian, and on a D whistle that line asks for C natural.",
  ),
  entry(
    "mixolydian",
    "Mixolydian",
    "Notes and breath",
    "A major-colored scale with a lowered seventh step. Many tunes in D are mixolydian, so they use C natural where D major would use C♯.",
  ),
  entry(
    "concert-pitch",
    "Concert pitch",
    "Notes and breath",
    "The shared tuning of bands and orchestras, with A at 440 cycles a second. Whistles sit near it and drift as they warm. Once low D will hold, this app listens to your whistle.",
    ["A440"],
  ),
  entry(
    "warming",
    "Warming",
    "Notes and breath",
    "The pitch rises as the tube warms up. Let it settle. The home note you hold after that is the one that counts.",
  ),

  entry(
    "ornament",
    "Ornament",
    "Ornaments",
    "A short gesture on a note you meant to play. The note is still that note. Cut, tap, and roll are the first three.",
  ),
  entry(
    "cut",
    "Cut",
    "Ornaments",
    "A finger snaps open and shut, too short to hum, on the pulse. The pitch flicks up and the note returns. If you can hum the flick, it has become a note of its own.",
  ),
  entry(
    "tap",
    "Tap",
    "Ornaments",
    "A finger below the sounding note flicks shut and open. The pitch dips and returns.",
    ["strike"],
  ),
  entry(
    "roll",
    "Roll",
    "Ornaments",
    "A cut near the start of a long note, a tap later in it. You still hear the note you meant, with those two gestures on it.",
  ),
  entry(
    "long-roll",
    "Long roll",
    "Ornaments",
    "A roll with room for both gestures. On a dance tune it often fills a note of three pulses: a cut near the start, a tap on a later one.",
    ["standard roll"],
  ),
  entry(
    "short-roll",
    "Short roll",
    "Ornaments",
    "A roll on a shorter note. The cut and the tap still both happen, with less time between them.",
  ),
  entry(
    "cran",
    "Cran",
    "Ornaments",
    "Several cuts in a row, stepping down the fingers, usually onto low D. It comes from the uilleann pipes. A later color on the whistle.",
    ["crann"],
  ),
  entry(
    "slide-gesture",
    "Slide",
    "Ornaments",
    "The pitch glides from one note to the next as a finger rolls off a hole, or onto one. The arrival is the note you meant. Sessions also use slide for a dance tune in 12/8.",
    ["glide"],
  ),
  entry(
    "double-tap",
    "Double tap",
    "Ornaments",
    "Two taps, one after the other, on the same note.",
  ),
  entry(
    "triplet",
    "Triplet",
    "Ornaments",
    "Three quick notes squeezed into the time of a beat’s smaller step. A flick of the rhythm.",
  ),
  entry(
    "treble",
    "Treble",
    "Ornaments",
    "The same note three times, very quick, separated by cuts. A triplet that stays on one pitch.",
  ),
  entry(
    "vibrato",
    "Vibrato",
    "Ornaments",
    "A gentle waver in a held note, from a finger hovering near an open hole or from the breath. An air may use a little. Dance tunes usually stay steady.",
  ),
  entry(
    "grace-note",
    "Grace note",
    "Ornaments",
    "A tiny note written small, just before the main note. A cut is sometimes printed that way. Here the gesture is marked beside the tune.",
  ),

  entry(
    "tune",
    "Tune",
    "Kinds of tune",
    "A melody with a name, short enough to learn by heart. An air, a jig, a reel.",
  ),
  entry(
    "air",
    "Air",
    "Kinds of tune",
    "A slow melody with room to breathe. The pulse is loose, and there is no click under it. The May Morning Dew is the air in the first pack. Often written in 3/4 or 4/4 and played freely.",
  ),
  entry(
    "slow-air",
    "Slow air",
    "Kinds of tune",
    "An air played very freely, with time to shape each note. The name to use when a dance tune shares the title.",
  ),
  entry(
    "song-air",
    "Song air",
    "Kinds of tune",
    "An air that carries words. A phrase is a line of the song. The breath comes where the line ends. Down by the Salley Gardens is one.",
    ["song"],
  ),
  entry(
    "lament",
    "Lament",
    "Kinds of tune",
    "A slow air of mourning, played freely.",
  ),
  entry(
    "march",
    "March",
    "Kinds of tune",
    "A walking tune. Two or four steady beats in the bar, sometimes written in 6/8. The pulse is for stepping.",
  ),
  entry(
    "polka",
    "Polka",
    "Kinds of tune",
    "A quick dance in two, with a short line and a bounce on each beat. Written in 2/4.",
  ),
  entry(
    "jig",
    "Jig",
    "Kinds of tune",
    "A dance in groups of three. When someone says jig and nothing more, they mean a double jig: two groups of three in a bar, written in 6/8.",
  ),
  entry(
    "double-jig",
    "Double jig",
    "Kinds of tune",
    "The ordinary jig. Two groups of three even notes in a bar, written in 6/8. Strong, weak, weak, strong, weak, weak.",
  ),
  entry(
    "single-jig",
    "Single jig",
    "Kinds of tune",
    "A jig with a long-short lilt inside each group of three, written in 6/8 or 12/8. Lighter than the even notes of a double jig.",
  ),
  entry(
    "slip-jig",
    "Slip jig",
    "Kinds of tune",
    "Three groups of three in a bar, written in 9/8. A longer line than a double jig.",
  ),
  entry(
    "hop-jig",
    "Hop jig",
    "Kinds of tune",
    "A name players use for the slip jig. Some keep it for a lighter tune in 9/8.",
  ),
  entry(
    "slide-tune",
    "Slide",
    "Kinds of tune",
    "A dance tune in 12/8: four groups of three, with a swing and a longer line than a jig. Strongly associated with Sliabh Luachra, on the Cork and Kerry border. The same word is a finger glide onto a note.",
  ),
  entry(
    "reel",
    "Reel",
    "Kinds of tune",
    "Even running notes in four, written in 4/4, often felt as two beats. Straighter than a hornpipe. The tune a session plays most.",
  ),
  entry(
    "hornpipe",
    "Hornpipe",
    "Kinds of tune",
    "A dance in four with a dotted swing: long, short, long, short. The phrase often ends on three steadier notes. Written in 4/4. The swing is a feel you copy.",
  ),
  entry(
    "barndance",
    "Barndance",
    "Kinds of tune",
    "A dance in four with a mild swing, gentler than a reel. Written in 4/4.",
    ["barn dance"],
  ),
  entry(
    "schottische",
    "Schottische",
    "Kinds of tune",
    "A couple dance in four, steady, with a light lift. In Irish sessions it sits near the barndance.",
  ),
  entry(
    "waltz",
    "Waltz",
    "Kinds of tune",
    "Three even beats in a bar, written in 3/4. Slower and smoother than a mazurka.",
  ),
  entry(
    "mazurka",
    "Mazurka",
    "Kinds of tune",
    "A dance in three, written in 3/4, with a lift on the second beat. Heard often in Donegal.",
  ),
  entry(
    "strathspey",
    "Strathspey",
    "Kinds of tune",
    "A Scottish dance in four, slower than a reel. A short note jumps into a longer one, the Scotch snap, and the pattern can flip the other way. Some Irish sessions play them, especially in the north.",
    ["Scotch snap"],
  ),
  entry(
    "highland",
    "Highland",
    "Kinds of tune",
    "A Scottish dance tune in four, with a dotted lift, often played in a set with marches and reels. Also called a fling.",
    ["fling", "highland fling"],
  ),
  entry(
    "galop",
    "Galop",
    "Kinds of tune",
    "A fast dance in two, written in 2/4. The Winster Gallop is one.",
    ["gallop"],
  ),
  entry(
    "set",
    "Set",
    "Kinds of tune",
    "Two or more tunes of one kind, played one into the next. A set of reels, a set of jigs.",
    ["set of tunes"],
  ),
  entry(
    "set-dance",
    "Set dance",
    "Kinds of tune",
    "A solo dance with its own tune and its own length of phrase, such as The Blackbird.",
  ),
  entry(
    "planxty",
    "Planxty",
    "Kinds of tune",
    "Turlough O’Carolan’s word for a tune in someone’s honor. Often harp-like. The time may walk like a march, stretch like an air, or lilt like a jig.",
    ["O'Carolan", "Carolan"],
  ),
  entry(
    "lift",
    "Lift",
    "Kinds of tune",
    "The bounce in a dance tune: some notes a little longer, some a little shorter, so the beat dances. A hornpipe has a strong lift. A reel runs straighter.",
    ["lilt", "swing"],
  ),

  entry(
    "traditional",
    "Traditional",
    "On the page",
    "Music learned by ear and passed from player to player. The airs and dance tunes on this path are Irish traditional music.",
    ["trad"],
  ),
  entry(
    "by-ear",
    "By ear",
    "On the page",
    "Learning a phrase by hearing it, then copying it, before any picture. Every phrase here starts that way.",
  ),
  entry(
    "setting",
    "Setting",
    "On the page",
    "One way of playing a tune: the notes chosen, the ornaments, the speed. Another player’s setting can differ. The pack’s setting is written for a new low D.",
  ),
  entry(
    "session",
    "Session",
    "On the page",
    "Players in a room, taking turns to start a set. You bring a tune you can begin and hold.",
    ["seisiún", "seisun"],
  ),
  entry(
    "staff",
    "Staff",
    "On the page",
    "Five lines that show how the notes rise and fall. A map of a phrase you have already heard. Readers see it after Hear.",
    ["stave"],
  ),
  entry(
    "letter-names",
    "Letter names",
    "On the page",
    "D, E, F♯, G, A, B, C♯, and the rest. They sit on the holes as labels, once the holes are familiar.",
  ),
  entry(
    "solfege",
    "Solfege",
    "On the page",
    "Syllables for the notes: Do, Re, Mi, Fa, Sol, La, Si. Here they are fixed, with C as Do, so this whistle’s home note D is Re. Readers see them under a phrase they have already heard. Some singers say Ti for Si.",
    ["sol-fa", "fixed do"],
  ),
  entry(
    "abc",
    "ABC",
    "On the page",
    "A tune written as letters in a plain text line. The pack stores melodies that way. The lesson starts from the sound.",
  ),
  entry(
    "bar",
    "Bar",
    "On the page",
    "The beats grouped between two barlines. A jig bar holds two groups of three. A reel bar holds four beats.",
    ["measure"],
  ),
  entry(
    "strain",
    "Strain",
    "On the page",
    "A section of a tune, often eight bars. The opening is one strain. The next part is the second strain.",
  ),
  entry(
    "pickup",
    "Pickup",
    "On the page",
    "A note or two before the first strong beat. Many jigs and reels begin that way.",
    ["upbeat", "anacrusis"],
  ),
  entry(
    "time-signature",
    "Time signature",
    "On the page",
    "The two numbers that say how beats are grouped, such as 6/8 or 4/4. You can clap the grouping from the recording before you read the numbers.",
    ["meter", "time"],
  ),
  entry(
    "quaver",
    "Quaver",
    "On the page",
    "A short note. In a jig, three quavers fill one beat. In a reel, two quavers fill one beat.",
    ["eighth note"],
  ),
  entry(
    "crotchet",
    "Crotchet",
    "On the page",
    "A note twice as long as a quaver. In a reel, one crotchet is one beat.",
    ["quarter note"],
  ),
  entry(
    "metronome",
    "Metronome",
    "On the page",
    "A steady click, for when you want a dance pulse held still. Airs stay free of it.",
    ["click"],
  ),
  entry(
    "legato",
    "Legato",
    "On the page",
    "The written word for notes joined into one breath. On this path that is called on the breath.",
  ),
  entry(
    "staccato",
    "Staccato",
    "On the page",
    "Notes started one by one, with a small gap. On whistle that is usually the tongue on each note.",
  ),
  entry(
    "rubato",
    "Rubato",
    "On the page",
    "Giving and taking time inside a phrase. The way a slow air moves.",
  ),
  entry(
    "embouchure",
    "Embouchure",
    "On the page",
    "The way flute players shape the breath with their lips. A whistle’s fipple makes the sound. The lips rest on the mouthpiece and seal.",
  ),
  entry(
    "transposition",
    "Transposition",
    "On the page",
    "Moving every note of a tune by the same distance so it rests somewhere else. On a whistle, another key is another whistle.",
  ),

  entry(
    "hold",
    "Hold",
    "While you play",
    "Low D, all holes covered, kept still. A hold that settles is how the app remembers where this whistle speaks.",
  ),
  entry(
    "idle",
    "Idle",
    "While you play",
    "Ready. Nothing is being heard just now.",
  ),
  entry(
    "wait",
    "Wait",
    "While you play",
    "Your turn to play. Playback has stopped, so the microphone hears the whistle.",
  ),
  entry(
    "sounding",
    "Sounding",
    "While you play",
    "A note is coming in. The screen stays on the whistle and the remark.",
  ),
  entry(
    "feedback",
    "Feedback",
    "While you play",
    "One sentence about the attempt. Then you can play it again.",
  ),
  entry(
    "remark",
    "Remark",
    "While you play",
    "The one sentence after an attempt. The kind of thing a player might say across a table.",
  ),
  entry(
    "hear",
    "Hear",
    "While you play",
    "The reference phrase, played so you can copy it. Grading stays off until it finishes.",
  ),
  entry(
    "slower",
    "Slower",
    "While you play",
    "The same reference, at three-quarter speed, so a phrase is easier to copy. Grading stays off while it plays.",
  ),
  entry(
    "headphones",
    "Headphones",
    "While you play",
    "Over the ears, so the speaker does not feed the microphone. The whistle is what gets heard.",
  ),
];

const seen = new Set<string>();
for (const item of GLOSSARY) {
  if (seen.has(item.id)) throw new Error(`duplicate glossary id ${item.id}`);
  seen.add(item.id);
}

export function glossaryEntries(): readonly GlossaryEntry[] {
  return GLOSSARY;
}

function fold(value: string): string {
  return value
    .toLowerCase()
    .replace(/♯/g, "#")
    .replace(/♮/g, "")
    .normalize("NFD")
    .replace(/\p{M}/gu, "");
}

function sectionDomId(section: string): string {
  return "gloss-" + section.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/(^-|-$)/g, "");
}

function escapeHtml(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function rank(item: GlossaryEntry, query: string): number {
  const term = fold(item.term);
  if (term === query) return 0;
  if (term.startsWith(query)) return 1;
  if (term.includes(query)) return 2;
  if ((item.aka ?? []).some((name) => fold(name).includes(query))) return 3;
  return 4;
}

function namedHit(item: GlossaryEntry, query: string): boolean {
  return rank(item, query) < 4;
}

function haystack(item: GlossaryEntry): string {
  return fold([item.term, item.section, ...(item.aka ?? []), item.body].join("\n"));
}

function inSection(section: GlossarySection): GlossaryEntry[] {
  return GLOSSARY.filter((item) => item.section === section).sort((a, b) =>
    fold(a.term).localeCompare(fold(b.term), "en"),
  );
}

function searched(query: string): GlossaryEntry[] {
  const named = GLOSSARY.filter((item) => namedHit(item, query));
  const pool = named.length > 0 ? named : GLOSSARY.filter((item) => haystack(item).includes(query));
  return pool.sort((a, b) => {
    const byRank = rank(a, query) - rank(b, query);
    if (byRank) return byRank;
    const byTerm = fold(a.term).localeCompare(fold(b.term), "en");
    if (byTerm) return byTerm;
    return GLOSSARY_SECTIONS.indexOf(a.section) - GLOSSARY_SECTIONS.indexOf(b.section);
  });
}

function hitsFor(query: string): GlossaryEntry[] {
  const folded = fold(query).trim();
  if (!folded) return GLOSSARY;
  return searched(folded);
}

export function glossaryCountLabel(query: string): string {
  const folded = fold(query).trim();
  const count = hitsFor(query).length;
  if (folded && count === 0) return "No word under that.";
  return count === 1 ? "1 word" : `${count} words`;
}

function entryHtml(item: GlossaryEntry, showSection: boolean): string {
  const names = showSection ? [item.section, ...(item.aka ?? [])] : (item.aka ?? []);
  const aka =
    names.length > 0
      ? `<p class="gloss-aka">${names.map((name) => escapeHtml(name)).join(" · ")}</p>`
      : "";
  return `<article class="gloss-entry" id="term-${escapeHtml(item.id)}">
    <h3>${escapeHtml(item.term)}</h3>
    ${aka}
    <p class="gloss-body">${escapeHtml(item.body)}</p>
  </article>`;
}

export function glossaryListHtml(query: string): string {
  const folded = fold(query).trim();
  if (folded) {
    const hits = searched(folded);
    if (hits.length === 0) return "";
    return `<div class="card">${hits.map((item) => entryHtml(item, true)).join("")}</div>`;
  }
  return GLOSSARY_SECTIONS.map((section) => {
    const items = inSection(section);
    return `<section class="card gloss-section" id="${sectionDomId(section)}">
      <h2>${escapeHtml(section)}</h2>
      <p class="gloss-intro">${escapeHtml(INTRO[section])}</p>
      ${items.map((item) => entryHtml(item, false)).join("")}
    </section>`;
  }).join("");
}

export function glossaryShellHtml(query = ""): string {
  const jumps = GLOSSARY_SECTIONS.map(
    (section) =>
      `<button type="button" data-gloss-jump="${sectionDomId(section)}">${escapeHtml(section)}</button>`,
  ).join("");
  const hidden = query.trim() ? " hidden" : "";
  return `<div class="app-shell glossary">
    <div class="topbar">
      <p class="eyebrow">Words</p>
      <button type="button" class="ghost" id="glossary-back">Back</button>
    </div>
    <h1>Glossary</h1>
    <p class="lede">Whistle words, the gestures on a note, and the kinds of tune a session plays. Air, on this page, is a tune. The breath you blow is under Breath.</p>
    <div class="field">
      <label for="glossary-q">Find a word</label>
      <input id="glossary-q" type="search" placeholder="air, reel, cut" autocomplete="off" spellcheck="false" value="${escapeHtml(query)}" />
    </div>
    <div class="glossary-jumps" id="glossary-jumps"${hidden}>${jumps}</div>
    <p class="meta" id="glossary-count">${escapeHtml(glossaryCountLabel(query))}</p>
    <div id="glossary-list">${glossaryListHtml(query)}</div>
  </div>`;
}
