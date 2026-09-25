// Игровой тик: таймеры, обновление спеллов, сиквенсы (w2p.cpp:3574-3688)
#![allow(dead_code)]

use crate::ai::*;
use crate::aitick::*;
use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::mechanics::*;
use crate::names::*;
use crate::orders::*;
use crate::orig_fn;
use crate::patch::*;
use crate::player::*;
use crate::saveload::*;
use crate::sort::*;
use crate::state::*;

pub unsafe fn unit_timer() {
    // timer for transport and runestone heal
    find_all_alive_units(ANY_MEN);
    sort_hidden();
    sort_stat(S_ID, U_HTRANSPORT as i32, CMP_NEQ);
    sort_stat(S_ID, U_OTRANSPORT as i32, CMP_NEQ);
    for i in 0..units {
        let mut r = rb(unit[i as usize] + (S_KILLS + 1) as u32);
        if r > 0 {
            r -= 1;
        }
        set_stat(unit[i as usize], r as i32, S_KILLS + 1);
    }
}

#[no_mangle]
pub extern "C" fn update_spells() {
    unsafe {
        orig_fn!(g_proc_0045271B, extern "C" fn())(); // original
        // this function called every game tick
        // you can place your self-writed functions here in the end if need
        unit_timer();
        if A_portal {
            portal();
        }
        if A_transport {
            transport();
        }
        if A_autoheal {
            paladin();
        }
        if A_runestone {
            runestone();
        }
        for i in 0..255usize {
            if m_slow_aura[i] != 255 {
                slow_aura(m_slow_aura[i]);
            } else {
                break; // C++: i = 256
            }
        }
        for i in 0..255usize {
            if m_death_aura[i] != 255 {
                death_aura(m_death_aura[i]);
            } else {
                break; // C++: i = 256
            }
        }
        for i in 0..255usize {
            if m_sneak[i] != 255 {
                sneak(m_sneak[i]);
            } else {
                break; // C++: i = 256
            }
        }
        if saveload_fixed {
            tech_reinit();
        }
        if ai_fixed {
            sap_behaviour();
            unstuk();
            goldmine_ai();
        }
        if vizs_n > 0 {
            for i in 0..vizs_n {
                viz_area(
                    vizs_areas[i as usize].x,
                    vizs_areas[i as usize].y,
                    vizs_areas[i as usize].p,
                    vizs_areas[i as usize].s,
                );
            }
        }
        // FIX: в C++ w2p.cpp:3628 vizs_n не сбрасывался после применения —
        // зоны видимости применялись каждый тик бесконечно
        vizs_n = 0;
        // PLACE your new functions here
    }
}

pub unsafe fn seq_change(u: u32, tt: u8) {
    // change animation speeds
    // USE this function CAREFULLY
    let _t = tt;
    /*
    if (FALSE)//remove this and USE this function CAREFULLY
    {
        if (t == 1)
        {
            byte t = *((byte*)((uintptr_t)u + S_ANIMATION_TIMER));
            byte id = *((byte*)((uintptr_t)u + S_ID));
            if (id == U_ARCHER)
            {
                byte a = *((byte*)((uintptr_t)u + S_ANIMATION));
                byte f = *((byte*)((uintptr_t)u + S_FRAME));
                if (a == ANIM_MOVE)
                    if (t > 1)t -= 1;//all archer movement timers will be 1 frame faster
                //but cannot be less than 1 !!!!!!!!!!
                if (a == ANIM_ATTACK)
                {
                    if (f != 0)
                    {
                        if (t > 5)t -= 5;
                        else t = 1;//ultra fast shooting for archer
                    }
                    else
                    {
                        if (t > 25)t -= 25;
                        else t = 1;//ultra fast shooting for archer
                    }
                    set_stat(u, 1, S_ATTACK_COUNTER);//ultra fast shooting for archer
                }
                set_stat(u, t, S_ANIMATION_TIMER);
            }
        }
    }
    */
}

#[no_mangle]
pub extern "C" fn seq_run(u: u32) -> i32 {
    unsafe {
        let t = rb(u + S_ANIMATION_TIMER as u32);
        let original = orig_fn!(g_proc_004522B9, extern "C" fn(u32) -> i32)(u); // original
        seq_change(u, t);
        original
    }
}
