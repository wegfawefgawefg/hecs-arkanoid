use glam::Vec2;
use rand::{rngs::StdRng, SeedableRng};

use crate::{
    audio_playing::AudioCommandBuffer,
    input_processing::{PlayingInputs, TitleInputs},
    message_stream::ExpiringMessages,
    rendering::RenderCommandBuffer,
};

pub const FRAMES_PER_SECOND: u32 = 60;

#[derive(Clone, Copy)]
pub enum GameMode {
    Title,
    Playing,
    GameOver,
}

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    pub rng: StdRng,

    pub game_mode: GameMode,
    pub next_game_mode: Option<GameMode>,

    pub expiring_messages: ExpiringMessages,

    pub audio_command_buffer: AudioCommandBuffer,
    pub render_command_buffer: RenderCommandBuffer,

    pub title_inputs: TitleInputs,
    pub playing_inputs: PlayingInputs,
    pub mouse_screen_pos: Vec2,
}

impl State {
    pub fn new() -> Self {
        let rng: StdRng = StdRng::from_entropy();

        let game_mode = GameMode::Title;
        let transition_to: Option<GameMode> = None;

        let expiring_messages = ExpiringMessages::new();

        let render_command_buffer: RenderCommandBuffer = RenderCommandBuffer::new();
        let audio_command_buffer: AudioCommandBuffer = AudioCommandBuffer::new();

        let title_inputs = TitleInputs::new();
        let playing_inputs = PlayingInputs::new();
        let mouse_screen_pos = Vec2::ZERO;

        Self {
            running: true,
            time_since_last_update: 0.0,

            rng,

            game_mode,
            next_game_mode: transition_to,

            expiring_messages,

            audio_command_buffer,
            render_command_buffer,

            title_inputs,
            playing_inputs,
            mouse_screen_pos,
        }
    }
}