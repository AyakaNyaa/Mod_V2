// Инициализация: замены по умолчанию, установка триггера, хуки (w2p.cpp:5746-5974)
#![allow(dead_code)]

use crate::ai::*;
use crate::buildui::*;
use crate::damage::*;
use crate::defs::*;
use crate::fixes::*;
use crate::heroes::*;
use crate::names::*;
use crate::net::*;
use crate::orig_fn;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::place::*;
use crate::player::*;
use crate::saveload::*;
use crate::state::*;
use crate::tick::*;
use crate::uihooks::*;
use crate::victory::*;

pub unsafe fn replace_def() {
    // set all vars to default
    ua = [255; 255];
    ut = [255; 255];
    runes = [0; 9];
    hhero = [0; 180];
    ohero = [0; 180];
    heros = [0; 16];
    herosb = [false; 16];
    m_slow_aura = [255; 255];
    m_death_aura = [255; 255];
    m_sneak = [255; 255];
    m_devotion = [255; 255];
    m_vampire = [255; 255];
    m_prvnt = [255; 255];
    vizs_areas = [Vizs { x: 0, y: 0, p: 0, s: 0 }; 2000];
    vizs_n = 0;
    table = false;
    agr = false;
    cpt = false;
    pcpt = false;
    thcpt = false;
    ucpt = false;
    steal = false;
    aport = false;
    mport = false;
    b3rune = false;
    b3port = false;
    b3cirl = false;
    b3mine = false;
    b3forest = false;
    apn = false;
    manaburn = false;
    A_runestone = false;
    A_portal = false;
    A_transport = false;
    A_autoheal = false;
    blood_f = false;
    more_res = false;
    path_fixed = false;
    ai_fixed = false;
    saveload_fixed = false;
    peon_steal = false;
}

pub unsafe fn replace_common() {
    // peon can build any buildings
    let ballbuildings = [0x0u8, 0x0]; // d1 05
    patch_set!(BUILD_ALL_BUILDINGS1, ballbuildings);
    let ballbuildings2 = [0x0u8]; // 0a
    patch_set!(BUILD_ALL_BUILDINGS2, ballbuildings2);
    let ballbuildings3 = [0x0u8]; // 68
    patch_set!(BUILD_ALL_BUILDINGS3, ballbuildings3);

    // any building can train any unit
    let ballunits = [0xebu8]; // 0x74
    patch_set!(BUILD_ALL_UNITS1, ballunits);
    let ballunits2 = [0xA1u8, 0xBC, 0x47, 0x49, 0x0, 0x90, 0x90]; // 8b 04 85 bc 47 49 00
    patch_set!(BUILD_ALL_UNITS2, ballunits2);

    // show kills
    let d = S_KILLS;
    let mut sdmg = [0x8au8, 0x90, 0x82, 0x0, 0x0, 0x0, 0x8b, 0xfa]; // units
    sdmg[2] = d;
    patch_set!(SPEED_STAT_UNITS, sdmg);
    let mut sdmg2 = [0x8au8, 0x82, 0x82, 0x0, 0x0, 0x0, 0x90, 0x90, 0x90]; // catas
    sdmg2[2] = d;
    patch_set!(SPEED_STAT_CATS, sdmg2);
    let mut sdmg3 = [0x8au8, 0x88, 0x82, 0x0, 0x0, 0x0, 0x90, 0x90, 0x90]; // archers
    sdmg3[2] = d;
    patch_set!(SPEED_STAT_ARCHERS, sdmg3);
    let mut sdmg4 = [0x8au8, 0x82, 0x82, 0x0, 0x0, 0x0, 0x90, 0x90, 0x90]; // berserkers
    sdmg4[2] = d;
    patch_set!(SPEED_STAT_BERSERKERS, sdmg4);
    let mut sdmg5 = [0x8au8, 0x88, 0x82, 0x0, 0x0, 0x0, 0x90, 0x90, 0x90]; // ships
    sdmg5[2] = d;
    patch_set!(SPEED_STAT_SHIPS, sdmg5);

    let dmg_fix = [0xebu8];
    patch_set!(DMG_FIX, dmg_fix);

    th_change(true);
    draw_stats_fix(true);
}

pub unsafe fn replace_back() {
    // replace all to default
    comps_vision(false);
    sheep(false);
    build3(false);
    demon(false);
    th_change(false);
    repair_all(false);
    repair_cat(false);
    trigger_time(0xc8);
    fireball_dmg(40);
    buff_time(0, 0xd0, 0x7);
    buff_time(1, 0xe8, 0x3);
    buff_time(2, 0xf4, 0x1);
    buff_time(3, 0xe8, 0x3);
    buff_time(4, 0xe8, 0x3);
    upgr(SWORDS, 2);
    upgr(ARMOR, 2);
    upgr(ARROWS, 1);
    upgr(SHIP_DMG, 5);
    upgr(SHIP_ARMOR, 5);
    upgr(CATA_DMG, 15);
    manacost(VISION, 70);
    manacost(HEAL, 6);
    manacost(GREATER_HEAL, 5);
    manacost(EXORCISM, 4);
    manacost(FIREBALL, 100);
    manacost(FLAME_SHIELD, 80);
    manacost(SLOW, 50);
    manacost(INVIS, 200);
    manacost(POLYMORPH, 200);
    manacost(BLIZZARD, 25);
    manacost(EYE_OF_KILROG, 70);
    manacost(BLOOD, 50);
    manacost(RAISE_DEAD, 50);
    manacost(COIL, 100);
    manacost(WHIRLWIND, 100);
    manacost(HASTE, 50);
    manacost(UNHOLY_ARMOR, 100);
    manacost(RUNES, 200);
    manacost(DEATH_AND_DECAY, 25);
    no_random_dmg(false);
    blood_fix(false);
    draw_stats_fix(false);
    brclik(false);
    rc_jmp(false);
    autoheal(false);
    multicast_fix(false);
    pathfind_fix(false);
    ai_fix_plugin(false);
}

pub unsafe fn replace_trigger() {
    replace_back();
    replace_def();
    replace_common();

    // ai_fix_plugin(true);//UNCOMMENT THIS if you want to enable AI FIX

    // pathfind_fix(true);//UNCOMMENT THIS if you want to enable PATHFINDING FIX

    // replace original victory trigger
    let trig_jmp = [0x74u8, 0x1A]; // 74 0F
    patch_set!(VICTORY_JMP, trig_jmp);
    let mut rep = [0xc7u8, 0x05, 0x38, 0x0d, 0x4c, 0x0, 0x0, 0x0, 0x0, 0x0];
    rep[6..10].copy_from_slice(&(trig as usize as u32).to_le_bytes());
    patch_set!(VICTORY_TRIGGER, rep);
    trig_init();
}

#[no_mangle]
pub extern "C" fn new_game(a: i32, b: i32, c: i32) {
    unsafe {
        a_custom = (b % 256) as u8; // custom game or campaign
        if a_custom != 0 {
            wb(LEVEL_OBJ, 53); // remember custom obj
        } else if rb(LEVEL_OBJ) == 53 {
            a_custom = 1; // fix for when saved game loads custom get broken
        }
        replace_trigger();
        orig_fn!(g_proc_0042A4A1, extern "C" fn(i32, i32, i32))(a, b, c); // original
    }
}

#[no_mangle]
pub extern "C" fn load_game(a: i32) -> i32 {
    unsafe {
        let original = orig_fn!(g_proc_0041F7E4, extern "C" fn(i32) -> i32)(a); // original
        replace_trigger();
        original
    }
}

pub unsafe fn common_hooks() {
    hook(0x0045271B, &mut g_proc_0045271B, update_spells as usize as u32);
    hook(0x004522B9, &mut g_proc_004522B9, seq_run as usize as u32);

    hook(0x0041038E, &mut g_proc_0041038E, damage1 as usize as u32);
    hook(0x00409F3B, &mut g_proc_00409F3B, damage2 as usize as u32);
    hook(0x0040AF70, &mut g_proc_0040AF70, damage3 as usize as u32);
    hook(0x0040AF99, &mut g_proc_0040AF99, damage4 as usize as u32);
    hook(0x00410762, &mut g_proc_00410762, damage5 as usize as u32);
    hook(0x004428AD, &mut g_proc_004428AD, damage6 as usize as u32);

    hook(0x0043BAE1, &mut g_proc_0043BAE1, rc_snd as usize as u32);
    hook(0x0043B943, &mut g_proc_0043B943, rc_build_click as usize as u32);
    hook(0x0040DF71, &mut g_proc_0040DF71, bld_unit_create as usize as u32);
    hook(0x0040AFBF, &mut g_proc_0040AFBF, tower_find_attacker as usize as u32);
    hook(0x00451728, &mut g_proc_00451728, unit_kill_deselect as usize as u32);

    hook(0x0045614E, &mut g_proc_0045614E, receive_cheat as usize as u32);

    hook(0x004526FE, &mut g_proc_004526FE, grow_structure as usize as u32);
    hook(0x00418FFE, &mut g_proc_00418FFE, create_building as usize as u32);
    hook(0x0043A974, &mut g_proc_0043A974, placebox_query as usize as u32);
    hook(0x0043ABAB, &mut g_proc_0043ABAB, placebox_query as usize as u32);
    hook(0x00424745, &mut g_proc_00424745, goods_into_inventory as usize as u32);
    hook(0x004529C0, &mut g_proc_004529C0, goods_into_inventory as usize as u32);

    hook(0x00451054, &mut g_proc_00451054, count_add_to_tables_load_game as usize as u32);
    hook(0x00438A5C, &mut g_proc_00438A5C, unset_peon_ai_flags as usize as u32);
    hook(0x00438985, &mut g_proc_00438985, unset_peon_ai_flags as usize as u32);

    hook(0x0040EEDD, &mut g_proc_0040EEDD, upgrade_tower as usize as u32);
    hook(0x00442E25, &mut g_proc_00442E25, create_skeleton as usize as u32);
    hook(0x00425D1C, &mut g_proc_00425D1C, cast_raise as usize as u32);
    hook(0x00424F94, &mut g_proc_00424F94, cast_runes as usize as u32);
    hook(0x00424FD7, &mut g_proc_00424FD7, cast_runes as usize as u32);
    hook(0x0042757E, &mut g_proc_0042757E, ai_spell as usize as u32);
    hook(0x00427FAE, &mut g_proc_00427FAE, ai_attack as usize as u32);

    hook(0x0042A4A1, &mut g_proc_0042A4A1, new_game as usize as u32);
    hook(0x0041F7E4, &mut g_proc_0041F7E4, load_game as usize as u32);
}
