use hecs::World;

use crate::{
    components::{CTransform, Paddle, Shape},
    game_mode_transitions::{spawn_level, BASE_PADDLE_SHAPE},
    state::{GameMode, State, LEVEL_CHANGE_DELAY_DEFAULT},
    DIMS,
};

pub fn process_inputs(ecs: &mut World, state: &mut State) {
    for (ctransform, shape) in ecs
        .query::<(&mut CTransform, &mut Shape)>()
        .with::<&Paddle>()
        .iter()
    {
        shape.dims.x = BASE_PADDLE_SHAPE.x * state.paddle_width_scale;
        shape.dims.y = BASE_PADDLE_SHAPE.y;
        ctransform.pos.x = state.mouse_screen_pos.x - shape.dims.x / 2.0;
        ctransform.pos.x = ctransform
            .pos
            .x
            .clamp(1.0, DIMS.x as f32 - shape.dims.x - 1.0);
    }

    if state.level_change_delay > 0 {
        return;
    }
    if state.playing_inputs.next_level {
        state.next_game_mode = Some(GameMode::LevelComplete);
        return;
    } else if state.playing_inputs.previous_level {
        if state.level == 1 {
            return;
        }
        state.level -= 1;
        state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
        spawn_level(ecs, state.level);
    }

    if state.playing_inputs.restart_level {
        state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
        state.next_game_mode = Some(crate::state::GameMode::PrepareLevel);
    }
}
