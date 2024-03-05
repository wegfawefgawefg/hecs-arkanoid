use glam::Vec2;
use hecs::{Entity, World};
use raylib::{prelude::Vector2, RaylibHandle};

use crate::{
    audio_playing::AudioCommand,
    components::{Paddle, Player},
    entity_archetypes::spawn_ball,
    state::{GameMode, GameOverMode, LevelCompleteMode, PrepareLevelMode, State, WinGameMode},
    systems::{self},
    DIMS, TS_RATIO, WINDOW_DIMS,
};

pub fn step(rl: &mut RaylibHandle, ecs: &mut World, state: &mut State) {
    match state.game_mode {
        GameMode::Title => {
            title_step(state, ecs);
        }
        GameMode::PrepareLevel => {
            prepare_level_step(rl, state, ecs);
        }
        GameMode::Playing => {
            playing_step(state, ecs);
        }
        GameMode::LevelComplete => {
            level_complete_step(state, ecs);
        }
        GameMode::WinGame => {
            win_game_step(state, ecs);
        }
        GameMode::GameOver => {
            game_over_step(state, ecs);
        }
    }
}

////////////////////////    PER GAME MODE STEPPING     ////////////////////////
pub fn title_step(state: &mut State, ecs: &mut World) {}

pub fn prepare_level_step(rl: &mut RaylibHandle, state: &mut State, ecs: &mut World) {
    if state.prepare_level_state.countdown > 0 {
        state.prepare_level_state.countdown -= 1;
    }

    systems::playing::rendering::render(ecs, state);

    match state.prepare_level_state.mode {
        PrepareLevelMode::SpawnStuffIn => {
            if state.prepare_level_state.countdown == 0 {
                state.prepare_level_state.mode = PrepareLevelMode::AnnounceLevel;
                state.prepare_level_state.countdown = (60.0 * TS_RATIO) as u32;
                state.audio_command_buffer.push(AudioCommand::LevelStart);
            }
        }
        PrepareLevelMode::AnnounceLevel => {
            if state.prepare_level_state.countdown == 0 {
                state.prepare_level_state.mode = PrepareLevelMode::ShortPause;
                state.prepare_level_state.countdown = (20.0 * TS_RATIO) as u32;
            }
        }
        PrepareLevelMode::ShortPause => {
            if state.prepare_level_state.countdown == 0 {
                state.prepare_level_state.mode = PrepareLevelMode::SpawnBall;
                state.prepare_level_state.countdown = (20.0 * TS_RATIO) as u32;

                let mut paddle_entity: Option<Entity> = None;
                for (entity, _) in ecs.query::<&Paddle>().iter() {
                    paddle_entity = Some(entity);
                }

                // spawn balls
                for i in 0..2 {
                    let pos = Vec2::new(DIMS.x as f32 / 2.0 + i as f32 * 4.0, DIMS.y as f32 * 0.8);
                    let vel = Vec2::new(0.0, -20.0);
                    if let Some(players_paddle) = paddle_entity {
                        spawn_ball(ecs, state, pos, vel, players_paddle);
                    }
                }
            }
        }
        PrepareLevelMode::SpawnBall => {
            if state.prepare_level_state.countdown == 0 {
                // set mouse position
                let center = (*WINDOW_DIMS).as_vec2();
                rl.set_mouse_position(Vector2::new(center.x / 2.0, center.y));
                state.next_game_mode = Some(GameMode::Playing);
            }
        }
    }
}

pub fn playing_step(state: &mut State, ecs: &mut World) {
    if state.level_change_delay > 0 {
        state.level_change_delay -= 1;
    }

    // systems::playing::physics::constantly_resize_paddle(ecs, state);

    systems::playing::input_processing::process_inputs(ecs, state);
    // systems::playing::physics::boundary_checking(ecs, state);

    // all reshaping needs to happen before the ecs is synced to physics

    systems::playing::physics::set_ball_to_angle(ecs, state);
    systems::playing::physics::sync_ecs_to_physics(ecs, state);
    systems::playing::physics::step_physics(ecs, state);
    systems::playing::physics::respond_to_collisions(ecs, state);
    systems::playing::cleanup::process_deletion_events(ecs, state);
    systems::playing::state_changing::check_for_level_complete(ecs, state);
    systems::playing::state_changing::check_for_level_lost(ecs, state);
    systems::playing::rendering::render(ecs, state);
}

pub fn level_complete_step(state: &mut State, ecs: &mut World) {
    if state.level_complete_state.countdown > 0 {
        state.level_complete_state.countdown -= 1;
    }

    systems::playing::rendering::render(ecs, state);

    match state.level_complete_state.mode {
        LevelCompleteMode::Announce => {
            if state.level_complete_state.countdown == 0 {
                state.level_complete_state.mode = LevelCompleteMode::Announce2;
                state.level_complete_state.countdown = (40.0 * TS_RATIO) as u32;
            }
        }
        LevelCompleteMode::Announce2 => {
            if state.level_complete_state.countdown == 0 {
                state.level_complete_state.mode = LevelCompleteMode::Pause;
                state.level_complete_state.countdown = (40.0 * TS_RATIO) as u32;
            }
        }
        LevelCompleteMode::Pause => {
            if state.prepare_level_state.countdown == 0 {
                state.level += 1;
                state.next_game_mode = Some(GameMode::PrepareLevel);
            }
        }
    }
}

pub fn win_game_step(state: &mut State, ecs: &mut World) {
    if state.win_game_state.countdown > 0 {
        state.win_game_state.countdown -= 1;
    }

    systems::playing::rendering::render(ecs, state);

    match state.win_game_state.mode {
        WinGameMode::Announce => {
            if state.win_game_state.countdown == 0 {
                state.win_game_state.mode = WinGameMode::Announce2;
                state.win_game_state.countdown = (40.0 * TS_RATIO) as u32;
            }
        }
        WinGameMode::Announce2 => {
            if state.win_game_state.countdown == 0 {
                state.win_game_state.mode = WinGameMode::Pause;
                state.win_game_state.countdown = (40.0 * TS_RATIO) as u32;
            }
        }
        WinGameMode::Pause => {
            if state.prepare_level_state.countdown == 0 {
                state.next_game_mode = Some(GameMode::Title);
            }
        }
    }
}

pub fn game_over_step(state: &mut State, ecs: &mut World) {
    if state.game_over_state.countdown > 0 {
        state.game_over_state.countdown -= 1;
    }

    systems::playing::rendering::render(ecs, state);

    match state.game_over_state.mode {
        GameOverMode::Announce => {
            if state.game_over_state.countdown == 0 {
                state.game_over_state.mode = GameOverMode::Announce2;
                state.game_over_state.countdown = (40.0 * TS_RATIO) as u32;
            }
        }
        GameOverMode::Announce2 => {
            if state.game_over_state.countdown == 0 {
                state.game_over_state.mode = GameOverMode::Pause;
                state.game_over_state.countdown = (40.0 * TS_RATIO) as u32;
            }
        }
        GameOverMode::Pause => {
            if state.prepare_level_state.countdown == 0 {
                state.next_game_mode = Some(GameMode::Title);
            }
        }
    }
}
