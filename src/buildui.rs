// UI строительства: кнопки, лес, демоны, герои-иконки (w2p.cpp:2012-2288)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::state::*;

// always return false function (int empty_false(byte))
#[no_mangle]
pub extern "C" fn empty_false(_: u8) -> i32 {
    0
}

// always return true function (int empty_true(byte))
#[no_mangle]
pub extern "C" fn empty_true(_: u8) -> i32 {
    1
}

// адрес отдаётся игре — cdecl, как в C++
#[no_mangle]
pub extern "C" fn empty_build(id: i32) {
    unsafe { f_train_unit()(id) } // original build unit func
}

// адрес отдаётся игре — cdecl, как в C++
#[no_mangle]
pub extern "C" fn build_forest(_: i32) {
    unsafe {
        let local = rb(LOCAL_PLAYER);
        // C++: p = &slot; if (p) { p = *p; if (p) {... } } — rd() уже выполняет
        // единственное разыменование (*p = указатель на юнита)
        let p: u32 = rd(UNITS_SELECTED + 9 * 4 * local as u32);
        if p != 0 {
            let id = rb(p + S_ID as u32);
            let o = rb(p + S_OWNER as u32);
            if o == local {
                if id == U_PEASANT {
                    f_build_building()(U_PIGFARM as i32); // original build func
                }
                if id == U_PEON {
                    f_build_building()(U_FARM as i32); // original build func
                }
            }
        }
    }
}

// int _2tir() — проверка 2 тира ратуши; адрес кладётся в таблицы кнопок
#[no_mangle]
pub extern "C" fn _2tir() -> i32 {
    unsafe {
        if get_val(TH2, rb(LOCAL_PLAYER) as i32) != 0
            || get_val(TH3, rb(LOCAL_PLAYER) as i32) != 0
        {
            1
        } else {
            0
        }
    }
}

// адрес отдаётся игре — cdecl, как в C++
#[no_mangle]
pub extern "C" fn check_hero(id: u8) -> i32 {
    unsafe {
        let local = rb(LOCAL_PLAYER);
    for i in 0..16usize {
        if heros[i] == id && herosb[i] {
            // return false if player already builds that unit
            return 0;
        }
    }
    // substitute for original war2 func that check if player have that unit
    for i in 0..16u32 {
        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
        while p != 0 {
            let o = rb(p + S_OWNER as u32);
            if o == rb(LOCAL_PLAYER) {
                let idd = rb(p + S_ID as u32);
                let f = idd == id;
                if f && !check_unit_dead(p) {
                    // return false if player already have that unit
                    return 0;
                }
                let f = idd == U_TOWN_HALL
                    || idd == U_GREAT_HALL
                    || idd == U_STRONGHOLD
                    || idd == U_KEEP
                    || idd == U_CASTLE
                    || idd == U_FORTRESS;
                if f && !check_unit_dead(p) && check_unit_complete(p) {
                    if rb(p + S_BUILD_ORDER as u32) == 0 {
                        if rb(p + S_BUILD_TYPE as u32) == id {
                            if rw(p + S_BUILD_PROGRES as u32) != 0 {
                                // return false if player already building that unit
                                return 0;
                            }
                        }
                    }
                }
            }
            p = rd(p + S_NEXT_UNIT_POINTER as u32);
        }
    }
    1
    }
}

pub unsafe fn build3(b: bool) {
    // third build button for peons
    if b {
        let psnt = [0x9u8];
        patch_set!(PEASANT_BUTTONS, psnt);
        let mut peon = [0x9u8, 0x0, 0x0, 0x0, 0x20, 0xf7, 0x48, 0x0];
        peon[4..8].copy_from_slice(&(PEON_BUILD3.as_ptr() as u32).to_le_bytes());
        patch_set!(PEON_BUTTONS, peon);

        let p1 = [
            0x8u8, 0x0, 0x6d, 0x0, 0xf0, 0x40, 0x44, 0x0, 0x70, 0xa6, 0x44, 0x0, 0x0, 0x6d,
            0x38, 0x1, 0x0, 0x0, 0x0, 0x0,
        ];
        patch_set!(PEASANT_RE_BUTTONS, p1);

        let b3p = BUILD_3.as_mut_ptr() as u32;
        if b3rune {
            patch::patch_setdword(b3p + 4, F_ALWAYS_TRUE);
        } else {
            patch::patch_setdword(b3p + 4, empty_false as extern "C" fn(u8) -> i32 as u32);
        }

        if b3port {
            patch::patch_setdword(b3p + 24, F_ALWAYS_TRUE);
        } else {
            patch::patch_setdword(b3p + 24, empty_false as extern "C" fn(u8) -> i32 as u32);
        }

        if b3cirl {
            patch::patch_setdword(b3p + 44, F_ALWAYS_TRUE);
        } else {
            patch::patch_setdword(b3p + 44, empty_false as extern "C" fn(u8) -> i32 as u32);
        }

        if b3mine {
            patch::patch_setdword(b3p + 64, F_ALWAYS_TRUE);
        } else {
            patch::patch_setdword(b3p + 64, empty_false as extern "C" fn(u8) -> i32 as u32);
        }

        if b3forest {
            patch::patch_setdword(b3p + 84, F_ALWAYS_TRUE);
        } else {
            patch::patch_setdword(b3p + 84, empty_false as extern "C" fn(u8) -> i32 as u32);
        }

        patch::patch_setdword(b3p + 88, build_forest as usize as u32);

        let mut b3 = [0x6u8, 0x0, 0x0, 0x0, 0x0, 0xf8, 0x48, 0x0];
        b3[4..8].copy_from_slice(&(b3p).to_le_bytes());
        patch_set!(DEAD_BLDG_BUTTONS, b3);
    } else {
        let psnt = [0x8u8];
        patch_set!(PEASANT_BUTTONS, psnt);
        let peon = [0x8u8, 0x0, 0x0, 0x0, 0x28, 0x22, 0x4a, 0x0];
        patch_set!(PEON_BUTTONS, peon);
        let p1 = [
            0x0u8, 0x0, 0x54, 0x0, 0xf0, 0x40, 0x44, 0x0, 0x90, 0x66, 0x43, 0x0, 0x0, 0x0,
            0x88, 0x1, 0x2, 0x0, 0x0, 0x0,
        ];
        patch_set!(PEASANT_RE_BUTTONS, p1);
        let b3 = [0x0u8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0];
        patch_set!(DEAD_BLDG_BUTTONS, b3);
    }
}

pub unsafe fn sheep(b: bool) {
    // sheeps from farms
    if b {
        let mut farm = [
            0x1u8, 0x0, 0x0, 0x0, 0x0, 0xf7, 0x48, 0x0, 0x1, 0x0, 0x0, 0x0, 0x0, 0xf7, 0x48,
            0x0,
        ];
        farm[4..8].copy_from_slice(&(SHEEP_BUILD.as_ptr() as u32).to_le_bytes());
        farm[12..16].copy_from_slice(&(SHEEP_BUILD.as_ptr() as u32).to_le_bytes());
        patch_set!(FARM_BUTTONS, farm);

        // sheep check function / sheep build function
        let sbp = SHEEP_BUILD.as_mut_ptr() as u32;
        patch::patch_setdword(sbp + 4, empty_true as extern "C" fn(u8) -> i32 as u32);
        patch::patch_setdword(sbp + 8, empty_build as usize as u32);
    } else {
        let farm = [0x0u8; 16];
        patch_set!(FARM_BUTTONS, farm);
    }
}

pub unsafe fn demon(b: bool) {
    if b {
        let mut d = [0x1u8, 0x0, 0x0, 0x0, 0x0, 0xf7, 0x48, 0x0];
        d[4..8].copy_from_slice(&(DEMON_BUILD.as_ptr() as u32).to_le_bytes());
        patch_set!(CIRCLE_BUTTONS, d);

        // need 2 tier TH to build demon
        patch::patch_setdword(DEMON_BUILD.as_mut_ptr() as u32 + 4, _2tir as extern "C" fn() -> i32 as u32);
    } else {
        let d = [0x0u8; 8];
        patch_set!(CIRCLE_BUTTONS, d);
    }
}

pub unsafe fn get_icon(id: u8) -> u8 {
    match id {
        U_ALLERIA => 187,   // alleria
        U_TERON => 189,     // teron
        U_KURDRAN => 191,   // kurdran
        U_DENTARG => 194,   // dentarg
        U_HADGAR => 193,    // hadgar
        U_GROM => 190,      // grom
        U_DEATHWING => 192, // deathwing
        U_TYRALYON => 195,  // tyralyon
        U_DANATH => 188,    // danat
        U_KARGATH => 186,   // kargat
        U_CHOGAL => 36,     // chogal
        U_LOTHAR => 32,     // lotar
        U_GULDAN => 33,     // guldan
        U_UTER => 34,       // uter
        U_ZULJIN => 35,     // zuljin
        _ => return 113,
    }
}

pub unsafe fn get_tbl(id: u8) -> u8 {
    match id {
        U_ALLERIA => 0x10, // alleria
        U_TERON => 0x17,   // teron
        U_KURDRAN => 0x27, // kurdran
        U_DENTARG => 0x14, // dentarg
        U_HADGAR => 0x19,  // hadgar
        U_GROM => 0x1a,    // grom
        U_DEATHWING => 0x28, // deathwing
        U_TYRALYON => 0x13, // tyralyon
        U_DANATH => 0x1b,  // danat
        U_KARGATH => 0x22, // kargat
        U_CHOGAL => 0x16,  // chogal
        U_LOTHAR => 0x21,  // lotar
        U_GULDAN => 0x1f,  // guldan
        U_UTER => 0x15,    // uter
        U_ZULJIN => 0xf,   // zuljin
        _ => return 0xdf,
    }
}
