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

const BASE_PADDLE_SHAPE: Vec2 = Vec2 { x: 20.0, y: 8.0 };
pub fn playing_init_state(ecs: &mut World, state: &mut State) {
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

    // spawn a player physics body
    // let player_collider =
    //     ColliderBuilder::cuboid(BASE_PADDLE_SHAPE.x / 2.0, BASE_PADDLE_SHAPE.y / 2.0);
    // let player_rigid_body = RigidBodyBuilder::dynamic()
    //     .translation(Vector::new(player_pos.x, player_pos.y))
    //     .rotation(player_rot.x)
    //     .build();
    // let player_body_handle = state.physics.rigid_body_set.insert(player_rigid_body);
    // state.physics.collider_set.insert_with_parent(
    //     player_collider,
    //     player_body_handle,
    //     &mut state.physics.rigid_body_set,
    // );

    // spawn ball

    // spawn a few random balls
    for _ in 0..1 {
        let pos = Vec2::new(
            rand::thread_rng().gen_range(0.0..DIMS.x as f32),
            rand::thread_rng().gen_range(0.0..DIMS.y as f32),
        );

        const VEL_MAX: f32 = 200.0;
        let vel = Vec2::new(
            rand::thread_rng().gen_range(-VEL_MAX..VEL_MAX),
            rand::thread_rng().gen_range(-VEL_MAX..VEL_MAX),
        );

        // let pos = DIMS.as_vec2() / 2.0;
        // let vel = Vec2::new(0.0, m2p(-1.0));

        spawn_ball(ecs, state, pos, vel, player)
    }

    // state.level = 1;
    // spawn_level(ecs, state, state.level);

    state.game_mode = GameMode::Playing;
    state.next_game_mode = None;
}

pub fn game_over_init_state(ecs: &mut World, state: &mut State) {
    ecs.clear();
    state.game_mode = GameMode::GameOver;
    state.next_game_mode = None;
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
    let level = level.clamp(0, 35);
    let level_data = level_data::LEVEL_BLOCK_DATA[level as usize];
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
            spawn_block(ecs, state, cursor, BLOCK_SHAPE, color);

            // advance cursor x by block width
            cursor.x += BLOCK_WIDTH;

            // skip the block on y == 11
        }
        // advance cursor y by block height
        cursor.y += BLOCK_HEIGHT;
    }
}
