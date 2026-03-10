use glam::Vec2;
use hecs::World;
use rand::RngExt;

use crate::{
    audio_playing::AudioCommand,
    components::{
        Ball, Block, CTransform, Health, LaserShot, OwnedBy, Paddle, Physics, PowerUp, PowerUpDrop,
        PowerUpType, Shape, StrongBlock,
    },
    entity_archetypes::{spawn_ball, spawn_laser_shot, spawn_powerup_drop},
    state::{DeletionEvent, State, FRAMES_PER_SECOND},
    DIMS,
};

const POWERUP_DROP_CHANCE: f32 = 0.12;
const LASER_COOLDOWN_FRAMES: u32 = 18;

#[derive(Clone, Copy)]
struct Rect {
    pos: Vec2,
    dims: Vec2,
}

impl Rect {
    fn right(self) -> f32 {
        self.pos.x + self.dims.x
    }

    fn bottom(self) -> f32 {
        self.pos.y + self.dims.y
    }

    fn overlaps(self, other: Self) -> bool {
        self.pos.x < other.right()
            && self.right() > other.pos.x
            && self.pos.y < other.bottom()
            && self.bottom() > other.pos.y
    }
}

fn rect_for(pos: Vec2, dims: Vec2) -> Rect {
    Rect { pos, dims }
}

fn roll_powerup() -> PowerUpType {
    let weighted = [
        PowerUpType::Enlarge,
        PowerUpType::Enlarge,
        PowerUpType::SpeedUp,
        PowerUpType::Enlarge,
        PowerUpType::SlowDown,
        PowerUpType::BallSplit,
        PowerUpType::Catch,
        PowerUpType::ExtraLife,
        PowerUpType::Lasers,
        PowerUpType::BombBall,
        PowerUpType::Shrink,
    ];
    let mut rng = rand::rng();
    weighted[rng.random_range(0..weighted.len())]
}

pub fn maybe_spawn_powerup_drop(ecs: &mut World, pos: Vec2) {
    let mut rng = rand::rng();
    if rng.random_range(0.0..1.0) <= POWERUP_DROP_CHANCE {
        spawn_powerup_drop(ecs, pos, roll_powerup());
    }
}

fn apply_powerup(ecs: &mut World, state: &mut State, power_up_type: PowerUpType) {
    match power_up_type {
        PowerUpType::Enlarge => {
            state.paddle_width_scale = (state.paddle_width_scale + 0.35).clamp(0.5, 2.0);
        }
        PowerUpType::Shrink => {
            state.paddle_width_scale = (state.paddle_width_scale - 0.25).clamp(0.5, 2.0);
        }
        PowerUpType::SpeedUp => {
            state.ball_speed_scale = (state.ball_speed_scale + 0.2).clamp(0.6, 2.0);
        }
        PowerUpType::SlowDown => {
            state.ball_speed_scale = (state.ball_speed_scale - 0.2).clamp(0.6, 2.0);
        }
        PowerUpType::BallSplit => {
            let source_balls: Vec<_> = ecs
                .query::<(&Ball, &CTransform, &Physics, &OwnedBy)>()
                .iter()
                .map(|(_, ctransform, physics, owner)| (ctransform.pos, physics.vel, owner.owner))
                .collect();

            for (pos, vel, owner) in source_balls {
                let speed = vel.length().max(80.0);
                spawn_ball(
                    ecs,
                    pos + Vec2::new(-2.0, -2.0),
                    Vec2::new(-speed * 0.9, -speed),
                    owner,
                );
                spawn_ball(
                    ecs,
                    pos + Vec2::new(2.0, -2.0),
                    Vec2::new(speed * 0.9, -speed),
                    owner,
                );
            }
        }
        PowerUpType::Catch => {
            state.sticky_mode = true;
        }
        PowerUpType::ExtraLife => {
            state.lives = state.lives.saturating_add(1);
        }
        PowerUpType::Lasers => {
            state.laser_mode = true;
        }
        PowerUpType::BombBall => {
            state.fireball_mode = true;
        }
    }

    state.score = state.score.saturating_add(25);
    state.audio_command_buffer.push(AudioCommand::PowerUpCatch);
}

pub fn pre_physics(ecs: &mut World, state: &mut State) {
    if state.laser_cooldown > 0 {
        state.laser_cooldown -= 1;
    }

    if !state.laser_mode || !state.playing_inputs.shoot || state.laser_cooldown > 0 {
        return;
    }

    let paddle = ecs
        .query::<(&CTransform, &Shape)>()
        .with::<&Paddle>()
        .iter()
        .next()
        .map(|(ctransform, shape)| (ctransform.pos, shape.dims));

    if let Some((paddle_pos, paddle_dims)) = paddle {
        let left = paddle_pos + Vec2::new(2.0, -6.0);
        let right = paddle_pos + Vec2::new(paddle_dims.x - 4.0, -6.0);
        spawn_laser_shot(ecs, left);
        spawn_laser_shot(ecs, right);
        state.laser_cooldown = LASER_COOLDOWN_FRAMES;
        state.audio_command_buffer.push(AudioCommand::PaddleLaser);
    }
}

pub fn post_physics(ecs: &mut World, state: &mut State) {
    step_powerup_drops(ecs, state);
    step_laser_shots(ecs, state);
}

fn step_powerup_drops(ecs: &mut World, state: &mut State) {
    let dt = 1.0 / FRAMES_PER_SECOND as f32;
    let paddle = ecs
        .query::<(&CTransform, &Shape)>()
        .with::<&Paddle>()
        .iter()
        .next()
        .map(|(ctransform, shape)| rect_for(ctransform.pos, shape.dims));

    let entities: Vec<_> = ecs
        .query::<(hecs::Entity, &PowerUpDrop)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    for entity in entities {
        let mut collected = None;
        let mut remove = false;
        {
            let Ok((ctransform, physics, shape, power_up)) =
                ecs.query_one_mut::<(&mut CTransform, &Physics, &Shape, &PowerUp)>(entity)
            else {
                continue;
            };

            ctransform.pos += physics.vel * dt;

            if ctransform.pos.y > DIMS.y as f32 {
                remove = true;
            } else if let Some(paddle_rect) = paddle {
                if rect_for(ctransform.pos, shape.dims).overlaps(paddle_rect) {
                    collected = Some(power_up.power_up_type);
                    remove = true;
                }
            }
        }

        if remove {
            state.deletion_events.push(DeletionEvent::Entity { entity });
        }

        if let Some(power_up_type) = collected {
            apply_powerup(ecs, state, power_up_type);
        }
    }
}

fn step_laser_shots(ecs: &mut World, state: &mut State) {
    let dt = 1.0 / FRAMES_PER_SECOND as f32;
    let entities: Vec<_> = ecs
        .query::<(hecs::Entity, &LaserShot)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    for entity in entities {
        let Ok((ctransform, physics, shape)) =
            ecs.query_one_mut::<(&mut CTransform, &Physics, &Shape)>(entity)
        else {
            continue;
        };

        ctransform.pos += physics.vel * dt;

        if ctransform.pos.y + shape.dims.y < 0.0 {
            state.deletion_events.push(DeletionEvent::Entity { entity });
            continue;
        }

        let laser_rect = rect_for(ctransform.pos, shape.dims);
        let mut hit_block = None;
        for (block_entity, _, block_transform, block_shape) in ecs
            .query::<(hecs::Entity, &Block, &CTransform, &Shape)>()
            .iter()
        {
            if laser_rect.overlaps(rect_for(block_transform.pos, block_shape.dims)) {
                hit_block = Some((block_entity, block_transform.pos));
                break;
            }
        }

        let Some((block_entity, block_pos)) = hit_block else {
            continue;
        };

        state.deletion_events.push(DeletionEvent::Entity { entity });

        if ecs.satisfies::<&StrongBlock>(block_entity) {
            state.score = state.score.saturating_add(5);
            state
                .audio_command_buffer
                .push(AudioCommand::BallSturdyBlockBounce);
            continue;
        }

        let mut destroyed = false;
        if let Ok((_block, health)) = ecs.query_one_mut::<(&Block, &mut Health)>(block_entity) {
            state.score = state.score.saturating_add(10);
            if health.hp > 0 {
                health.hp -= 1;
            }
            destroyed = health.hp == 0;
        }

        if destroyed {
            state.score = state.score.saturating_add(90);
            maybe_spawn_powerup_drop(ecs, block_pos + Vec2::new(6.0, 4.0));
            state.deletion_events.push(DeletionEvent::Entity {
                entity: block_entity,
            });
            state
                .audio_command_buffer
                .push(AudioCommand::BallBlockBounce);
        } else {
            state
                .audio_command_buffer
                .push(AudioCommand::BallSturdyBlockBounce);
        }
    }
}
