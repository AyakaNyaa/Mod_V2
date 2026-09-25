// Герои: постройка из ратуш, ремонт (w2p.cpp:2289-2452)
#![allow(dead_code)]

use crate::buildui::*;
use crate::defs::*;
use crate::game::*;
use crate::names::*;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::player::*;
use crate::state::*;

// DWORD CALLBACK hero_thread(LPVOID lpParam) — поток WinAPI (stdcall)
#[no_mangle]
pub extern "system" fn hero_thread(_lp: *mut u8) -> u32 {
    unsafe {
        Sleep(2000);
        for i in 0..16usize {
            herosb[i] = false;
        }
        ExitThread(0);
        0 // недостижимо: ExitThread не возвращается (в C++ возврат отсутствовал)
    }
}

// адрес отдаётся игре — cdecl, как в C++
#[no_mangle]
pub extern "C" fn build_hero(id: i32) {
    unsafe {
        for i in 0..16usize {
            if !herosb[i] && heros[i] == (id % 256) as u8 {
                herosb[i] = true;
                // FIX: в C++ w2p.cpp:2307 CreateThread вызывался без CloseHandle (утечка хендла)
                CloseHandle(CreateThread(
                    core::ptr::null_mut(),
                    0,
                    hero_thread,
                    core::ptr::null_mut(), // lpParam в C++ указывал на локальную DWORD, тред его не использует
                    0,
                    core::ptr::null_mut(),
                ));
            }
        }
        f_train_unit()(id); // original build func
        f_change_status()(0x6e); // change status
        f_status_redraw()(); // status redraw
    }
}

pub unsafe fn heroes(b: bool) {
    if b {
        for i in 1..20usize {
            hhero[i] = BUILD_3[100 + i];
            ohero[i] = BUILD_3[100 + i];
        }
        let local = rb(LOCAL_PLAYER);
        let l = local as i32;
        let mut mx: u8 = 2;
        if get_val(TH2, l) != 0 {
            mx = 5;
        }
        if get_val(TH3, l) != 0 {
            mx = 8;
        }
        let mut k: u8 = 1;
        for i in 1..9u8 {
            let mut id = heros[(i - 1) as usize];
            if id == 0 {
                id = U_CRITTER;
            }
            if id != 0 {
                k += 1;
                let ico = get_icon(id);
                hhero[(20 * i + 0) as usize] = i; // button id
                hhero[(20 * i + 1) as usize] = 0x0; // button id?
                hhero[(20 * i + 2) as usize] = ico; // icon
                hhero[(20 * i + 3) as usize] = 0x0; // icon

                // hero check function
                let r: u32 = if i > mx || id == U_CRITTER {
                    empty_false as extern "C" fn(u8) -> i32 as u32
                } else {
                    check_hero as usize as u32
                };
                patch::patch_setdword(hhero.as_mut_ptr() as u32 + (20 * i + 4) as u32, r);

                // hero build function
                patch::patch_setdword(
                    hhero.as_mut_ptr() as u32 + (20 * i + 8) as u32,
                    build_hero as usize as u32,
                );

                hhero[(20 * i + 12) as usize] = id; // arg
                hhero[(20 * i + 13) as usize] = id; // unit id
                hhero[(20 * i + 14) as usize] = get_tbl(id); // string from tbl
                hhero[(20 * i + 15) as usize] = 0x1; // string from tbl
                hhero[(20 * i + 16) as usize] = 0x0; // flags?
                hhero[(20 * i + 17) as usize] = 0x0; // flags?
                hhero[(20 * i + 18) as usize] = 0x0; // flags?
                hhero[(20 * i + 19) as usize] = 0x0; // flags?
            } else {
                break; // C++: i = 10
            }
        }
        let mut b1 = [0x0u8, 0x0, 0x0, 0x0, 0x0, 0xf8, 0x48, 0x0];
        b1[0] = k;
        b1[4..8].copy_from_slice(&(hhero.as_ptr() as u32).to_le_bytes());
        patch_set!(DEAD_BLDG2_BUTTONS, b1);
        k = 1;
        for i in 1..9u8 {
            let mut id = heros[(i + 8 - 1) as usize];
            if id == 0 {
                id = U_CRITTER;
            }
            if id != 0 {
                k += 1;
                let ico = get_icon(id);
                ohero[(20 * i + 0) as usize] = i; // button id
                ohero[(20 * i + 1) as usize] = 0x0; // button id?
                ohero[(20 * i + 2) as usize] = ico; // icon
                ohero[(20 * i + 3) as usize] = 0x0; // icon

                // hero check function
                let r: u32 = if i > mx || id == U_CRITTER {
                    empty_false as extern "C" fn(u8) -> i32 as u32
                } else {
                    check_hero as usize as u32
                };
                patch::patch_setdword(ohero.as_mut_ptr() as u32 + (20 * i + 4) as u32, r);

                // hero build function
                patch::patch_setdword(
                    ohero.as_mut_ptr() as u32 + (20 * i + 8) as u32,
                    build_hero as usize as u32,
                );

                ohero[(20 * i + 12) as usize] = id; // arg
                ohero[(20 * i + 13) as usize] = id; // unit id
                ohero[(20 * i + 14) as usize] = get_tbl(id); // string from tbl
                ohero[(20 * i + 15) as usize] = 0x1; // string from tbl
                ohero[(20 * i + 16) as usize] = 0x0; // flags?
                ohero[(20 * i + 17) as usize] = 0x0; // flags?
                ohero[(20 * i + 18) as usize] = 0x0; // flags?
                ohero[(20 * i + 19) as usize] = 0x0; // flags?
            } else {
                break; // C++: i = 10
            }
        }
        let mut b2 = [0x0u8, 0x0, 0x0, 0x0, 0x0, 0xf8, 0x48, 0x0];
        b2[0] = k;
        b2[4..8].copy_from_slice(&(ohero.as_ptr() as u32).to_le_bytes());
        patch_set!(DEAD_BLDG3_BUTTONS, b2);
    } else {
        let buf = [0x0u8; 7];
        patch_set!(DEAD_BLDG2_BUTTONS, buf);
        patch_set!(DEAD_BLDG3_BUTTONS, buf);
    }
}

pub unsafe fn repair_all(b: bool) {
    // peon can repair all units
    if b {
        let bau = [0xebu8]; // 0x75
        patch_set!(REPAIR_FLAG_CHECK, bau);
    } else {
        let bau = [0x75u8];
        patch_set!(REPAIR_FLAG_CHECK, bau);
    }
}

pub unsafe fn repair_cat(b: bool) {
    // peon can repair unit if it have transport flag OR catapult flag
    if b {
        let r1 = [0xebu8, 0x75, 0x90, 0x90, 0x90]; // f6 c4 04 74 14
        patch_set!(REPAIR_FLAG_CHECK2, r1);
        let r2 = [0x66u8, 0xa9, 0x04, 0x04, 0x74, 0x9c, 0xeb, 0x86];
        patch_set!(REPAIR_CODE_CAVE, r2);
    } else {
        let r1 = [0xf6u8, 0xc4, 0x4, 0x74, 0x14];
        patch_set!(REPAIR_FLAG_CHECK2, r1);
    }
}

pub unsafe fn call_default_kill() {
    // default kill all victory
    let l = rb(LOCAL_PLAYER);
    if !slot_alive(l) {
        lose(true);
    } else if !check_opponents(l) {
        win(true);
    }
}
