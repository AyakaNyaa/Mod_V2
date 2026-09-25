// Игроки, ауры, ресурсы, союз, визоры (w2p.cpp:1677-2011)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::sort::*;
use crate::state::*;

pub unsafe fn slow_aura(id: u8) {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == id;
            if f && !check_unit_dead(p) {
                let x = rb(p + S_X as u32);
                let y = rb(p + S_Y as u32);
                set_region(x as i32 - 5, y as i32 - 5, x as i32 + 5, y as i32 + 5); // set region around myself
                find_all_alive_units(ANY_MEN);
                sort_in_region();
                let o = rb(p + S_OWNER as u32);
                for ui in 0..16u8 {
                    if check_ally(o, ui) {
                        // only enemies
                        sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                    }
                }
                set_stat_all(S_HASTE, 0xfcdf); // -800
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn death_aura(id: u8) {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == id;
            if f && !check_unit_dead(p) {
                let mp = rb(p + S_MANA as u32);
                let x = rb(p + S_X as u32);
                let y = rb(p + S_Y as u32);
                let xx = rb(p + S_ORDER_X as u32);
                let yy = rb(p + S_ORDER_Y as u32);
                set_stat(p, 255, S_MANA);
                set_stat(p, x as i32, S_ORDER_X);
                set_stat(p, y as i32, S_ORDER_Y);
                let buf = [0x90u8, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90];
                patch_set!(RAISE_DEAD_DOING_SPELL1, buf);
                let buf3 = [0x90u8, 0x90, 0x90];
                patch_set!(RAISE_DEAD_DOING_SPELL2, buf3);
                f_raise_dead()(p);
                let buf2 = [0x6au8, 0x2, 0x53, 0xe8, 0x9f, 0x3, 0x1, 0x0];
                patch_set!(RAISE_DEAD_DOING_SPELL1, buf2);
                let buf4 = [0x83u8, 0xc4, 0x8];
                patch_set!(RAISE_DEAD_DOING_SPELL2, buf4);
                set_stat(p, mp as i32, S_MANA);
                set_stat(p, xx as i32, S_ORDER_X);
                set_stat(p, yy as i32, S_ORDER_Y);
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn sneak(id: u8) {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == id;
            if f && !check_unit_dead(p) {
                let o = rb(p + S_ORDER as u32);
                let n = rw(p + S_INVIZ as u32);
                if o == ORDER_STAND && n <= 10 {
                    set_stat(p, 10, S_INVIZ);
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn slot_alive(p: u8) -> bool {
    (get_val(ALL_BUILDINGS, p as i32) + get_val(ALL_UNITS, p as i32)) != 0 // no units and buildings
}

pub unsafe fn ally(p1: u8, p2: u8, a: u8) {
    // set ally bytes in table
    wb(ALLY + p1 as u32 + 16 * p2 as u32, a);
    wb(ALLY + p2 as u32 + 16 * p1 as u32, a);
    f_reset_colors()(); // orig war2 func reset colors of sqares around units
}

pub unsafe fn ally_one_sided(p1: u8, p2: u8, a: u8) {
    // set ally bytes in table
    wb(ALLY + p1 as u32 + 16 * p2 as u32, a);
    f_reset_colors()(); // orig war2 func reset colors of sqares around units
}

pub unsafe fn check_opponents(player: u8) -> bool {
    // check if player have opponents
    // (byte o = C_NOBODY в C++ не используется)
    let mut f = false;
    for i in 0..8u8 {
        if player != i && slot_alive(i) && !check_ally(player, i) {
            // if enemy and not dead
            f = true;
        }
    }
    f
}

pub unsafe fn viz(p1: i32, p2: i32, a: u8) {
    // set vision bits (C++: int-арифметика с усечением до byte при записи)
    let mut v = rb(VIZ + p1 as u32) as i32;
    if a == 0 {
        v &= !(1 << p2);
    } else {
        v |= 1 << p2;
    }
    wb(VIZ + p1 as u32, v as u8);

    let mut v = rb(VIZ + p2 as u32) as i32;
    if a == 0 {
        v &= !(1 << p1);
    } else {
        v |= 1 << p1;
    }
    wb(VIZ + p2 as u32, v as u8);
}

pub unsafe fn viz_one_sided(p1: i32, p2: i32, a: u8) {
    // set vision bits
    let mut v = rb(VIZ + p1 as u32) as i32;
    if a == 0 {
        v &= !(1 << p2);
    } else {
        v |= 1 << p2;
    }
    wb(VIZ + p1 as u32, v as u8);
}

pub unsafe fn comps_vision(v: bool) {
    // allow comps give vision too
    if v {
        let o = [0x00u8];
        patch_set!(COMPS_VIZION, o);
    } else {
        let o = [0xAAu8];
        patch_set!(COMPS_VIZION, o);
    }
}

pub unsafe fn change_res(p: u8, r: u8, k: u8, m: i32) {
    let mut a: u32 = GOLD;
    let mut s = false;
    if p <= 8 {
        // player id (p >= 0 для byte тождественно)
        match r {
            // select resource and add or substract it
            0 => {
                a = GOLD + 4 * p as u32;
                s = false;
            }
            1 => {
                a = LUMBER + 4 * p as u32;
                s = false;
            }
            2 => {
                a = OIL + 4 * p as u32;
                s = false;
            }
            3 => {
                a = GOLD + 4 * p as u32;
                s = true;
            }
            4 => {
                a = LUMBER + 4 * p as u32;
                s = true;
            }
            5 => {
                a = OIL + 4 * p as u32;
                s = true;
            }
            _ => {}
        }
        if r <= 5 {
            let rs = rd(a) as i32; // resourse pointer
            let mut res: u32 = 0;
            if s {
                if rs > k as i32 * m {
                    res = (rs - k as i32 * m) as u32;
                } else {
                    res = 0; // canot go smaller than 0
                }
            } else {
                if rs <= 256 * 256 * 256 * 32 {
                    res = (rs + k as i32 * m) as u32;
                } else {
                    // FIX: в C++ w2p.cpp:1912 при превышении капа ресурс занулялся
                    res = rs as u32;
                }
            }
            patch::patch_setdword(a, res);
        }
    }
}

pub unsafe fn add_total_res(p: u8, r: u8, k: u8, m: i32) {
    let mut a: u32 = GOLD_TOTAL;
    if p <= 8 {
        // player id
        match r {
            // select resource and add or substract it
            0 => {
                a = GOLD_TOTAL + 4 * p as u32;
            }
            1 => {
                a = LUMBER_TOTAL + 4 * p as u32;
            }
            2 => {
                a = OIL_TOTAL + 4 * p as u32;
            }
            _ => {}
        }
        if r <= 2 {
            let rs = rd(a) as i32; // resourse pointer
            let mut res: u32 = 0;
            if rs <= 256 * 256 * 256 * 32 {
                res = (rs + k as i32 * m) as u32;
            } else {
                // FIX: в C++ w2p.cpp:1943 при превышении капа ресурс занулялся
                res = rs as u32;
            }
            patch::patch_setdword(a, res);
        }
    }
}

pub unsafe fn set_res(p: u8, r: u8, k1: u8, k2: u8, k3: u8, k4: u8) {
    // as before but dnt add or sub res, just set given value
    let mut a: u32 = 0;
    if p <= 8 {
        match r {
            0 => {
                a = GOLD + 4 * p as u32;
            }
            1 => {
                a = LUMBER + 4 * p as u32;
            }
            2 => {
                a = OIL + 4 * p as u32;
            }
            _ => {}
        }
        if r <= 2 {
            let buf = [k1, k2, k3, k4];
            patch_set!(a, buf);
        }
    }
}

pub unsafe fn cmp_res(p: u8, r: u8, k1: u8, k2: u8, k3: u8, k4: u8, cmp: u8) -> bool {
    // compare resource to value
    let mut a: u32 = GOLD;
    if p <= 8 {
        match r {
            0 => {
                a = GOLD + 4 * p as u32;
            }
            1 => {
                a = LUMBER + 4 * p as u32;
            }
            2 => {
                a = OIL + 4 * p as u32;
            }
            _ => {}
        }
        if r <= 2 {
            let v = (k1 as i32)
                .wrapping_add((256i32).wrapping_mul(k2 as i32))
                .wrapping_add((256i32 * 256).wrapping_mul(k3 as i32))
                .wrapping_add((256i32 * 256 * 256).wrapping_mul(k4 as i32));
            return cmp_args4(cmp, rd(a) as i32, v);
        }
    }
    false
}
