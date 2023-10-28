use crate::{
    audio_playing::AudioCommand,
    components::Block,
    state::{GameMode, State},
};
use hecs::World;

pub fn check_for_level_complete(ecs: &World, state: &mut State) {
    if ecs.query::<&Block>().iter().next().is_none() {
        state.next_game_mode = Some(GameMode::LevelComplete);
        state.audio_command_buffer.push(AudioCommand::LevelWin);
    }
}
