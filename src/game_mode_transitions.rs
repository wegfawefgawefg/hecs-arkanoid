use glam::Vec2;
use hecs::World;
use raylib::prelude::Color;

use crate::{
    components::{Block, CTransform, InputControlled, Paddle, Physics, Player, Shape},
    level_data,
    state::{GameMode, State},
    DIMS,
};

pub fn transition_game_mode(ecs: &mut World, state: &mut State) {
    if let Some(transition_to) = state.next_game_mode {
        match transition_to {
            GameMode::Title => {
                title_init_state(ecs, state);
            }
            GameMode::Playing => {
                playing_init_state(ecs, state);
            }
            GameMode::GameOver => game_over_init_state(ecs, state),
        }
    }
}

////////////////////////    PER GAME MODE STATE TRANSITIONS     ////////////////////////
pub fn title_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();

    state.game_mode = GameMode::Title;
    state.next_game_mode = None;
}

const BASE_PADDLE_SHAPE: Vec2 = Vec2 { x: 20.0, y: 10.0 };
pub fn playing_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();

    // add players paddle
    ecs.spawn((
        CTransform {
            pos: Vec2::new(100.0, DIMS.y as f32 * 0.9),
            rot: Vec2::new(0.0, 1.0),
        },
        Physics {
            vel: Vec2::ZERO,
            rot_vel: 0.0,
        },
        InputControlled,
        Player,
        Paddle { size: 1 },
        Shape {
            dims: BASE_PADDLE_SHAPE,
        },
    ));

    state.game_mode = GameMode::Playing;
    state.next_game_mode = None;
}

pub fn game_over_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();
    state.game_mode = GameMode::GameOver;
    state.next_game_mode = None;
}

pub fn spawn_level(ecs: &mut World, state: &mut State, level: u32) {
    // clear all blocks
    let blocks: Vec<_> = ecs
        .query::<&Block>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();
    for block in blocks {
        let _ = ecs.despawn(block);
    }
    ecs.flush();

    // clamp level between 0 and 35
    let level = level.clamp(0, 35);
    let level_data = level_data::LEVEL_BLOCK_DATA[level as usize];
    const GAP_SIZE: f32 = 1.0;
    const BLOCK_WIDTH: f32 = 20.0;
    const BLOCK_HEIGHT: f32 = 8.0;
    let cursor_x_start = 4.0;
    let mut cursor = Vec2::new(cursor_x_start, 2.0);
    for y in 0..=24 {
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
            ecs.spawn((
                CTransform {
                    pos: cursor,
                    rot: Vec2::new(0.0, 1.0),
                },
                Shape {
                    dims: Vec2 {
                        x: BLOCK_WIDTH,
                        y: BLOCK_HEIGHT,
                    },
                },
                Block { color },
            ));

            // advance cursor x by block width
            cursor.x += BLOCK_WIDTH;

            // skip the block on y == 11
        }
        // advance cursor y by block height
        cursor.y += BLOCK_HEIGHT;
    }
}
