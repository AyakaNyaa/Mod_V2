// Поведение саппёров, анстак, золотая шахта (w2p.cpp:3287-3573)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::patch;
use crate::patch::*;
use crate::sort::*;
use crate::state::*;

pub unsafe fn sap_behaviour() {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == U_DWARWES || rb(p + S_ID as u32) == U_GOBLINS;
            if f && !check_unit_dead(p) && !check_unit_hidden(p) {
                let o = rb(p + S_OWNER as u32);
                if rb(CONTROLER_TYPE + o as u32) == C_COMP {
                    let ord = rb(p + S_ORDER as u32);
                    let x = rb(p + S_X as u32);
                    let y = rb(p + S_Y as u32);
                    if ord != ORDER_DEMOLISH && ord != ORDER_DEMOLISH_NEAR && ord != ORDER_DEMOLISH_AT {
                        set_region(x as i32 - 12, y as i32 - 12, x as i32 + 12, y as i32 + 12); // set region around myself
                        find_all_alive_units(ANY_UNITS);
                        sort_in_region();
                        sort_stat(S_MOVEMENT_TYPE, MOV_LAND as i32, CMP_EQ);
                        for ui in 0..16u8 {
                            if check_ally(o, ui) {
                                // only not allied units
                                sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                            }
                        }
                        if units != 0 {
                            let ord = rd(ORDER_FUNCTIONS + 4 * ORDER_DEMOLISH as u32) as i32;
                            f_give_order()(p, 0, 0, unit[0], ord);
                        }
                        set_region(x as i32 - 5, y as i32 - 5, x as i32 + 5, y as i32 + 5); // set region around myself
                        find_all_alive_units(ANY_UNITS);
                        sort_in_region();
                        sort_stat(S_MOVEMENT_TYPE, MOV_LAND as i32, CMP_EQ);
                        for ui in 0..16u8 {
                            if check_ally(o, ui) {
                                // only not allied units
                                sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                            }
                        }
                        if units != 0 {
                            let ord = rd(ORDER_FUNCTIONS + 4 * ORDER_DEMOLISH as u32) as i32;
                            f_give_order()(p, 0, 0, unit[0], ord);
                        }
                        set_region(x as i32 - 1, y as i32 - 1, x as i32 + 1, y as i32 + 1); // set region around myself
                        find_all_alive_units(ANY_UNITS);
                        sort_in_region();
                        sort_stat(S_MOVEMENT_TYPE, MOV_LAND as i32, CMP_EQ);
                        for ui in 0..16u8 {
                            if check_ally(o, ui) {
                                // only not allied units
                                sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                            }
                        }
                        if units != 0 {
                            let ord = rd(ORDER_FUNCTIONS + 4 * ORDER_DEMOLISH as u32) as i32;
                            f_give_order()(p, 0, 0, unit[0], ord);
                        }
                    }
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn unstuk() {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let id = rb(p + S_ID as u32);
            let ord = rb(p + S_ORDER as u32);
            let aord = rb(p + 94);
            let f = ((id < U_CRITTER) && !check_unit_preplaced(p) && (ord == ORDER_STOP) && (aord == 2))
                || (id == U_PEASANT)
                || (id == U_PEON);
            if f && !check_unit_dead(p) && !check_unit_hidden(p) {
                let o = rb(p + S_OWNER as u32);
                if rb(CONTROLER_TYPE + o as u32) == C_COMP {
                    let mut st = rb(p + S_NEXT_FIRE as u32);
                    let frm = rb(p + S_FRAME as u32);
                    let pfrm = rb(p + (S_NEXT_FIRE + 1) as u32);
                    if st == 0 {
                        let map = rb(MAP_SIZE).wrapping_sub(1);
                        let x = rb(p + S_X as u32);
                        let y = rb(p + S_Y as u32);
                        let mut xx: i32 = x as i32;
                        let mut yy: i32 = y as i32;
                        let mut dir = f_net_random()();
                        dir %= 8;
                        if dir == 0 && yy > 0 {
                            yy -= 1;
                        }
                        if dir == 1 {
                            if yy > 0 {
                                yy -= 1;
                            }
                            if xx < map as i32 {
                                xx += 1;
                            }
                        }
                        if dir == 2 && xx < map as i32 {
                            xx += 1;
                        }
                        if dir == 3 {
                            if xx < map as i32 {
                                xx += 1;
                            }
                            if yy < map as i32 {
                                yy += 1;
                            }
                        }
                        if dir == 4 && yy < map as i32 {
                            yy += 1;
                        }
                        if dir == 5 {
                            if yy < map as i32 {
                                yy += 1;
                            }
                            if xx > 0 {
                                xx -= 1;
                            }
                        }
                        if dir == 6 && xx > 0 {
                            xx -= 1;
                        }
                        if dir == 7 {
                            if xx > 0 {
                                xx -= 1;
                            }
                            if yy > 0 {
                                yy -= 1;
                            }
                        }
                        if id != U_PEON && id != U_PEASANT {
                            let mut trg: u32 = 0;
                            find_all_alive_units(ANY_UNITS);
                            sort_hidden();
                            let mv = rb(p + S_MOVEMENT_TYPE as u32);
                            if mv == MOV_LAND {
                                sort_stat(S_MOVEMENT_TYPE, MOV_LAND as i32, CMP_EQ);
                            }
                            sort_attack_can_hit_range(p);
                            for ui in 0..16u8 {
                                if check_ally(o, ui) {
                                    // only not allied units
                                    sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                                }
                            }
                            if units == 0 {
                                find_all_alive_units(ANY_UNITS);
                                sort_hidden();
                                sort_attack_can_hit_range(p);
                                for ui in 0..16u8 {
                                    if check_ally(o, ui) {
                                        // only not allied units
                                        sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                                    }
                                }
                            }
                            if units != 0 {
                                let mut dist: u16 = 0xFFFF;
                                let mut ndu: i32 = -1;
                                for j in 0..units {
                                    let mut l = GPoint {
                                        x: rw(p + S_X as u32),
                                        y: rw(p + S_Y as u32),
                                    };
                                    let dst = f_mtx_dist()(&mut l, unit[j as usize]); // mtx dist
                                    if dst < dist {
                                        dist = dst;
                                        ndu = j;
                                    }
                                }
                                if ndu != -1 {
                                    trg = unit[ndu as usize];
                                }
                            }
                            if trg != 0 {
                                let mut l = GPoint {
                                    x: rw(trg + S_X as u32),
                                    y: rw(trg + S_Y as u32),
                                };
                                units = 1;
                                unit[0] = trg;
                                sort_attack_can_hit_range(p);
                                let mut sa = 0;
                                if units != 0 {
                                    sa = f_ice_set_ai_order()(p, 2, &mut l as *mut GPoint as u32);
                                }
                                if sa != 0 {
                                    give_order(p, (xx % 256) as u8, (yy % 256) as u8, ORDER_ATTACK_AREA);
                                }                            }
                        } else {
                            give_order(p, (xx % 256) as u8, (yy % 256) as u8, ORDER_MOVE);
                        }
                        st = 10;
                    }
                    if st > 0 {
                        st -= 1;
                    }
                    if frm != pfrm {
                        st = 255;
                    }
                    set_stat(p, st as i32, S_NEXT_FIRE);
                    set_stat(p, frm as i32, S_NEXT_FIRE + 1);
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}

pub unsafe fn goldmine_ai() {
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let f = rb(p + S_ID as u32) == U_MINE;
            if f && check_unit_complete(p) {
                let mut x = rb(p + S_X as u32);
                let mut y = rb(p + S_Y as u32);
                set_region(x as i32 - 9, y as i32 - 9, x as i32 + 8, y as i32 + 8);
                find_all_alive_units(ANY_BUILDING_4x4);
                sort_in_region();
                sort_stat(S_ID, U_PORTAL as i32, CMP_NEQ);
                let th = units != 0;
                let x1: u8;
                let y1: u8;
                let x2: u8;
                let y2: u8;
                if x > 3 {
                    x1 = x - 3;
                } else {
                    x1 = 0;
                }
                if y > 3 {
                    y1 = y - 3;
                } else {
                    y1 = 0;
                }
                x = x.wrapping_add(3); // C++: byte += (усечение при переполнении)
                y = y.wrapping_add(3);
                if x >= (127 - 3) {
                    x2 = 127;
                } else {
                    x2 = x + 3;
                }
                if y >= (127 - 3) {
                    y2 = 127;
                } else {
                    y2 = y + 3;
                }
                let sq = rd(MAP_SQ_POINTER);
                let mxs = rb(MAP_SIZE); // map size
                for xx in x1..x2 {
                    for yy in y1..y2 {
                        let mut buf = [rb(sq + 2 * xx as u32 + 2 * yy as u32 * mxs as u32 + 1)];
                        if th {
                            buf[0] |= (SQ_AI_BUILDING >> 8) as u8;
                        } else {
                            buf[0] &= !((SQ_AI_BUILDING >> 8) as u8);
                        }
                        patch::patch_setbytes(sq + 2 * xx as u32 + 2 * yy as u32 * mxs as u32 + 1, &buf);
                    }
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
}
