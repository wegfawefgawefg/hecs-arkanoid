use hecs::World;

use crate::state::{GameMode, State};

pub fn transition_game_mode(ecs: &mut World, state: &mut State) {
    if let Some(transition_to) = state.next_game_mode {
        match transition_to {
            GameMode::Title => {
                title_init_state(ecs, state);
            }
            GameMode::Playing => {
                playing_init_state(ecs, state);
            }
            GameMode::GameOver => game_over_init_state(ecs, state),
        }
    }
}

////////////////////////    PER GAME MODE STATE TRANSITIONS     ////////////////////////
pub fn title_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();

    state.game_mode = GameMode::Title;
    state.next_game_mode = None;
}

pub fn playing_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();

    // // add a player
    // state.ecs.push((
    //     CTransform {
    //         pos: Vec2::new(100.0, 100.0),
    //         rot: Vec2::new(0.0, 1.0),
    //     },
    //     Physics {
    //         vel: Vec2::new(1.0, 1.0),
    //         rot_vel: 30.0,
    //     },
    //     InputControlled,
    //     Player,
    //     Gun {
    //         wants_to_shoot: false,
    //         fire_delay: 10,
    //         cooldown: 0,
    //     },
    // ));
    state.game_mode = GameMode::Playing;
    state.next_game_mode = None;
}

pub fn game_over_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();
    state.game_mode = GameMode::GameOver;
    state.next_game_mode = None;
}
