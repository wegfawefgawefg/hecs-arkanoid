use glam::Vec2;
use raylib::RaylibHandle;

use crate::state::{GameMode, State};

pub fn process_input(rl: &mut RaylibHandle, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_process_input(rl, state);
        }
        GameMode::PrepareLevel => {
            prepare_level_process_input(rl, state);
        }
        GameMode::Playing => {
            playing_process_input(rl, state);
        }
        GameMode::LevelComplete => {
            level_complete_process_input(rl, state);
        }
        GameMode::WinGame => {
            win_game_process_input(rl, state);
        }
        GameMode::GameOver => {
            game_over_process_input(rl, state);
        }
    }
}

////////////////////////    PER GAME MODE INPUT PROCESSING     ////////////////////////
pub fn title_process_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mouse_pos_rl = rl.get_mouse_position();
    let mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);
    state.mouse_screen_pos = mouse_pos;

    let mut title_inputs = TitleInputs { confirm: false };
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        title_inputs.confirm = true;
    }
    if title_inputs.confirm {
        state.next_game_mode = Some(GameMode::PrepareLevel);
    }
    state.title_inputs = title_inputs;
}

pub fn prepare_level_process_input(rl: &mut RaylibHandle, state: &mut State) {}

pub fn playing_process_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mouse_pos_rl = rl.get_mouse_position();
    let mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);
    state.mouse_screen_pos = mouse_pos;

    let mut inputs = PlayingInputs {
        left: false,
        right: false,
        confirm: false,
        next_level: false,
        previous_level: false,
    };
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
        inputs.left = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
        inputs.right = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
        inputs.confirm = true;
    }

    // advance level up and down if right or left arrow key is pressed
    // if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT) && state.level < 35 {
    //     inputs.next_level = true;
    // }
    // if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT) && state.level > 0 {
    //     inputs.previous_level = true;
    // }

    state.playing_inputs = inputs;
}

pub fn level_complete_process_input(rl: &mut RaylibHandle, state: &mut State) {}

pub fn win_game_process_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mut title_inputs = TitleInputs { confirm: false };
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        title_inputs.confirm = true;
    }

    if title_inputs.confirm {
        state.next_game_mode = Some(GameMode::Title);
    }
    state.title_inputs = title_inputs;
}

pub fn game_over_process_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_SPACE) {
        state.next_game_mode = Some(GameMode::Title);
    }
}

////////////////////////    INPUT DEFS    ////////////////////////
pub struct TitleInputs {
    pub confirm: bool,
}
impl TitleInputs {
    pub fn new() -> TitleInputs {
        TitleInputs { confirm: false }
    }
}

pub struct PlayingInputs {
    pub left: bool,
    pub right: bool,
    pub confirm: bool,

    pub next_level: bool,
    pub previous_level: bool,
}
impl PlayingInputs {
    pub fn new() -> PlayingInputs {
        PlayingInputs {
            left: false,
            right: false,
            confirm: false,

            next_level: false,
            previous_level: false,
        }
    }
}
