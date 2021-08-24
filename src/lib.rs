use log::*;
use screeps_arena::{
    prelude::*,
    game,
    constants::{prototypes, Part},
};
use wasm_bindgen::prelude::*;

mod logging;

fn setup() {
    logging::setup_logging(logging::Info);
}

// add wasm_bindgen to any function you would like to expose for call from js
// to use a reserved name as a function name, use `js_name`:
#[wasm_bindgen(js_name = loop)]
pub fn tick() {
    let tick = game::utils::get_ticks();

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
        let spawns = game::utils::get_objects_by_prototype(prototypes::STRUCTURE_SPAWN);
        warn!("spawns {}", spawns.len());
        for spawn in spawns {
            if spawn.my().unwrap_or(false) {
                spawn.spawn_creep(&[Part::Move, Part::Attack]);
            } else {
                enemy_spawn = Some(spawn);
            }
        }

        let creeps = game::utils::get_objects_by_prototype(prototypes::CREEP);
        warn!("creeps {}", creeps.len());
        for creep in creeps {
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
