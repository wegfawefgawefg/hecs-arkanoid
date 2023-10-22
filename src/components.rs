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

pub struct Paddle {
    pub size: u32,
}

#[derive(Clone, Copy)]
pub struct Shape {
    pub dims: Vec2,
}

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

pub struct Wall {
    pub color: Color,
}

pub struct HasRigidBody;
pub struct HasSensor;
