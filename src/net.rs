// Сеть: чит-пакеты, кнопки autoheal/рабочие, кнопки ратуши (w2p.cpp:4186-4464)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::buildui::*;
use crate::heroes::*;
use crate::names::*;
use crate::orders::*;
use crate::orig_fn;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::sort::*;
use crate::state::*;

pub unsafe fn send_cheat(c: u8) {
    let mut b = rd(CHEATBITS) as i32;
    if (b & (1 << c)) != 0 {
        b &= !(1 << c);
    } else {
        b |= 1 << c;
    }
    f_send_cheat_packet()(b);
    // локально в CHEATBITS не пишем — бит применяется при приёме пакета (receive_cheat)
}

pub unsafe fn rec_autoheal() {
    let p = rb(LOCAL_PLAYER); // player
    let mut b = rb(SPELLS_LEARNED + 4 * p as u32);
    let sp = GREATER_HEAL;
    if (b & (1 << sp)) != 0 {
        b &= !(1 << sp);
    } else {
        b |= 1 << sp;
    }
    let buf = [b];
    patch_set!(SPELLS_LEARNED + 4 * p as u32, buf);

    if (rb(SPELLS_LEARNED + 4 * p as u32) & (1 << L_GREATER_HEAL)) != 0 {
        CHURC[20 * 3 + 2] = 0x5b; // icon
        let msg = b"autoheal\x05 enabled\0";
        show_message(10, msg.as_ptr());
    } else {
        CHURC[20 * 3 + 2] = 0x6d; // icon
        let msg = b"autoheal\x03 disabled\0";
        show_message(10, msg.as_ptr());
    }
    f_status_redraw()();
}

pub unsafe fn rec_peons() {
    let l = rb(LOCAL_PLAYER); // player
    // C++: p = (int*)(UNITS_SELECTED + ...); if (p) { p = *p; if (p) {... } }
    // rd() ниже уже выполняет единственное разыменование (*p = указатель на юнита)
    let p: u32 = rd(UNITS_SELECTED + 9 * 4 * l as u32);
    if p != 0 {
        let id = rb(p + S_ID as u32);
        let fid = id == U_TOWN_HALL
            || id == U_GREAT_HALL
            || id == U_KEEP
            || id == U_STRONGHOLD
            || id == U_CASTLE
            || id == U_FORTRESS;
        if fid {
            let o = rb(p + S_OWNER as u32);
            if o == l {
                let x = rb(p + S_X as u32);
                let y = rb(p + S_Y as u32);
                set_region(x as i32 - 5, y as i32 - 5, x as i32 + 8, y as i32 + 8); // set region around myself rad 5
                find_all_alive_units(ANY_MEN);
                sort_in_region();
                sort_hidden();
                sort_stat(S_OWNER, o as i32, CMP_EQ); // my owner
                sort_stat(S_ID, U_ATTACK_PEASANT as i32, CMP_BIGGER_EQ);
                sort_stat(S_ID, U_ATTACK_PEON as i32, CMP_SMALLER_EQ);
                if units != 0 {
                    for i in 0..units {
                        let id = rb(unit[i as usize] + S_ID as u32); // unit id
                        let mhp = rw(UNIT_HP_TABLE + 2 * id as u32); // max hp
                        let hp = rw(unit[i as usize] + S_HP as u32); // unit hp
                        let idd = id % 2;
                        let mhp2 = rw(UNIT_HP_TABLE + 2 * (U_PEASANT + idd) as u32); // max hp
                        let mut thp = hp as i32 + (mhp2 as i32 - mhp as i32);
                        if thp < 1 {
                            thp = 1;
                        }
                        let hp = (thp % (256 * 256)) as u16;
                        set_stat(unit[i as usize], hp as i32, S_HP);
                        set_stat(unit[i as usize], (U_PEASANT + idd) as i32, S_ID);
                        set_stat(unit[i as usize], (U_PEASANT + idd) as i32, S_COMMANDS);
                        set_stat(unit[i as usize], (U_PEASANT + idd) as i32, S_SPRITE);
                        set_stat(unit[i as usize], ORDER_STOP as i32, S_NEXT_ORDER);
                    }
                    let msg = b"workers attack mode\x03 disabled\0";
                    show_message(10, msg.as_ptr());
                } else {
                    find_all_alive_units(ANY_MEN);
                    sort_in_region();
                    sort_hidden();
                    sort_stat(S_OWNER, o as i32, CMP_EQ); // my owner
                    sort_stat(S_ID, U_PEASANT as i32, CMP_BIGGER_EQ);
                    sort_stat(S_ID, U_PEON as i32, CMP_SMALLER_EQ);
                    if units != 0 {
                        for i in 0..units {
                            let id = rb(unit[i as usize] + S_ID as u32); // unit id
                            let mhp = rw(UNIT_HP_TABLE + 2 * id as u32); // max hp
                            let hp = rw(unit[i as usize] + S_HP as u32); // unit hp
                            let idd = id % 2;
                            let mhp2 = rw(UNIT_HP_TABLE + 2 * (U_ATTACK_PEASANT + idd) as u32); // max hp
                            let mut thp = hp as i32 + (mhp2 as i32 - mhp as i32);
                            if thp < 1 {
                                thp = 1;
                            }
                            let hp = (thp % (256 * 256)) as u16;
                            set_stat(unit[i as usize], hp as i32, S_HP);
                            set_stat(unit[i as usize], (U_ATTACK_PEASANT + idd) as i32, S_ID);
                            set_stat(unit[i as usize], (U_ATTACK_PEASANT + idd) as i32, S_COMMANDS);
                            set_stat(unit[i as usize], (U_PEASANT + idd) as i32, S_SPRITE);
                            set_stat(unit[i as usize], ORDER_STOP as i32, S_NEXT_ORDER);
                        }
                        let msg = b"workers attack mode\x05 enabled\0";
                        show_message(10, msg.as_ptr());
                    }
                }
            }
        }
    }
    f_status_redraw()();
}

#[no_mangle]
pub extern "C" fn receive_cheat(c: i32, a1: i32) {
    unsafe {
        // received cheat packet
        let mut f = true;
        if (c & (1 << 8)) != 0 {
            // 8 - autoheal
            rec_autoheal();
            f = false;
        }
        if (c & (1 << 9)) != 0 {
            // 9 - attack peons
            rec_peons();
            f = false;
        }
        if f {
            orig_fn!(g_proc_0045614E, extern "C" fn(i32, i32))(c, a1); // orig
        } else {
            let buf = [0x0u8];
            patch_set!(PLAYER_CHEATED, buf);
        }
    }
}

// void (*r1) (int) = button_autoheal — игровой колбэк кнопки (cdecl)
#[no_mangle]
pub extern "C" fn button_autoheal(_: i32) {
    unsafe {
        send_cheat(8);
        if (rb(SPELLS_LEARNED + 4 * rb(LOCAL_PLAYER) as u32) & (1 << L_GREATER_HEAL)) != 0 {
            CHURC[20 * 3 + 2] = 0x5b; // icon
        } else {
            CHURC[20 * 3 + 2] = 0x6d; // icon
        }
        f_status_redraw()();
    }
}

pub unsafe fn autoheal(b: bool) {
    if b {
        if (rb(SPELLS_LEARNED + 4 * rb(LOCAL_PLAYER) as u32) & (1 << L_GREATER_HEAL)) != 0 {
            CHURC[20 * 3 + 2] = 0x5b; // icon
        } else {
            CHURC[20 * 3 + 2] = 0x6d; // icon
        }

        patch::patch_setdword(
            CHURC.as_mut_ptr() as u32 + (20 * 3 + 8) as u32,
            button_autoheal as usize as u32,
        );

        let mut b1 = [0x04u8, 0x0, 0x0, 0x0, 0x68, 0x37, 0x4a, 0x0];
        b1[4..8].copy_from_slice(&(CHURC.as_ptr() as u32).to_le_bytes());
        patch_set!(CHURCH_BUTTONS, b1);
        A_autoheal = true;
    } else {
        let b1 = [0x03u8, 0x0, 0x0, 0x0, 0x68, 0x37, 0x4a, 0x0];
        patch_set!(CHURCH_BUTTONS, b1);
        A_autoheal = false;
    }
}

// void (*r1) (int) = button_peons — игровой колбэк кнопки (cdecl)
#[no_mangle]
pub extern "C" fn button_peons(_: i32) {
    unsafe {
        send_cheat(9);
        f_status_redraw()();
    }
}

pub unsafe fn th_change(b: bool) {
    // add new buttons to TH
    if b {
        heroes(true);
        let hth = [0x5u8];
        patch_set!(HUMAN_TH1_BUTTONS, hth);
        patch_set!(HUMAN_TH2_BUTTONS, hth);
        patch_set!(HUMAN_TH3_BUTTONS, hth);
        let mut oth = [0x5u8, 0x0, 0x0, 0x0, 0x20, 0xf7, 0x48, 0x0];
        oth[4..8].copy_from_slice(&(OTH_BUILD.as_ptr() as u32).to_le_bytes());
        patch_set!(ORC_TH1_BUTTONS, oth);
        patch_set!(ORC_TH2_BUTTONS, oth);
        patch_set!(ORC_TH3_BUTTONS, oth);

        let mut af = false;
        for i in 0..8usize {
            if heros[i] != 0 {
                af = true;
            }
        }
        let hthp = HTH_BUILD.as_mut_ptr() as u32;
        if af {
            patch::patch_setdword(hthp + 4, F_ALWAYS_TRUE);
        } else {
            patch::patch_setdword(hthp + 4, empty_false as usize as u32);
        }
        af = false;
        for i in 8..16usize {
            if heros[i] != 0 {
                af = true;
            }
        }
        let othp = OTH_BUILD.as_mut_ptr() as u32;
        if af {
            patch::patch_setdword(othp + 64, F_ALWAYS_TRUE);
        } else {
            patch::patch_setdword(othp + 64, empty_false as usize as u32);
        }

        if apn {
            // if can build attack peons
            patch::patch_setdword(hthp + 24, F_ALWAYS_TRUE);
            patch::patch_setdword(othp + 84, F_ALWAYS_TRUE);

            let r = button_peons as usize as u32;
            patch::patch_setdword(hthp + 28, r);
            patch::patch_setdword(othp + 88, r);

            let bufb = [
                0x0au8, 0x0, 0x0, 0x0, 0x38, 0x1e, 0x4a, 0x0, 0xa, 0x0, 0x0, 0x0, 0x0, 0x1f,
                0x4a, 0x0,
            ];
            patch_set!(HUMAN_TH_ONE_BUTTON, bufb);
        } else {
            patch::patch_setdword(hthp + 24, empty_false as usize as u32);
            patch::patch_setdword(othp + 84, empty_false as usize as u32);

            let bufb = [
                0x2u8, 0x0, 0x0, 0x0, 0xc8, 0x1f, 0x4a, 0x0, 0x2, 0x0, 0x0, 0x0, 0xf0, 0x1f,
                0x4a, 0x0,
            ];
            patch_set!(HUMAN_TH_ONE_BUTTON, bufb);
        }

        // C++: PATCH_SET(HUMAN_TH_COMMON, hth_build) пишет sizeof(hth_build)-1 = 40
        // байт (без завершающего NUL таблицы)
        patch_set!(HUMAN_TH_COMMON, HTH_BUILD[..HTH_BUILD.len() - 1]);
    } else {
        heroes(false);
        let hth = [0x3u8];
        patch_set!(HUMAN_TH1_BUTTONS, hth);
        patch_set!(HUMAN_TH2_BUTTONS, hth);
        patch_set!(HUMAN_TH3_BUTTONS, hth);
        let oth = [0x3u8, 0x0, 0x0, 0x0, 0x78, 0x34, 0x4a, 0x0];
        patch_set!(ORC_TH1_BUTTONS, oth);
        patch_set!(ORC_TH2_BUTTONS, oth);
        patch_set!(ORC_TH3_BUTTONS, oth);
        let p1 = [
            0x0u8, 0x0, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0xa0, 0x44, 0x44, 0x0, 0xd0, 0xe6,
            0x40, 0x0, 0x1, 0x3, 0xd, 0x1, 0x0, 0x0, 0x0, 0x0, 0x1, 0x0, 0x43, 0x0, 0xb0,
            0x42, 0x44, 0x0, 0x10, 0xe7, 0x40, 0x0, 0x0, 0x59, 0x3e, 0x1, 0x0, 0x0, 0x0, 0x0,
        ];
        patch_set!(HUMAN_TH_COMMON, p1);
        let bufb = [
            0x2u8, 0x0, 0x0, 0x0, 0xc8, 0x1f, 0x4a, 0x0, 0x2, 0x0, 0x0, 0x0, 0xf0, 0x1f,
            0x4a, 0x0,
        ];
        patch_set!(HUMAN_TH_ONE_BUTTON, bufb);
    }
}
