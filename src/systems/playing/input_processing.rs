use hecs::World;

use crate::{
    components::{CTransform, Paddle, Physics},
    game_mode_transitions::spawn_level,
    physics_engine::p2m,
    state::{State, LEVEL_CHANGE_DELAY_DEFAULT},
};

const PLAYER_BASE_MOVE_SPEED: f32 = 300.0;

pub fn process_inputs(ecs: &mut World, state: &mut State) {
    for (entity, paddle) in ecs.query::<&Paddle>().iter() {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
                if state.playing_inputs.right {
                    rigid_body.set_linvel(
                        nalgebra::Vector2::new(p2m(PLAYER_BASE_MOVE_SPEED), 0.0),
                        true,
                    );
                } else if state.playing_inputs.left {
                    rigid_body.set_linvel(
                        nalgebra::Vector2::new(-p2m(PLAYER_BASE_MOVE_SPEED), 0.0),
                        true,
                    );
                } else {
                    rigid_body.set_linvel(nalgebra::Vector2::new(0.0, 0.0), true);
                }
            }
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
