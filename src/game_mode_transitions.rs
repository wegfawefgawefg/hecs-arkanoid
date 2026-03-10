use glam::Vec2;
use hecs::World;
use raylib::prelude::Color;

use crate::{
    components::Block,
    entity_archetypes::{spawn_block, spawn_paddle, spawn_walls},
    juice, level_data,
    state::{GameMode, GameOverMode, LevelCompleteMode, PrepareLevelMode, State, WinGameMode},
    DIMS, TS_RATIO,
};

pub fn transition_game_mode(ecs: &mut World, state: &mut State) {
    // TODO: rip out the transition_game_mode abstraction
    if let Some(transition_to) = state.next_game_mode {
        match transition_to {
            GameMode::Title => {
                title_init_state(ecs, state);
            }
            GameMode::PrepareLevel => {
                prepare_level_init_state(ecs, state);
            }
            GameMode::Playing => {
                playing_init_state(ecs, state);
            }
            GameMode::LevelComplete => {
                level_complete_init_state(ecs, state);
            }
            GameMode::WinGame => {
                win_game_init_state(ecs, state);
            }
            GameMode::GameOver => game_over_init_state(ecs, state),
        }
        state.game_mode = transition_to;
        state.next_game_mode = None;
    }
}

////////////////////////    PER GAME MODE STATE TRANSITIONS     ////////////////////////
pub fn title_init_state(ecs: &mut World, _state: &mut State) {
    ecs.clear();
}

pub fn prepare_level_init_state(ecs: &mut World, state: &mut State) {
    state.prepare_level_state.mode = PrepareLevelMode::SpawnStuffIn;
    state.prepare_level_state.countdown = (20.0 * TS_RATIO) as u32;
    state.near_clear_frames = 0;

    ecs.clear();

    spawn_walls(ecs);

    // add players paddle
    let player_pos = Vec2::new(DIMS.x as f32 / 2.0, DIMS.y as f32 * 0.9);
    let _player = spawn_paddle(ecs, player_pos, BASE_PADDLE_SHAPE, Color::WHITE);

    spawn_level(ecs, state.level);
    juice::add_camera_shake(state, 0.8);
    juice::add_zoom_pulse(state, 0.015);
    juice::add_screen_flash(state, 0.08);
}

pub const BASE_PADDLE_SHAPE: Vec2 = Vec2 { x: 30.0, y: 8.0 };
pub fn playing_init_state(_ecs: &mut World, _state: &mut State) {
    println!("playing init");
}

pub fn level_complete_init_state(_ecs: &mut World, state: &mut State) {
    state.level_complete_target_level =
        (state.level + 1).clamp(1, level_data::LEVEL_BLOCK_DATA.len() as u32);
    state.level_complete_state.mode = LevelCompleteMode::Announce;
    state.level_complete_state.countdown = (60.0 * TS_RATIO) as u32;
    juice::add_hitstop(state, 3);
    juice::add_camera_shake(state, 1.5);
    juice::add_zoom_pulse(state, 0.025);
    juice::add_screen_flash(state, 0.12);
}

pub fn win_game_init_state(_ecs: &mut World, state: &mut State) {
    state.win_game_state.mode = WinGameMode::Announce;
    state.win_game_state.countdown = (60.0 * TS_RATIO) as u32;
    juice::add_hitstop(state, 4);
    juice::add_camera_shake(state, 2.0);
    juice::add_zoom_pulse(state, 0.03);
    juice::add_screen_flash(state, 0.16);
}

pub fn game_over_init_state(_ecs: &mut World, state: &mut State) {
    state.game_over_state.mode = GameOverMode::Announce;
    state.game_over_state.countdown = (60.0 * TS_RATIO) as u32;
    juice::add_hitstop(state, 5);
    juice::add_camera_shake(state, 2.2);
    juice::add_zoom_pulse(state, 0.03);
    juice::add_screen_flash(state, 0.18);
}

pub fn delete_all_blocks(ecs: &mut World) {
    let blocks: Vec<_> = ecs
        .query::<(hecs::Entity, &Block)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();
    for block in blocks {
        let _ = ecs.despawn(block);
    }
    ecs.flush();
}

pub fn spawn_level(ecs: &mut World, level: u32) {
    delete_all_blocks(ecs);

    // clamp level between 0 and 35
    let level = level.clamp(1, 36);
    let level_index = (level - 1) as usize;
    let level_data = level_data::LEVEL_BLOCK_DATA[level_index];
    const GAP_SIZE: f32 = 1.0;
    const BLOCK_WIDTH: f32 = 20.0;
    const BLOCK_HEIGHT: f32 = 8.0;
    const BLOCK_SHAPE: Vec2 = Vec2::new(BLOCK_WIDTH, BLOCK_HEIGHT);
    let cursor_x_start = 4.0;
    let mut cursor = Vec2::new(cursor_x_start, 2.0);
    for y in 0..=13 {
        cursor.x = cursor_x_start;
        // advance cursor y by gap
        cursor.y += GAP_SIZE;
        for x in 0..=11 {
            // advance cursor x by gap
            cursor.x += GAP_SIZE;
            if x == 11 {
                break;
            }
            let color_index = level_data[y + 2][x];
            if color_index == 0 {
                cursor.x += BLOCK_WIDTH;
                continue;
            }
            let color = level_data::RL_COLOR_PALETTE[color_index as usize];

            // put a block
            // hp is either 1 or 2 if color_index is 9
            let hp = if color_index == 9 { 2 } else { 1 };
            let ball_unbreakable = color_index == 10;
            spawn_block(ecs, cursor, BLOCK_SHAPE, color, hp, ball_unbreakable);

            // advance cursor x by block width
            cursor.x += BLOCK_WIDTH;

            // skip the block on y == 11
        }
        // advance cursor y by block height
        cursor.y += BLOCK_HEIGHT;
    }
}
