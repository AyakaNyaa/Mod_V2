// ИИ-фиксы и заклинания ИИ (w2p.cpp:2958-3286)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::orig_fn;
use crate::patch;
use crate::patch_set;
use crate::patch::*;
use crate::sort::*;
use crate::state::*;

pub unsafe fn building_start_build(u: u32, id: u8, o: u8) {
    f_bldg_start_build()(u, id, o);
}

pub unsafe fn build_inventor(u: u32) {
    if check_unit_complete(u) {
        let f = rb(u + S_FLAGS1 as u32);
        if (f & UF_BUILD_ON) == 0 {
            let id = rb(u + S_ID as u32);
            let o = rb(u + S_OWNER as u32);
            let spr = get_val(ACTIVE_SAPPERS, o as i32);
            let nspr = rb(AIP_NEED_SAP + 48 * o as u32);
            if nspr as i32 > spr {
                if id == U_INVENTOR {
                    building_start_build(u, U_DWARWES, 0);
                }
                if id == U_ALCHEMIST {
                    building_start_build(u, U_GOBLINS, 0);
                }
            }
            let flr = get_val(ACTIVE_FLYER, o as i32);
            let nflr = rb(AIP_NEED_FLYER + 48 * o as u32);
            if nflr as i32 > flr {
                if id == U_INVENTOR {
                    building_start_build(u, U_FLYER, 0);
                }
                if id == U_ALCHEMIST {
                    building_start_build(u, U_ZEPPELIN, 0);
                }
            }
        }
    }
}

pub unsafe fn build_sap_fix(f: bool) {
    if f {
        let mut b1 = [0x80u8, 0xfa, 0x40, 0x0];
        b1[0..4].copy_from_slice(&(build_inventor as usize as u32).to_le_bytes());
        patch_set!(BLDG_WAIT_INVENTOR, b1); // human inv
        patch_set!(BLDG_WAIT_INVENTOR + 4, b1); // orc inv
    } else {
        let b1 = [0x80u8, 0xfa, 0x40, 0x0];
        patch_set!(BLDG_WAIT_INVENTOR, b1); // human inv
        patch_set!(BLDG_WAIT_INVENTOR + 4, b1); // orc inv
    }
}

pub unsafe fn ai_fix_plugin(f: bool) {
    if f {
        let b1 = [0xb2u8, 0x02];
        patch_set!(AIFIX_PEONS_REP, b1); // 2 peon rep
        let b21 = [0xbbu8, 0x8];
        patch_set!(AIFIX_GOLD_LUMB1, b21); // gold lumber
        let b22 = [0xb4u8, 0x4];
        patch_set!(AIFIX_GOLD_LUMB2, b22); // gold lumber
        let b3 = [0x1u8];
        patch_set!(AIFIX_BUILD_SIZE, b3); // packed build
        let b4 = [0xbeu8, 0x0, 0x0, 0x0, 0x0, 0x90, 0x90];
        patch_set!(AIFIX_FIND_HOME, b4); // th corner
        let b5 = [0x90u8, 0x90];
        patch_set!(AIFIX_DD_BLIZ_FIX, b5); // fix dd/bliz
        let b6 = [0x90u8, 0x90, 0x90, 0x90, 0x90, 0x90];
        patch_set!(AIFIX_POWERBUILD, b6); // powerbuild
        let b7 = [0x90u8, 0x90, 0x90, 0x90];
        patch_set!(AIFIX_CATA_AFRAID, b7); // cata afraid
        let b8 = [0x70u8, 0x79];
        patch_set!(AIFIX_SHIPS_PATROL, b8); // ships random patrol

        let m1 = [
            0xa9u8, 0x0, 0x0, 0x0, 0x4, 0x74, 0x16, 0x8b, 0x44, 0x24, 0xc, 0x66, 0x83, 0x78,
            0x44, 0x0, 0x75, 0xb, 0x90, 0x90, 0x90, 0x90,
        ];
        patch_set!(AIFIX_INVIZ_COND, m1); // inviz cond
        let m2 = [0x90u8, 0x90, 0x90];
        patch_set!(AIFIX_BLIZ_3MP1, m2); // no x3 bliz mp
        patch_set!(AIFIX_BLIZ_3MP2, m2); // no x3 bliz mp
        let m3 = [
            0x33u8, 0xd2, 0x8a, 0x50, 0x27, 0x3e, 0x8b, 0x4, 0x95, 0x24, 0xf5, 0x4c, 0x0,
            0xa8, 0x20, 0x75, 0x19, // 17 байт кода
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
            0x90, 0x90, 0x90, 0x90, // 17 NOP — всего 34 байта, как в C++ w2p.cpp:3036
        ];
        patch_set!(AIFIX_FIREBALL_COND, m3); // fire cond
        let m4 = [
            0x90u8, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
            0x90, 0x90,
        ];
        patch_set!(AIFIX_BLIZ_COND, m4); // bliz cond
        let m5 = [0x96u8];
        patch_set!(AIFIX_INV_POLY_JMP, m5); // no inv poly > jmp exit
        let m6 = [0xe9u8, 0x6c, 0x2, 0x0, 0x0];
        patch_set!(AIFIX_INV_POLY_CAVE, m6); // jmp to exit
        let m7 = [0x90u8, 0x90];
        patch_set!(AIFIX_RUNES_INV, m7); // runes no inviz
        let m8 = [0xebu8];
        patch_set!(AIFIX_STARTING_MAGE, m8); // starting mage

        build_sap_fix(true);

        ai_fixed = true;
    } else {
        let b1 = [0x8au8, 0xd0];
        patch_set!(AIFIX_PEONS_REP, b1); // 2 peon rep
        let b21 = [0xd0u8, 0x7];
        patch_set!(AIFIX_GOLD_LUMB1, b21); // gold lumber
        let b22 = [0xf4u8, 0x1];
        patch_set!(AIFIX_GOLD_LUMB2, b22); // gold lumber
        let b3 = [0x6u8];
        patch_set!(AIFIX_BUILD_SIZE, b3); // packed build
        let b4 = [0xe8u8, 0xf8, 0x2a, 0x1, 0x0, 0x8b, 0xf0];
        patch_set!(AIFIX_FIND_HOME, b4); // th corner
        let b5 = [0x75u8, 0x7];
        patch_set!(AIFIX_DD_BLIZ_FIX, b5); // fix dd/bliz
        let b6 = [0xfu8, 0x84, 0x78, 0x1, 0x0, 0x0];
        patch_set!(AIFIX_POWERBUILD, b6); // powerbuild
        let b7 = [0x89u8, 0x44, 0x24, 0x10];
        patch_set!(AIFIX_CATA_AFRAID, b7); // cata afraid
        let b8 = [0x10u8, 0x7A];
        patch_set!(AIFIX_SHIPS_PATROL, b8); // ships random patrol

        let m1 = [
            0x80u8, 0x7a, 0x5e, 0x2, 0x75, 0x17, 0xa9, 0x0, 0x0, 0x2, 0x0, 0x75, 0x9, 0x80,
            0xb9, 0xf0, 0xfe, 0x4c, 0x0, 0x1, 0x74, 0x7,
        ];
        patch_set!(AIFIX_INVIZ_COND, m1); // inviz cond
        let mut m2 = [0x8du8, 0x4, 0x40];
        patch_set!(AIFIX_BLIZ_3MP1, m2); // no x3 bliz mp
        m2[1] = 0x14;
        patch_set!(AIFIX_BLIZ_3MP2, m2); // no x3 bliz mp
        let m3 = [
            0x6au8, 0x3, 0x50, 0x56, 0xe8, 0x3, 0xff, 0xff, 0xff, 0x83, 0xc4, 0xc, 0x85, 0xc0,
            0x74, 0x12, 0x56, 0xe8, 0xe6, 0xd8, 0x2, 0x0, 0x66, 0xd1, 0xe8, 0x83, 0xc4, 0x4,
            0x66, 0x39, 0x46, 0x22, 0x73, 0x8,
        ];
        patch_set!(AIFIX_FIREBALL_COND, m3); // fire cond
        let m4 = [
            0x6au8, 0x3, 0x50, 0x51, 0xe8, 0x96, 0xfe, 0xff, 0xff, 0x83, 0xc4, 0xc, 0x85,
            0xc0, 0x75, 0x07,
        ];
        patch_set!(AIFIX_BLIZ_COND, m4); // bliz cond
        let m5 = [0xcu8];
        patch_set!(AIFIX_INV_POLY_JMP, m5); // no inv poly > jmp exit
        let m6 = [0x90u8, 0x90, 0x90, 0x90, 0x90];
        patch_set!(AIFIX_INV_POLY_CAVE, m6); // jmp to exit
        let m7 = [0x74u8, 0x1d];
        patch_set!(AIFIX_RUNES_INV, m7); // runes no inviz
        let m8 = [0x74u8];
        patch_set!(AIFIX_STARTING_MAGE, m8); // starting mage

        build_sap_fix(false);

        ai_fixed = false;
    }
}

#[no_mangle]
pub extern "C" fn upgrade_tower(u: u32, mut id: i32, b: i32) {
    unsafe {
        if ai_fixed {
            let o = rb(u + S_OWNER as u32);
            if get_val(LUMBERMILL, o as i32) == 0 {
                id += 2;
            }
            if get_val(SMITH, o as i32) != 0 && (get_val(TOWER, o as i32) % 2) == 0 {
                id += 2;
            }
        }
        orig_fn!(g_proc_0040EEDD, extern "C" fn(u32, i32, i32))(u, id, b); // original
    }
}

#[no_mangle]
pub extern "C" fn create_skeleton(x: i32, y: i32, id: i32, o: i32) {
    unsafe {
        if ai_fixed {
            unit_create(x / 32 + 1, y / 32, id, (o % 256) as u8, 1);
        } else {
            orig_fn!(g_proc_00442E25, extern "C" fn(i32, i32, i32, i32))(x, y, id, o); // original
        }
    }
}

#[no_mangle]
pub extern "C" fn cast_raise(u: u32, a1: i32, a2: i32, a3: i32) -> u32 {
    unsafe {
        if ai_fixed {
            let o = rb(u + S_OWNER as u32);
            find_all_alive_units(U_SKELETON);
            sort_stat(S_OWNER, o as i32, CMP_EQ);
            if units < 10 {
                // FIX: в C++ w2p.cpp:3132 стоял бит RAISE_DEAD(12) вместо L_RAISE(13);
                // читался 1 байт, куда биты 12/13 не помещаются (проверка была мёртвой) —
                // читаем DWORD, бит спелла в SPELLS_LEARNED = L_RAISE(13)
                if (rd(SPELLS_LEARNED + 4 * o as u32) & (1 << L_RAISE)) != 0 {
                    return 0;
                }
                let mp = rb(u + S_MANA as u32);
                let cost = rb(MANACOST + 2 * RAISE_DEAD as u32);
                if mp < cost {
                    return 0;
                }
                let x = rb(u + S_X as u32);
                let y = rb(u + S_Y as u32);
                set_region(x as i32 - 8, y as i32 - 8, x as i32 + 8, y as i32 + 8); // set region around myself
                find_all_units(ANY_BUILDING); // dead body
                sort_in_region();
                sort_hidden();
                sort_near_death();
                if units != 0 {
                    let xx = rb(unit[0] + S_X as u32);
                    let yy = rb(unit[0] + S_Y as u32);
                    give_order(u, xx, yy, ORDER_SPELL_RAISEDEAD);
                    return unit[0];
                }
            }
            return 0;
        }
        orig_fn!(g_proc_00425D1C, extern "C" fn(u32, i32, i32, i32) -> u32)(u, a1, a2, a3) // original
    }
}

#[no_mangle]
pub extern "C" fn cast_runes(u: u32, a1: i32, a2: i32, a3: i32) -> u32 {
    unsafe {
        if ai_fixed {
            let o = rb(u + S_OWNER as u32);
            // FIX: в C++ w2p.cpp:3164 стоял бит RUNES(17) вместо L_RUNES(18);
            // читался 1 байт (биты 17/18 туда не помещались — проверка была мёртвой) —
            // читаем DWORD, бит спелла в SPELLS_LEARNED = L_RUNES(18)
            if (rd(SPELLS_LEARNED + 4 * o as u32) & (1 << L_RUNES)) != 0 {
                return 0;
            }
            let mp = rb(u + S_MANA as u32);
            let cost = rb(MANACOST + 2 * RUNES as u32);
            if mp < cost {
                return 0;
            }
            let x = rb(u + S_X as u32);
            let y = rb(u + S_Y as u32);
            set_region(x as i32 - 14, y as i32 - 14, x as i32 + 14, y as i32 + 14); // set region around myself
            find_all_alive_units(ANY_MEN);
            sort_in_region();
            sort_hidden();
            sort_stat(S_MOVEMENT_TYPE, MOV_LAND as i32, CMP_EQ);
            for ui in 0..16u8 {
                if check_ally(o, ui) {
                    // only not allied units
                    sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                }
            }
            sort_rune_near();
            if units != 0 {
                let xx = rb(unit[0] + S_X as u32);
                let yy = rb(unit[0] + S_Y as u32);
                give_order(u, xx, yy, ORDER_SPELL_RUNES);
                return unit[0];
            }
            return 0;
        }
        orig_fn!(g_proc_00424F94, extern "C" fn(u32, i32, i32, i32) -> u32)(u, a1, a2, a3) // original
    }
}

#[no_mangle]
pub extern "C" fn ai_spell(u: u32) -> i32 {
    unsafe {
        if ai_fixed {
            let id = rb(u + S_ID as u32);
            // FIX: в C++ w2p.cpp:3200 стояло присваивание (id = U_DK) вместо сравнения
            if id == U_MAGE || id == U_DK {
                let x = rb(u + S_X as u32);
                let y = rb(u + S_Y as u32);
                set_region(x as i32 - 30, y as i32 - 30, x as i32 + 30, y as i32 + 30); // set region around myself
                find_all_alive_units(ANY_UNITS);
                sort_in_region();
                let o = rb(u + S_OWNER as u32);
                for ui in 0..16u8 {
                    if check_ally(o, ui) {
                        sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                    }
                }
                if units != 0 {
                    return orig_fn!(g_proc_0042757E, extern "C" fn(u32) -> i32)(u); // original
                }
            } else {
                return orig_fn!(g_proc_0042757E, extern "C" fn(u32) -> i32)(u); // original
            }
            return 0;
        }
        orig_fn!(g_proc_0042757E, extern "C" fn(u32) -> i32)(u) // original
    }
}

#[no_mangle]
pub extern "C" fn ai_attack(u: u32, b: i32, a: i32) {
    unsafe {
        if ai_fixed {
            let o = rb(u + S_OWNER as u32);
            for i in 0..16u32 {
                let mut p: u32 = rd(UNITS_LISTS + 4 * i);
                while p != 0 {
                    let f = rb(p + S_ID as u32) == U_MAGE || rb(p + S_ID as u32) == U_DK;
                    if f && !check_unit_dead(p) && !check_unit_hidden(p) {
                        let ow = rb(p + S_OWNER as u32);
                        if rb(CONTROLER_TYPE + o as u32) == C_COMP {
                            let inv = rw(p + S_INVIZ as u32);
                            if inv == 0 {
                                let aor = rb(p + 94);
                                if aor != 2 {
                                    let x = rb(u + S_X as u32);
                                    let y = rb(u + S_Y as u32);
                                    set_region(x as i32 - 30, y as i32 - 30, x as i32 + 30, y as i32 + 30); // set region around myself
                                    find_all_alive_units(ANY_UNITS);
                                    sort_in_region();
                                    for ui in 0..16u8 {
                                        if check_ally(ow, ui) {
                                            sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                                        }
                                    }
                                    if units == 0 {
                                        f_ice_set_ai_order()(p, AI_ORDER_ATTACK as i32, a as u32); // ai attack
                                    }
                                }
                            }
                        }
                    }
                    p = rd(p + S_NEXT_UNIT_POINTER as u32);
                }
            }

            find_all_alive_units(ANY_MEN);
            sort_stat(S_ID, U_DWARWES as i32, CMP_BIGGER_EQ);
            sort_stat(S_ID, U_GOBLINS as i32, CMP_SMALLER_EQ);
            sort_stat(S_OWNER, o as i32, CMP_EQ);
            sort_stat(S_AI_ORDER, AI_ORDER_ATTACK as i32, CMP_NEQ); // not attack already
            for i in 0..units {
                f_ice_set_ai_order()(unit[i as usize], AI_ORDER_ATTACK as i32, a as u32); // ai attack
            }
        }
        orig_fn!(g_proc_00427FAE, extern "C" fn(u32, i32, i32))(u, b, a); // original
    }
}
