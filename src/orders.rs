// Заказы, дамаг/хил, перемещения, визоры (w2p.cpp:1008-1353)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::patch;
use crate::patch::*;
use crate::state::*;

pub unsafe fn flame(p: u32) {
    // p - unit
    // original war2 func creates 1 flame with selected animation frame
    // flameshield have 5 flames
    f_create_flame()(p, 0, 0);
    f_create_flame()(p, 10, 4);
    f_create_flame()(p, 20, 2);
    f_create_flame()(p, 30, 3);
    f_create_flame()(p, 40, 5);
}

pub unsafe fn flame_all() {
    for i in 0..units {
        flame(unit[i as usize]);
    }
}

pub unsafe fn damag(p: u32, n1: u8, n2: u8) {
    let hp = rw(p + S_HP as u32); // unit hp
    let n = n1 as u16 + 256 * n2 as u16;
    if hp > n {
        set_stat(p, (hp - n) as i32, S_HP);
    } else {
        set_stat(p, 0, S_HP);
        unit_kill(p);
    }
}

pub unsafe fn damag_all(n1: u8, n2: u8) {
    for i in 0..units {
        damag(unit[i as usize], n1, n2);
    }
}

pub unsafe fn heal(p: u32, n1: u8, n2: u8) {
    let id = rb(p + S_ID as u32); // unit id
    let mhp = rw(UNIT_HP_TABLE + 2 * id as u32); // max hp
    let hp = rw(p + S_HP as u32); // unit hp
    let n = n1 as u16 + 256 * n2 as u16;
    if hp < mhp {
        let mut hp = hp + n;
        if hp > mhp {
            hp = mhp; // canot heal more than max hp
        }
        set_stat(p, hp as i32, S_HP);
    }
}

pub unsafe fn heal_all(n1: u8, n2: u8) {
    for i in 0..units {
        heal(unit[i as usize], n1, n2);
    }
}

pub unsafe fn mana_regen(p: u32, n: u8) {
    let tid = rb(p + S_ID as u32);
    let f = tid == U_MAGE
        || tid == U_DK
        || tid == U_PALADIN
        || tid == U_OGREMAGE
        || tid == U_HADGAR
        || tid == U_TERON
        || tid == U_GULDAN
        || tid == U_UTER
        || tid == U_TYRALYON
        || tid == U_CHOGAL
        || tid == U_DENTARG;
    if f {
        let mut mp = rb(p + S_MANA as u32); // unit mana
        if mp as i32 + n as i32 > 255 {
            mp = 255;
        } else {
            mp += n;
        }
        set_stat(p, mp as i32, S_MANA);
    }
}

pub unsafe fn mana_regen_all(n: u8) {
    for i in 0..units {
        mana_regen(unit[i as usize], n);
    }
}

pub unsafe fn peon_load(u: u32, r: u8) {
    let mut f = rb(u + S_PEON_FLAGS as u32);
    if (f & PEON_LOADED) == 0 {
        if r == 0 {
            f |= PEON_LOADED;
            f |= PEON_HARVEST_GOLD;
            set_stat(u, f as i32, S_PEON_FLAGS);
            f_group_set()(u);
        } else {
            f |= PEON_LOADED;
            f |= PEON_HARVEST_LUMBER;
            set_stat(u, f as i32, S_PEON_FLAGS);
            f_group_set()(u);
        }
    }
}

pub unsafe fn peon_load_all(r: u8) {
    for i in 0..units {
        peon_load(unit[i as usize], r);
    }
}

pub unsafe fn viz_area(x: u8, y: u8, pl: u8, sz: u8) {
    let vf: extern "C" fn(u16, u16, u8) = match sz {
        0 => core::mem::transmute(F_VISION2 as usize),
        1 => core::mem::transmute(F_VISION2 as usize),
        2 => core::mem::transmute(F_VISION2 as usize),
        3 => core::mem::transmute(F_VISION3 as usize),
        4 => core::mem::transmute(F_VISION4 as usize),
        5 => core::mem::transmute(F_VISION5 as usize),
        6 => core::mem::transmute(F_VISION6 as usize),
        7 => core::mem::transmute(F_VISION7 as usize),
        8 => core::mem::transmute(F_VISION8 as usize),
        9 => core::mem::transmute(F_VISION9 as usize),
        _ => core::mem::transmute(F_VISION2 as usize),
    };
    for i in 0..8u8 {
        if ((1u8 << i) & pl) != 0 {
            vf(x as u16, y as u16, i);
        }
    }
}

pub unsafe fn viz_area_add(x: u8, y: u8, pl: u8, sz: u8) {
    if vizs_n >= 0 && vizs_n <= 255 {
        vizs_areas[vizs_n as usize].x = x;
        vizs_areas[vizs_n as usize].y = y;
        vizs_areas[vizs_n as usize].p = pl;
        vizs_areas[vizs_n as usize].s = sz;
        vizs_n += 1;
    }
}

pub unsafe fn viz_area_all(pl: u8, sz: u8) {
    for i in 0..units {
        let x = rb(unit[i as usize] + S_X as u32);
        let y = rb(unit[i as usize] + S_Y as u32);
        viz_area_add(x, y, pl, sz);
    }
}

pub unsafe fn give(p: u32, owner: u8) {
    f_capture()(p, owner, 1); // original capture unit war2 func
    let a = RESCUED_UNITS + 2 * owner as u32;
    wb(a, rb(a).wrapping_sub(1)); // reset number of captured units
}

pub unsafe fn give_all(o: u8) {
    for i in 0..units {
        give(unit[i as usize], o);
    }
}

pub unsafe fn unit_move(x: u8, y: u8, u: u32) -> bool {
    // (x<0/y<0 в C++ невозможны для byte — пропущено)
    let mxs = rb(MAP_SIZE); // map size
    if x >= mxs {
        return false;
    }
    if y >= mxs {
        return false; // canot go outside map
    }
    if check_unit_hidden(u) {
        return false; // if unit not hidden
    }
    let cl = rb(u + S_MOVEMENT_TYPE as u32); // movement type
    let mt = rw(GLOBAL_MOVEMENT_TERRAIN_FLAGS + 2 * cl as u32); // movement terrain flags

    let mut aa = 1;
    if cl == 0 || cl == 3 {
        // land and docked transport
        aa = f_xy_passable()(x as i32, y as i32, mt as i32); // original war2 func if terrain passable with that movement type
    }
    if (x % 2 == 0) && (y % 2 == 0) {
        // air and water
        if cl == 1 || cl == 2 {
            aa = f_xy_passable()(x as i32, y as i32, mt as i32);
        }
    }
    if aa == 0 {
        f_unit_unplace()(u); // unplace
        set_stat(u, x as i32, S_X);
        set_stat(u, y as i32, S_Y); // change real coords
        set_stat(u, x as i32 * 32, S_DRAW_X);
        set_stat(u, y as i32 * 32, S_DRAW_Y); // change draw sprite coords
        f_unit_place()(u); // place
        return true;
    }
    false
}

pub unsafe fn move_all(x: u8, y: u8) {
    sort_stat(S_ID, U_FARM as i32, CMP_SMALLER); // non buildings
    sort_stat(S_ANIMATION, 2, CMP_EQ); // only if animation stop
    for i in 0..units {
        let mut xx: i32 = 0;
        let mut yy: i32 = 0;
        let mut k: i32 = 1;
        let mut f = unit_move(x, y, unit[i as usize]);
        xx -= 1;
        while (!f) & (k < 5) {
            // goes in spiral like original war2 (size 5)
            while (!f) & (yy < k) {
                f = unit_move((x as i32 + xx) as u8, (y as i32 + yy) as u8, unit[i as usize]);
                yy += 1;
            }
            while (!f) & (xx < k) {
                f = unit_move((x as i32 + xx) as u8, (y as i32 + yy) as u8, unit[i as usize]);
                xx += 1;
            }
            while (!f) & (yy > -k) {
                f = unit_move((x as i32 + xx) as u8, (y as i32 + yy) as u8, unit[i as usize]);
                yy -= 1;
            }
            while (!f) & (xx >= -k) {
                f = unit_move((x as i32 + xx) as u8, (y as i32 + yy) as u8, unit[i as usize]);
                xx -= 1;
            }
            k += 1;
        }
    }
}

pub unsafe fn give_order(u: u32, x: u8, y: u8, o: u8) {
    let id = rb(u + S_ID as u32);
    if id < U_FARM {
        let mut buf = [0u8];
        let f = o >= ORDER_SPELL_VISION && o <= ORDER_SPELL_ROT;
        if f {
            buf[0] = o;
            patch::patch_setbytes(GW_ACTION_TYPE, &buf);
        }
        let mut tr: u32 = 0;
        for i in 0..16u32 {
            let mut p: u32 = rd(UNITS_LISTS + 4 * i); // pointer to units list for each player
            while p != 0 {
                if !check_unit_dead(p) {
                    let xx = rb(p + S_X as u32);
                    let yy = rb(p + S_Y as u32);
                    if ((x as i32 - xx as i32).abs() <= 2) && ((y as i32 - yy as i32).abs() <= 2) {
                        if f {
                            let idd = rb(p + S_ID as u32);
                            if idd < U_FARM {
                                tr = p;
                            }
                        } else {
                            tr = p;
                        }
                    }
                }
                p = rd(p + S_NEXT_UNIT_POINTER as u32);
            }
        }
        let aoe = o == ORDER_SPELL_VISION
            || o == ORDER_SPELL_EXORCISM
            || o == ORDER_SPELL_FIREBALL
            || o == ORDER_SPELL_BLIZZARD
            || o == ORDER_SPELL_EYE
            || o == ORDER_SPELL_RAISEDEAD
            || o == ORDER_SPELL_DRAINLIFE
            || o == ORDER_SPELL_WHIRLWIND
            || o == ORDER_SPELL_RUNES
            || o == ORDER_SPELL_ROT
            || o == ORDER_MOVE
            || o == ORDER_PATROL
            || o == ORDER_ATTACK_AREA
            || o == ORDER_ATTACK_WALL
            || o == ORDER_STAND
            || o == ORDER_ATTACK_GROUND
            || o == ORDER_ATTACK_GROUND_MOVE
            || o == ORDER_DEMOLISH
            || o == ORDER_HARVEST
            || o == ORDER_RETURN
            || o == ORDER_UNLOAD_ALL;

        if o != ORDER_ATTACK_WALL {
            let ord = rd(ORDER_FUNCTIONS + 4 * o as u32) as i32; // orders functions
            if !aoe && (tr != 0) && (tr != u) {
                f_give_order()(u, 0, 0, tr, ord); // original war2 order
            }
            if aoe {
                f_give_order()(u, x as i32, y as i32, 0, ord); // original war2 order
            }
        } else {
            let oru = rb(u + S_ORDER as u32);
            if oru != ORDER_ATTACK_WALL {
                let ord = rd(ORDER_FUNCTIONS + 4 * ORDER_STOP as u32) as i32; // orders functions
                f_give_order()(u, 0, 0, 0, ord); // original war2 order
            }
            set_stat(u, ORDER_ATTACK_WALL as i32, S_NEXT_ORDER);
            set_stat(u, x as i32, S_ORDER_X);
            set_stat(u, y as i32, S_ORDER_Y);
        }

        if f {
            buf[0] = 0;
            patch::patch_setbytes(GW_ACTION_TYPE, &buf);
        }
    }
}

pub unsafe fn order_all(x: u8, y: u8, o: u8) {
    for i in 0..units {
        give_order(unit[i as usize], x, y, o);
    }
}

pub unsafe fn check_ally(p1: u8, p2: u8) -> bool {
    // check allied table
    rb(ALLY + p1 as u32 + 16 * p2 as u32) != 0 && rb(ALLY + p2 as u32 + 16 * p1 as u32) != 0
}
