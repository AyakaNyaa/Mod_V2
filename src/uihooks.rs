// UI-хуки: башни, правый клик, создание юнитов (w2p.cpp:3844-4099)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::orig_fn;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::sort::*;
use crate::state::*;

pub unsafe fn tower_set_target(p: u32, x: i32, y: i32) {
    set_stat(p, 0, S_RETARGET_X1 - 2);
    set_stat(p, 0, S_RETARGET_X1 - 1);
    let mut u: u32 = 0;
    set_region(x - 3, y - 3, x, y);
    find_all_alive_units(ANY_BUILDING_4x4);
    sort_in_region();
    sort_hidden();
    sort_attack_can_hit(p);
    if units != 0 {
        u = unit[0];
    }
    set_region(x - 2, y - 2, x, y);
    find_all_alive_units(ANY_BUILDING_3x3);
    sort_in_region();
    sort_hidden();
    sort_attack_can_hit(p);
    if units != 0 {
        u = unit[0];
    }
    set_region(x - 1, y - 1, x, y);
    find_all_alive_units(ANY_BUILDING_2x2);
    sort_in_region();
    sort_hidden();
    sort_attack_can_hit(p);
    if units != 0 {
        u = unit[0];
    }
    set_region(x, y, x, y);
    find_all_alive_units(ANY_UNITS);
    sort_in_region();
    sort_hidden();
    sort_attack_can_hit(p);
    if units != 0 {
        u = unit[0];
    }
    if u != 0 {
        let fx = f_unit_fixup()(u, 1); // fixup save
        set_stat(p, (fx % 256) as i32, S_RETARGET_X1 - 2);
        set_stat(p, (fx / 256) as i32, S_RETARGET_X1 - 1);
    }
}

pub unsafe fn brclik(b: bool) {
    if b {
        let r = [0x90u8, 0x90, 0x90, 0x90, 0x90, 0x90];
        patch_set!(RIGHT_CLICK_ALLOW_BUILDINGS, r);
    } else {
        let r = [0xfu8, 0x84, 0x26, 0x01, 0x0, 0x0];
        patch_set!(RIGHT_CLICK_ALLOW_BUILDINGS, r);
    }
}

#[no_mangle]
pub extern "C" fn rc_snd(p: u32) {
    unsafe {
        // acknowlegement sound
        // (game crash cause buildings not have this sounds so this hooked function fix it)
        if rb(p + S_ID as u32) < U_FARM {
            orig_fn!(g_proc_0043BAE1, extern "C" fn(u32))(p);
        }
    }
}

#[no_mangle]
pub extern "C" fn rc_build_click(p: u32, x: i32, y: i32, t: u32, a: i32) {
    unsafe {
        let id = rb(p + S_ID as u32);
        if id >= U_FARM {
            set_stat(p, x | 128, S_RETARGET_X1 - 2);
            set_stat(p, y, S_RETARGET_X1 - 1);
            if id == U_HARROWTOWER
                || id == U_OARROWTOWER
                || id == U_HCANONTOWER
                || id == U_OCANONTOWER
                || id == U_HTOWER
                || id == U_OTOWER
            {
                tower_set_target(p, x, y);
            }
        } else {
            orig_fn!(g_proc_0043B943, extern "C" fn(u32, i32, i32, u32, i32))(p, x, y, t, a); // original
        }
    }
}

pub unsafe fn rc_jmp(b: bool) {
    if b {
        let r = [0xfu8, 0x84, 0xa2, 0x0, 0x0, 0x0];
        patch_set!(RIGHT_CLICK_1, r);
        patch::patch_ljmp(RIGHT_CLICK_CODE_CAVE, RIGHT_CLICK_2);
    } else {
        let r = [0xfu8, 0x84, 0x8b, 0x0, 0x0, 0x0];
        patch_set!(RIGHT_CLICK_1, r);
    }
}

#[no_mangle]
pub extern "C" fn bld_unit_create(a1: i32, a2: i32, a3: i32, a4: u8, a5: u32) -> u32 {
    unsafe {
        // this function called when building finished training unit
        // and new unit should be created
        let b = rd(UNIT_RUN_UNIT_POINTER); // building that processed right now
        let u = orig_fn!(g_proc_0040DF71, extern "C" fn(i32, i32, i32, u8, u32) -> u32)(
            a1, a2, a3, a4, a5,
        );
        if b != 0 {
            // building
            if u != 0 {
                // unit that was created (will be NULL if was not created (ex: no place))
                let mut x = rb(b + (S_RETARGET_X1 - 2) as u32);
                let mut y = rb(b + (S_RETARGET_X1 - 1) as u32);
                let bp = x & 128;
                if bp != 0 {
                    x &= !128;
                    let uid = rb(u + S_ID as u32);
                    let mut o = ORDER_ATTACK_AREA;
                    if uid == U_PEON
                        || uid == U_PEASANT
                        || uid == U_HTANKER
                        || uid == U_OTANKER
                    {
                        o = ORDER_HARVEST;
                    }
                    give_order(u, x, y, o);
                    set_stat(u, x as i32, S_RETARGET_X1);
                    set_stat(u, y as i32, S_RETARGET_Y1);
                    set_stat(u, o as i32, S_RETARGET_ORDER);
                }
                if ai_fixed {
                    let o = rb(u + S_OWNER as u32);
                    let m = rb(u + S_MANA as u32);
                    if rb(CONTROLER_TYPE + o as u32) == C_COMP {
                        // FIX: в C++ w2p.cpp:3970 стояло присваивание (m = 0x55) вместо сравнения
                        if m == 0x55 {
                            // 85 default starting mana
                            let buf = [0xA0u8]; // 160
                            patch_set!(u + S_MANA as u32, buf);
                        }
                    }
                }
            }
        }
        u
    }
}

#[no_mangle]
pub extern "C" fn tower_find_attacker(p: u32) -> u32 {
    unsafe {
        let mut tr: u32 = 0;
        let id = rb(p + S_ID as u32);
        if id == U_HARROWTOWER
            || id == U_OARROWTOWER
            || id == U_HCANONTOWER
            || id == U_OCANONTOWER
        {
            let a1 = rb(p + (S_RETARGET_X1 - 2) as u32);
            let a2 = rb(p + (S_RETARGET_X1 - 1) as u32);
            tr = (a1 as u32) + 256 * (a2 as u32);
            tr = f_unit_fixup()(tr, 0); // fixup load
            if tr != 0 {
                if !check_unit_near_death(tr) && !check_unit_dead(tr) && !check_unit_hidden(tr) {
                    let a = f_attack_can_hit()(p, tr);
                    if a != 0 {
                        let id = rb(tr + S_ID as u32);
                        let szx = rb(UNIT_SIZE_TABLE + 4 * id as u32);
                        let szy = rb(UNIT_SIZE_TABLE + 4 * id as u32 + 2);
                        let idd = rb(p + S_ID as u32);
                        let rng = rb(UNIT_RANGE_TABLE + idd as u32);
                        let ms = rb(MAP_SIZE);
                        let xx = rb(tr + S_X as u32);
                        let yy = rb(tr + S_Y as u32);
                        let mut x1 = rb(p + S_X as u32);
                        let mut y1 = rb(p + S_Y as u32);
                        let mut x2 = x1;
                        let mut y2 = y1;
                        if x1 < rng {
                            x1 = 0;
                        } else {
                            x1 -= rng;
                        }
                        if y1 < rng {
                            y1 = 0;
                        } else {
                            y1 -= rng;
                        }
                        if (x2 as i32 + rng as i32 + 1) > ms as i32 {
                            x2 = ms;
                        } else {
                            x2 = x2.wrapping_add(rng).wrapping_add(1);
                        }
                        if (y2 as i32 + rng as i32 + 1) > ms as i32 {
                            y2 = ms;
                        } else {
                            y2 = y2.wrapping_add(rng).wrapping_add(1);
                        }
                        if !((xx >= x1) && (xx <= x2) && (yy >= y1) && (yy <= y2)) {
                            tr = 0;
                        }
                        let _ = (szx, szy); // прочитаны в C++, но не используются
                    }
                } else {
                    tr = 0;
                }
            }
        }
        if tr == 0 {
            orig_fn!(g_proc_0040AFBF, extern "C" fn(u32) -> u32)(p) // original
        } else {
            tr
        }
    }
}

#[no_mangle]
pub extern "C" fn unit_kill_deselect(u: u32) {
    unsafe {
        let ud = u;
        orig_fn!(g_proc_00451728, extern "C" fn(u32))(u); // original
        f_status_redraw()(); // status redraw
        for i in 0..16u32 {
            let mut p: u32 = rd(UNITS_LISTS + 4 * i);
            while p != 0 {
                let id = rb(p + S_ID as u32);
                let f = id == U_HARROWTOWER
                    || id == U_OARROWTOWER
                    || id == U_HCANONTOWER
                    || id == U_OCANONTOWER;
                let f2 = id == U_DWARWES || id == U_GOBLINS;
                if f && !check_unit_dead(p) && check_unit_complete(p) {
                    let a1 = rb(p + S_RETARGET_X1 as u32);
                    let a2 = rb(p + (S_RETARGET_X1 + 1) as u32);
                    let a3 = rb(p + (S_RETARGET_X1 + 2) as u32);
                    let a4 = rb(p + (S_RETARGET_X1 + 3) as u32);
                    let tr = a1 as u32 + 256 * a2 as u32 + 256 * 256 * a3 as u32 + 256 * 256 * 256 * a4 as u32;
                    if tr == ud {
                        set_stat(p, 0, S_RETARGET_X1);
                        set_stat(p, 0, S_RETARGET_X1 + 1);
                        set_stat(p, 0, S_RETARGET_X1 + 2);
                        set_stat(p, 0, S_RETARGET_X1 + 3);
                    }
                }
                if f2 && ai_fixed && !check_unit_dead(p) {
                    let a1 = rb(p + S_ORDER_UNIT_POINTER as u32);
                    let a2 = rb(p + (S_ORDER_UNIT_POINTER + 1) as u32);
                    let a3 = rb(p + (S_ORDER_UNIT_POINTER + 2) as u32);
                    let a4 = rb(p + (S_ORDER_UNIT_POINTER + 3) as u32);
                    let tr = a1 as u32 + 256 * a2 as u32 + 256 * 256 * a3 as u32 + 256 * 256 * 256 * a4 as u32;
                    if tr == ud {
                        set_stat(p, 0, S_ORDER_UNIT_POINTER);
                        set_stat(p, 0, S_ORDER_UNIT_POINTER + 1);
                        set_stat(p, 0, S_ORDER_UNIT_POINTER + 2);
                        set_stat(p, 0, S_ORDER_UNIT_POINTER + 3);
                        give_order(p, 0, 0, ORDER_STOP);
                    }
                }
                p = rd(p + S_NEXT_UNIT_POINTER as u32);
            }
        }
    }
}
