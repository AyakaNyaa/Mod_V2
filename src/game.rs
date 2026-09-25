// Обёртки вызовов игры и общие helpers (w2p.cpp:73-309)
#![allow(dead_code)]

use crate::defs::*;
use crate::game_fn;
use crate::names::*;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::state::*;

// ---- Типизированные указатели на функции игры (все вызовы — cdecl = extern "C") ----
game_fn!(f_map_msg, F_MAP_MSG, extern "C" fn(*const u8, i32, i32));
game_fn!(f_lose, F_LOSE, extern "C" fn());
game_fn!(f_win, F_WIN, extern "C" fn());
game_fn!(f_video, F_VIDEO, extern "C" fn(i32, u8));
game_fn!(f_tile_remove_trees, F_TILE_REMOVE_TREES, extern "C" fn(i32, i32));
game_fn!(f_tile_remove_rocks, F_TILE_REMOVE_ROCKS, extern "C" fn(i32, i32));
game_fn!(f_tile_remove_walls, F_TILE_REMOVE_WALLS, extern "C" fn(i32, i32));
game_fn!(f_unit_convert, F_UNIT_CONVERT, extern "C" fn(u8, i32, i32, i32));
game_fn!(f_unit_create, F_UNIT_CREATE, extern "C" fn(i32, i32, i32, u8, u32));
game_fn!(f_unit_kill, F_UNIT_KILL, extern "C" fn(u32));
game_fn!(f_create_flame, F_CREATE_FLAME, extern "C" fn(u32, i32, i32));
game_fn!(f_capture, F_CAPTURE, extern "C" fn(u32, u8, u8));
game_fn!(f_xy_passable, F_XY_PASSABLE, extern "C" fn(i32, i32, i32) -> i32);
game_fn!(f_unit_unplace, F_UNIT_UNPLACE, extern "C" fn(u32));
game_fn!(f_unit_place, F_UNIT_PLACE, extern "C" fn(u32));
game_fn!(f_bullet_create, F_BULLET_CREATE, extern "C" fn(u16, u16, u8));
game_fn!(f_bullet_create_unit, F_BULLET_CREATE_UNIT, extern "C" fn(u32, u8));
game_fn!(f_spell_sound_xy, F_SPELL_SOUND_XY, extern "C" fn(u16, u16, u8));
game_fn!(f_spell_sound_unit, F_SPELL_SOUND_UNIT, extern "C" fn(u32, u8));
game_fn!(f_reset_colors, F_RESET_COLORS, extern "C" fn());
game_fn!(f_minimap_click, F_MINIMAP_CLICK, extern "C" fn(u8, u8));
game_fn!(f_send_cheat_packet, F_SEND_CHEAT_PACKET, extern "C" fn(i32));
game_fn!(f_raise_dead, F_RAISE_DEAD, extern "C" fn(u32));
game_fn!(f_net_random, F_NET_RANDOM, extern "C" fn() -> i32);
game_fn!(f_attack_can_hit, F_ATTACK_CAN_HIT, extern "C" fn(u32, u32) -> i32);
game_fn!(f_train_unit, F_TRAIN_UNIT, extern "C" fn(i32));
game_fn!(f_build_building, F_BUILD_BUILDING, extern "C" fn(i32));
game_fn!(f_change_status, F_CHANGE_STATUS, extern "C" fn(i32));
game_fn!(f_status_redraw, F_STATUS_REDRAW, extern "C" fn());
game_fn!(f_vision2, F_VISION2, extern "C" fn(u16, u16, u8));
game_fn!(f_vision3, F_VISION3, extern "C" fn(u16, u16, u8));
game_fn!(f_vision4, F_VISION4, extern "C" fn(u16, u16, u8));
game_fn!(f_vision5, F_VISION5, extern "C" fn(u16, u16, u8));
game_fn!(f_vision6, F_VISION6, extern "C" fn(u16, u16, u8));
game_fn!(f_vision7, F_VISION7, extern "C" fn(u16, u16, u8));
game_fn!(f_vision8, F_VISION8, extern "C" fn(u16, u16, u8));
game_fn!(f_vision9, F_VISION9, extern "C" fn(u16, u16, u8));
game_fn!(f_dead_building_redraw, F_DEAD_BUILDING_REDRAW, extern "C" fn(u32));
game_fn!(f_group_set, F_GROUP_SET, extern "C" fn(u32));
game_fn!(f_ai_cast, F_AI_CAST, extern "C" fn(u32));
game_fn!(f_give_order, F_GIVE_ORDER, extern "C" fn(u32, i32, i32, u32, i32));
game_fn!(f_tech_built, F_TECH_BUILT, extern "C" fn(i32, u8));
game_fn!(f_mtx_dist, F_MTX_DIST, extern "C" fn(*mut GPoint, u32) -> u16);
game_fn!(f_ice_set_ai_order, F_ICE_SET_AI_ORDER, extern "C" fn(u32, i32, u32) -> i32);
game_fn!(f_bldg_start_build, F_BLDG_START_BUILD, extern "C" fn(u32, u8, u8));
game_fn!(f_unit_fixup, F_UNIT_FIXUP, extern "C" fn(u32, i32) -> u32);

pub unsafe fn show_message(time: u8, text: *const u8) {
    f_map_msg()(text, 15, time as i32 * 10); // original war2 show msg func
}

pub unsafe fn clear_chat() {
    for i in 0..12 {
        let mut p: u32 = CHAT_POINTER; // C++: p = (int*)CHAT_POINTER (адрес, не разыменование)
        p = rd(p); // C++: p = (int*)(*p) — единственное разыменование
        p = p.wrapping_sub(100 * i as u32); // C++: p(int*) -= 100/4*i => -100*i байт
        let buf = [0u8]; // just set 0 in all messages to empty
        patch::patch_setbytes(p, &buf);
    }
}

pub unsafe fn get_val(adress: u32, player: i32) -> i32 {
    rw(adress + (player * 2) as u32) as i32 // player*2 cause all vals is WORD
}

pub unsafe fn cmp_args(m: u8, v: u8, c: u8) -> bool {
    // compare bytes
    match m {
        CMP_EQ => v == c,
        CMP_NEQ => v != c,
        CMP_BIGGER_EQ => v >= c,
        CMP_SMALLER_EQ => v <= c,
        CMP_BIGGER => v > c,
        CMP_SMALLER => v < c,
        _ => false,
    }
}

pub unsafe fn cmp_args2(m: u8, v: u16, c: u16) -> bool {
    // compare words
    match m {
        CMP_EQ => v == c,
        CMP_NEQ => v != c,
        CMP_BIGGER_EQ => v >= c,
        CMP_SMALLER_EQ => v <= c,
        CMP_BIGGER => v > c,
        CMP_SMALLER => v < c,
        _ => false,
    }
}

pub unsafe fn cmp_args4(m: u8, v: i32, c: i32) -> bool {
    // compare 4 bytes (for resources)
    match m {
        CMP_EQ => v == c,
        CMP_NEQ => v != c,
        CMP_BIGGER_EQ => v >= c,
        CMP_SMALLER_EQ => v <= c,
        CMP_BIGGER => v > c,
        CMP_SMALLER => v < c,
        _ => false,
    }
}

pub unsafe fn lose(t: bool) {
    if t {
        let buf = [0u8]; // if need to show table
        patch_set!(LOSE_SHOW_TABLE, buf);
    } else {
        let buf = [0x3bu8];
        patch_set!(LOSE_SHOW_TABLE, buf);
    }
    if !first_step {
        let l = [2u8, 0, 0, 0];
        patch_set!(ENDGAME_STATE + 4 * rb(LOCAL_PLAYER) as u32, l);
        f_lose()(); // original lose func
    } else {
        patch::patch_setdword(0x004C0D38, F_LOSE);
    }
}

pub unsafe fn win(t: bool) {
    if t {
        let buf = [0xEBu8]; // if need to show table
        patch_set!(WIN_SHOW_TABLE, buf);
    } else {
        let buf = [0x74u8];
        patch_set!(WIN_SHOW_TABLE, buf);
    }
    if !first_step {
        let l = [3u8, 0, 0, 0];
        patch_set!(ENDGAME_STATE + 4 * rb(LOCAL_PLAYER) as u32, l);
        f_win()(); // original win func
    } else {
        patch::patch_setdword(0x004C0D38, F_WIN);
    }
}

pub unsafe fn lose2(t: bool, vid: u8) {
    lose(t);
    f_video()(vid as i32, 1); // original war2 func that show video by id
}

pub unsafe fn win2(t: bool, vid: u8) {
    win(t);
    f_video()(vid as i32, 1); // original war2 func that show video by id
}

pub unsafe fn tile_remove_trees(x: i32, y: i32) {
    f_tile_remove_trees()(x, y);
}

pub unsafe fn tile_remove_rocks(x: i32, y: i32) {
    f_tile_remove_rocks()(x, y);
}

pub unsafe fn tile_remove_walls(x: i32, y: i32) {
    f_tile_remove_walls()(x, y);
}

pub unsafe fn stat_byte(s: u8) -> bool {
    // check if unit stat is 1 or 2 byte
    let f = s == S_DRAW_X
        || s == S_DRAW_Y
        || s == S_X
        || s == S_Y
        || s == S_HP
        || s == S_INVIZ
        || s == S_SHIELD
        || s == S_BLOOD
        || s == S_HASTE
        || s == S_AI_SPELLS
        || s == S_NEXT_FIRE
        || s == S_LAST_HARVEST_X
        || s == S_LAST_HARVEST_Y
        || s == S_BUILD_PROGRES
        || s == S_BUILD_PROGRES_TOTAL
        || s == S_RESOURCES
        || s == S_ORDER_X
        || s == S_ORDER_Y
        || s == S_RETARGET_X1
        || s == S_RETARGET_Y1
        || s == S_RETARGET_X2
        || s == S_RETARGET_Y2;
    !f
}

pub unsafe fn cmp_stat(p: u32, v: i32, pr: u8, cmp: u8) -> bool {
    // p - unit, v - value, pr - property, cmp - compare method
    let mut f = false;
    if stat_byte(pr) {
        let ob = (v % 256) as u8;
        let b = rb(p + pr as u32);
        if cmp_args(cmp, b, ob) {
            f = true;
        }
    } else if cmp_args2(cmp, rw(p + pr as u32), v as u16) {
        f = true;
    }
    f
}

pub unsafe fn set_stat(p: u32, v: i32, pr: u8) {
    if stat_byte(pr) {
        let buf = [(v % 256) as u8];
        patch::patch_setbytes(p + pr as u32, &buf);
    } else {
        let buf = [(v % 256) as u8, ((v / 256) % 256) as u8];
        patch::patch_setbytes(p + pr as u32, &buf);
    }
}

pub unsafe fn unit_convert(player: u8, who: i32, tounit: i32, a: i32) {
    // original war2 func converts units
    f_unit_convert()(player, who, tounit, a);
}

pub unsafe fn unit_create(x: i32, y: i32, id: i32, owner: u8, mut n: u8) {
    while n > 0 {
        n -= 1;
        let p = UNIT_SIZE_TABLE + 4 * id as u32; // unit sizes table
        f_unit_create()(x, y, id, owner, p); // original war2 func creates unit
        // just call n times to create n units
    }
}

pub unsafe fn unit_kill(u: u32) {
    f_unit_kill()(u); // original war2 func kills unit
}

pub unsafe fn unit_remove(u: u32) {
    let f = rb(u + S_FLAGS3 as u32) | SF_HIDDEN;
    set_stat(u, f as i32, S_FLAGS3);
    unit_kill(u); // hide unit then kill
}

pub unsafe fn unit_cast(u: u32) {
    // unit autocast
    f_ai_cast()(u); // original war2 ai cast spells
}
