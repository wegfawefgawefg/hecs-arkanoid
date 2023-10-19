use glam::Vec2;
use hecs::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CTransform {
    pub pos: Vec2,
    pub rot: Vec2,
}

pub struct Player;

pub struct Ball;

pub struct InputControlled;

pub struct Health {
    pub hp: u32,
}

pub struct VelocityUncapped;

pub struct Physics {
    pub vel: Vec2,
    pub rot_vel: f32,
}

pub struct CaptureInPlayField;

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