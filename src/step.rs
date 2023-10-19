use raylib::RaylibHandle;

use crate::state::{GameMode, State};

pub fn step(rl: &mut RaylibHandle, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_step(rl, state);
        }
        GameMode::Playing => {
            playing_step(rl, state);
        }
        GameMode::GameOver => {
            game_over_step(rl, state);
        }
    }
}

////////////////////////    PER GAME MODE STEPPING     ////////////////////////
pub fn title_step(rl: &mut RaylibHandle, state: &mut State) {}

pub fn playing_step(rl: &mut RaylibHandle, state: &mut State) {}

pub fn game_over_step(rl: &mut RaylibHandle, state: &mut State) {}
