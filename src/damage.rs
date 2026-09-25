// Перехват дамага (w2p.cpp:3689-3843)
#![allow(dead_code)]

use crate::combat::*;
use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::orig_fn;
use crate::patch::*;
use crate::state::*;

pub unsafe fn damage(atk: u32, trg: u32, dmg: i8) {
    FDMG = dmg;
    if trg != 0 && atk != 0 {
        if !check_unit_dead(trg) {
            let aid = rb(atk + S_ID as u32); // attacker id
            let tid = rb(trg + S_ID as u32); // target id
            // FIX: в C++ w2p.cpp:3703 dmg2 объявлен как byte (u8) — zero-extension char→byte.
            // Тип u8 важен: для dmg >= 128 sign-extension меняла бы арифметику blood/devotion.
            let mut dmg2 = dmg as u8; // can deal damage by default
            let mut i = 0usize;
            while i < 255 {
                if (tid == ut[i]) && (aid != ua[i]) {
                    dmg2 = 0; // canot deal damage
                }
                if (tid == ut[i]) && (aid == ua[i]) {
                    // check if only some certain units can attack that unit
                    dmg2 = dmg as u8; // can deal damage
                    break;
                }
                // FIX №7: сентинел проверяем до инкремента. В C++ w2p.cpp:3717
                // после i++ читался ua[i+1], включая ua[255] за границей массива.
                if ua[i] == 255 {
                    // pairs must go in a row
                    break;
                }
                i += 1;
            }
            if rw(trg + S_SHIELD as u32) != 0 {
                dmg2 = 0;
            }
            if blood_f {
                let mut bdmg = dmg2 as i32; // C++ int = byte (zero-extension)
                if rw(atk + S_BLOOD as u32) != 0 {
                    bdmg *= 2;
                }
                if bdmg > 255 {
                    bdmg = 255;
                }
                if bdmg != 0 {
                    dmg2 = (bdmg % 256) as u8;
                }
            }
            FDMG = dmg2 as i8; // C++ char fdmg = byte dmg2 (битовое усечение)
            if agr {
                comp_aggro(trg, atk); // check if allied comps go agro
            }
            if FDMG != 0 {
                if cpt || ucpt {
                    if capture(trg, atk) {
                        FDMG = 0; // check if buildings or units captured on low hp
                    }
                }
                if steal {
                    steal_res(trg, atk);
                }
                if manaburn {
                    mana_burn(trg, atk);
                }
                if peon_steal {
                    peon_steal_attack(trg, atk);
                }
                let mut f = false;
                for i in 0..255 {
                    if (m_devotion[i as usize] != 255) && (!f) {
                        f = devotion_aura(trg, m_devotion[i as usize]);
                    } else {
                        break; // C++: i = 256
                    }
                }
                if f {
                    // defence
                    dmg2 = FDMG as u8; // C++ byte dmg2 = char fdmg (битовое копирование)
                    if dmg2 > 3 {
                        dmg2 -= 3; // сравнение байтов беззнаковое, как в C++
                    } else {
                        dmg2 = 0;
                    }
                    FDMG = dmg2 as i8;
                }
                f = false;
                for i in 0..255 {
                    if (m_prvnt[i as usize] != 255) && (!f) {
                        f = m_prvnt[i as usize] == tid;
                    } else {
                        break; // C++: i = 256
                    }
                }
                if f {
                    // prevent
                    let hp = rw(trg + S_HP as u32);
                    if hp > 0 && hp as i32 <= FDMG as i32 {
                        FDMG = 0;
                        set_stat(trg, 300, S_SHIELD);
                        set_stat(trg, 0, S_HP);
                        flame(trg);
                    }
                }
                f = false;
                for i in 0..255 {
                    if (m_vampire[i as usize] != 255) && (!f) {
                        f = vamp_aura(trg, atk, m_vampire[i as usize]);
                    } else {
                        break; // C++: i = 256
                    }
                }
                if f {
                    // vampire
                    let hp = rw(trg + S_HP as u32);
                    let mult = 2; // 2=50%
                    if hp as i32 > (FDMG as i32 / mult + 1) {
                        heal(atk, (FDMG as i32 / mult + 1) as u8, 0);
                    } else {
                        heal(atk, (hp % 256) as u8, 0);
                    }
                    let xx = rw(atk + S_DRAW_X as u32);
                    let yy = rw(atk + S_DRAW_Y as u32);
                    f_bullet_create()(xx + 16, yy + 16, B_SHOT_FIRE); // create effect
                }
            }
            let hp = rw(trg + S_HP as u32); // unit hp
            // FIX: в C++ w2p.cpp:3797 стояло || — условие было всегда истинно;
            // исправлено на && (убийства не засчитывались транспортам)
            if (tid < U_FARM) && (FDMG as i32 >= hp as i32) {
                if aid != U_HTRANSPORT && aid != U_OTRANSPORT {
                    let mut k = rb(atk + S_KILLS as u32);
                    if k < 255 {
                        k += 1;
                    }
                    set_stat(atk, k as i32, S_KILLS);
                }
            }
        }
    }
}

// Хуки дамага (cdecl, вызываются игрой через call)
#[no_mangle]
pub extern "C" fn damage1(atk: u32, trg: u32, dmg: i8) {
    unsafe {
        damage(atk, trg, dmg);
        orig_fn!(g_proc_00409F3B, extern "C" fn(u32, u32, i8))(atk, trg, FDMG);
    }
}

#[no_mangle]
pub extern "C" fn damage2(atk: u32, trg: u32, dmg: i8) {
    unsafe {
        damage(atk, trg, dmg);
        orig_fn!(g_proc_0041038E, extern "C" fn(u32, u32, i8))(atk, trg, FDMG);
    }
}

#[no_mangle]
pub extern "C" fn damage3(atk: u32, trg: u32, dmg: i8) {
    unsafe {
        damage(atk, trg, dmg);
        orig_fn!(g_proc_0040AF70, extern "C" fn(u32, u32, i8))(atk, trg, FDMG);
    }
}

#[no_mangle]
pub extern "C" fn damage4(atk: u32, trg: u32, dmg: i8) {
    unsafe {
        damage(atk, trg, dmg);
        orig_fn!(g_proc_0040AF99, extern "C" fn(u32, u32, i8))(atk, trg, FDMG);
    }
}

#[no_mangle]
pub extern "C" fn damage5(atk: u32, trg: u32, dmg: i8) {
    unsafe {
        damage(atk, trg, dmg);
        orig_fn!(g_proc_00410762, extern "C" fn(u32, u32, i8))(atk, trg, FDMG);
    }
}

#[no_mangle]
pub extern "C" fn damage6(atk: u32, trg: u32, dmg: i8) {
    unsafe {
        damage(atk, trg, dmg);
        orig_fn!(g_proc_004428AD, extern "C" fn(u32, u32, i8))(atk, trg, FDMG);
    }
}
