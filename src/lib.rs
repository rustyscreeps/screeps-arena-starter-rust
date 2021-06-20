use std::cell::RefCell;
use std::collections::HashMap;

use js_sys::{Array, JsString, Object};
use log::*;
use screeps_arena::{prelude::*, game, objects::*, constants::Part};
use wasm_bindgen::prelude::*;

mod logging;

fn setup() {
    logging::setup_logging(logging::Info);
}

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_time();

    if tick == 1 {
        setup();
    }
    warn!("hello arena! {}", tick);

    let info = game::arena_info();
    warn!("arena_info: {:?}", info);

    // strategy for spawn and swamp arena, which will conditionally compile in
    // only when this feature is enabled for the crate
    #[cfg(feature = "arena-spawn-and-swamp")]
    {
        let mut enemy_spawn = None;
        let spawn_array = game::utils::get_objects_by_prototype(&STRUCTURE_SPAWN_PROTOTYPE);
        warn!("spawns {}", spawn_array.length());
        for spawn in spawn_array.iter().map(StructureSpawn::from) {
            if spawn.my().unwrap_or(false) {
                spawn.spawn_creep(&[Part::Move, Part::Attack]);
            } else {
                enemy_spawn = Some(spawn);
            }
        }

        let creep_array = game::utils::get_objects_by_prototype(&CREEP_PROTOTYPE);
        warn!("creeps {}", creep_array.length());
        for creep in creep_array.iter().map(Creep::from) {
            if creep.my() {
                match &enemy_spawn {
                    Some(t) => {
                        creep.move_to(t.as_ref(), None);
                        creep.attack(t);
                    }
                    None => {}
                }
            }
        }
    }
    
}
