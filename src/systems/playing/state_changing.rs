use crate::{
    audio_playing::AudioCommand,
    components::{Block, CTransform, Player},
    render_commands::{RenderCommand, RenderCommandBuffer},
    state::{GameMode, State},
    DIMS,
};
use glam::Vec2;
use hecs::World;
use raylib::prelude::Color;

pub fn check_for_level_complete(ecs: &World, state: &mut State) {
    if ecs.query::<&Block>().iter().next().is_none() {
        state.next_game_mode = Some(GameMode::LevelComplete);
        state.audio_command_buffer.push(AudioCommand::LevelWin);
    }
}
