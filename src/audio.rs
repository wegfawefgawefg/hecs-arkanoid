use raylib::prelude::*;
use raylib::{prelude::RaylibAudio, RaylibHandle, RaylibThread};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub enum Song {
    Playing,
}

#[derive(Copy, Clone, Debug, EnumIter, PartialEq, Eq, Hash)]
pub enum SoundEffect {
    Confirm,
    SuperConfirm,
    SmallLaser,
    ExplosionOne,
    ExplosionTwo,
    ExplosionThree,

    BallBounce1,
    BallBounce2,
    BallBounce3,
    BallBounce4,

    BallSturdyBlockBounce,

    BallHitPaddle,
    BallWallBounce,

    BallDrop,

    LevelStart,
    LevelWin,
    LevelLost,
}

pub struct Audio {
    pub rl_audio_device: RaylibAudio,
    pub songs: Vec<Music>,
    pub sounds: Vec<Sound>,
    pub music_volume: f32,
    pub sound_effects_volume: f32,
}

impl Audio {
    pub fn new(_rl: &mut RaylibHandle, rlt: &RaylibThread) -> Self {
        let rl_audio_device = RaylibAudio::init_audio_device();

        let error = "Error loading audio";
        let mut songs = Vec::new();
        let file_names = vec!["playing"];
        for name in file_names {
            let path = format!("assets/music/{}.ogg", name);
            let music = Music::load_music_stream(rlt, path.as_str()).expect(error);
            songs.push(music);
        }

        let error = "Error loading audio";
        let mut sounds = Vec::new();

        for sound_effect in SoundEffect::iter() {
            let file_name_prefix = get_sound_file_name(sound_effect);
            let path = format!("assets/sounds/{}.ogg", file_name_prefix);
            // let music = Music::load_music_stream(rlt, path.as_str()).expect(error);
            let sound = Sound::load_sound(path.as_str()).expect(error);
            sounds.push(sound);
        }

        Self {
            rl_audio_device,
            songs,
            sounds,
            music_volume: 1.0,
            sound_effects_volume: 1.0,
        }
    }

    // pub fn play_sound_effect(&mut self, sound_effect: SoundEffect) {
    //     let sound_effect = &mut self.sounds[sound_effect as usize];
    //     self.rl_audio_device.play_sound(sound_effect);
    // }

    // pub fn set_sound_volumes(&mut self) {
    //     for sound_effect in SoundEffect::iter() {
    //         let sound_effect = &mut self.sounds[sound_effect as usize];
    //         self.rl_audio_device
    //             .set_sound_volume(sound_effect, self.sound_effects_volume);
    //     }
    // }
}

pub fn get_sound_file_name(sound_effect: SoundEffect) -> &'static str {
    match sound_effect {
        SoundEffect::Confirm => "confirm",
        SoundEffect::SuperConfirm => "super_confirm",
        SoundEffect::SmallLaser => "small_laser",
        SoundEffect::ExplosionOne => "explosion_1",
        SoundEffect::ExplosionTwo => "explosion_2",
        SoundEffect::ExplosionThree => "explosion_3",

        SoundEffect::BallBounce1 => "ball_bounce_1",
        SoundEffect::BallBounce2 => "ball_bounce_2",
        SoundEffect::BallBounce3 => "ball_bounce_3",
        SoundEffect::BallBounce4 => "ball_bounce_4",

        SoundEffect::BallSturdyBlockBounce => "sturdy_block_bounced_on",

        SoundEffect::BallHitPaddle => "ball_hit_paddle",
        SoundEffect::BallWallBounce => "ball_wall_bounce",

        SoundEffect::LevelStart => "level_start",
        SoundEffect::LevelWin => "level_win",
        SoundEffect::BallDrop => "ball_drop",
        SoundEffect::LevelLost => "level_loss",
    }
}
