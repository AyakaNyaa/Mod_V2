// Поиск юнитов и первые сортировки (w2p.cpp:310-693)
#![allow(dead_code)]

use crate::defs::*;
use crate::game::*;
use crate::names::*;
use crate::patch::*;
use crate::state::*;

pub unsafe fn set_region(x1: i32, y1: i32, x2: i32, y2: i32) {
    let mut x1 = x1;
    let mut y1 = y1;
    let mut x2 = x2;
    let mut y2 = y2;
    if x1 < 0 {
        x1 = 0;
    }
    if x1 > 127 {
        x1 = 127;
    }
    if y1 < 0 {
        y1 = 0;
    }
    if y1 > 127 {
        y1 = 127;
    }
    if x2 < 0 {
        x2 = 0;
    }
    if x2 > 127 {
        x2 = 127;
    }
    if y2 < 0 {
        y2 = 0;
    }
    if y2 > 127 {
        y2 = 127;
    }
    reg[0] = (x1 % 256) as u8;
    reg[1] = (y1 % 256) as u8;
    reg[2] = (x2 % 256) as u8;
    reg[3] = (y2 % 256) as u8;
}

pub unsafe fn in_region(x: u8, y: u8, x1: u8, y1: u8, x2: u8, y2: u8) -> bool {
    // dnt know why but without this big monstrous ussless code gam crash
    // (все аргументы byte — мод-256 и клампы ниже в C++ тождественны, оставлены комментарием)
    let mut x1 = x1;
    let mut y1 = y1;
    let mut x2 = x2;
    let mut y2 = y2;
    if x2 < x1 {
        core::mem::swap(&mut x1, &mut x2);
    }
    if y2 < y1 {
        core::mem::swap(&mut y1, &mut y2);
    }
    // just check if coords inside region
    x >= x1 && y >= y1 && x <= x2 && y <= y2
}

pub unsafe fn check_unit_dead(p: u32) -> bool {
    let mut dead = false;
    if p != 0 {
        if (rb(p + S_FLAGS3 as u32) & (SF_DEAD | SF_DIEING | SF_UNIT_FREE)) != 0 {
            dead = true;
        }
    } else {
        dead = true;
    }
    dead
}

pub unsafe fn check_unit_complete(p: u32) -> bool {
    // for buildings
    let mut f = false;
    if p != 0 {
        if (rb(p + S_FLAGS3 as u32) & SF_COMPLETED) != 0 {
            // flags3 last bit
            f = true;
        }
    }
    f
}

pub unsafe fn check_unit_hidden(p: u32) -> bool {
    let mut f = false;
    if p != 0 {
        if (rb(p + S_FLAGS3 as u32) & SF_HIDDEN) != 0 {
            // flags3 4 bit
            f = true;
        }
    } else {
        f = true;
    }
    f
}

pub unsafe fn check_unit_preplaced(p: u32) -> bool {
    let mut f = false;
    if p != 0 {
        if (rb(p + S_FLAGS3 as u32) & SF_PREPLACED) != 0 {
            // flags3
            f = true;
        }
    }
    f
}

pub unsafe fn check_unit_near_death(p: u32) -> bool {
    let mut dead = false;
    if p != 0 {
        if (rb(p + S_FLAGS3 as u32) & SF_DIEING) != 0
            && (rb(p + S_FLAGS3 as u32) & (SF_DEAD | SF_UNIT_FREE)) == 0
        {
            dead = true;
        }
    } else {
        dead = true;
    }
    dead
}

pub unsafe fn check_peon_loaded(p: u32, r: u8) -> bool {
    let mut f = false;
    if p != 0 {
        if r == 0 {
            if (rb(p + S_PEON_FLAGS as u32) & PEON_LOADED) != 0
                && (rb(p + S_PEON_FLAGS as u32) & PEON_HARVEST_GOLD) != 0
            {
                f = true;
            }
        }
        if r == 1 {
            if (rb(p + S_PEON_FLAGS as u32) & PEON_LOADED) != 0
                && (rb(p + S_PEON_FLAGS as u32) & PEON_HARVEST_LUMBER) != 0
            {
                f = true;
            }
        }
        if r == 2 && (rb(p + S_PEON_FLAGS as u32) & PEON_LOADED) != 0 {
            f = true;
        }
    }
    f
}

pub unsafe fn find_all_units(id: u8) {
    // CAREFUL with this function - ALL units get into massive
    // even if their memory was cleared already
    // all units by id will go in array
    units = 0;
    let mut p: u32 = rd(UNITS_MASSIVE); // pointer to units
    let mut k = rd(UNITS_NUMBER) as i32;
    while k > 0 {
        let f = rb(p + S_ID as u32) == id;
        if f {
            // FIX: в C++ w2p.cpp:477 запись unit[units] шла без проверки границы 1610
            if units < 1610 {
                unit[units as usize] = p;
                units += 1;
            }
        }
        p = p.wrapping_add(0x98);
        k -= 1;
    }
}

pub unsafe fn find_all_alive_units(id: u8) {
    // all units by id will go in array
    units = 0;
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i); // pointer to units list for each player
        while p != 0 {
            let mut f = rb(p + S_ID as u32) == id;
            if id == ANY_BUILDING {
                f = rb(p + S_ID as u32) >= U_FARM; // buildings
            }
            if id == ANY_MEN {
                f = rb(p + S_ID as u32) < U_FARM; // all nonbuildings
            }
            if id == ANY_UNITS {
                f = true; // all ALL units
            }
            if id == ANY_BUILDING_2x2 {
                // small buildings
                let sz = rb(UNIT_SIZE_TABLE + rb(p + S_ID as u32) as u32 * 4);
                f = sz == 2;
            }
            if id == ANY_BUILDING_3x3 {
                // med buildings
                let sz = rb(UNIT_SIZE_TABLE + rb(p + S_ID as u32) as u32 * 4);
                f = sz == 3;
            }
            if id == ANY_BUILDING_4x4 {
                // big buildings
                let sz = rb(UNIT_SIZE_TABLE + rb(p + S_ID as u32) as u32 * 4);
                f = sz == 4;
            }
            if f && !check_unit_dead(p) {
                // FIX: в C++ w2p.cpp:523 запись unit[units] шла без проверки границы 1610
                if units < 1610 {
                    unit[units as usize] = p;
                    units += 1;
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn sort_complete() {
    // only completed units stay in array
    let mut k = 0;
    for i in 0..units {
        if check_unit_complete(unit[i as usize]) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_in_region() {
    // only units in region stay in array
    let mut k = 0;
    for i in 0..units {
        let x = rw(unit[i as usize] + S_DRAW_X as u32) / 32;
        let y = rw(unit[i as usize] + S_DRAW_Y as u32) / 32;
        if in_region(x as u8, y as u8, reg[0], reg[1], reg[2], reg[3]) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_not_in_region() {
    // only units not in region stay in array
    let mut k = 0;
    for i in 0..units {
        let x = rw(unit[i as usize] + S_DRAW_X as u32) / 32;
        let y = rw(unit[i as usize] + S_DRAW_Y as u32) / 32;
        if !in_region(x as u8, y as u8, reg[0], reg[1], reg[2], reg[3]) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_target_in_region() {
    // only units that have order coords in region stay in array
    let mut k = 0;
    for i in 0..units {
        let x = rb(unit[i as usize] + S_ORDER_X as u32);
        let y = rb(unit[i as usize] + S_ORDER_Y as u32);
        if in_region(x, y, reg[0], reg[1], reg[2], reg[3]) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_stat(pr: u8, v: i32, cmp: u8) {
    // only units stay in array if have property compared to value is true
    let mut k = 0;
    for i in 0..units {
        if cmp_stat(unit[i as usize], v, pr, cmp) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_tp_flag() {
    // if not teleported by portal
    let mut k = 0;
    for i in 0..units {
        if (rb(unit[i as usize] + S_FLAGS3 as u32) & SF_TELEPORT) == 0 {
            // unused in actual game flag
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_hidden() {
    // only not hidden units stay in array
    let mut k = 0;
    for i in 0..units {
        if !check_unit_hidden(unit[i as usize]) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_self(u: u32) {
    // unit remove self from array
    let mut k = 0;
    for i in 0..units {
        if unit[i as usize] != u {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}
