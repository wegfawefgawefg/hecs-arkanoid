use hecs::World;

use crate::{
    components::{CTransform, Paddle, Shape},
    game_mode_transitions::spawn_level,
    state::{State, LEVEL_CHANGE_DELAY_DEFAULT},
};

pub fn process_inputs(ecs: &mut World, state: &mut State) {
    for (_, (ctransform, shape)) in ecs
        .query::<(&mut CTransform, &Shape)>()
        .with::<&Paddle>()
        .iter()
    {
        ctransform.pos.x = state.mouse_screen_pos.x - shape.dims.x / 2.0;
    }

    if state.level_change_delay > 0 {
        return;
    }
    if state.playing_inputs.next_level {
        state.level += 1;
        state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
        spawn_level(ecs, state, state.level);
    } else if state.playing_inputs.previous_level {
        if state.level == 1 {
            return;
        }
        state.level -= 1;
        state.level_change_delay = LEVEL_CHANGE_DELAY_DEFAULT;
        spawn_level(ecs, state, state.level);
    }
}
