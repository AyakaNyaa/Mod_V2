// Параметры игры и разные фиксы (w2p.cpp:2453-2589, 4100-4185, 4809-4842)
#![allow(dead_code)]

use crate::defs::*;
use crate::game::*;
use crate::names::*;
use crate::orig_fn;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::state::*;

pub unsafe fn fireball_dmg(dmg: u8) {
    let fb = [dmg]; // 40 default
    patch_set!(FIREBALL_DMG, fb);
}

pub unsafe fn buff_time(bf: u8, t1: u8, t2: u8) {
    let tm = [t1, t2];
    match bf {
        0 => {
            patch_set!(INVIZ_TIME, tm); // 2000 default
        }
        1 => {
            patch_set!(BLOOD_TIME, tm); // 1000 default
        }
        2 => {
            patch_set!(SHIELD_TIME, tm); // 500 default
        }
        3 => {
            patch_set!(HASTE_TIME1, tm); // 1000 default
            patch_set!(HASTE_TIME2, tm);
        }
        4 => {
            let tt = 0xffff - (t1 as i32 + 256 * t2 as i32);
            let tm = [(tt % 256) as u8, (tt / 256) as u8];
            patch_set!(SLOW_TIME1, tm); // 1000 default
            patch_set!(SLOW_TIME2, tm);
        }
        _ => {}
    }
}

pub unsafe fn fireshield_flyers(b: bool) {
    // allow cast fireshield on flyers
    if b {
        let bau = [0x90u8, 0x90]; // 75 51
        patch_set!(FIRESHIELD_FLYERS, bau);
    } else {
        let bau = [0x75u8, 0x51];
        patch_set!(FIRESHIELD_FLYERS, bau);
    }
}

pub unsafe fn trigger_time(tm: u8) {
    // war2 will call victory check function every 200 game ticks
    let ttime = [tm]; // 200 default
    patch_set!(TRIG_TIME, ttime);
}

pub unsafe fn manacost(id: u8, c: u8) {
    // spells cost of mana
    let mana = [c];
    patch_set!(MANACOST + 2 * id as u32, mana);
}

pub unsafe fn upgr(id: u8, c: u8) {
    // upgrades power
    let up = [c];
    patch_set!(UPGRD + id as u32, up);
}

pub unsafe fn no_random_dmg(b: bool) {
    // always max damage instead of random (SC1-style)
    if b {
        let b1 = [0x1au8];
        patch_set!(DAMAGE_AREA1, b1);
        let b2 = [0x19u8];
        patch_set!(DAMAGE_AREA2, b2);
        let b3 = [0x90u8, 0x90];
        patch_set!(DAMAGE_AREA3, b3);
        patch_set!(DAMAGE_AREA4, b3);
        let b4 = [
            0x90u8, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90,
            0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, // 24 NOP, как в C++ w2p.cpp:2547
        ];
        patch_set!(DAMAGE_UNIT_VS_UNIT, b4);
    } else {
        let b1 = [0x6u8];
        patch_set!(DAMAGE_AREA1, b1);
        let b2 = [0x5u8];
        patch_set!(DAMAGE_AREA2, b2);
        let b3 = [0xf7u8, 0xf1];
        patch_set!(DAMAGE_AREA3, b3);
        patch_set!(DAMAGE_AREA4, b3);
        let b4 = [
            0x8du8, 0x47, 0x1, 0x99, 0x2b, 0xc2, 0x8b, 0xf8, 0xd1, 0xff, 0xe8, 0x8d, 0x91, 0x6,
            0x0, 0x8d, 0x4f, 0x1, 0x33, 0xd2, 0xf7, 0xf1, 0x3, 0xfa,
        ];
        patch_set!(DAMAGE_UNIT_VS_UNIT, b4);
    }
}

pub unsafe fn blood_fix(b: bool) {
    // remove bloodlust from game (and will make X2 in damage() function manually)
    if b {
        blood_f = true; // to make X2 in damage() function manually
        let b1 = [0x90u8, 0x90];
        patch_set!(BLOOD_CALC1, b1);
        let b2 = [0x90u8, 0x90];
        patch_set!(BLOOD_CALC2, b2);
    } else {
        blood_f = false;
        let b1 = [0x3u8, 0xf6];
        patch_set!(BLOOD_CALC1, b1);
        let b2 = [0x3u8, 0xc0];
        patch_set!(BLOOD_CALC2, b2);
    }
}

pub unsafe fn center_view(x: u8, y: u8) {
    f_minimap_click()(x, y); // original war2 func that called when player click on minimap
}

pub unsafe fn multicast_fix(f: bool) {
    if f {
        let rep = [0xebu8];
        patch_set!(MULTICAST_FIX, rep);
    } else {
        let rep = [0x74u8];
        patch_set!(MULTICAST_FIX, rep);
    }
}

#[no_mangle]
pub extern "C" fn pathfind_mov(um: u32, s: i32) -> i32 {
    unsafe {
        let mut s = s;
        if path_fixed {
            // C++: S_X - S_MOV_PATH01 + 1 = -24 (int-арифметика, um указывает на path[1])
            let x = rb(um.wrapping_add((S_X as i32 - S_MOV_PATH01 as i32 + 1) as u32));
            let y = rb(um.wrapping_add((S_Y as i32 - S_MOV_PATH01 as i32 + 1) as u32));
            let ox = rb(um.wrapping_add((S_ORDER_X as i32 - S_MOV_PATH01 as i32 + 1) as u32));
            let oy = rb(um.wrapping_add((S_ORDER_Y as i32 - S_MOV_PATH01 as i32 + 1) as u32));
            let xx = (x as i32 - ox as i32).abs() as u8;
            let yy = (y as i32 - oy as i32).abs() as u8;
            let ss = if xx < yy { xx } else { yy };
            if ss <= 8 {
                s = 1;
            }
        }
        orig_fn!(g_proc_0044FF20, extern "C" fn(u32, i32) -> i32)(um, s) // original
    }
}

pub unsafe fn pathfind_fix(f: bool) {
    if f {
        let rep = [0x1u8]; // cheap path
        patch_set!(0x0044FF2C, rep);
        let rep2 = [0xebu8, 0x41, 0x90];
        patch_set!(0x00450306, rep2);
        let rep3 = [0x66u8, 0x81, 0xf9, 0x0, 0x4, 0xeb, 0xb8]; // 1024 buffer
        patch_set!(0x00450349, rep3);
        path_fixed = true;
    } else {
        let rep = [0x7u8];
        patch_set!(0x0044FF2C, rep);
        let rep2 = [0x83u8, 0xf9, 0x32];
        patch_set!(0x00450306, rep2);
        let rep3 = [0x90u8, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90];
        patch_set!(0x00450349, rep3);
        path_fixed = false;
    }
}

pub unsafe fn allow_table(p: u8, t: i32, n: u8, a: u8) {
    let t = if t == 0 {
        ALLOWED_UNITS + (4 * p as u32) + (n as u32 / 8)
    } else if t == 1 {
        ALLOWED_UPGRADES + (4 * p as u32) + (n as u32 / 8)
    } else if t == 2 {
        ALLOWED_SPELLS + (4 * p as u32) + (n as u32 / 8)
    } else {
        SPELLS_LEARNED + (4 * p as u32) + (n as u32 / 8)
    };
    let mut b = rb(t);
    if a == 1 {
        b |= 1 << (n % 8);
    } else {
        b &= !(1 << (n % 8));
    }
    let buf = [b];
    patch_set!(t, buf);
}

pub unsafe fn draw_stats_fix(b: bool) {
    if b {
        let buf = [0xa0u8, 0x5b];
        patch_set!(DEMON_STATS_DRAW, buf); // demon
        patch_set!(CRITTER_STATS_DRAW, buf); // critter
    } else {
        let buf = [0xf0u8, 0x57];
        patch_set!(DEMON_STATS_DRAW, buf); // demon
        patch_set!(CRITTER_STATS_DRAW, buf); // critter
    }
}

pub unsafe fn sounds_ready_table_set(id: u8, snd: u16) {
    let buf = [(snd % 256) as u8, (snd / 256) as u8];
    patch_set!(UNIT_SOUNDS_READY_TABLE + 2 * id as u32, buf);
}

pub unsafe fn sounds_tables() {
    sounds_ready_table_set(U_CRITTER, 247);
    sounds_ready_table_set(U_DEMON, 301);

    sounds_ready_table_set(U_ALLERIA, 254);
    sounds_ready_table_set(U_DANATH, 263);
    sounds_ready_table_set(U_HADGAR, 272);
    sounds_ready_table_set(U_KURDRAN, 281);
    sounds_ready_table_set(U_TYRALYON, 290);
    sounds_ready_table_set(U_UTER, 154);
    sounds_ready_table_set(U_LOTHAR, 155);

    sounds_ready_table_set(U_DEATHWING, 300);
    sounds_ready_table_set(U_DENTARG, 310);
    sounds_ready_table_set(U_GROM, 318);
    sounds_ready_table_set(U_KARGATH, 327);
    sounds_ready_table_set(U_TERON, 336);
    sounds_ready_table_set(U_ZULJIN, 204);
    sounds_ready_table_set(U_CHOGAL, 179);
    sounds_ready_table_set(U_GULDAN, 88);
}
