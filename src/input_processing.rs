use glam::Vec2;
use raylib::RaylibHandle;

use crate::state::{GameMode, State};

pub fn process_input(rl: &mut RaylibHandle, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_process_input(rl, state);
        }
        GameMode::Playing => {
            playing_process_input(rl, state);
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
        state.next_game_mode = Some(GameMode::Playing);
    }
    state.title_inputs = title_inputs;
}

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
    };
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_A)
    {
        inputs.left = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_D)
    {
        inputs.right = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
        inputs.confirm = true;
    }
    state.playing_inputs = inputs;
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
}
impl PlayingInputs {
    pub fn new() -> PlayingInputs {
        PlayingInputs {
            left: false,
            right: false,
            confirm: false,
        }
    }
}
