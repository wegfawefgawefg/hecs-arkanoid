use crate::{
    audio_playing::AudioCommand,
    components::{Ball, Block, StrongBlock},
    juice,
    state::{GameMode, State, FRAMES_PER_SECOND},
};
use hecs::World;

pub fn check_for_level_complete(ecs: &World, state: &mut State) {
    let breakable_blocks = ecs
        .query::<&Block>()
        .without::<&StrongBlock>()
        .iter()
        .count();

    if breakable_blocks == 0 {
        state.score = state.score.saturating_add(1_000);
        state.next_game_mode = Some(GameMode::LevelComplete);
        state.audio_command_buffer.push(AudioCommand::LevelWin);
        juice::add_hitstop(state, 3);
        juice::add_camera_shake(state, 1.4);
        juice::add_zoom_pulse(state, 0.025);
        juice::add_screen_flash(state, 0.12);
        state.near_clear_frames = 0;
        return;
    }

    if breakable_blocks <= 3 {
        state.near_clear_frames += 1;
        if state.near_clear_frames >= 10 * FRAMES_PER_SECOND {
            state.score = state.score.saturating_add(500);
            state.next_game_mode = Some(GameMode::LevelComplete);
            state.audio_command_buffer.push(AudioCommand::LevelWin);
            juice::add_hitstop(state, 2);
            juice::add_camera_shake(state, 1.0);
            juice::add_zoom_pulse(state, 0.015);
            juice::add_screen_flash(state, 0.08);
            state.near_clear_frames = 0;
        }
    } else {
        state.near_clear_frames = 0;
    }
}

pub fn check_for_level_lost(ecs: &World, state: &mut State) {
    if ecs.query::<&Ball>().iter().next().is_none() {
        state.score = state.score.saturating_sub(250);
        juice::add_hitstop(state, 5);
        juice::add_camera_shake(state, 2.0);
        juice::add_zoom_pulse(state, 0.02);
        juice::add_screen_flash(state, 0.16);
        if state.lives > 1 {
            state.lives -= 1;
            state.reset_powerup_state();
            state.next_game_mode = Some(GameMode::PrepareLevel);
        } else {
            state.lives = 0;
            state.next_game_mode = Some(GameMode::GameOver);
            state.audio_command_buffer.push(AudioCommand::LevelLost);
        }
    }
}
