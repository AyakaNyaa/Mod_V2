// Сохранение/загрузка и реинициализация технологий (w2p.cpp:2837-2958)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orig_fn;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::sort::*;
use crate::state::*;

#[no_mangle]
pub extern "C" fn count_add_to_tables_load_game(u: u32) {
    unsafe {
        if saveload_fixed {
            let f = rb(u + S_AI_AIFLAGS as u32);
            let ff = f | AI_PASSIVE;
            set_stat(u, ff as i32, S_AI_AIFLAGS);
            orig_fn!(g_proc_00451054, extern "C" fn(u32))(u); // original
            set_stat(u, f as i32, S_AI_AIFLAGS);
        } else {
            orig_fn!(g_proc_00451054, extern "C" fn(u32))(u); // original
        }
    }
}

#[no_mangle]
pub extern "C" fn unset_peon_ai_flags(u: u32) {
    unsafe {
        orig_fn!(g_proc_00438A5C, extern "C" fn(u32))(u); // original
        if saveload_fixed {
            let rep = [0x0u8, 0x0];
            for i in 0..8u32 {
                let mut p = rw(SGW_REPAIR_PEONS + 2 * i);
                if p > 1600 {
                    patch_set!(SGW_REPAIR_PEONS + 2 * i, rep);
                }
                p = rw(SGW_GOLD_PEONS + 2 * i);
                if p > 1600 {
                    patch_set!(SGW_GOLD_PEONS + 2 * i, rep);
                }
                p = rw(SGW_TREE_PEONS + 2 * i);
                if p > 1600 {
                    patch_set!(SGW_TREE_PEONS + 2 * i, rep);
                }
            }
        }
    }
}

pub unsafe fn tech_built(p: i32, t: u8) {
    f_tech_built()(p, t);
}

pub unsafe fn tech_reinit() {
    for i in 0..8u8 {
        let o = rb(CONTROLER_TYPE + i as u32);
        if o == C_COMP {
            let mut a = rb(GB_ARROWS + i as u32);
            if a > 0 {
                tech_built(i as i32, UP_ARROW1);
            }
            if a > 1 {
                tech_built(i as i32, UP_ARROW2);
            }
            a = rb(GB_SWORDS + i as u32);
            if a > 0 {
                tech_built(i as i32, UP_SWORD1);
            }
            if a > 1 {
                tech_built(i as i32, UP_SWORD2);
            }
            a = rb(GB_SHIELDS + i as u32);
            if a > 0 {
                tech_built(i as i32, UP_SHIELD1);
            }
            if a > 1 {
                tech_built(i as i32, UP_SHIELD2);
            }
            a = rb(GB_BOAT_ATTACK + i as u32);
            if a > 0 {
                tech_built(i as i32, UP_BOATATK1);
            }
            if a > 1 {
                tech_built(i as i32, UP_BOATATK2);
            }
            a = rb(GB_BOAT_ARMOR + i as u32);
            if a > 0 {
                tech_built(i as i32, UP_BOATARM1);
            }
            if a > 1 {
                tech_built(i as i32, UP_BOATARM2);
            }
            a = rb(GB_CAT_DMG + i as u32);
            if a > 0 {
                tech_built(i as i32, UP_CATDMG1);
            }
            if a > 1 {
                tech_built(i as i32, UP_CATDMG2);
            }
            a = rb(GB_RANGER + i as u32);
            if a != 0 {
                tech_built(i as i32, UP_RANGER);
            }
            a = rb(GB_MARKS + i as u32);
            if a != 0 {
                tech_built(i as i32, UP_SKILL1);
            }
            a = rb(GB_LONGBOW + i as u32);
            if a != 0 {
                tech_built(i as i32, UP_SKILL2);
            }
            a = rb(GB_SCOUTING + i as u32);
            if a != 0 {
                tech_built(i as i32, UP_SKILL3);
            }

            let s = rd(SPELLS_LEARNED + 4 * i as u32) as i32;
            if s & (1 << L_ALTAR_UPGR) != 0 {
                tech_built(i as i32, UP_CLERIC);
            }
            if s & (1 << L_HEAL) != 0 {
                tech_built(i as i32, UP_CLERIC1);
            }
            if s & (1 << L_BLOOD) != 0 {
                tech_built(i as i32, UP_CLERIC1);
            }
            if s & (1 << L_EXORCISM) != 0 {
                tech_built(i as i32, UP_CLERIC2);
            }
            if s & (1 << L_RUNES) != 0 {
                tech_built(i as i32, UP_CLERIC2);
            }
            if s & (1 << L_FLAME_SHIELD) != 0 {
                tech_built(i as i32, UP_WIZARD1);
            }
            if s & (1 << L_RAISE) != 0 {
                tech_built(i as i32, UP_WIZARD1);
            }
            if s & (1 << L_SLOW) != 0 {
                tech_built(i as i32, UP_WIZARD2);
            }
            if s & (1 << L_HASTE) != 0 {
                tech_built(i as i32, UP_WIZARD2);
            }
            if s & (1 << L_INVIS) != 0 {
                tech_built(i as i32, UP_WIZARD3);
            }
            if s & (1 << L_WIND) != 0 {
                tech_built(i as i32, UP_WIZARD3);
            }
            if s & (1 << L_POLYMORF) != 0 {
                tech_built(i as i32, UP_WIZARD4);
            }
            if s & (1 << L_UNHOLY) != 0 {
                tech_built(i as i32, UP_WIZARD4);
            }
            if s & (1 << L_BLIZZARD) != 0 {
                tech_built(i as i32, UP_WIZARD5);
            }
            if s & (1 << L_DD) != 0 {
                tech_built(i as i32, UP_WIZARD5);
            }

            find_all_alive_units(U_KEEP);
            sort_stat(S_OWNER, i as i32, CMP_EQ);
            if units != 0 {
                tech_built(i as i32, UP_KEEP);
            }
            find_all_alive_units(U_STRONGHOLD);
            sort_stat(S_OWNER, i as i32, CMP_EQ);
            if units != 0 {
                tech_built(i as i32, UP_KEEP);
            }
            find_all_alive_units(U_CASTLE);
            sort_stat(S_OWNER, i as i32, CMP_EQ);
            if units != 0 {
                tech_built(i as i32, UP_KEEP);
                tech_built(i as i32, UP_CASTLE);
            }
            find_all_alive_units(U_FORTRESS);
            sort_stat(S_OWNER, i as i32, CMP_EQ);
            if units != 0 {
                tech_built(i as i32, UP_KEEP);
                tech_built(i as i32, UP_CASTLE);
            }
        }
    }
}
