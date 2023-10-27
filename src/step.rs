use glam::Vec2;
use hecs::{Entity, World};
use raylib::RaylibHandle;

use crate::{
    audio_playing::AudioCommand,
    components::{Paddle, Player},
    entity_archetypes::spawn_ball,
    state::{GameMode, LevelCompleteMode, PrepareLevelMode, State},
    systems::{self},
    DIMS,
};

pub fn step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_step(rl, ecs, state);
        }
        GameMode::PrepareLevel => {
            prepare_level_step(rl, ecs, state);
        }
        GameMode::Playing => {
            playing_step(rl, ecs, state);
        }
        GameMode::LevelComplete => {
            level_complete_step(rl, ecs, state);
        }
        GameMode::WinGame => {
            win_game_step(rl, ecs, state);
        }
        GameMode::GameOver => {
            game_over_step(rl, ecs, state);
        }
    }
}

////////////////////////    PER GAME MODE STEPPING     ////////////////////////
pub fn title_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {}

pub fn prepare_level_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    if state.prepare_level_state.countdown > 0 {
        state.prepare_level_state.countdown -= 1;
    }

    systems::playing::rendering::render(ecs, state);

    match state.prepare_level_state.mode {
        PrepareLevelMode::SpawnStuffIn => {
            if state.prepare_level_state.countdown == 0 {
                state.prepare_level_state.mode = PrepareLevelMode::AnnounceLevel;
                state.prepare_level_state.countdown = 60;
                state.audio_command_buffer.push(AudioCommand::LevelStart);
            }
        }
        PrepareLevelMode::AnnounceLevel => {
            if state.prepare_level_state.countdown == 0 {
                state.prepare_level_state.mode = PrepareLevelMode::ShortPause;
                state.prepare_level_state.countdown = 20;
            }
        }
        PrepareLevelMode::ShortPause => {
            if state.prepare_level_state.countdown == 0 {
                state.prepare_level_state.mode = PrepareLevelMode::SpawnBall;
                state.prepare_level_state.countdown = 60;

                // spawn ball
                let pos = Vec2::new(DIMS.x as f32 / 2.0, DIMS.y as f32 * 0.8);

                let vel = Vec2::new(0.0, 20.0);

                let mut paddle_entity: Option<Entity> = None;
                for (entity, _) in ecs.query::<&Paddle>().iter() {
                    paddle_entity = Some(entity);
                }
                if let Some(players_paddle) = paddle_entity {
                    spawn_ball(ecs, state, pos, vel, players_paddle);
                }
            }
        }
        PrepareLevelMode::SpawnBall => {
            if state.prepare_level_state.countdown == 0 {
                state.next_game_mode = Some(GameMode::Playing);
            }
        }
    }
}

pub fn playing_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    if state.level_change_delay > 0 {
        state.level_change_delay -= 1;
    }

    systems::playing::input_processing::process_inputs(ecs, state);
    // systems::playing::physics::boundary_checking(ecs, state);
    systems::playing::physics::sync_ecs_to_physics(ecs, state);
    systems::playing::physics::step_physics(ecs, state);
    systems::playing::physics::damage_blocks(ecs, state);
    systems::playing::rendering::render(ecs, state);
    systems::playing::physics::set_ball_to_angle(ecs, state);
    systems::playing::cleanup::process_deletion_events(ecs, state);
    systems::playing::state_changing::check_for_level_complete(ecs, state);
}

pub fn level_complete_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    if state.level_complete_state.countdown > 0 {
        state.level_complete_state.countdown -= 1;
    }

    systems::playing::rendering::render(ecs, state);

    match state.level_complete_state.mode {
        LevelCompleteMode::Announce => {
            if state.level_complete_state.countdown == 0 {
                state.level_complete_state.mode = LevelCompleteMode::Announce2;
                state.level_complete_state.countdown = 60;
            }
        }
        LevelCompleteMode::Announce2 => {
            if state.level_complete_state.countdown == 0 {
                state.level_complete_state.mode = LevelCompleteMode::Pause;
                state.level_complete_state.countdown = 60;
            }
        }
        LevelCompleteMode::Pause => {
            if state.prepare_level_state.countdown == 0 {
                if state.level < 1 {
                    state.level += 1;
                    state.next_game_mode = Some(GameMode::PrepareLevel);
                } else {
                    state.next_game_mode = Some(GameMode::WinGame);
                }
            }
        }
    }
}

pub fn win_game_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {}

pub fn game_over_step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {}
