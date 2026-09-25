// Адреса и флаги из defs.h
#![allow(dead_code)]

pub const CIRCLE_UNITS: u32 = 0x004AD304;
pub const RESCUED_UNITS: u32 = 0x004ACBE0;
pub const LOST_BUILDINGS: u32 = 0x004ACB6C;
pub const LOST_UNITS: u32 = 0x004AD368;
pub const KILLS_BUILDINGS: u32 = 0x004ACE80;
pub const KILLS_UNITS: u32 = 0x004AD328;
pub const ALL_BUILDINGS: u32 = 0x004AD9B4;
pub const ALL_UNITS: u32 = 0x004ADA7C;
pub const INVENTOR: u32 = 0x004AD9D8;
pub const LUMBERMILL: u32 = 0x004AD9FC;
pub const DRAGON: u32 = 0x004ADA1C;
pub const GRUNT: u32 = 0x004ADA3C;
pub const ACTIVE_MAGUS: u32 = 0x004ADA5C;
pub const ACTIVE_DESTROYER: u32 = 0x004ADA9C;
pub const ACTIVER_ARCHER: u32 = 0x004ADABC;
pub const TH1: u32 = 0x004ADADC;
pub const RUNESTONE: u32 = 0x004ADAFC;
pub const BATTLESHIP: u32 = 0x004ADB1C;
pub const ACTIVE_SAPPERS: u32 = 0x004ADB3C;
pub const PORTAL: u32 = 0x004ADB5C;
pub const BARRACKS: u32 = 0x004ADB7C;
pub const ACTIVE_TRANSPORT: u32 = 0x004ADB9C;
pub const ALTAR: u32 = 0x004ADBBC;
pub const SAPPERS: u32 = 0x004ADBDC;
pub const TANKER: u32 = 0x004ADBFC;
pub const FOOD_LIMIT: u32 = 0x004ADC1C;
pub const ARCHER: u32 = 0x004ADC3C;
pub const ACTIVE_WATER: u32 = 0x004ADC5C;
pub const TH3: u32 = 0x004ADC7C;
pub const STABLES: u32 = 0x004ADC9C;
pub const KNIGHT: u32 = 0x004ADCBC;
pub const CATAPULT: u32 = 0x004ADCDC;
pub const SMITH: u32 = 0x004ADCFC;
pub const SHIPYARD: u32 = 0x004ADD1C;
pub const BUILINGS_: u32 = 0x004ADD3C;
pub const ACTIVE_GRUNT: u32 = 0x004ADD5C;
pub const PEON: u32 = 0x004ADD7C;
pub const OIL_PLATFORM: u32 = 0x004ADD9C;
pub const FOUNDRY: u32 = 0x004ADDBC;
pub const ACTIVE_DRAGON: u32 = 0x004ADDDC;
pub const ACTIVE_BATTLESHIP: u32 = 0x004ADDFC;
pub const SUBMARINE: u32 = 0x004ADE1C;
pub const TOWER: u32 = 0x004ADE3C;
pub const WAR_BUILDINGS: u32 = 0x004ADE5C;
pub const DESTROYER: u32 = 0x004ADE7C;
pub const TH2: u32 = 0x004ADE9C;
pub const FLYER: u32 = 0x004ADEBC;
pub const MAGUS: u32 = 0x004ADEDC;
pub const NPC: u32 = 0x004ADEFC;
pub const ACTIVE_SUBMARINE: u32 = 0x004ADF1C;
pub const REFINERY: u32 = 0x004ADF3C;
pub const ACTIVE_AIR: u32 = 0x004ADF5C;
pub const ACTIVE_FLYER: u32 = 0x004ADF7C;
pub const TRANSPORT: u32 = 0x004ADF9C;
pub const AVIARY: u32 = 0x004ADFBC;
pub const ACTIVE_CATAPULT: u32 = 0x004ADFDC;
pub const ACTIVE_RANGER: u32 = 0x004ADFFC;
pub const ACTIVE_KNIGHT: u32 = 0x004AE01C;
pub const RANGER: u32 = 0x004AE03C;
pub const FARM: u32 = 0x004AE05C;

pub const TRIG_TIME: u32 = 0x004599C9;
pub const GB_MULTIPLAYER: u32 = 0x004B497E;
pub const LOCAL_PLAYER: u32 = 0x004ACB18;
pub const LEVEL_OBJ: u32 = 0x004AD300;

pub const CHEATBITS: u32 = 0x004AD98C;
pub const PLAYER_CHEATED: u32 = 0x004B48D6;

pub const VIZ: u32 = 0x004AB984;
pub const ALLY: u32 = 0x004ABD54;

pub const GOLD: u32 = 0x004ABAC8;
pub const LUMBER: u32 = 0x004ACB1C;
pub const OIL: u32 = 0x004ABBAC;
pub const GOLD_TOTAL: u32 = 0x004AB9C4;
pub const LUMBER_TOTAL: u32 = 0x004AD3A8;
pub const OIL_TOTAL: u32 = 0x004ABC98;

pub const GB_HORSES: u32 = 0x004AD354; // GB_HORSES is ussles massive in game (16 bytes)
// BUT this massive saved to *.sav file and those 16 bytes can be used to load values between games

pub const MANACOST: u32 = 0x004A1C88;
pub const UPGRD: u32 = 0x004958D8;
pub const FIREBALL_DMG: u32 = 0x00410A9D;
pub const FIRESHIELD_FLYERS: u32 = 0x0044291F;

pub const INVIZ_TIME: u32 = 0x00442b0A;
pub const BLOOD_TIME: u32 = 0x00442d6A;
pub const SHIELD_TIME: u32 = 0x0044345F;
pub const HASTE_TIME1: u32 = 0x004433B3;
pub const HASTE_TIME2: u32 = 0x004433B8;
pub const SLOW_TIME1: u32 = 0x00442A63;
pub const SLOW_TIME2: u32 = 0x00442A68;

pub const CAMERA_Y: u32 = 0x004ABEF8;
pub const CAMERA_X: u32 = 0x004ABEFA;

pub const UNITS_LISTS: u32 = 0x004BE214;
pub const UNITS_MASSIVE: u32 = 0x004AEC44;
pub const UNITS_NUMBER: u32 = 0x004AE220;
pub const UNITS_SELECTED: u32 = 0x004BB6D8;

pub const UNIT_SOUNDS_READY_TABLE: u32 = 0x00498BD4;
pub const UNIT_SIZE_TABLE: u32 = 0x004CEE1C;
pub const UNIT_HP_TABLE: u32 = 0x004CFD34;
pub const UNIT_ARMOR_TABLE: u32 = 0x004CF974;
pub const UNIT_STRENGTH_TABLE: u32 = 0x004CF6DC;
pub const UNIT_PIERCE_TABLE: u32 = 0x004CEB5C;
pub const UNIT_GOLD_TABLE: u32 = 0x004CE8EC;
pub const UNIT_LUMBER_TABLE: u32 = 0x004CEC3C;
pub const UNIT_OIL_TABLE: u32 = 0x004CEDAC;
pub const UNIT_TIME_TABLE: u32 = 0x004CFE80;
pub const UNIT_RANGE_TABLE: u32 = 0x004CFEF0;
pub const UNIT_VISION_FUNCTIONS_TABLE: u32 = 0x004CF7BC;
pub const UNIT_MULTISELECTABLE: u32 = 0x004CE960;

pub const ORDER_FUNCTIONS: u32 = 0x00495FCC;
pub const GW_ACTION_TYPE: u32 = 0x004BE258;
pub const UNIT_RUN_UNIT_POINTER: u32 = 0x004AB844;

pub const GLOBAL_MOVEMENT_TERRAIN_FLAGS: u32 = 0x00496CA0;
pub const MAP_ERA: u32 = 0x004ACC10;
pub const MAP_SIZE: u32 = 0x004ACBDC;
pub const MAP_CELLS_POINTER: u32 = 0x004AD5CC;
pub const MAP_REG_POINTER: u32 = 0x004AD600;
pub const CAN_PLACE_TBL: u32 = 0x004B5064;

pub const MAP_SQ_POINTER: u32 = 0x004AD5C0;
pub const SQ_BOMB: u16 = 0x1000; // mine/bomb on square
pub const SQ_BUILDING: u16 = 0x0800; // building on square
pub const SQ_AI_BUILDING: u16 = 0x0400; // ICE wants to build on square
pub const SQ_MAN_AIR: u16 = 0x0200; // flying unit on square
pub const SQ_MAN: u16 = 0x0100; // man standing on square
pub const SQ_UNPASSABLE: u16 = 0x0080; // unpassable regular terrain (also used for normal shore tiles)
pub const SQ_WATER: u16 = 0x0040; // water is normally unpassable
pub const SQ_VICTORY: u16 = 0x0020; // victory square
pub const SQ_UNBUILDABLE: u16 = 0x0010; // can't place building
pub const SQ_C_WALL: u16 = 0x0008; // computer wall
pub const SQ_P_WALL: u16 = 0x0004; // player wall
pub const SQ_SHORE: u16 = 0x0002; // unpassable unless WATER_CANDOCK
pub const SQ_LAND: u16 = 0x0001; // unpassable for ships
pub const SQ_NONE: u16 = 0x0000;

pub const CONTROLER_TYPE: u32 = 0x004ACB5C;
pub const C_PLAYER: u8 = 0;
pub const C_COMP: u8 = 1;
pub const C_PASSIVE_COMP: u8 = 2;
pub const C_NOBODY: u8 = 3;

pub const ALLOWED_UNITS: u32 = 0x004ACAD8;
pub const ALLOWED_UPGRADES: u32 = 0x004ACEA4;
pub const ALLOWED_SPELLS: u32 = 0x004ACB9C;
pub const SPELLS_LEARNED: u32 = 0x004ABEFC;

pub const GB_ARROWS: u32 = 0x004ABD40;
pub const GB_SWORDS: u32 = 0x004ACE54;
pub const GB_SHIELDS: u32 = 0x004ABEE8;
pub const GB_BOAT_ATTACK: u32 = 0x004ACE44;
pub const GB_BOAT_ARMOR: u32 = 0x004ABB30;
pub const GB_CAT_DMG: u32 = 0x004ABD18;
pub const GB_RANGER: u32 = 0x004ACDDC;
pub const GB_MARKS: u32 = 0x004ACC14;
pub const GB_LONGBOW: u32 = 0x004AB968;
pub const GB_SCOUTING: u32 = 0x004ACB8C;

pub const UNIT_GLOBAL_FLAGS: u32 = 0x004CF524;
pub const IS_WALKING: u32 = 0x00000001;
pub const IS_FLYER: u32 = 0x00000002;
pub const IS_ROLLING: u32 = 0x00000004;
pub const IS_SHIP: u32 = 0x00000008;
pub const IS_MONSTER: u32 = 0x00000010;
pub const IS_BLDG: u32 = 0x00000020;
pub const IS_SUB: u32 = 0x00000040;
pub const IS_SEESUBS: u32 = 0x00000080;
pub const IS_PEON: u32 = 0x00000100;
pub const IS_TANKER: u32 = 0x00000200;
pub const IS_TRANSPORT: u32 = 0x00000400;
pub const IS_OILRIG: u32 = 0x00000800;
pub const IS_TOWNHALL: u32 = 0x00001000;
pub const IS_DEAD: u32 = 0x00002000;
pub const IS_ATTACKS_GROUND: u32 = 0x00004000;
pub const IS_UNDEAD: u32 = 0x00008000;
pub const IS_SHORE_BLDG: u32 = 0x00010000;
pub const IS_CASTER: u32 = 0x00020000;
pub const IS_LUMBER: u32 = 0x00040000;
pub const IS_ATTACKER: u32 = 0x00080000;
pub const IS_TOWER: u32 = 0x00100000;
pub const IS_OILPATCH: u32 = 0x00200000;
pub const IS_GOLDMINE: u32 = 0x00400000;
pub const IS_NPC: u32 = 0x00800000;
pub const IS_RETURN_OIL: u32 = 0x01000000;
pub const IS_BOMBER: u32 = 0x02000000;
pub const IS_WIZARD: u32 = 0x04000000;
pub const IS_FLESHY: u32 = 0x08000000;
pub const IS_MAN: u32 = IS_WALKING | IS_FLYER | IS_ROLLING | IS_SHIP | IS_MONSTER;
pub const IS_NON_AGGRESSIVE: u32 = IS_PEON | IS_WIZARD | IS_BOMBER | IS_TANKER;

// upgrades (F_TECH_* from original defs.h are commented out there - skipped)

pub const VICTORY_JMP: u32 = 0x004581D7;
pub const VICTORY_TRIGGER: u32 = 0x004581F3;

pub const DMG_FIX: u32 = 0x004184F7; // hadgar+portal

pub const BUILD_ALL_BUILDINGS1: u32 = 0x00455BE8;
pub const BUILD_ALL_BUILDINGS2: u32 = 0x00475FD4;
pub const BUILD_ALL_BUILDINGS3: u32 = 0x00475FC4;
pub const BUILD_ALL_UNITS1: u32 = 0x0040D9F6;
pub const BUILD_ALL_UNITS2: u32 = 0x0040D9FC;

pub const SPEED_STAT_UNITS: u32 = 0x00445E16;
pub const SPEED_STAT_CATS: u32 = 0x00446695;
pub const SPEED_STAT_ARCHERS: u32 = 0x00446C87;
pub const SPEED_STAT_BERSERKERS: u32 = 0x00447579;
pub const SPEED_STAT_SHIPS: u32 = 0x0044792A;

pub const CHAT_POINTER: u32 = 0x004B3FB4;

pub const COMPS_VIZION: u32 = 0x0045216B;

pub const MULTICAST_FIX: u32 = 0x00444477;

pub const LOSE_SHOW_TABLE: u32 = 0x00459C14;
pub const WIN_SHOW_TABLE: u32 = 0x00459AEC;
pub const ENDGAME_STATE: u32 = 0x004AB98C;

pub const RAISE_DEAD_DOING_SPELL1: u32 = 0x00442D89;
pub const RAISE_DEAD_DOING_SPELL2: u32 = 0x00442DA2;

pub const DEMON_STATS_DRAW: u32 = 0x004A40E4;
pub const CRITTER_STATS_DRAW: u32 = 0x004A40F4;

pub const PEASANT_BUTTONS: u32 = 0x004A39A8;
pub const PEON_BUTTONS: u32 = 0x004A39B0;
pub const PEASANT_RE_BUTTONS: u32 = 0x004A2228;
pub const DEAD_BLDG_BUTTONS: u32 = 0x004A3D00;
pub const DEAD_BLDG2_BUTTONS: u32 = 0x004A3CF0;
pub const DEAD_BLDG3_BUTTONS: u32 = 0x004A3CF8;
pub const FARM_BUTTONS: u32 = 0x004A3B68;
pub const CIRCLE_BUTTONS: u32 = 0x004A3CB8;
pub const CHURCH_BUTTONS: u32 = 0x004A3B88;
pub const HUMAN_TH1_BUTTONS: u32 = 0x004A3BE8;
pub const HUMAN_TH2_BUTTONS: u32 = 0x004A3C58;
pub const HUMAN_TH3_BUTTONS: u32 = 0x004A3C68;
pub const ORC_TH1_BUTTONS: u32 = 0x004A3BF0;
pub const ORC_TH2_BUTTONS: u32 = 0x004A3C60;
pub const ORC_TH3_BUTTONS: u32 = 0x004A3C70;
pub const HUMAN_TH_COMMON: u32 = 0x004A3474;
pub const HUMAN_TH_ONE_BUTTON: u32 = 0x004A3A18;

pub const REPAIR_FLAG_CHECK: u32 = 0x00436A3C;
pub const REPAIR_FLAG_CHECK2: u32 = 0x00436A3E;
pub const REPAIR_CODE_CAVE: u32 = 0x00436AB5;

pub const DAMAGE_AREA1: u32 = 0x004183FB;
pub const DAMAGE_AREA2: u32 = 0x00418335;
pub const DAMAGE_AREA3: u32 = 0x00418418;
pub const DAMAGE_AREA4: u32 = 0x00418351;
pub const DAMAGE_UNIT_VS_UNIT: u32 = 0x00410744;

pub const BLOOD_CALC1: u32 = 0x0041823F;
pub const BLOOD_CALC2: u32 = 0x004181A3;

pub const RIGHT_CLICK_ALLOW_BUILDINGS: u32 = 0x0043BA65;
pub const RIGHT_CLICK_1: u32 = 0x0043B8BA;
pub const RIGHT_CLICK_2: u32 = 0x0043B90A;
pub const RIGHT_CLICK_CODE_CAVE: u32 = 0x0043B962;

pub const SGW_REPAIR_PEONS: u32 = 0x004B500C;
pub const SGW_GOLD_PEONS: u32 = 0x004B4FCC;
pub const SGW_TREE_PEONS: u32 = 0x004B4FEC;

pub const RUNEMAP_TIMERS: u32 = 0x004ABB44;
pub const RUNEMAP_X: u32 = 0x004ABC30;
pub const RUNEMAP_Y: u32 = 0x004ABC64;

pub const AIFIX_PEONS_REP: u32 = 0x00439022;
pub const AIFIX_GOLD_LUMB1: u32 = 0x004392C0;
pub const AIFIX_GOLD_LUMB2: u32 = 0x004392DF;
pub const AIFIX_BUILD_SIZE: u32 = 0x0043A3FF;
pub const AIFIX_FIND_HOME: u32 = 0x004271E3;
pub const AIFIX_DD_BLIZ_FIX: u32 = 0x0042626F;
pub const AIFIX_POWERBUILD: u32 = 0x004390AD;
pub const AIFIX_CATA_AFRAID: u32 = 0x0040B1AB;
pub const AIFIX_SHIPS_PATROL: u32 = 0x0049D934;
pub const AIFIX_INVIZ_COND: u32 = 0x00425C79;
pub const AIFIX_BLIZ_3MP1: u32 = 0x0042558E;
pub const AIFIX_BLIZ_3MP2: u32 = 0x004256CC;
pub const AIFIX_FIREBALL_COND: u32 = 0x00425B14;
pub const AIFIX_BLIZ_COND: u32 = 0x00425B81;
pub const AIFIX_INV_POLY_JMP: u32 = 0x004255D2;
pub const AIFIX_INV_POLY_CAVE: u32 = 0x00425569;
pub const AIFIX_RUNES_INV: u32 = 0x00424F85;
pub const AIFIX_STARTING_MAGE: u32 = 0x0040F8BE;

pub const BLDG_WAIT_INVENTOR: u32 = 0x00494BC8;

pub const AIP_NEED_FLYER: u32 = 0x004AF0E6;
pub const AIP_NEED_SAP: u32 = 0x004AF0E7;

pub const F_ALWAYS_TRUE: u32 = 0x004440F0; // void
pub const F_LOSE: u32 = 0x00459BE0; // void
pub const F_WIN: u32 = 0x00459AC0; // void
pub const F_VIDEO: u32 = 0x0043B130; // video 1
pub const F_UNIT_CONVERT: u32 = 0x004532A0; // unit unit player capture
pub const F_UNIT_CREATE: u32 = 0x00451A70; // x y unit player
pub const F_UNIT_KILL: u32 = 0x004514C0; // unit
pub const F_CREATE_FLAME: u32 = 0x00411010; // unit anim1 anim2
pub const F_CAPTURE: u32 = 0x00452C70; // unit player
pub const F_XY_PASSABLE: u32 = 0x00416BC0; // x y unit
pub const F_UNIT_UNPLACE: u32 = 0x004172A0; // unit
pub const F_UNIT_PLACE: u32 = 0x00417240; // unit
pub const F_BULLET_CREATE: u32 = 0x00410000; // x y bullet
pub const F_BULLET_CREATE_UNIT: u32 = 0x00410080; // unit bullet
pub const F_SPELL_SOUND_XY: u32 = 0x004237F0; // x y sound
pub const F_SPELL_SOUND_UNIT: u32 = 0x00423830; // unit sound
pub const F_RESET_COLORS: u32 = 0x00453660; // void
pub const F_MINIMAP_CLICK: u32 = 0x00411B00; // x y
pub const F_MAP_MSG: u32 = 0x0042CA40; // msg from_who time
pub const F_SEND_CHEAT_PACKET: u32 = 0x004566D0; // cheatbits
pub const F_RAISE_DEAD: u32 = 0x00442D80; // caster
pub const F_TILE_REMOVE_TREES: u32 = 0x0044E8B0; // x y
pub const F_TILE_REMOVE_ROCKS: u32 = 0x0044E840; // x y
pub const F_TILE_REMOVE_WALLS: u32 = 0x0044E920; // x y
pub const F_NET_RANDOM: u32 = 0x004798E0; // return int
pub const F_ATTACK_CAN_HIT: u32 = 0x0040AC70; // attacker target
pub const F_TRAIN_UNIT: u32 = 0x0040E6D0; // unit_id
pub const F_BUILD_BUILDING: u32 = 0x00436B80; // unit_id
pub const F_CHANGE_STATUS: u32 = 0x0044A670; // unit_id
pub const F_STATUS_REDRAW: u32 = 0x0044A950; // void
pub const F_VISION2: u32 = 0x0042D030; // x y player
pub const F_VISION3: u32 = 0x0042D0B0; // x y player
pub const F_VISION4: u32 = 0x0042D130; // x y player
pub const F_VISION5: u32 = 0x0042D1B0; // x y player
pub const F_VISION6: u32 = 0x0042D230; // x y player
pub const F_VISION7: u32 = 0x0042D2B0; // x y player
pub const F_VISION8: u32 = 0x0042D330; // x y player
pub const F_VISION9: u32 = 0x0042D3B0; // x y player
pub const F_DEAD_BUILDING_REDRAW: u32 = 0x00454FC0; // unit
pub const F_GROUP_SET: u32 = 0x00423AB0; // unit
pub const F_AI_CAST: u32 = 0x00424F70; // unit
pub const F_GIVE_ORDER: u32 = 0x00451070; // unit x y target function
pub const F_TECH_BUILT: u32 = 0x0044C200; // player tech
pub const F_MTX_DIST: u32 = 0x00427830; // GPOINT* unit
pub const F_ICE_SET_AI_ORDER: u32 = 0x004275B0; // unit AI_ORDER GPOINT*
pub const F_BLDG_START_BUILD: u32 = 0x0040E2A0; // building id build_order
pub const F_MTX_UNIT_ON_REGION: u32 = 0x00416980; // unit region(vict_region=4ABFA0)
pub const F_UNIT_FIXUP: u32 = 0x00451100; // unit save (return unit)
