use hecs::World;

use crate::{
    components::{CTransform, Paddle, Physics},
    game_mode_transitions::spawn_level,
    state::{State, LEVEL_CHANGE_DELAY_DEFAULT},
};

const PLAYER_BASE_MOVE_SPEED: f32 = 10.0;

pub fn process_inputs(ecs: &mut World, state: &mut State) {
    for (_, (_, ctransform, physics)) in ecs
        .query::<(&Paddle, &mut CTransform, &mut Physics)>()
        .iter()
    {
        if state.playing_inputs.right {
            physics.vel.x = PLAYER_BASE_MOVE_SPEED;
        } else if state.playing_inputs.left {
            physics.vel.x = -PLAYER_BASE_MOVE_SPEED;
        } else {
            physics.vel.x = 0.0;
        }
    }

    if state.level_change_delay > 0 {
        return;
    }
    if state.playing_inputs.next_level {
        state.level += 1;
        state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
        spawn_level(ecs, state, state.level);
    } else if state.playing_inputs.previous_level {
        state.level -= 1;
        state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
        spawn_level(ecs, state, state.level);
    }
}
