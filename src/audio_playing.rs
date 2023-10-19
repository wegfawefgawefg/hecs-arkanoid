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
    AsteroidExplosion,
    Shoot,
    PlayerExplosion,
    PlayerHit,
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
            AudioCommand::Shoot => {
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[SoundEffect::SmallLaser as usize]);
            }
            AudioCommand::AsteroidExplosion => {
                let explosion_variants = [
                    SoundEffect::ExplosionOne as usize,
                    SoundEffect::ExplosionTwo as usize,
                    SoundEffect::ExplosionThree as usize,
                ];
                let random_explosion = explosion_variants[rng.gen_range(0..3)];
                audio
                    .rl_audio_device
                    .play_sound(&audio.sounds[random_explosion]);
            }
            _ => {}
        }
    }
}
