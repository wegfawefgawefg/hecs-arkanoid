use hecs::World;
use raylib::RaylibHandle;

use crate::{
    state::{GameMode, State},
    systems::{self},
};

pub fn step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_step(rl, ecs, state);
        }
        GameMode::Playing => {
            playing_step(rl, ecs, state);
        }
        GameMode::GameOver => {
            game_over_step(rl, ecs, state);
        }
    }
}

////////////////////////    PER GAME MODE STEPPING     ////////////////////////
pub fn title_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {}

pub fn playing_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    if state.level_change_delay > 0 {
        state.level_change_delay -= 1;
    }

    systems::playing::input_processing::process_inputs(ecs, state);
    systems::playing::physics::physics(ecs, state);
    systems::playing::physics::damage_blocks(ecs, state);
    // systems::playing::physics::boundary_checking(ecs, state);
    systems::playing::rendering::render(ecs, state);
}

pub fn game_over_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {}
