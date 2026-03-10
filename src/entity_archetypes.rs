use glam::Vec2;
use hecs::{Entity, World};
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, BallEater, Block, Bouncy, CTransform, Health, InputControlled, OwnedBy, Paddle,
        Physics, Player, Shape, StrongBlock, Wall,
    },
    DIMS,
};

pub fn spawn_walls(ecs: &mut World) {
    let wall_thickness = 20.0;

    ecs.spawn((
        CTransform {
            pos: Vec2::new(0.0, -wall_thickness + 1.0),
            rot: Vec2::ZERO,
        },
        Shape {
            dims: Vec2::new(DIMS.x as f32, wall_thickness),
        },
        Wall,
    ));

    ecs.spawn((
        CTransform {
            pos: Vec2::new(0.0, DIMS.y as f32 - 1.0),
            rot: Vec2::ZERO,
        },
        Shape {
            dims: Vec2::new(DIMS.x as f32, wall_thickness),
        },
        Wall,
        BallEater,
    ));

    ecs.spawn((
        CTransform {
            pos: Vec2::new(-wall_thickness + 1.0, 0.0),
            rot: Vec2::ZERO,
        },
        Shape {
            dims: Vec2::new(wall_thickness, DIMS.y as f32),
        },
        Wall,
    ));

    ecs.spawn((
        CTransform {
            pos: Vec2::new(DIMS.x as f32 - 1.0, 0.0),
            rot: Vec2::ZERO,
        },
        Shape {
            dims: Vec2::new(wall_thickness, DIMS.y as f32),
        },
        Wall,
    ));
}

pub const BALL_SHAPE: Vec2 = Vec2::new(4.0, 4.0);

pub fn spawn_ball(ecs: &mut World, pos: Vec2, vel: Vec2, owner: Entity) {
    ecs.spawn((
        Ball,
        CTransform {
            pos,
            rot: Vec2::ZERO,
        },
        Physics { vel, rot_vel: 0.0 },
        OwnedBy { owner },
        Shape { dims: BALL_SHAPE },
        Bouncy,
    ));
}

pub fn spawn_block(
    ecs: &mut World,
    pos: Vec2,
    shape: Vec2,
    color: Color,
    hp: u32,
    ball_unbreakable: bool,
) {
    let block_entity = ecs.spawn((
        CTransform {
            pos,
            rot: Vec2::new(0.0, 1.0),
        },
        Shape { dims: shape },
        Block { color },
        Health { hp },
    ));

    if ball_unbreakable {
        ecs.insert_one(block_entity, StrongBlock).unwrap();
    }
}

pub fn spawn_paddle(ecs: &mut World, pos: Vec2, shape: Vec2, _color: Color) -> Entity {
    ecs.spawn((
        CTransform {
            pos,
            rot: Vec2::ZERO,
        },
        Physics {
            vel: Vec2::ZERO,
            rot_vel: 0.0,
        },
        InputControlled,
        Player,
        Paddle { size: 1 },
        Shape { dims: shape },
    ))
}
