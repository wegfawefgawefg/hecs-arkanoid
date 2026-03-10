use std::collections::HashSet;

use rand::RngExt;
use raylib::prelude::*;

use crate::audio::{Audio, SoundEffect};

pub type AudioCommandBuffer = Vec<AudioCommand>;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AudioCommand {
    BallWallBounce,
    BallBlockBounce,
    BallPaddleBounce,
    BallSturdyBlockBounce,
    BallDrop,
    PowerUpCatch,
    PaddleLaser,

    LevelStart,
    LevelWin,
    LevelLost,
}

pub fn execute_audio_command_buffer(
    _rl: &mut RaylibHandle,
    audio: &mut Audio,
    audio_command_buffer: &mut AudioCommandBuffer,
) {
    let unique_commands: HashSet<_> = audio_command_buffer.iter().cloned().collect();
    let mut rng = rand::rng();
    for command in unique_commands.iter() {
        match command {
            AudioCommand::BallBlockBounce => {
                let explosion_variants = [
                    SoundEffect::BallBounce1 as usize,
                    SoundEffect::BallBounce2 as usize,
                    SoundEffect::BallBounce3 as usize,
                    SoundEffect::BallBounce4 as usize,
                ];
                let random_explosion = explosion_variants[rng.random_range(0..4)];
                audio.sounds[random_explosion].set_pitch(rng.random_range(0.96..1.06));
                audio.sounds[random_explosion].set_volume(rng.random_range(0.85..1.0));
                audio.sounds[random_explosion].play();
            }
            AudioCommand::BallWallBounce => {
                audio.sounds[SoundEffect::BallWallBounce as usize]
                    .set_pitch(rng.random_range(0.98..1.04));
                audio.sounds[SoundEffect::BallWallBounce as usize]
                    .set_volume(rng.random_range(0.8..0.95));
                audio.sounds[SoundEffect::BallWallBounce as usize].play();
            }
            AudioCommand::BallSturdyBlockBounce => {
                audio.sounds[SoundEffect::BallSturdyBlockBounce as usize]
                    .set_pitch(rng.random_range(0.94..1.0));
                audio.sounds[SoundEffect::BallSturdyBlockBounce as usize]
                    .set_volume(rng.random_range(0.85..1.0));
                audio.sounds[SoundEffect::BallSturdyBlockBounce as usize].play();
            }
            AudioCommand::BallPaddleBounce => {
                audio.sounds[SoundEffect::BallHitPaddle as usize]
                    .set_pitch(rng.random_range(0.97..1.05));
                audio.sounds[SoundEffect::BallHitPaddle as usize]
                    .set_volume(rng.random_range(0.85..1.0));
                audio.sounds[SoundEffect::BallHitPaddle as usize].play();
            }
            AudioCommand::LevelStart => {
                audio.sounds[SoundEffect::LevelStart as usize].play();
            }
            AudioCommand::LevelWin => {
                audio.sounds[SoundEffect::LevelWin as usize].play();
            }
            AudioCommand::BallDrop => {
                audio.sounds[SoundEffect::BallDrop as usize].play();
            }
            AudioCommand::PowerUpCatch => {
                audio.sounds[SoundEffect::Confirm as usize].set_pitch(rng.random_range(0.98..1.08));
                audio.sounds[SoundEffect::Confirm as usize].play();
            }
            AudioCommand::PaddleLaser => {
                audio.sounds[SoundEffect::SmallLaser as usize]
                    .set_pitch(rng.random_range(0.98..1.04));
                audio.sounds[SoundEffect::SmallLaser as usize].play();
            }
            AudioCommand::LevelLost => {
                audio.sounds[SoundEffect::LevelLost as usize].play();
            }
        }
    }
}
