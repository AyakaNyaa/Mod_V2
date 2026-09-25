// Остальные сортировки и массовые операции (w2p.cpp:694-1007)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::patch::*;
use crate::state::*;

pub unsafe fn sort_full_hp() {
    // if hp not full
    let mut k = 0;
    for i in 0..units {
        let id = rb(unit[i as usize] + S_ID as u32); // unit id
        let mhp = rw(UNIT_HP_TABLE + 2 * id as u32); // max hp
        let hp = rw(unit[i as usize] + S_HP as u32); // unit hp
        if hp < mhp {
            // hp not full
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_fleshy() {
    // only fleshy units stay in array
    let mut k = 0;
    for i in 0..units {
        let id = rb(unit[i as usize] + S_ID as u32); // unit id
        if (rd(UNIT_GLOBAL_FLAGS + id as u32 * 4) & IS_FLESHY) != 0 {
            // fleshy global flag
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_order_hp() {
    // order array by hp from low to high
    for i in 0..units {
        let mut sm = i;
        for j in (i + 1)..units {
            let hpsm = rw(unit[sm as usize] + S_HP as u32); // unit hp
            let hpj = rw(unit[j as usize] + S_HP as u32); // unit hp
            if hpj < hpsm {
                sm = j;
            }
        }
        unit.swap(i as usize, sm as usize);
    }
}

pub unsafe fn sort_preplaced() {
    let mut k = 0;
    for i in 0..units {
        if !check_unit_preplaced(unit[i as usize]) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_near_death() {
    let mut k = 0;
    for i in 0..units {
        if check_unit_near_death(unit[i as usize]) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_attack_can_hit(p: u32) {
    // only units stay in array that *p can attack them
    let mut k = 0;
    for i in 0..units {
        let a = f_attack_can_hit()(p, unit[i as usize]); // attack can hit original war2 function
        if a != 0 {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_attack_can_hit_range(p: u32) {
    // only units stay in array that *p can attack them and have passable terrain in attack range
    let mut k = 0;
    for i in 0..units {
        let a = f_attack_can_hit()(p, unit[i as usize]); // attack can hit
        if a != 0 {
            let id = rb(unit[i as usize] + S_ID as u32);
            let szx = rb(UNIT_SIZE_TABLE + 4 * id as u32);
            let szy = rb(UNIT_SIZE_TABLE + 4 * id as u32 + 2);
            let idd = rb(p + S_ID as u32);
            let rng = rb(UNIT_RANGE_TABLE + idd as u32);
            let _ms = rb(MAP_SIZE);
            let mut xx = rb(unit[i as usize] + S_X as u32);
            let mut yy = rb(unit[i as usize] + S_Y as u32);
            if xx < rng {
                xx = 0;
            } else {
                xx -= rng;
            }
            if yy < rng {
                yy = 0;
            } else {
                yy -= rng;
            }
            let cl = rb(p + S_MOVEMENT_TYPE as u32); // movement type
            let mt = rw(GLOBAL_MOVEMENT_TERRAIN_FLAGS + 2 * cl as u32); // movement terrain flags

            let mut f = false;
            let mut x = xx as i32;
            while (x < szx as i32 + xx as i32 + rng as i32 * 2 + 1) && (x < 127) {
                let mut y = yy as i32;
                // FIX: в C++ w2p.cpp:842 внутренний цикл по y проверял x<127 вместо y<127
                while (y < szy as i32 + yy as i32 + rng as i32 * 2 + 1) && (y < 127) {
                    let mut aa = 1;
                    if cl == 0 || cl == 3 {
                        // land and docked transport
                        aa = f_xy_passable()(x, y, mt as i32); // original war2 func if terrain passable with that movement type
                    }
                    if (x % 2 == 0) && (y % 2 == 0) {
                        // air and water
                        if cl == 1 || cl == 2 {
                            aa = f_xy_passable()(x, y, mt as i32);
                        }
                    }
                    if aa == 0 {
                        f = true;
                    }
                    y += 1;
                }
                x += 1;
            }
            if f {
                unitt[k as usize] = unit[i as usize];
                k += 1;
            }
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_rune_near() {
    let mut k = 0;
    for i in 0..units {
        let x = rb(unit[i as usize] + S_X as u32);
        let y = rb(unit[i as usize] + S_Y as u32);
        let mut f = false;
        for r in 0..50u32 {
            // max runes 50
            let d = rw(RUNEMAP_TIMERS + 2 * r);
            if d != 0 {
                let xx = rb(RUNEMAP_X + r);
                let yy = rb(RUNEMAP_Y + r);
                if xx == x {
                    if yy > y {
                        if (yy - y) == 1 {
                            f = true;
                        }
                    } else if (y - yy) == 1 {
                        f = true;
                    }
                }
                if yy == y {
                    if xx > x {
                        if (xx - x) == 1 {
                            f = true;
                        }
                    } else if (x - xx) == 1 {
                        f = true;
                    }
                }
            }
        }
        if !f {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_peon_loaded(r: u8) {
    let mut k = 0;
    for i in 0..units {
        if check_peon_loaded(unit[i as usize], r) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn sort_peon_not_loaded(r: u8) {
    let mut k = 0;
    for i in 0..units {
        if !check_peon_loaded(unit[i as usize], r) {
            unitt[k as usize] = unit[i as usize];
            k += 1;
        }
    }
    units = k;
    for i in 0..units {
        unit[i as usize] = unitt[i as usize];
    }
}

pub unsafe fn set_stat_all(pr: u8, v: i32) {
    for i in 0..units {
        set_stat(unit[i as usize], v, pr); // set stat to all units in array
    }
}

pub unsafe fn set_tp_flag(f: bool) {
    for i in 0..units {
        // set if unit can be teleported by portal (that flag unused in actual game)
        if f {
            wb(
                unit[i as usize] + S_FLAGS3 as u32,
                rb(unit[i as usize] + S_FLAGS3 as u32) | SF_TELEPORT,
            );
        } else {
            wb(
                unit[i as usize] + S_FLAGS3 as u32,
                rb(unit[i as usize] + S_FLAGS3 as u32) & !SF_TELEPORT,
            );
        }
    }
}

pub unsafe fn kill_all() {
    for i in 0..units {
        unit_kill(unit[i as usize]); // just kill all in array
    }
    units = 0;
}

pub unsafe fn remove_all() {
    for i in 0..units {
        unit_remove(unit[i as usize]); // just kill all in array
    }
    units = 0;
}

pub unsafe fn cast_all() {
    for i in 0..units {
        unit_cast(unit[i as usize]); // casting spells
    }
    units = 0;
}
