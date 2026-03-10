use glam::Vec2;
use hecs::{Entity, World};
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, BallEater, Block, Bouncy, CTransform, Health, ImpactParticle, ImpactParticleKind,
        InputControlled, LaserShot, OwnedBy, Paddle, Physics, Player, PowerUp, PowerUpDrop,
        PowerUpType, ScorePopup, Shape, StrongBlock, Wall,
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
pub const POWERUP_SHAPE: Vec2 = Vec2::new(8.0, 8.0);
pub const LASER_SHAPE: Vec2 = Vec2::new(2.0, 6.0);

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

pub fn spawn_powerup_drop(ecs: &mut World, pos: Vec2, power_up_type: PowerUpType) {
    ecs.spawn((
        PowerUpDrop,
        PowerUp { power_up_type },
        CTransform {
            pos,
            rot: Vec2::ZERO,
        },
        Physics {
            vel: Vec2::new(0.0, 24.0),
            rot_vel: 0.0,
        },
        Shape {
            dims: POWERUP_SHAPE,
        },
    ));
}

pub fn spawn_laser_shot(ecs: &mut World, pos: Vec2) {
    ecs.spawn((
        LaserShot,
        CTransform {
            pos,
            rot: Vec2::ZERO,
        },
        Physics {
            vel: Vec2::new(0.0, -220.0),
            rot_vel: 0.0,
        },
        Shape { dims: LASER_SHAPE },
    ));
}

pub fn spawn_impact_particle(
    ecs: &mut World,
    pos: Vec2,
    vel: Vec2,
    color: Color,
    size: f32,
    frames_left: u32,
) {
    ecs.spawn((
        ImpactParticle {
            kind: ImpactParticleKind::Square,
            color,
            frames_left,
            max_frames: frames_left.max(1),
            gravity: 0.0,
            drag: 0.88,
            grow_per_frame: 0.0,
        },
        CTransform {
            pos,
            rot: Vec2::ZERO,
        },
        Physics { vel, rot_vel: 0.0 },
        Shape {
            dims: Vec2::splat(size),
        },
    ));
}

pub fn spawn_effect_particle(
    ecs: &mut World,
    pos: Vec2,
    vel: Vec2,
    color: Color,
    size: Vec2,
    frames_left: u32,
    kind: ImpactParticleKind,
    gravity: f32,
    drag: f32,
    grow_per_frame: f32,
) {
    ecs.spawn((
        ImpactParticle {
            kind,
            color,
            frames_left,
            max_frames: frames_left.max(1),
            gravity,
            drag,
            grow_per_frame,
        },
        CTransform {
            pos,
            rot: Vec2::ZERO,
        },
        Physics { vel, rot_vel: 0.0 },
        Shape { dims: size },
    ));
}

pub fn spawn_score_popup(ecs: &mut World, pos: Vec2, value: u32, color: Color) {
    ecs.spawn((
        ScorePopup {
            value,
            color,
            frames_left: 42,
            max_frames: 42,
        },
        CTransform {
            pos,
            rot: Vec2::ZERO,
        },
        Physics {
            vel: Vec2::new(0.0, -10.0),
            rot_vel: 0.0,
        },
    ));
}
