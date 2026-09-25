#![allow(static_mut_refs)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

pub mod ai;
pub mod aitick;
pub mod buildui;
pub mod combat;
pub mod damage;
pub mod defs;
pub mod find;
pub mod fixes;
pub mod game;
pub mod heroes;
pub mod init;
pub mod mechanics;
pub mod names;
pub mod net;
pub mod orders;
pub mod patch;
pub mod place;
pub mod player;
pub mod saveload;
pub mod sort;
pub mod state;
pub mod tick;
pub mod uihooks;
pub mod victory;

#[no_mangle]
pub extern "C" fn w2p_init() {
    unsafe {
        init::common_hooks(); // hook functions
        fixes::sounds_tables(); // fix hero sounds
    }
}
