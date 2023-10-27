use std::collections::HashSet;

use rand::Rng;
use raylib::prelude::*;

use crate::{
    audio::{Audio, SoundEffect},
    state::State,
};

pub type AudioCommandBuffer = Vec<AudioCommand>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AudioCommand {
    BallWallBounce,
    BallBlockBounce,
    BallPaddleBounce,
    BallSturdyBlockBounce,

    LevelStart,
    LevelWin,
}

pub fn execute_audio_command_buffer(
    rl: &mut RaylibHandle,
    audio: &mut Audio,
    audio_command_buffer: &mut AudioCommandBuffer,
) {
    let unique_commands: HashSet<_> = audio_command_buffer.iter().cloned().collect();
    let mut rng = rand::thread_rng();
    for command in unique_commands.iter() {
        match command {
            AudioCommand::BallBlockBounce => {
                let explosion_variants = [
                    SoundEffect::BallBounce1 as usize,
                    SoundEffect::BallBounce2 as usize,
                    SoundEffect::BallBounce3 as usize,
                    SoundEffect::BallBounce4 as usize,
                ];
                let random_explosion = explosion_variants[rng.gen_range(0..4)];
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[random_explosion]);
            }
            AudioCommand::BallWallBounce => {
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[SoundEffect::BallWallBounce as usize]);
            }
            AudioCommand::BallSturdyBlockBounce => {
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[SoundEffect::BallSturdyBlockBounce as usize]);
            }
            AudioCommand::BallPaddleBounce => {
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[SoundEffect::BallHitPaddle as usize]);
            }
            AudioCommand::LevelStart => {
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[SoundEffect::LevelStart as usize]);
            }
            AudioCommand::LevelWin => {
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[SoundEffect::LevelWin as usize]);
            }
        }
    }
}
