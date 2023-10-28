use glam::Vec2;
use hecs::{Entity, World};
use nalgebra::vector;
use rapier2d::prelude::{ActiveEvents, ColliderBuilder, Point, RigidBodyBuilder};
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, Block, Bouncy, CTransform, HasRigidBody, Health, InputControlled, OwnedBy, Paddle,
        Physics, Player, PositionManaged, Shape, VelocityManaged, Wall,
    },
    physics_engine::p2m,
    state::State,
    DIMS,
};

pub fn spawn_walls(ecs: &mut World, state: &mut State) {
    println!("Spawning walls");
    let wall_color = Color::RAYWHITE;
    let wall_thickness = 20.0;
    // top wall
    let top_wall = ecs.spawn((
        CTransform {
            pos: Vec2::new(0.0, 0.0),
            rot: Vec2::new(0.0, 0.0),
        },
        Shape {
            dims: Vec2 {
                x: DIMS.x as f32,
                y: wall_thickness,
            },
        },
        Wall { color: wall_color },
        HasRigidBody,
    ));
    let top_wall_collider =
        ColliderBuilder::cuboid(p2m(DIMS.x as f32) / 2.0, p2m(wall_thickness) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let top_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![
            p2m(DIMS.x as f32 / 2.0),
            -p2m(wall_thickness / 2.0)
        ])
        .can_sleep(false)
        .build();
    let top_wall_body_handle = state.physics.rigid_body_set.insert(top_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        top_wall_collider,
        top_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(top_wall, top_wall_body_handle);

    // bottom wall
    let bottom_wall = ecs.spawn((
        CTransform {
            pos: Vec2::new(0.0, DIMS.y as f32 - 1.0),
            rot: Vec2::new(0.0, 0.0),
        },
        Shape {
            dims: Vec2 {
                x: DIMS.x as f32,
                y: 0.0,
            },
        },
        Wall { color: wall_color },
    ));
    let bottom_wall_collider =
        ColliderBuilder::cuboid(p2m(DIMS.x as f32) / 2.0, p2m(wall_thickness) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let bottom_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![
            p2m(DIMS.x as f32 / 2.0),
            p2m(DIMS.y as f32 + wall_thickness / 2.0)
        ])
        .can_sleep(false)
        .build();
    let bottom_wall_body_handle = state.physics.rigid_body_set.insert(bottom_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        bottom_wall_collider,
        bottom_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(bottom_wall, bottom_wall_body_handle);

    // left wall
    let left_wall = ecs.spawn((
        CTransform {
            pos: Vec2::new(0.0, 0.0),
            rot: Vec2::new(0.0, 0.0),
        },
        Shape {
            dims: Vec2 {
                x: 1.0,
                y: DIMS.y as f32,
            },
        },
        Wall { color: wall_color },
    ));
    let left_wall_collider =
        ColliderBuilder::cuboid(p2m(wall_thickness) / 2.0, p2m(DIMS.y as f32) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let left_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![
            p2m(-wall_thickness / 2.0),
            p2m(DIMS.y as f32 / 2.0)
        ])
        .can_sleep(false)
        .build();
    let left_wall_body_handle = state.physics.rigid_body_set.insert(left_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        left_wall_collider,
        left_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(left_wall, left_wall_body_handle);

    // right wall
    let right_wall = ecs.spawn((
        CTransform {
            pos: Vec2::new(DIMS.x as f32 - 1.0, 0.0),
            rot: Vec2::new(0.0, 0.0),
        },
        Shape {
            dims: Vec2 {
                x: 1.0,
                y: DIMS.y as f32,
            },
        },
        Wall { color: wall_color },
    ));
    let right_wall_collider =
        ColliderBuilder::cuboid(p2m(wall_thickness) / 2.0, p2m(DIMS.y as f32) / 2.0)
            .restitution(1.0)
            .friction(0.0)
            .build();
    let right_wall_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![
            p2m(DIMS.x as f32 + wall_thickness / 2.0),
            p2m(DIMS.y as f32 / 2.0)
        ])
        .can_sleep(false)
        .build();
    let right_wall_body_handle = state.physics.rigid_body_set.insert(right_wall_rigid_body);
    state.physics.collider_set.insert_with_parent(
        right_wall_collider,
        right_wall_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(right_wall, right_wall_body_handle);
}

pub fn spawn_ball(ecs: &mut World, state: &mut State, pos: Vec2, vel: Vec2, owner: Entity) {
    let ball_entity = ecs.spawn((
        Ball,
        CTransform {
            pos,
            rot: Vec2::new(0.0, 0.0),
        },
        Physics { vel, rot_vel: 0.0 },
        OwnedBy { owner },
        Shape {
            dims: Vec2::new(4.0, 4.0),
        },
        Bouncy,
        HasRigidBody,
        VelocityManaged,
    ));
    // let ball_collider = ColliderBuilder::ball(p2m(8.0) / 2.0)
    let ball_collider = ColliderBuilder::cuboid(p2m(4.0) / 2.0, p2m(4.0) / 2.0)
        .restitution(1.0)
        .friction(0.0)
        .mass(0.0001)
        .active_events(ActiveEvents::COLLISION_EVENTS)
        .build();
    let ball_rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![p2m(pos.x), p2m(pos.y)])
        .linvel(vector![p2m(vel.x), p2m(vel.y)])
        .lock_rotations()
        .linear_damping(0.0)
        .angular_damping(0.0)
        .can_sleep(false)
        .ccd_enabled(true)
        .build();
    let ball_body_handle = state.physics.rigid_body_set.insert(ball_rigid_body);
    state.physics.collider_set.insert_with_parent(
        ball_collider,
        ball_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(ball_entity, ball_body_handle);
}

pub fn spawn_block(
    ecs: &mut World,
    state: &mut State,
    pos: Vec2,
    shape: Vec2,
    color: Color,
    hp: u32,
) {
    let block_entity = ecs.spawn((
        CTransform {
            pos,
            rot: Vec2::new(0.0, 1.0),
        },
        Shape { dims: shape },
        Block { color },
        Health { hp },
        HasRigidBody,
    ));

    let block_collider = ColliderBuilder::cuboid(p2m(shape.x) / 2.0, p2m(shape.y) / 2.0)
        .restitution(1.0)
        .friction(0.0)
        .build();
    let block_rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![
            p2m(pos.x + shape.x / 2.0),
            p2m(pos.y + shape.y / 2.0)
        ])
        .can_sleep(false)
        .build();

    let block_body_handle = state.physics.rigid_body_set.insert(block_rigid_body);
    state.physics.collider_set.insert_with_parent(
        block_collider,
        block_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(block_entity, block_body_handle);
}

pub fn spawn_paddle(
    ecs: &mut World,
    state: &mut State,
    pos: Vec2,
    shape: Vec2,
    color: Color,
) -> Entity {
    let paddle_entity = ecs.spawn((
        CTransform {
            pos,
            rot: Vec2::new(0.0, 0.0),
        },
        Physics {
            vel: Vec2::ZERO,
            rot_vel: 0.0,
        },
        InputControlled,
        Player,
        Paddle { size: 1 },
        Shape { dims: shape },
        HasRigidBody,
        PositionManaged,
    ));

    let paddle_collider = ColliderBuilder::cuboid(p2m(shape.x) / 2.0, p2m(shape.y) / 2.0)
        .restitution(1.0)
        .build();
    let paddle_rigid_body = RigidBodyBuilder::kinematic_position_based()
        .translation(vector![
            p2m(pos.x + shape.x / 2.0),
            p2m(pos.y + shape.y / 2.0)
        ])
        .can_sleep(false)
        .build();

    let paddle_body_handle = state.physics.rigid_body_set.insert(paddle_rigid_body);
    state.physics.collider_set.insert_with_parent(
        paddle_collider,
        paddle_body_handle,
        &mut state.physics.rigid_body_set,
    );
    state
        .physics
        .set_rigid_body_mapping(paddle_entity, paddle_body_handle);
    paddle_entity
}
