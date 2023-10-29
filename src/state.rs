use glam::Vec2;
use hecs::Entity;
use rand::{rngs::StdRng, SeedableRng};
use rapier2d::prelude::RigidBodyHandle;

use crate::{
    audio_playing::AudioCommandBuffer,
    components::Physics,
    input_processing::{PlayingInputs, TitleInputs},
    message_stream::ExpiringMessages,
    physics_engine::PhysicsEngine,
    render_commands::RenderCommandBuffer,
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
    pub rng: StdRng,

    pub game_mode: GameMode,
    pub next_game_mode: Option<GameMode>,

    pub prepare_level_state: Box<PrepareLevelState>,
    pub level_complete_state: Box<LevelCompleteState>,
    pub win_game_state: Box<WinGameState>,
    pub game_over_state: Box<GameOverState>,

    pub expiring_messages: ExpiringMessages,

    pub audio_command_buffer: AudioCommandBuffer,
    pub render_command_buffer: RenderCommandBuffer,

    pub title_inputs: TitleInputs,
    pub playing_inputs: PlayingInputs,
    pub mouse_screen_pos: Vec2,

    // pub collision_events: Vec<Collision>,
    pub level: u32,
    pub level_change_delay: u32,

    pub physics: PhysicsEngine,

    pub deletion_events: Vec<DeletionEvent>,
}

impl State {
    pub fn new() -> Self {
        let rng: StdRng = StdRng::from_entropy();

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

        let expiring_messages = ExpiringMessages::new();

        let render_command_buffer: RenderCommandBuffer = RenderCommandBuffer::new();
        let audio_command_buffer: AudioCommandBuffer = AudioCommandBuffer::new();

        let title_inputs = TitleInputs::new();
        let playing_inputs = PlayingInputs::new();
        let mouse_screen_pos = Vec2::ZERO;

        let physics = PhysicsEngine::new();

        let deletion_events: Vec<DeletionEvent> = Vec::new();

        Self {
            fps: 0.0,
            running: true,
            time_since_last_update: 0.0,

            rng,

            game_mode,
            next_game_mode: transition_to,

            prepare_level_state,
            level_complete_state,
            win_game_state,
            game_over_state,

            expiring_messages,

            audio_command_buffer,
            render_command_buffer,

            title_inputs,
            playing_inputs,
            mouse_screen_pos,

            // collision_events: Vec::new(),
            level: 1,
            level_change_delay: 0,

            physics,

            deletion_events,
        }
    }
}

pub enum DeletionEvent {
    Entity { entity: Entity },
    Physics { entity: Entity },
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
