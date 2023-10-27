use glam::Vec2;
use hecs::World;
use rand::Rng;
use rapier2d::prelude::*;
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, Block, Bouncy, CTransform, InputControlled, OwnedBy, Paddle, Physics, Player, Shape,
        Wall,
    },
    entity_archetypes::{spawn_ball, spawn_block, spawn_paddle, spawn_walls},
    level_data,
    physics_engine::{m2p, p2m, PhysicsEngine},
    state::{GameMode, LevelCompleteMode, PrepareLevelMode, State},
    DIMS,
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
pub fn title_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();
}

pub fn prepare_level_init_state(ecs: &mut World, state: &mut State) {
    state.prepare_level_state.mode = PrepareLevelMode::SpawnStuffIn;
    state.prepare_level_state.countdown = 1 * 60;

    ecs.clear();
    state.physics = PhysicsEngine::new();

    spawn_walls(ecs, state);

    // add players paddle
    let player_pos = Vec2::new(100.0, DIMS.y as f32 * 0.9);
    let player = spawn_paddle(
        ecs,
        state,
        player_pos,
        BASE_PADDLE_SHAPE,
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        },
    );

    spawn_level(ecs, state, state.level);
}

const BASE_PADDLE_SHAPE: Vec2 = Vec2 { x: 20.0, y: 8.0 };
pub fn playing_init_state(ecs: &mut World, state: &mut State) {
    println!("playing init");
}

pub fn level_complete_init_state(ecs: &mut World, state: &mut State) {
    state.level_complete_state.mode = LevelCompleteMode::Announce;
    state.level_complete_state.countdown = 60;
}

pub fn win_game_init_state(ecs: &mut World, state: &mut State) {}

pub fn game_over_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();
}

pub fn delete_all_blocks(ecs: &mut World, state: &mut State) {
    let blocks: Vec<_> = ecs
        .query::<&Block>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();
    for block in blocks {
        // remove all rigid bodies
        if let Some(rigid_body_handle) = state.physics.get_rigid_body_handle(block) {
            state.physics.rigid_body_set.remove(
                rigid_body_handle,
                &mut state.physics.island_manager,
                &mut state.physics.collider_set,
                &mut state.physics.impulse_joint_set,
                &mut state.physics.multibody_joint_set,
                true,
            );
        }

        // remove from physics
        // if let Some(collider_handle) = state.physics.get_collider_handle(block) {
        //     state.physics.collider_set.remove(
        //         collider_handle,
        //         &mut state.physics.island_manager,
        //         &mut state.physics.rigid_body_set,
        //         true,
        //     );
        // }

        let _ = ecs.despawn(block);
    }
    ecs.flush();
}

pub fn spawn_level(ecs: &mut World, state: &mut State, level: u32) {
    delete_all_blocks(ecs, state);

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
            spawn_block(ecs, state, cursor, BLOCK_SHAPE, color, hp);

            // advance cursor x by block width
            cursor.x += BLOCK_WIDTH;

            // skip the block on y == 11
        }
        // advance cursor y by block height
        cursor.y += BLOCK_HEIGHT;
    }
}
