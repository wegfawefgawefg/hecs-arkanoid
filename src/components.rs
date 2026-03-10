#![allow(dead_code)]

use glam::Vec2;
use hecs::Entity;
use raylib::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTransform {
    pub pos: Vec2,
    pub rot: Vec2,
}

pub struct Player;

pub struct Ball;
pub struct Bouncy;

pub struct InputControlled;

pub struct Block {
    pub color: Color,
}

pub struct Health {
    pub hp: u32,
}

pub struct StrongBlock;

pub struct Paddle {
    pub size: u32,
}

#[derive(Clone, Copy)]
pub struct Shape {
    pub dims: Vec2,
}

#[derive(Clone, Copy)]
pub struct Physics {
    pub vel: Vec2,
    pub rot_vel: f32,
}

pub struct CaptureInPlayField;

pub struct FreeToLeavePlayField;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    pub owner: Entity,
    pub score: u32,
}

pub struct OwnedBy {
    pub owner: Entity,
}

pub struct AttachedTo {
    pub entity: Entity,
    pub offset: Vec2,
}

#[derive(Clone, Copy)]
pub struct GrabZone {
    pub radius: f32,
}

pub struct Attachable;

pub struct WantsToGoTo {
    pub pos: Vec2,
}

pub struct LookAt {
    pub entity: Entity,
}

#[derive(Clone, Copy)]
pub struct Enemy;

pub struct Wall;
pub struct BallEater;

pub struct HasRigidBody;
pub struct HasSensor;

pub struct VelocityManaged;
pub struct PositionManaged;

#[derive(Clone, Copy)]
pub struct PowerUpDrop;

#[derive(Clone, Copy)]
pub struct LaserShot;

#[derive(Clone, Copy)]
pub enum ImpactParticleKind {
    Square,
    Smoke,
    Ember,
    LaserStreak,
    Shard,
    Melt,
}

#[derive(Clone, Copy)]
pub struct ImpactParticle {
    pub kind: ImpactParticleKind,
    pub color: Color,
    pub frames_left: u32,
    pub max_frames: u32,
    pub gravity: f32,
    pub drag: f32,
    pub grow_per_frame: f32,
}

#[derive(Clone, Copy)]
pub struct ScorePopup {
    pub value: u32,
    pub color: Color,
    pub frames_left: u32,
    pub max_frames: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PowerUpType {
    Enlarge,
    Shrink,

    SpeedUp,
    SlowDown,

    BallSplit,
    Catch,
    ExtraLife,

    Lasers,
    BombBall,
}

pub struct PowerUp {
    pub power_up_type: PowerUpType,
}
