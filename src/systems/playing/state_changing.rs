use crate::{
    audio_playing::AudioCommand,
    components::{Ball, Block},
    state::{GameMode, State},
};
use hecs::World;

pub fn check_for_level_complete(ecs: &World, state: &mut State) {
    if ecs.query::<&Block>().iter().next().is_none() {
        state.next_game_mode = Some(GameMode::LevelComplete);
        state.audio_command_buffer.push(AudioCommand::LevelWin);
    }
}

pub fn check_for_level_lost(ecs: &World, state: &mut State) {
    if ecs.query::<&Ball>().iter().next().is_none() {
        state.next_game_mode = Some(GameMode::GameOver);
        state.audio_command_buffer.push(AudioCommand::LevelStart);
    }
}
