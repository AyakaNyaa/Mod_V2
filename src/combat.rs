// Комбат-механики: агро, каптюр, воровство, ауры (w2p.cpp:2590-2836)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::patch::*;
use crate::player::*;
use crate::sort::*;
use crate::state::*;

pub unsafe fn comp_aggro(trg: u32, atk: u32) {
    let own1 = rb(trg + S_OWNER as u32);
    let own2 = rb(atk + S_OWNER as u32);
    if own1 != own2 && check_ally(own1, own2) {
        // if we allies
        let o1 = rb(CONTROLER_TYPE + own1 as u32);
        let o2 = rb(CONTROLER_TYPE + own2 as u32);
        if o1 == C_COMP && o2 == C_PLAYER {
            // if target unit is comp and attacker is real player
            ally(own1, own2, 0);
            viz(own1 as i32, own2 as i32, 0);
            // turn off ally and viz
        }
    }
}

pub unsafe fn capture(trg: u32, atk: u32) -> bool {
    let own1 = rb(trg + S_OWNER as u32);
    let own2 = rb(atk + S_OWNER as u32);
    if own1 != own2 {
        if !pcpt || (rb(atk + S_ID as u32) == U_PEASANT) || (rb(atk + S_ID as u32) == U_PEON) {
            // if only peons can capture and attacker is peon(or peasant)
            if cmp_stat(trg, U_FARM as i32, S_ID, CMP_BIGGER_EQ) {
                // only buildings (id >= farm)
                if cpt {
                    if check_unit_complete(trg) {
                        // completed buildings
                        let tid = rb(trg + S_ID as u32); // unit id
                        let mhp = rw(UNIT_HP_TABLE + 2 * tid as u32); // max hp
                        let hp = rw(trg + S_HP as u32); // unit hp
                        // C++ считает порог в double (литералы 100.0 и т.д.)
                        if hp as f64 <= (mhp as f64 / 100.0 * 5.0) + 1.0 {
                            // if hp<=5%
                            let hl = ((mhp as f64 / 100.0 * 5.0) + 1.0) as u16;
                            heal(trg, (hl % 256) as u8, (hl / 256) as u8); // heal 5% hp so that it will not die suddenly
                            if thcpt {
                                // if th captured, capture all
                                let mut mf =
                                    tid == U_TOWN_HALL || tid == U_GREAT_HALL;
                                if get_val(TH2, own1 as i32) != 0 {
                                    mf = tid == U_KEEP || tid == U_STRONGHOLD;
                                }
                                if get_val(TH3, own1 as i32) != 0 {
                                    mf = tid == U_CASTLE || tid == U_FORTRESS;
                                }
                                if mf {
                                    // THs of 1 2 and 3 tier
                                    captk = 0;
                                    for i in 0..16u32 {
                                        let mut p: u32 = rd(UNITS_LISTS + 4 * i);
                                        while p != 0 {
                                            if !check_unit_dead(p)
                                                && cmp_stat(p, own1 as i32, S_OWNER, CMP_EQ)
                                            {
                                                // capture all units of that player
                                                capt[captk as usize] = p;
                                                captk += 1;
                                            }
                                            p = rd(p + S_NEXT_UNIT_POINTER as u32);
                                        }
                                    }
                                    for i in 0..captk {
                                        give(capt[i as usize], own2);
                                    }
                                }
                            }
                            give(trg, own2);
                            return true;
                        }
                    }
                }
            } else {
                // units capture
                if ucpt && !check_unit_dead(trg) {
                    let tid = rb(trg + S_ID as u32); // unit id
                    let mhp = rw(UNIT_HP_TABLE + 2 * tid as u32); // max hp
                    let hp = rw(trg + S_HP as u32); // unit hp
                    if hp as f64 <= (mhp as f64 / 100.0 * 5.0) + 1.0 {
                        // if hp<=5%
                        let hl = ((mhp as f64 / 100.0 * 50.0) + 1.0) as u16;
                        heal(trg, (hl % 256) as u8, (hl / 256) as u8); // heal 50% hp so that it will not die suddenly
                        give(trg, own2);
                        return true;
                    }
                }
            }
        }
    }
    false
}

pub unsafe fn steal_res(trg: u32, atk: u32) {
    let own1 = rb(trg + S_OWNER as u32);
    let own2 = rb(atk + S_OWNER as u32);
    if (own1 != own2) && !check_ally(own1, own2) {
        if cmp_stat(trg, U_FARM as i32, S_ID, CMP_BIGGER_EQ) {
            // only buildings (id >= farm)
            if check_unit_complete(trg) {
                // completed buildings
                if cmp_stat(atk, U_FARM as i32, S_ID, CMP_SMALLER) {
                    // only units (not towers)
                    let aid = rb(atk + S_ID as u32);
                    let ar = rb(UNIT_RANGE_TABLE + aid as u32);
                    if ar < 2 {
                        if cmp_res(own1, 0, 2, 0, 0, 0, CMP_BIGGER_EQ) {
                            // 2 gold
                            change_res(own1, 3, 2, 1);
                            change_res(own2, 0, 2, 1);
                        }
                        if cmp_res(own1, 1, 1, 0, 0, 0, CMP_BIGGER_EQ) {
                            // 1 lumber
                            change_res(own1, 4, 1, 1);
                            change_res(own2, 1, 1, 1);
                        }
                    }
                }
            }
        }
    }
}

pub unsafe fn mana_burn(trg: u32, atk: u32) {
    let aid = rb(atk + S_ID as u32);
    if aid == U_DEMON {
        // only demon have manaburn (you can put any other unit id here)
        let tid = rb(trg + S_ID as u32);
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
            let mut mp = rb(trg + S_MANA as u32);
            let mut dmp = mp;
            if dmp > 30 {
                dmp = 30;
            }
            mp -= dmp;
            set_stat(trg, mp as i32, S_MANA);
            let mut hp = rw(trg + S_HP as u32);
            if hp != 0 {
                if hp > dmp as u16 {
                    hp -= dmp as u16;
                } else {
                    hp = 1;
                }
            }
            set_stat(trg, hp as i32, S_HP);
        }
    }
}

pub unsafe fn vamp_aura(trg: u32, atk: u32, id: u8) -> bool {
    let tid = rb(trg + S_ID as u32); // unit id
    if (rd(UNIT_GLOBAL_FLAGS + tid as u32 * 4) & IS_FLESHY) != 0 {
        // fleshy global flag
        let aid = rb(atk + S_ID as u32); // unit id
        if (rd(UNIT_GLOBAL_FLAGS + aid as u32 * 4) & IS_FLESHY) != 0 {
            // fleshy global flag
            let x = rb(atk + S_X as u32);
            let y = rb(atk + S_Y as u32);
            set_region(x as i32 - 5, y as i32 - 5, x as i32 + 5, y as i32 + 5); // set region around myself
            find_all_alive_units(ANY_MEN);
            sort_in_region();
            sort_stat(S_ID, id as i32, CMP_EQ);
            let o = rb(atk + S_OWNER as u32);
            for ui in 0..16u8 {
                if !check_ally(o, ui) {
                    sort_stat(S_OWNER, ui as i32, CMP_NEQ);
                }
            }
            return units != 0;
        }
    }
    false
}

pub unsafe fn devotion_aura(trg: u32, id: u8) -> bool {
    let tid = rb(trg + S_ID as u32); // unit id
    if tid < U_FARM {
        // not buildings
        let x = rb(trg + S_X as u32);
        let y = rb(trg + S_Y as u32);
        set_region(x as i32 - 5, y as i32 - 5, x as i32 + 5, y as i32 + 5); // set region around myself
        find_all_alive_units(id);
        sort_in_region();
        let o = rb(trg + S_OWNER as u32);
        for ui in 0..16u8 {
            if !check_ally(o, ui) {
                sort_stat(S_OWNER, ui as i32, CMP_NEQ);
            }
        }
        return units != 0;
    }
    false
}

pub unsafe fn peon_steal_attack(trg: u32, atk: u32) {
    let tid = rb(trg + S_ID as u32);
    let aid = rb(atk + S_ID as u32);
    if aid == U_PEASANT || aid == U_PEON {
        if tid == U_PEASANT || tid == U_PEON {
            let mut tf = rb(trg + S_PEON_FLAGS as u32);
            let mut af = rb(atk + S_PEON_FLAGS as u32);
            if (af & PEON_LOADED) == 0 && (tf & PEON_LOADED) != 0 {
                tf &= !PEON_LOADED;
                af |= PEON_LOADED;
                if (tf & PEON_HARVEST_GOLD) != 0 {
                    tf &= !PEON_HARVEST_GOLD;
                    af |= PEON_HARVEST_GOLD;
                }
                if (tf & PEON_HARVEST_LUMBER) != 0 {
                    tf &= !PEON_HARVEST_LUMBER;
                    af |= PEON_HARVEST_LUMBER;
                }
                set_stat(trg, tf as i32, S_PEON_FLAGS);
                set_stat(atk, af as i32, S_PEON_FLAGS);
                f_group_set()(trg);
                f_group_set()(atk);
            }
        }
    }
}
