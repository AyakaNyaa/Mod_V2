// Механики: рунный камень, портал, верфь, паладин, транспорт (w2p.cpp:1354-1676)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::patch::*;
use crate::sort::*;
use crate::state::*;

pub unsafe fn runestone() {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == U_RUNESTONE;
            if f && !check_unit_dead(p) && check_unit_complete(p) {
                // alive and completed runestone
                let x = rb(p + S_X as u32);
                let y = rb(p + S_Y as u32);
                set_region(x as i32 - 4, y as i32 - 4, x as i32 + 5, y as i32 + 5); // set region around myself
                find_all_alive_units(ANY_MEN);
                sort_in_region();
                if runes[8] == 1 {
                    // only allied units can can recieve bufs
                    let o = rb(p + S_OWNER as u32);
                    for ui in 0..16u8 {
                        if !check_ally(o, ui) {
                            // tp
                            sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                        }
                    }
                }
                if runes[0] == 1 {
                    set_stat_all(S_INVIZ, rw(INVIZ_TIME) as i32); // inviz
                }
                if runes[1] == 1 {
                    set_stat_all(S_SHIELD, rw(SHIELD_TIME) as i32); // shield
                }
                if runes[2] == 1 {
                    set_stat_all(S_BLOOD, rw(BLOOD_TIME) as i32); // blood
                }
                if runes[3] == 1 {
                    set_stat_all(S_HASTE, rw(HASTE_TIME1) as i32); // haste
                }
                if runes[4] == 1 {
                    flame_all();
                }
                // 5 mana
                // 6 heal
                if runes[7] == 1 {
                    // kill all not my owner units
                    let o = rb(p + S_OWNER as u32);
                    sort_stat(S_OWNER, o as i32, CMP_NEQ);
                    kill_all();
                }
                sort_stat(S_KILLS + 1, 0, CMP_EQ);
                set_stat_all(S_KILLS + 1, 100);
                if runes[5] != 0 {
                    mana_regen_all(runes[5]);
                }
                if runes[6] != 0 {
                    heal_all(runes[6], 0);
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn portal() {
    for i in 0..16u32 {
        units = 0;
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        let mut fp: u32 = 0;
        while p != 0 {
            let f = rb(p + S_ID as u32) == U_PORTAL;
            if f && !check_unit_dead(p) && check_unit_complete(p) {
                // alive and completed portal
                if fp == 0 {
                    fp = p; // remember first portal
                }
                let tx = rb(p + S_X as u32) + 1;
                let ty = rb(p + S_Y as u32) + 1; // exit point is in center of portal
                move_all(tx, ty); // teleport from previous portal
                set_tp_flag(true);
                set_stat_all(S_NEXT_ORDER, ORDER_STOP as i32);
                set_stat_all(S_ORDER_X, 128);
                set_stat_all(S_ORDER_Y, 128);
                let x = rb(p + S_X as u32);
                let y = rb(p + S_Y as u32);
                set_region(x as i32 - 1, y as i32 - 1, x as i32 + 4, y as i32 + 4); // set region around myself
                find_all_alive_units(ANY_MEN);
                sort_in_region();
                if aport {
                    let o = rb(p + S_OWNER as u32);
                    for ui in 0..16u8 {
                        if !check_ally(o, ui) {
                            // only allied units can tp
                            sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                        }
                    }
                }
                sort_tp_flag(); // flag show if unit was not teleported
                let mut mp = true;
                if mport {
                    // only teleport if some caster near
                    mp = false;
                    for ui in 0..units {
                        let uid = rb(unit[ui as usize] + S_ID as u32);
                        if uid == U_MAGE
                            || uid == U_DK
                            || uid == U_TERON
                            || uid == U_HADGAR
                            || uid == U_GULDAN
                        {
                            mp = true; // can tp only if mage nearby (teron hadgar and guldan too)
                        }
                    }
                }
                if !mp {
                    units = 0;
                } else {
                    sort_stat(S_ORDER, ORDER_STOP as i32, CMP_EQ);
                    sort_stat(S_ORDER_UNIT_POINTER, 0, CMP_EQ);
                    sort_stat(S_ORDER_UNIT_POINTER + 1, 0, CMP_EQ);
                    sort_stat(S_ORDER_UNIT_POINTER + 2, 0, CMP_EQ);
                    sort_stat(S_ORDER_UNIT_POINTER + 3, 0, CMP_EQ);
                    set_region(x as i32, y as i32, x as i32 + 3, y as i32 + 3); // set region inside myself
                    sort_target_in_region();
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
        if fp != 0 {
            // first portal teleports from last
            let tx = rb(fp + S_X as u32) + 1;
            let ty = rb(fp + S_Y as u32) + 1;
            move_all(tx, ty);
            set_tp_flag(true);
            set_stat_all(S_NEXT_ORDER, ORDER_STOP as i32);
            set_stat_all(S_ORDER_X, 128);
            set_stat_all(S_ORDER_Y, 128);
        }
    }
    find_all_alive_units(ANY_MEN);
    set_tp_flag(false); // reset tp flags to all
}

pub unsafe fn wharf() {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == U_SHIPYARD || rb(p + S_ID as u32) == U_WHARF;
            if f && !check_unit_dead(p) && check_unit_complete(p) {
                let x = rb(p + S_X as u32);
                let y = rb(p + S_Y as u32);
                set_region(x as i32 - 2, y as i32 - 2, x as i32 + 4, y as i32 + 4); // set region around myself
                find_all_alive_units(ANY_MEN);
                sort_in_region();
                sort_hidden();
                sort_stat(S_MOVEMENT_TYPE, MOV_WATER as i32, CMP_BIGGER_EQ); // find ships - movement type >= water (2 or 3 actually(ships=2 transport=3))
                let o = rb(p + S_OWNER as u32);
                for ui in 0..16u8 {
                    if !check_ally(o, ui) {
                        // only allied ships
                        sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                    }
                }
                heal_all(4, 0);
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn paladin() {
    for ii in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * ii);
        while p != 0 {
            let f = rb(p + S_ID as u32) == U_PALADIN
                || rb(p + S_ID as u32) == U_UTER
                || rb(p + S_ID as u32) == U_TYRALYON;
            if f && !check_unit_dead(p) && !check_unit_hidden(p) {
                let o = rb(p + S_OWNER as u32);
                if (rb(SPELLS_LEARNED + 4 * o as u32) & (1 << L_HEAL)) != 0
                    && (rb(SPELLS_LEARNED + 4 * o as u32) & (1 << L_GREATER_HEAL)) != 0
                // if player learned heal and autoheal
                {
                    let x = rb(p + S_X as u32);
                    let y = rb(p + S_Y as u32);
                    set_region(x as i32 - 5, y as i32 - 5, x as i32 + 5, y as i32 + 5); // set region around myself
                    find_all_alive_units(ANY_MEN);
                    sort_in_region();
                    sort_hidden();
                    sort_fleshy(); // fleshy units (not heal cata and ships)
                    sort_full_hp(); // if unit hp not full
                    sort_self(p); // not heal self
                    sort_order_hp(); // heal lovest hp first
                    for ui in 0..16u8 {
                        if !check_ally(o, ui) {
                            // only allied units
                            sort_stat(S_OWNER, ui as i32, 1);
                        }
                    }
                    let cost = rb(MANACOST + 2 * GREATER_HEAL as u32); // 2* cause manacost is WORD
                    let mut i = 0;
                    while i < units {
                        let mut mp = rb(p + S_MANA as u32); // paladin mp
                        if mp >= cost {
                            let id = rb(unit[i as usize] + S_ID as u32); // unit id
                            let mhp = rw(UNIT_HP_TABLE + 2 * id as u32); // max hp
                            let hp = rw(unit[i as usize] + S_HP as u32); // unit hp
                            let mut shp = mhp - hp; // shortage of hp
                            // C++ считает в int: mp(byte) и shp*cost(WORD*byte) промоутятся
                            while !(mp as i32 >= shp as i32 * cost as i32) && (shp > 0) {
                                shp -= 1;
                            }
                            if shp > 0 {
                                // if can heal at least 1 hp
                                heal(unit[i as usize], shp as u8, 0);
                                mp = (mp as i32 - shp as i32 * cost as i32) as u8;
                                wb(p + S_MANA as u32, mp);
                                let xx = rw(unit[i as usize] + S_DRAW_X as u32);
                                let yy = rw(unit[i as usize] + S_DRAW_Y as u32);
                                f_bullet_create()(xx + 16, yy + 16, B_HEAL); // create heal effect
                                f_spell_sound_xy()(xx + 16, yy + 16, SS_HEAL); // heal sound
                            }
                        } else {
                            i = units; // break (C++: else i = units)
                        }
                        i += 1;
                    }
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn transport() {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == U_HTRANSPORT || rb(p + S_ID as u32) == U_OTRANSPORT;
            if f && !check_unit_dead(p) && cmp_stat(p, ANIM_STOP as i32, S_ANIMATION, CMP_EQ) {
                // if transport stop
                let x = rb(p + S_X as u32);
                let y = rb(p + S_Y as u32);
                let o = rb(p + S_OWNER as u32);
                for ui in 0..16u8 {
                    set_region(x as i32 - 1, y as i32 - 1, x as i32 + 1, y as i32 + 1); // set region around myself
                    find_all_alive_units(ANY_MEN);
                    sort_in_region();
                    sort_hidden();
                    sort_stat(S_MOVEMENT_TYPE, MOV_LAND as i32, CMP_EQ);
                    sort_stat(S_ORDER, ORDER_STOP as i32, CMP_EQ);
                    sort_stat(S_ANIMATION, ANIM_STOP as i32, CMP_EQ);
                    sort_stat(S_OWNER, ui as i32, CMP_EQ);
                    let mut f = false;
                    if rb(CONTROLER_TYPE + o as u32) == C_PLAYER {
                        f = true;
                        if rb(CONTROLER_TYPE + ui as u32) == C_COMP {
                            if !check_ally(o, ui) {
                                // only allied comps
                                sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                            }
                        }
                    }
                    if rb(CONTROLER_TYPE + o as u32) == C_COMP {
                        f = true;
                        if rb(CONTROLER_TYPE + ui as u32) == C_COMP {
                            if !check_ally(o, ui) || (ui == o) {
                                // only allied comps
                                sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                            }
                        }
                        if rb(CONTROLER_TYPE + ui as u32) == C_PLAYER {
                            if !check_ally(o, ui) {
                                // only allied players
                                sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                            }
                            sort_stat(S_ORDER_UNIT_POINTER, 0, CMP_EQ);
                            sort_stat(S_ORDER_UNIT_POINTER + 1, 0, CMP_EQ);
                            sort_stat(S_ORDER_UNIT_POINTER + 2, 0, CMP_EQ);
                            sort_stat(S_ORDER_UNIT_POINTER + 3, 0, CMP_EQ);
                            set_region(x as i32, y as i32, x as i32, y as i32); // set region inside myself
                            sort_target_in_region();
                        }
                    }
                    if f {
                        sort_stat(S_KILLS + 1, 0, CMP_EQ);
                        set_stat_all(S_KILLS + 1, 100);
                        set_stat_all(S_NEXT_ORDER, ORDER_ENTER_TRANSPORT as i32);
                        set_stat_all(S_ORDER_UNIT_POINTER, (p % 256) as i32);
                        set_stat_all(S_ORDER_UNIT_POINTER + 1, ((p / 256) % 256) as i32);
                        set_stat_all(S_ORDER_UNIT_POINTER + 2, (((p / 256) / 256) % 256) as i32);
                        set_stat_all(S_ORDER_UNIT_POINTER + 3, ((((p / 256) / 256) / 256) % 256) as i32);
                    }
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}
