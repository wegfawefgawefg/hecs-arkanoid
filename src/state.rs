use glam::Vec2;
use hecs::Entity;

use crate::{
    audio_playing::AudioCommandBuffer,
    input_processing::{PlayingInputs, TitleInputs},
};

pub const FRAMES_PER_SECOND: u32 = 120;

#[derive(Clone, Copy)]
pub enum GameMode {
    Title,
    PrepareLevel,
    Playing,
    LevelComplete,
    WinGame,
    GameOver,
}

pub const LEVEL_CHANGE_DELAY_DEFAULT: u32 = 10;
pub struct State {
    pub fps: f32,
    pub running: bool,
    pub time_since_last_update: f32,
    pub t: f32,

    pub game_mode: GameMode,
    pub next_game_mode: Option<GameMode>,

    pub prepare_level_state: Box<PrepareLevelState>,
    pub level_complete_state: Box<LevelCompleteState>,
    pub win_game_state: Box<WinGameState>,
    pub game_over_state: Box<GameOverState>,

    pub audio_command_buffer: AudioCommandBuffer,
    pub title_inputs: TitleInputs,
    pub playing_inputs: PlayingInputs,
    pub mouse_screen_pos: Vec2,

    // pub collision_events: Vec<Collision>,
    pub level: u32,
    pub lives: u32,
    pub score: u32,
    pub level_change_delay: u32,
    pub near_clear_frames: u32,
    pub paddle_width_scale: f32,
    pub ball_speed_scale: f32,
    pub sticky_mode: bool,
    pub laser_mode: bool,
    pub fireball_mode: bool,
    pub laser_cooldown: u32,

    pub deletion_events: Vec<DeletionEvent>,
}

impl State {
    pub fn new() -> Self {
        let game_mode = GameMode::Title;
        let transition_to: Option<GameMode> = None;

        let prepare_level_state = Box::new(PrepareLevelState {
            mode: PrepareLevelMode::SpawnStuffIn,
            countdown: 0,
        });
        let level_complete_state = Box::new(LevelCompleteState {
            mode: LevelCompleteMode::Announce,
            countdown: 0,
        });
        let win_game_state = Box::new(WinGameState {
            mode: WinGameMode::Announce,
            countdown: 0,
        });
        let game_over_state = Box::new(GameOverState {
            mode: GameOverMode::Announce,
            countdown: 0,
        });

        let audio_command_buffer: AudioCommandBuffer = AudioCommandBuffer::new();

        let title_inputs = TitleInputs::new();
        let playing_inputs = PlayingInputs::new();
        let mouse_screen_pos = Vec2::ZERO;

        let deletion_events: Vec<DeletionEvent> = Vec::new();

        Self {
            fps: 0.0,
            running: true,
            time_since_last_update: 0.0,

            t: 0.0,

            game_mode,
            next_game_mode: transition_to,

            prepare_level_state,
            level_complete_state,
            win_game_state,
            game_over_state,

            audio_command_buffer,
            title_inputs,
            playing_inputs,
            mouse_screen_pos,

            // collision_events: Vec::new(),
            level: 1,
            lives: 3,
            score: 0,
            level_change_delay: 0,
            near_clear_frames: 0,
            paddle_width_scale: 1.0,
            ball_speed_scale: 1.0,
            sticky_mode: false,
            laser_mode: false,
            fireball_mode: false,
            laser_cooldown: 0,

            deletion_events,
        }
    }

    pub fn reset_run(&mut self) {
        self.level = 1;
        self.lives = 3;
        self.score = 0;
        self.near_clear_frames = 0;
        self.reset_powerup_state();
    }

    pub fn reset_powerup_state(&mut self) {
        self.paddle_width_scale = 1.0;
        self.ball_speed_scale = 1.0;
        self.sticky_mode = false;
        self.laser_mode = false;
        self.fireball_mode = false;
        self.laser_cooldown = 0;
    }
}

pub enum DeletionEvent {
    Entity { entity: Entity },
}

pub enum PrepareLevelMode {
    SpawnStuffIn,
    AnnounceLevel,
    ShortPause,
    SpawnBall,
}

impl ToString for PrepareLevelMode {
    fn to_string(&self) -> String {
        match self {
            PrepareLevelMode::SpawnStuffIn => "SpawnStuffIn".to_string(),
            PrepareLevelMode::AnnounceLevel => "AnnounceLevel".to_string(),
            PrepareLevelMode::ShortPause => "ShortPause".to_string(),
            PrepareLevelMode::SpawnBall => "SpawnBall".to_string(),
        }
    }
}

pub enum LevelCompleteMode {
    Announce,
    Announce2,
    Pause,
}

impl ToString for LevelCompleteMode {
    fn to_string(&self) -> String {
        match self {
            LevelCompleteMode::Announce => "Announce".to_string(),
            LevelCompleteMode::Announce2 => "Announce2".to_string(),
            LevelCompleteMode::Pause => "Pause".to_string(),
        }
    }
}

pub struct PrepareLevelState {
    pub mode: PrepareLevelMode,
    pub countdown: u32,
}

pub struct LevelCompleteState {
    pub mode: LevelCompleteMode,
    pub countdown: u32,
}

pub enum WinGameMode {
    Announce,
    Announce2,
    Pause,
}
pub struct WinGameState {
    pub mode: WinGameMode,
    pub countdown: u32,
}

pub enum GameOverMode {
    Announce,
    Announce2,
    Pause,
}

impl ToString for GameOverMode {
    fn to_string(&self) -> String {
        match self {
            GameOverMode::Announce => "Announce".to_string(),
            GameOverMode::Announce2 => "Announce2".to_string(),
            GameOverMode::Pause => "Pause".to_string(),
        }
    }
}

pub struct GameOverState {
    pub mode: GameOverMode,
    pub countdown: u32,
}
