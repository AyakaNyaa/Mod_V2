// Размещение зданий, лес/шахта, инвентарь ресурсов (w2p.cpp:4466-4808)
#![allow(dead_code)]

use crate::defs::*;
use crate::find::*;
use crate::game::*;
use crate::names::*;
use crate::orders::*;
use crate::orig_fn;
use crate::patch;
use crate::patch::*;
use crate::patch_set;
use crate::player::*;
use crate::state::*;

pub unsafe fn def_stat(
    u: u8,
    hp: u16,
    str_: u8,
    prc: u8,
    arm: u8,
    rng: u8,
    gold: u8,
    lum: u8,
    oil: u8,
    time: u8,
) {
    // change some unit stats (changes for ALL units of this type)

    /*
    to change vision and multiselectable you can use this construction
    char buf[] = "\x0\x0\x0\x0";//fix vision
    patch_setdword((DWORD*)buf, (DWORD)F_VISION6);
    PATCH_SET((char*)(UNIT_VISION_FUNCTIONS_TABLE + 4 * U_DEMON), buf);
    char buf2[] = "\x1";
    PATCH_SET((char*)(UNIT_MULTISELECTABLE + U_DEMON), buf2);
    */

    let buf2 = [(hp % 256) as u8, (hp / 256) as u8];
    patch_set!(UNIT_HP_TABLE + 2 * u as u32, buf2);
    let buf = [str_];
    patch_set!(UNIT_STRENGTH_TABLE + u as u32, buf);
    let buf = [prc];
    patch_set!(UNIT_PIERCE_TABLE + u as u32, buf);
    let buf = [arm];
    patch_set!(UNIT_ARMOR_TABLE + u as u32, buf);
    let buf = [rng];
    patch_set!(UNIT_RANGE_TABLE + u as u32, buf);
    let buf = [gold];
    patch_set!(UNIT_GOLD_TABLE + u as u32, buf);
    let buf = [lum];
    patch_set!(UNIT_LUMBER_TABLE + u as u32, buf);
    let buf = [oil];
    patch_set!(UNIT_OIL_TABLE + u as u32, buf);
    let buf = [time];
    patch_set!(UNIT_TIME_TABLE + u as u32, buf);
}

#[no_mangle]
pub extern "C" fn grow_structure(p: u32) {
    unsafe {
        orig_fn!(g_proc_004526FE, extern "C" fn(u32))(p); // original
        if p != 0 {
            let id = rb(p + S_ID as u32);
            if id == U_FARM || id == U_PIGFARM {
                if check_unit_complete(p) {
                    let cid = rb(p + S_RESOURCES as u32);
                    if (id == U_FARM && cid == U_PEON) || (id == U_PIGFARM && cid == U_PEASANT) {
                        // if peon created human farm OR peasant created orc farm - its not normal so its considered to grow forest
                        let pg = rb(UNIT_GOLD_TABLE + id as u32); // gold
                        let pl = rb(UNIT_LUMBER_TABLE + id as u32); // lumber
                        let po = rb(UNIT_OIL_TABLE + id as u32); // oil
                        let o = rb(p + S_OWNER as u32);
                        change_res(o, 0, pg, 10); // return gold
                        change_res(o, 1, pl, 10); // return lumber
                        change_res(o, 2, po, 10); // return oil

                        let price = 500;
                        if cmp_res(o, 0, (price % 256) as u8, (price / 256) as u8, 0, 0, CMP_BIGGER_EQ) {
                            change_res(o, 3, 1, price); // get 1000 gold
                            let x = rb(p + S_X as u32);
                            let y = rb(p + S_Y as u32);
                            let mxs = rb(MAP_SIZE); // map size
                            let era = rb(MAP_ERA); // map era
                            let cel = rd(MAP_CELLS_POINTER); // map cells
                            let sq = rd(MAP_SQ_POINTER); // map cells
                            let reg_ptr = rd(MAP_REG_POINTER); // map reg (в C++ локальная reg тенила глобальную)
                            let bufte = [
                                0x88u8, 0x84, 0x82, 0x81, 0x81, 0x83, 0x7F, 0x85, 0x81, 0x83,
                                0x7F, 0x86, 0x81, 0x83, 0x7F, 0x85,
                            ]; // 4 era tiles
                            let mut buft = [0x7Eu8, 0x0];
                            let mut xx = x as i32;
                            while (xx < x as i32 + 2) && (xx < mxs as i32) {
                                let mut yy = y as i32;
                                while (yy < y as i32 + 2) && (yy < mxs as i32) {
                                    if xx == x as i32 && yy == y as i32 {
                                        buft[0] = bufte[era as usize * 4];
                                    }
                                    if xx == x as i32 + 1 && yy == y as i32 {
                                        buft[0] = bufte[era as usize * 4 + 1];
                                    }
                                    if xx == x as i32 && yy == y as i32 + 1 {
                                        buft[0] = bufte[era as usize * 4 + 2];
                                    }
                                    if xx == x as i32 + 1 && yy == y as i32 + 1 {
                                        buft[0] = bufte[era as usize * 4 + 3];
                                    }
                                    patch_set!(cel + 2 * xx as u32 + 2 * yy as u32 * mxs as u32, buft);
                                    let buf = [0x81u8, 0x0]; // unpassable land
                                    patch_set!(sq + 2 * xx as u32 + 2 * yy as u32 * mxs as u32, buf);
                                    let buf = [0xFEu8, 0xFF]; // tree tree
                                    patch_set!(reg_ptr + 2 * xx as u32 + 2 * yy as u32 * mxs as u32, buf);
                                    yy += 1;
                                }
                                xx += 1;
                            }
                        }
                        let mut flg = rb(p + S_FLAGS3 as u32);
                        flg |= SF_HIDDEN; // hide
                        set_stat(p, flg as i32, S_FLAGS3);
                        f_dead_building_redraw()(p); // unitdraw insert dead bldg
                        unit_kill(p);
                    }
                }
            }
            if id == U_MINE {
                if check_unit_complete(p) {
                    give(p, P_NEUTRAL);
                    set_stat(p, P_NEUTRAL as i32, S_COLOR);
                    let mhp = rw(UNIT_HP_TABLE + 2 * id as u32); // max hp
                    if mhp > 300 {
                        set_stat(p, 300, S_HP);
                    }
                    // int r = ((int (*)())F_NET_RANDOM)();
                    // r %= 50;
                    // r += 1;
                    let r = 50;
                    set_stat(p, r, S_RESOURCES);

                    let x = rb(p + S_X as u32);
                    let y = rb(p + S_Y as u32);
                    let mxs = rb(MAP_SIZE); // map size
                    let era = rb(MAP_ERA); // map era
                    let cel = rd(MAP_CELLS_POINTER); // map cells
                    let mut xx = x as i32;
                    while (xx < x as i32 + 3) && (xx < mxs as i32) {
                        let mut yy = y as i32;
                        while (yy < y as i32 + 3) && (yy < mxs as i32) {
                            let mut buf = [0x4bu8, 0x1];
                            if era == 0 {
                                buf[0] = 0x4e; // forest
                            }
                            if era == 1 {
                                buf[0] = 0x4b; // winter
                            }
                            if era == 2 {
                                buf[0] = 0x4a; // wast
                            }
                            if era == 3 {
                                buf[0] = 0x50; // swamp
                            }
                            patch_set!(cel + 2 * xx as u32 + 2 * yy as u32 * mxs as u32, buf);
                            yy += 1;
                        }
                        xx += 1;
                    }
                }
            }
            // if (id==U_ ??) you can add other building if need
        }
    }
}

#[no_mangle]
pub extern "C" fn create_building(a: i32, b: i32, id: i32, c: i32) -> u32 {
    unsafe {
        // this function called when building finished training unit
        // and new unit should be created
        let cr = rd(UNIT_RUN_UNIT_POINTER); // unit who created building
        let original = orig_fn!(g_proc_00418FFE, extern "C" fn(i32, i32, i32, i32) -> u32)(a, b, id, c); // original function
        if original != 0 {
            // created building
            if cr != 0 {
                // unit who created building
                if id == U_FARM as i32 || id == U_PIGFARM as i32 {
                    let cid = rb(cr + S_ID as u32);
                    set_stat(original, cid as i32, S_RESOURCES); // set id of creator (peon or pesant)
                }
                if id == U_CIRCLE as i32 {
                    let xx = rw(cr + S_DRAW_X as u32);
                    let yy = rw(cr + S_DRAW_Y as u32);
                    f_bullet_create()(xx + 16, yy + 16, 21);
                    unit_kill(cr); // sacrifice peon
                }
            }
        }
        original
    }
}

#[no_mangle]
pub extern "C" fn placebox_query(p: u32, x: u16, y: u16, id: u8) -> u16 {
    unsafe {
        // function that checking map cells when placing building
        let mut original = orig_fn!(g_proc_0043A974, extern "C" fn(u32, u16, u16, u8) -> u16)(p, x, y, id); // original
        let mxs = rb(MAP_SIZE); // map size
        let era = rb(MAP_ERA); // map era
        let cel = rd(MAP_CELLS_POINTER); // map cells
        let pid = rb(p + S_ID as u32); // builder
        if (id == U_FARM && pid == U_PEON) || (id == U_PIGFARM && pid == U_PEASANT) {
            // make new forest to grow
            let dr = 0x7Eu16; // destroyed forest
            let mut xx = x as i32;
            while (xx < x as i32 + 2) && (xx < mxs as i32) {
                let mut yy = y as i32;
                while (yy < y as i32 + 2) && (yy < mxs as i32) {
                    let c = rw(cel + 2 * xx as u32 + 2 * yy as u32 * mxs as u32);
                    if c != dr {
                        let buf = [0x7u8];
                        patch_set!(CAN_PLACE_TBL + (xx - x as i32) as u32 + 4 * (yy - y as i32) as u32, buf); // can place tbl
                        original = 7;
                    }
                    yy += 1;
                }
                xx += 1;
            }
        }
        if id == U_MINE {
            let mut dr = 0xA1u16; // destroyed rock
            if era == 0 {
                dr = 0xA6; // forest
            }
            if era == 1 {
                dr = 0xA1; // winter
            }
            if era == 2 {
                dr = 0xA3; // wast
            }
            if era == 3 {
                dr = 0xA1; // swamp
            }
            let mut xx = x as i32;
            while (xx < x as i32 + 3) && (xx < mxs as i32) {
                let mut yy = y as i32;
                while (yy < y as i32 + 3) && (yy < mxs as i32) {
                    let c = rw(cel + 2 * xx as u32 + 2 * yy as u32 * mxs as u32);
                    if c != dr {
                        let buf = [0x7u8];
                        patch_set!(CAN_PLACE_TBL + (xx - x as i32) as u32 + 4 * (yy - y as i32) as u32, buf); // can place tbl
                        original = 7;
                    }
                    yy += 1;
                }
                xx += 1;
            }
        }
        original
    }
}

#[no_mangle]
pub extern "C" fn goods_into_inventory(p: u32) -> i32 {
    unsafe {
        // use this function if you need to change amount of res peon or tanker bring back
        if true {
            let tr = rd(p + S_ORDER_UNIT_POINTER as u32);
            if tr != 0 {
                let mut f = false;
                let trg = tr;
                let o = rb(p + S_OWNER as u32);
                let id = rb(p + S_ID as u32);
                let tid = rb(trg + S_ID as u32);
                let mut pf = rb(p + S_PEON_FLAGS as u32);
                let pflag = rd(UNIT_GLOBAL_FLAGS + id as u32 * 4) as i32;
                let tflag = rd(UNIT_GLOBAL_FLAGS + tid as u32 * 4) as i32;
                let mut res: i32 = 100;
                if (pf & PEON_LOADED) != 0 {
                    if (pflag & IS_SHIP as i32) != 0 && (tflag & IS_OILRIG as i32) == 0 {
                        let r = get_val(REFINERY, o as i32);
                        if more_res {
                            res = 100 + 25 * r;
                        } else if r != 0 {
                            res = 125;
                        } else {
                            res = 100;
                        }
                        change_res(o, 2, 1, res);
                        add_total_res(o, 2, 1, res);
                        f = true;
                    } else {
                        if (tflag & IS_TOWNHALL as i32) != 0 || (tflag & IS_LUMBER as i32) != 0 {
                            if (tflag & IS_TOWNHALL as i32) != 0 {
                                pf |= PEON_IN_CASTLE;
                                set_stat(p, pf as i32, S_PEON_FLAGS);
                            }
                            if (pf & PEON_HARVEST_GOLD) != 0 && (tflag & IS_TOWNHALL as i32) != 0 {
                                let r2 = get_val(TH2, o as i32);
                                let r3 = get_val(TH3, o as i32);
                                if more_res {
                                    res = 100 + 10 * r2 + 20 * r3;
                                } else if r3 != 0 {
                                    res = 120;
                                } else if r2 != 0 {
                                    res = 110;
                                } else {
                                    res = 100;
                                }
                                pf &= !PEON_HARVEST_GOLD;
                                change_res(o, 0, 1, res);
                                add_total_res(o, 0, 1, res);
                                f = true;
                            } else if (pf & PEON_HARVEST_LUMBER) != 0 {
                                let r = get_val(LUMBERMILL, o as i32);
                                if more_res {
                                    res = 100 + 25 * r;
                                } else if r != 0 {
                                    res = 125;
                                } else {
                                    res = 100;
                                }
                                pf &= !PEON_HARVEST_LUMBER;
                                change_res(o, 1, 1, res);
                                add_total_res(o, 1, 1, res);
                                f = true;
                            }
                        }
                    }
                }
                if f {
                    pf &= !PEON_LOADED;
                    set_stat(p, pf as i32, S_PEON_FLAGS);
                    f_group_set()(p);
                    return 1;
                }
            }
            0
        } else {
            // orig_fn!(g_proc_00424745, extern "C" fn(u32) -> i32)(p) // original (dead branch в C++: if(TRUE))
            0
        }
    }
}
