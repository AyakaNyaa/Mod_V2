// Константы из names.h
#![allow(dead_code)]

// compare variable methods
pub const CMP_EQ: u8 = 0; //  ==
pub const CMP_NEQ: u8 = 1; //  !=
pub const CMP_BIGGER_EQ: u8 = 2; //  >=
pub const CMP_SMALLER_EQ: u8 = 3; //  <=
pub const CMP_BIGGER: u8 = 4; //  >
pub const CMP_SMALLER: u8 = 5; //  <

// upgrades ids
pub const ARROWS: u8 = 0;
pub const SWORDS: u8 = 1;
pub const ARMOR: u8 = 2;
pub const SHIP_DMG: u8 = 3;
pub const SHIP_ARMOR: u8 = 4;
// 5 unused
pub const CATA_DMG: u8 = 6;

// magic ids
pub const VISION: u8 = 0;
pub const HEAL: u8 = 1;
pub const GREATER_HEAL: u8 = 2;
pub const EXORCISM: u8 = 3;
pub const FLAME_SHIELD: u8 = 4;
pub const FIREBALL: u8 = 5;
pub const SLOW: u8 = 6;
pub const INVIS: u8 = 7;
pub const POLYMORPH: u8 = 8;
pub const BLIZZARD: u8 = 9;
pub const EYE_OF_KILROG: u8 = 10;
pub const BLOOD: u8 = 11;
pub const RAISE_DEAD: u8 = 12;
pub const COIL: u8 = 13;
pub const WHIRLWIND: u8 = 14;
pub const HASTE: u8 = 15;
pub const UNHOLY_ARMOR: u8 = 16;
pub const RUNES: u8 = 17;
pub const DEATH_AND_DECAY: u8 = 18;

// players ids
pub const P_RED: u8 = 0;
pub const P_BLUE: u8 = 1;
pub const P_GREEN: u8 = 2;
pub const P_VIOLET: u8 = 3;
pub const P_ORANGE: u8 = 4;
pub const P_BLACK: u8 = 5;
pub const P_WHITE: u8 = 6;
pub const P_YELLOW: u8 = 7;
pub const PLAYER9: u8 = 8;
pub const PLAYER10: u8 = 9;
pub const PLAYER11: u8 = 10;
pub const PLAYER12: u8 = 11;
pub const PLAYER13: u8 = 12;
pub const PLAYER14: u8 = 13;
pub const PLAYER15: u8 = 14;
pub const P_NEUTRAL: u8 = 15;

// AI upgrades ids
pub const UP_ARROW1: u8 = 0x80;
pub const UP_ARROW2: u8 = 0x81;
pub const UP_RANGER: u8 = 0x82;
pub const UP_SKILL1: u8 = 0x83;
pub const UP_SKILL2: u8 = 0x84;
pub const UP_SKILL3: u8 = 0x85;
pub const UP_SWORD1: u8 = 0x86;
pub const UP_SWORD2: u8 = 0x87;
pub const UP_SHIELD1: u8 = 0x88;
pub const UP_SHIELD2: u8 = 0x89;
pub const UP_CATDMG1: u8 = 0x8A;
pub const UP_CATDMG2: u8 = 0x8B;
pub const UP_BOATATK1: u8 = 0x8C;
pub const UP_BOATATK2: u8 = 0x8D;
pub const UP_BOATARM1: u8 = 0x8E;
pub const UP_BOATARM2: u8 = 0x8F;
pub const UP_CLERIC: u8 = 0x90;
pub const UP_CLERIC1: u8 = 0x91;
pub const UP_CLERIC2: u8 = 0x92;
pub const UP_WIZARD1: u8 = 0x93;
pub const UP_WIZARD2: u8 = 0x94;
pub const UP_WIZARD3: u8 = 0x95;
pub const UP_WIZARD4: u8 = 0x96;
pub const UP_WIZARD5: u8 = 0x97;
pub const UP_KEEP: u8 = 0x98;
pub const UP_CASTLE: u8 = 0x99;

// units
pub const U_FOOTMAN: u8 = 0;
pub const U_GRUNT: u8 = 1;
pub const U_PEASANT: u8 = 2;
pub const U_PEON: u8 = 3;
pub const U_BALLISTA: u8 = 4;
pub const U_CATAPULT: u8 = 5;
pub const U_KNIGHT: u8 = 6;
pub const U_OGRE: u8 = 7;
pub const U_ARCHER: u8 = 8;
pub const U_TROLL: u8 = 9;
pub const U_MAGE: u8 = 10;
pub const U_DK: u8 = 11;
pub const U_PALADIN: u8 = 12;
pub const U_OGREMAGE: u8 = 13;
pub const U_DWARWES: u8 = 14;
pub const U_GOBLINS: u8 = 15;
pub const U_ATTACK_PEASANT: u8 = 16;
pub const U_ATTACK_PEON: u8 = 17;
pub const U_RANGER: u8 = 18;
pub const U_BERSERK: u8 = 19;
pub const U_ALLERIA: u8 = 20;
pub const U_TERON: u8 = 21;
pub const U_KURDRAN: u8 = 22;
pub const U_DENTARG: u8 = 23;
pub const U_HADGAR: u8 = 24;
pub const U_GROM: u8 = 25;
pub const U_HTANKER: u8 = 26;
pub const U_OTANKER: u8 = 27;
pub const U_HTRANSPORT: u8 = 28;
pub const U_OTRANSPORT: u8 = 29;
pub const U_HDESTROYER: u8 = 30;
pub const U_ODESTROYER: u8 = 31;
pub const U_BATTLESHIP: u8 = 32;
pub const U_JUGGERNAUT: u8 = 33;
pub const TYPE_A100: u8 = 34;
pub const U_DEATHWING: u8 = 35;
pub const TYPE_B36: u8 = 36;
pub const TYPE_B37: u8 = 37;
pub const U_SUBMARINE: u8 = 38;
pub const U_TURTLE: u8 = 39;
pub const U_FLYER: u8 = 40;
pub const U_ZEPPELIN: u8 = 41;
pub const U_GRIFON: u8 = 42;
pub const U_DRAGON: u8 = 43;
pub const U_TYRALYON: u8 = 44;
pub const U_EYE: u8 = 45;
pub const U_DANATH: u8 = 46;
pub const U_KARGATH: u8 = 47;
pub const TYPE_A5: u8 = 48;
pub const U_CHOGAL: u8 = 49;
pub const U_LOTHAR: u8 = 50;
pub const U_GULDAN: u8 = 51;
pub const U_UTER: u8 = 52;
pub const U_ZULJIN: u8 = 53;
pub const TYPE_A600: u8 = 54;
pub const U_SKELETON: u8 = 55;
pub const U_DEMON: u8 = 56;
pub const U_CRITTER: u8 = 57;
pub const U_FARM: u8 = 58;
pub const U_PIGFARM: u8 = 59;
pub const U_HBARRACK: u8 = 60;
pub const U_OBARRACK: u8 = 61;
pub const U_CHURCH: u8 = 62;
pub const U_ALTAR: u8 = 63;
pub const U_HTOWER: u8 = 64;
pub const U_OTOWER: u8 = 65;
pub const U_STABLES: u8 = 66;
pub const U_OGREMOUND: u8 = 67;
pub const U_INVENTOR: u8 = 68;
pub const U_ALCHEMIST: u8 = 69;
pub const U_AVIARY: u8 = 70;
pub const U_DRAGONROOST: u8 = 71;
pub const U_SHIPYARD: u8 = 72;
pub const U_WHARF: u8 = 73;
pub const U_TOWN_HALL: u8 = 74;
pub const U_GREAT_HALL: u8 = 75;
pub const U_HLUMBER: u8 = 76;
pub const U_OLUMBER: u8 = 77;
pub const U_HFOUNDRY: u8 = 78;
pub const U_OFOUNDRY: u8 = 79;
pub const U_MAGE_TOWER: u8 = 80;
pub const U_TEMPLE: u8 = 81;
pub const U_HSMITH: u8 = 82;
pub const U_OSMITH: u8 = 83;
pub const U_HREFINERY: u8 = 84;
pub const U_OREFINERY: u8 = 85;
pub const U_HPLATFORM: u8 = 86;
pub const U_OPLATFROM: u8 = 87;
pub const U_KEEP: u8 = 88;
pub const U_STRONGHOLD: u8 = 89;
pub const U_CASTLE: u8 = 90;
pub const U_FORTRESS: u8 = 91;
pub const U_MINE: u8 = 92;
pub const U_OIL: u8 = 93;
pub const U_HSTART: u8 = 94;
pub const U_OSTART: u8 = 95;
pub const U_HARROWTOWER: u8 = 96;
pub const U_OARROWTOWER: u8 = 97;
pub const U_HCANONTOWER: u8 = 98;
pub const U_OCANONTOWER: u8 = 99;
pub const U_CIRCLE: u8 = 100;
pub const U_PORTAL: u8 = 101;
pub const U_RUNESTONE: u8 = 102;
pub const U_HWALL: u8 = 103;
pub const U_OWALL: u8 = 104;
pub const ANY_BUILDING: u8 = 105; // dead body
pub const ANY_MEN: u8 = 106; // dead building 1x1
pub const ANY_UNITS: u8 = 107; // dead building 2x2
pub const ANY_BUILDING_2x2: u8 = 108; // dead building 3x3
pub const ANY_BUILDING_3x3: u8 = 109; // dead building 4x4
pub const ANY_BUILDING_4x4: u8 = 110; // not unit

// bullets and effects
pub const B_LIGHTNING: u8 = 0;
pub const B_HAMMER: u8 = 1;
pub const B_FIREBALL: u8 = 2;
pub const B_FIRESHIELD: u8 = 3;
pub const B_FIRESPIN: u8 = 4;
pub const B_BLIZZARD: u8 = 5;
pub const B_ROT: u8 = 6;
pub const B_BIG_CANNON: u8 = 7;
pub const B_EXORCISM: u8 = 8;
pub const B_HEAL: u8 = 9;
pub const B_COIL: u8 = 10;
pub const B_RUNE: u8 = 11;
pub const B_TORNADO: u8 = 12;
pub const B_STONE: u8 = 13;
pub const B_BOLT: u8 = 14;
pub const B_ARROW: u8 = 15;
pub const B_AXE: u8 = 16;
pub const B_HTORPEDO: u8 = 17;
pub const B_OTORPEDO: u8 = 18;
pub const B_LIGHT_FIRE: u8 = 19;
pub const B_HEAVY_FIRE: u8 = 20;
pub const B_CAT_HIT: u8 = 21;
pub const B_SPARKLE: u8 = 22;
pub const B_BOOM_FIRE: u8 = 23;
pub const B_CANNON: u8 = 24;
pub const B_SHOT_FIRE: u8 = 25;
pub const B_CANNON_HIT: u8 = 26;
pub const B_DEMON_FIRE: u8 = 27;
pub const B_CROSS: u8 = 28;
pub const B_NONE: u8 = 29;

// spells sounds
pub const SS_BLOOD: u8 = 0;
pub const SS_DECAY: u8 = 1;
pub const SS_COIL: u8 = 2;
pub const SS_EXORCISM: u8 = 3;
pub const SS_FLAMESHIELD: u8 = 4;
pub const SS_HASTE: u8 = 5;
pub const SS_HEAL: u8 = 6;
pub const SS_VISION: u8 = 7;
pub const SS_BLIZZARD: u8 = 8;
pub const SS_INVIZ: u8 = 9;
pub const SS_EYE: u8 = 10;
pub const SS_POLYMOPH: u8 = 11;
pub const SS_SLOW: u8 = 12;
pub const SS_THUNDER: u8 = 13;
pub const SS_THDARK: u8 = 14;
pub const SS_ARMOR: u8 = 15;
pub const SS_WIND: u8 = 16;

// allowed units 4 bytes
pub const A_FOOT: u8 = 0;
pub const A_PEON: u8 = 1;
pub const A_CATA: u8 = 2;
pub const A_OGRE: u8 = 3;
pub const A_ARHC: u8 = 4;
pub const A_MAGE: u8 = 5;
pub const A_TANKER: u8 = 6;
pub const A_DESTROYER: u8 = 7;
pub const A_TRANSPORT: u8 = 8;
pub const A_BATTLE: u8 = 9;
pub const A_SUB: u8 = 10;
pub const A_FLYER: u8 = 11;
pub const A_DRAGON: u8 = 12;
pub const A_UNUSED: u8 = 13;
pub const A_SAP: u8 = 14;
pub const A_AVIARY: u8 = 15;
pub const A_FARM: u8 = 16;
pub const A_BARRACK: u8 = 17;
pub const A_LUMBER: u8 = 18;
pub const A_STABLES: u8 = 19;
pub const A_TEMPLE: u8 = 20;
pub const A_FOUDNRY: u8 = 21;
pub const A_REFINERY: u8 = 22;
pub const A_INVENTOR: u8 = 23;
pub const A_ALTAR: u8 = 24;
pub const A_TOWER: u8 = 25;
pub const A_TH1: u8 = 26;
pub const A_TH2: u8 = 27;
pub const A_TH3: u8 = 28;
pub const A_SMITH: u8 = 29;
pub const A_SHIPYARD: u8 = 30;

// allowed upgrades 4 bytes
pub const A_ARROW1: u8 = 0;
pub const A_ARROW2: u8 = 1;
pub const A_SWORD1: u8 = 2;
pub const A_SWORD2: u8 = 3;
pub const A_ARMOR1: u8 = 4;
pub const A_ARMOR2: u8 = 5;
pub const A_SHIP_ATTACK1: u8 = 6;
pub const A_SHIP_ATTACK2: u8 = 7;
pub const A_SHIP_ARMOR1: u8 = 8;
pub const A_SHIP_ARMOR2: u8 = 9;
pub const A_UNUSED_SHIP1: u8 = 10;
pub const A_UNUSED_SHIP2: u8 = 11;
pub const A_CATA1: u8 = 12;
pub const A_CATA2: u8 = 13;
pub const A_UNUSED_SPEED1: u8 = 14;
pub const A_UNUSED_SPEED2: u8 = 15;
pub const A_BERSERKERS: u8 = 16;
pub const A_RANGE: u8 = 17;
pub const A_SCOUT: u8 = 18;
pub const A_MARKS: u8 = 19;

// allowed spells 4 bytes
pub const A_VISION: u8 = 0;
pub const A_HEAL: u8 = 1;
pub const A_GREATER_HEAL: u8 = 2;
pub const A_EXORCISM: u8 = 3;
pub const A_FLAME_SHIELD: u8 = 4;
pub const A_FIREBALL: u8 = 5;
pub const A_SLOW: u8 = 6;
pub const A_INVIS: u8 = 7;
pub const A_POLYMORF: u8 = 8;
pub const A_BLIZZARD: u8 = 9;
pub const A_EYE: u8 = 10;
pub const A_BLOOD: u8 = 11;
pub const A_UNUSED_HALLUCINATE: u8 = 12;
pub const A_RAISE: u8 = 13;
pub const A_COIL: u8 = 14;
pub const A_WIND: u8 = 15;
pub const A_HASTE: u8 = 16;
pub const A_UNHOLY: u8 = 17;
pub const A_RUNES: u8 = 18;
pub const A_DD: u8 = 19;
pub const A_ALTAR_UPGR: u8 = 20;

// learned spells ids
pub const L_VISION: u8 = 0;
pub const L_HEAL: u8 = 1;
pub const L_GREATER_HEAL: u8 = 2;
pub const L_EXORCISM: u8 = 3;
pub const L_FLAME_SHIELD: u8 = 4;
pub const L_FIREBALL: u8 = 5;
pub const L_SLOW: u8 = 6;
pub const L_INVIS: u8 = 7;
pub const L_POLYMORF: u8 = 8;
pub const L_BLIZZARD: u8 = 9;
pub const L_EYE: u8 = 10;
pub const L_BLOOD: u8 = 11;
pub const L_UNUSED_HALLUCINATE: u8 = 12;
pub const L_RAISE: u8 = 13;
pub const L_COIL: u8 = 14;
pub const L_WIND: u8 = 15;
pub const L_HASTE: u8 = 16;
pub const L_UNHOLY: u8 = 17;
pub const L_RUNES: u8 = 18;
pub const L_DD: u8 = 19;
pub const L_ALTAR_UPGR: u8 = 20;

// unit stats
pub const S_DRAW_X: u8 = 0;
pub const S_DRAW_Y: u8 = 2;
pub const S_SEQ: u8 = 4;
pub const S_SEQ_FLAG: u8 = 6;
pub const S_ANIMATION_TIMER: u8 = 7;
pub const S_ANIMATION: u8 = 8;

pub const ANIM_DEAD: u8 = 0;
pub const ANIM_DIE: u8 = 1;
pub const ANIM_STOP: u8 = 2;
pub const ANIM_MOVE: u8 = 3;
pub const ANIM_ATTACK: u8 = 4;
pub const ANIM_BUILD: u8 = 5;
pub const ANIM_TRANSPORT_SHORE: u8 = 6;

pub const S_FRAME: u8 = 9;
pub const S_FACE: u8 = 10;
pub const FACE_N: u8 = 0;
pub const FACE_NE: u8 = 1;
pub const FACE_E: u8 = 2;
pub const FACE_SE: u8 = 3;
pub const FACE_S: u8 = 4;
pub const FACE_SW: u8 = 5;
pub const FACE_W: u8 = 6;
pub const FACE_NW: u8 = 7;
// 14 sequence bytes
pub const S_X: u8 = 24;
pub const S_Y: u8 = 26;

pub const S_FLAGS1: u8 = 28;
pub const UF_EVEN_ALIGN: u8 = 0x0002; // Unit should be even matrix
pub const UF_AIR: u8 = 0x0004; // Unit should use the AIR map
pub const UF_WATER: u8 = 0x0008; // Unit is in the water
pub const UF_BUILD_ON: u8 = 0x0010; //
pub const UF_BUILD_CANCEL: u8 = 0x0020; //
pub const UF_UNLOAD_ALL: u8 = 0x0040; // used by transports

pub const S_FLAGS2: u8 = 29;
pub const UF_RESCUE: u16 = 0x0100; // unit is being res (2-byte flag, как в C-int define)

pub const S_FLAGS3: u8 = 30;
pub const SF_UNIT_FREE: u8 = 0x0001; // unit not used
pub const SF_DIEING: u8 = 0x0002; // death sequence
pub const SF_DEAD: u8 = 0x0004; // no longer on map
pub const SF_HIDDEN: u8 = 0x0008; // inside building
pub const SF_SKIP_BOX: u8 = 0x0010; // not in visible window
pub const SF_PREPLACED: u8 = 0x0020; // unit was already on the map
pub const SF_TELEPORT: u8 = 0x0040; // 0040 is free mem (now used for teleport flag)
pub const SF_COMPLETED: u8 = 0x0080; // unit completed building

pub const S_FLAGS4: u8 = 31;
pub const SF_BUILD_FOUNDATION: u8 = 0x01; // early stages of building
pub const SF_INHIBIT_RETARG: u8 = 0x02;
pub const SF_MAP_HEADER: u8 = 0x04; // listed in map header
pub const SF_PAUSE_TOGGLE: u8 = 0x08; // used to tell a peon to pause
pub const SF_IS_PAUSED: u8 = 0x10; // set when peon actually pauses
pub const SF_SELECTED: u8 = 0x20; // selected by local player
pub const SF_TARGET_CHANGED: u8 = 0x40;
pub const SF_UNDER_ATTACK: u8 = 0x80;

pub const S_AIFLAGS1: u8 = 32;
pub const S_AIFLAGS2: u8 = 33;
pub const S_HP: u8 = 34;
pub const S_SOUND_COUNTER: u8 = 36;
pub const S_COMMANDS: u8 = 37;
pub const S_MANA: u8 = 38;
pub const S_ID: u8 = 39;
// 40 41 vision mask
pub const S_MOVEMENT_TYPE: u8 = 42;

pub const MOV_LAND: u8 = 0;
pub const MOV_AIR: u8 = 1;
pub const MOV_WATER: u8 = 2;
pub const MOV_SHORE: u8 = 3;

pub const S_SPRITE: u8 = 43;
pub const S_OWNER: u8 = 44;
pub const S_COLOR: u8 = 45;
pub const S_ORDER: u8 = 46;
pub const S_NEXT_ORDER: u8 = 47;

pub const ORDER_DEAD: u8 = 0;
pub const ORDER_DIE: u8 = 1;
pub const ORDER_STOP: u8 = 2;
pub const ORDER_MOVE: u8 = 3;
pub const ORDER_MOVE_PATROL: u8 = 4;
pub const ORDER_PATROL: u8 = 5;
pub const ORDER_FOLLOW: u8 = 6;
pub const ORDER_FOLLOW_GUARD: u8 = 7;
pub const ORDER_ATTACK: u8 = 8;
pub const ORDER_ATTACK_TARGET: u8 = 9;
pub const ORDER_ATTACK_AREA: u8 = 10;
pub const ORDER_ATTACK_WALL: u8 = 11;
pub const ORDER_DEFEND: u8 = 12;
pub const ORDER_STAND: u8 = 13;
pub const ORDER_STAND_ATTACK: u8 = 14;
pub const ORDER_DEFEND_GROUND: u8 = 15;
pub const ORDER_DEFEND_STOPPED: u8 = 16;
pub const ORDER_ATTACK_GROUND: u8 = 17;
pub const ORDER_ATTACK_GROUND_MOVE: u8 = 18;
pub const ORDER_DEMOLISH: u8 = 19;
pub const ORDER_DEMOLISH_NEAR: u8 = 20;
pub const ORDER_DEMOLISH_AT: u8 = 21;
pub const ORDER_PEON_BUILD: u8 = 22;
pub const ORDER_HARVEST: u8 = 23;
pub const ORDER_RETURN: u8 = 24;
pub const ORDER_ENTER: u8 = 25;
pub const ORDER_LEAVE: u8 = 26;
pub const ORDER_REPAIR: u8 = 27;
pub const ORDER_CREATE_BLDG: u8 = 28;
pub const ORDER_UNLOAD_ALL: u8 = 29;
pub const ORDER_DOCK: u8 = 30;
pub const ORDER_UNDOCK: u8 = 31;
pub const ORDER_WAIT: u8 = 32;
pub const ORDER_BLDG_WAIT: u8 = 33;
pub const ORDER_ENTER_TRANSPORT: u8 = 34;
pub const ORDER_LEAVE_TRANSPORT: u8 = 35;
pub const ORDER_TRAVELING: u8 = 36;
pub const ORDER_BLDG_BUILD: u8 = 37;
pub const ORDER_SPELL_VISION: u8 = 38;
pub const ORDER_SPELL_HEAL: u8 = 39;
pub const ORDER_SPELL_AREAHEAL: u8 = 40;
pub const ORDER_SPELL_EXORCISM: u8 = 41;
pub const ORDER_SPELL_FIRESHIELD: u8 = 42;
pub const ORDER_SPELL_FIREBALL: u8 = 43;
pub const ORDER_SPELL_SLOW: u8 = 44;
pub const ORDER_SPELL_INVIS: u8 = 45;
pub const ORDER_SPELL_POLYMORPH: u8 = 46;
pub const ORDER_SPELL_BLIZZARD: u8 = 47;
pub const ORDER_SPELL_EYE: u8 = 48;
pub const ORDER_SPELL_BLOODLUST: u8 = 49;
pub const ORDER_SPELL_RAISEDEAD: u8 = 50;
pub const ORDER_SPELL_DRAINLIFE: u8 = 51;
pub const ORDER_SPELL_WHIRLWIND: u8 = 52;
pub const ORDER_SPELL_HASTE: u8 = 53;
pub const ORDER_SPELL_ARMOR: u8 = 54;
pub const ORDER_SPELL_RUNES: u8 = 55;
pub const ORDER_SPELL_ROT: u8 = 56;
pub const ORDER_RESURRECT: u8 = 57;
pub const ORDER_WAIT_CONVERT: u8 = 58;
pub const ORDER_VICTORY_CIRCLE: u8 = 59;
pub const ORDER_NONE: u8 = 60;

pub const S_MOV_PATH01: u8 = 49;
// 20 movement path bytes

pub const S_INVIZ: u8 = 68;
pub const S_SHIELD: u8 = 70;
pub const S_BLOOD: u8 = 72;
pub const S_HASTE: u8 = 74;
pub const S_AI_SPELLS: u8 = 76;
/*
    US_DRAIN			0x0001
    US_POLYMORPH		0x0002
    US_VISION			0x0004
    US_EXORCISM			0x0008
    US_HEAL				0x0010
    US_RAISE			0x0020
    US_WHIRLWIND		0x0040
*/
pub const S_NEXT_FIRE: u8 = 78;
pub const S_FLASH_COLOR: u8 = 80;
pub const S_FLASH_COUNTER: u8 = 81;
// 82 83 unknown bytes
pub const S_ATTACKER_POINTER: u8 = 84;
// ai bytes
pub const S_AI_DEST_X: u8 = 88;
pub const S_AI_DEST_Y: u8 = 90;
pub const S_AI_DEST_REGION: u8 = 92;
pub const S_AI_ORDER: u8 = 94;

pub const AI_ORDER_NONE: u8 = 0;
pub const AI_ORDER_DEFEND: u8 = 1;
pub const AI_ORDER_ATTACK: u8 = 2;
pub const AI_ORDER_TRANSPORT: u8 = 3;
pub const AI_ORDER_PATROL: u8 = 4;
pub const AI_ORDER_GUARD: u8 = 5;
pub const AI_ORDER_SHIP_PATROL: u8 = 6;

pub const S_AI_AIFLAGS: u8 = 95;

// ai flags
pub const AI_PLEASE_TRANSPORT: u8 = 0x1;
pub const AI_PASSIVE: u8 = 0x2;

pub const S_NEXT_UNIT_POINTER: u8 = 104;

// peon type
pub const S_LAST_HARVEST_X: u8 = 108;
pub const S_LAST_HARVEST_Y: u8 = 110;
pub const S_PEON_GOLDMINE_POINTER: u8 = 112;
pub const S_PEON_TREE_CHOPS: u8 = 116;
pub const S_PEON_FLAGS: u8 = 117;

pub const PEON_HARVEST_GOLD: u8 = 0x80;
pub const PEON_HARVEST_LUMBER: u8 = 0x40;
pub const PEON_LOADED: u8 = 0x20;
pub const PEON_ENTERED: u8 = 0x10;
pub const PEON_SAVED_LOCATION: u8 = 0x08;
pub const PEON_IN_CASTLE: u8 = 0x04;
pub const PEON_CHOPPING: u8 = 0x02;

pub const S_PEON_AI_PARM: u8 = 118;

// build type
pub const S_BUILD_ORDER: u8 = 108;
pub const S_BUILD_TYPE: u8 = 109;
pub const S_BUILD_PROGRES: u8 = 110;
pub const S_BUILD_PROGRES_TOTAL: u8 = 112;

pub const S_KILLS: u8 = 120; // unknown byte used for kills
// byte 121?
pub const S_ATTACK_COUNTER: u8 = 122;
// unknown bytes

// peon type 2
pub const S_PEON_BUILD: u8 = 127;
pub const S_PEON_BUILD_X: u8 = 128;
pub const S_PEON_BUILD_Y: u8 = 130;

// build type 2
pub const S_RESOURCES: u8 = 130;
// byte 131?

pub const S_ORDER_X: u8 = 132;
pub const S_ORDER_Y: u8 = 134;
pub const S_ORDER_UNIT_POINTER: u8 = 136;
// unknown byte 140
pub const S_RETARGET_ORDER: u8 = 141;
// unknown byte timer? 142
// unknown byte 143
pub const S_RETARGET_X1: u8 = 144;
pub const S_RETARGET_Y1: u8 = 146;
pub const S_RETARGET_X2: u8 = 148;
pub const S_RETARGET_Y2: u8 = 150;
// 152 END
