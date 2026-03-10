use crate::{
    audio_playing::AudioCommand,
    components::{Ball, Block, StrongBlock},
    state::{GameMode, State},
};
use hecs::World;

pub fn check_for_level_complete(ecs: &World, state: &mut State) {
    if ecs
        .query::<&Block>()
        .without::<&StrongBlock>()
        .iter()
        .next()
        .is_none()
    {
        state.score = state.score.saturating_add(1_000);
        state.next_game_mode = Some(GameMode::LevelComplete);
        state.audio_command_buffer.push(AudioCommand::LevelWin);
    }
}

pub fn check_for_level_lost(ecs: &World, state: &mut State) {
    if ecs.query::<&Ball>().iter().next().is_none() {
        state.score = state.score.saturating_sub(250);
        if state.lives > 1 {
            state.lives -= 1;
            state.reset_powerup_state();
            state.next_game_mode = Some(GameMode::PrepareLevel);
        } else {
            state.lives = 0;
            state.next_game_mode = Some(GameMode::GameOver);
            state.audio_command_buffer.push(AudioCommand::LevelLost);
        }
    }
}
