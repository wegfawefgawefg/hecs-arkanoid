use glam::Vec2;
use hecs::World;

use crate::audio_playing::AudioCommand;
use crate::components::{
    Ball, BallEater, Block, CTransform, FreeToLeavePlayField, HasRigidBody, Health, Paddle,
    Physics, PositionManaged, Shape, StrongBlock, VelocityManaged, Wall,
};
use crate::game_mode_transitions::BASE_PADDLE_SHAPE;
use crate::physics_engine::{m2p, p2m};
use crate::state::{DeletionEvent, State};
use crate::{DIMS, TS_RATIO};

pub fn sync_ecs_to_physics(ecs: &World, state: &mut State) {
    for (entity, physics) in ecs
        .query::<(hecs::Entity, &mut Physics)>()
        .with::<(&HasRigidBody, &VelocityManaged)>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
                let vel = Vec2::new(p2m(physics.vel.x), p2m(physics.vel.y));
                rigid_body.set_linvel(vel, true);
            }
        }
    }

    for (entity, ctransform, shape) in ecs
        .query::<(hecs::Entity, &CTransform, &Shape)>()
        .with::<(&HasRigidBody, &PositionManaged)>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
                let center = ctransform.pos + shape.dims / 2.0;
                rigid_body.set_translation(Vec2::new(p2m(center.x), p2m(center.y)), true);
            }
        }
    }
}

const ANGLE_45_IN_RAD: f32 = std::f32::consts::PI / 3.0;
const BALL_VEL: f32 = 200.0 * (1.0 / TS_RATIO);

pub fn set_ball_to_angle(ecs: &World, state: &mut State) {
    for (entity, physics) in ecs
        .query::<(hecs::Entity, &mut Physics)>()
        .with::<(&HasRigidBody, &Ball)>()
        .iter()
    {
        if state.physics.get_rigid_body_handle(entity).is_some() {
            let x_sign = physics.vel.x.signum();
            let y_sign = physics.vel.y.signum();
            physics.vel.x = ANGLE_45_IN_RAD.cos() * BALL_VEL * x_sign;
            physics.vel.y = ANGLE_45_IN_RAD.sin() * BALL_VEL * y_sign;
        }
    }
}

pub fn step_physics(ecs: &World, state: &mut State) {
    state.physics.step();

    for (entity, ctransform, shape) in ecs
        .query::<(hecs::Entity, &mut CTransform, &Shape)>()
        .with::<&HasRigidBody>()
        .without::<&PositionManaged>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                let center = rigid_body.translation();
                let rot = rigid_body.rotation().angle();
                ctransform.pos = Vec2::new(
                    m2p(center.x) - shape.dims.x / 2.0,
                    m2p(center.y) - shape.dims.y / 2.0,
                );
                ctransform.rot = Vec2::new(rot.cos(), rot.sin());
            }
        }
    }

    for (entity, physics) in ecs
        .query::<(hecs::Entity, &mut Physics)>()
        .with::<&HasRigidBody>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                let vel = rigid_body.linvel();
                physics.vel = Vec2::new(m2p(vel.x), m2p(vel.y));
            }
        }
    }

    for (entity, ctransform, shape) in ecs
        .query::<(hecs::Entity, &mut CTransform, &Shape)>()
        .with::<&Paddle>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                let center = rigid_body.translation();
                ctransform.pos.x = m2p(center.x) - shape.dims.x / 2.0;
                ctransform.pos.y = m2p(center.y) - shape.dims.y / 2.0;
            }
        }
    }

    state.physics.collision_events.clear();
    while let Ok(event) = state.physics.collision_recv.try_recv() {
        state.physics.collision_events.push(event);
    }
}

#[allow(dead_code)]
pub fn constantly_resize_paddle(ecs: &mut World, state: &mut State) {
    let new_shape = Vec2::new(
        BASE_PADDLE_SHAPE.x * (1.0 + (state.t * 0.1).sin() / 2.0) + 10.0,
        BASE_PADDLE_SHAPE.y,
    );
    for (entity, shape) in ecs
        .query::<(hecs::Entity, &mut Shape)>()
        .with::<&Paddle>()
        .iter()
    {
        shape.dims = new_shape;

        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                for collider_handle in rigid_body.colliders() {
                    if let Some(collider) = state.physics.collider_set.get_mut(*collider_handle) {
                        collider.set_shape(rapier2d::geometry::ColliderShape::cuboid(
                            p2m(new_shape.x / 2.0),
                            p2m(new_shape.y / 2.0),
                        ));
                    }
                }
            }
        }
    }
}

fn ball_hits_paddle_side(
    ecs: &World,
    ball_entity: hecs::Entity,
    paddle_entity: hecs::Entity,
) -> Option<f32> {
    let mut paddle_query = ecs.query_one::<(&Paddle, &CTransform, &Shape)>(paddle_entity);
    let (_, paddle_transform, paddle_shape) = paddle_query.get().ok()?;
    let paddle_start = paddle_transform.pos.x;
    let paddle_end = paddle_start + paddle_shape.dims.x;

    let mut ball_query = ecs.query_one::<(&Ball, &CTransform, &Shape)>(ball_entity);
    let (_, ball_transform, ball_shape) = ball_query.get().ok()?;
    let ball_center = ball_transform.pos.x + ball_shape.dims.x / 2.0;

    let paddle_left_third_end = paddle_start + (paddle_end - paddle_start) / 3.0;
    if ball_center > paddle_start && ball_center < paddle_left_third_end {
        return Some(-1.0);
    }

    let paddle_right_third_start = paddle_end - (paddle_end - paddle_start) / 3.0;
    if ball_center > paddle_right_third_start && ball_center < paddle_end {
        return Some(1.0);
    }

    None
}

fn damage_block(
    ecs: &mut World,
    state: &mut State,
    block_entity: hecs::Entity,
    destroy_sound: AudioCommand,
    sturdy_sound: AudioCommand,
) -> bool {
    if let Ok((_block, health)) = ecs.query_one_mut::<(&Block, &mut Health)>(block_entity) {
        match health.hp {
            0 => {}
            1 => {
                health.hp -= 1;
                state.audio_command_buffer.push(destroy_sound);
                state.deletion_events.push(DeletionEvent::Entity {
                    entity: block_entity,
                });
                state.deletion_events.push(DeletionEvent::Physics {
                    entity: block_entity,
                });
                return true;
            }
            _ => {
                health.hp -= 1;
                state.audio_command_buffer.push(sturdy_sound);
                return true;
            }
        }
    }

    false
}

pub fn respond_to_collisions(ecs: &mut World, state: &mut State) {
    let collision_events = state.physics.collision_events.clone();
    for event in collision_events {
        if event.started() {
            continue;
        }

        let entity_a = state
            .physics
            .collider_set
            .get(event.collider1())
            .and_then(|collider| collider.parent())
            .and_then(|body| state.physics.get_entity_from_rigid_body_handle(body));

        let entity_b = state
            .physics
            .collider_set
            .get(event.collider2())
            .and_then(|collider| collider.parent())
            .and_then(|body| state.physics.get_entity_from_rigid_body_handle(body));

        let (Some(entity_a), Some(entity_b)) = (entity_a, entity_b) else {
            continue;
        };

        if ecs.satisfies::<&Ball>(entity_a) {
            if ecs.satisfies::<(&Block, &StrongBlock)>(entity_b) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallBlockBounce);
                continue;
            }

            if damage_block(
                ecs,
                state,
                entity_b,
                AudioCommand::BallBlockBounce,
                AudioCommand::BallSturdyBlockBounce,
            ) {
                continue;
            }

            if ecs.satisfies::<&Paddle>(entity_b) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallPaddleBounce);
                if let Some(new_direction) = ball_hits_paddle_side(ecs, entity_a, entity_b) {
                    if let Ok((_, physics)) = ecs.query_one_mut::<(&Ball, &mut Physics)>(entity_a) {
                        physics.vel.x = BALL_VEL * new_direction;
                        physics.vel.y = -BALL_VEL;
                    }
                }
                continue;
            }

            if ecs.satisfies::<&BallEater>(entity_b) {
                state.audio_command_buffer.push(AudioCommand::BallDrop);
                state
                    .deletion_events
                    .push(DeletionEvent::Entity { entity: entity_a });
                state
                    .deletion_events
                    .push(DeletionEvent::Physics { entity: entity_a });
                continue;
            }

            if ecs.satisfies::<&Wall>(entity_b) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallWallBounce);
                continue;
            }
        }

        if ecs.satisfies::<&Ball>(entity_b) {
            if ecs.satisfies::<(&Block, &StrongBlock)>(entity_a) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallBlockBounce);
                continue;
            }

            if damage_block(
                ecs,
                state,
                entity_a,
                AudioCommand::BallBlockBounce,
                AudioCommand::BallSturdyBlockBounce,
            ) {
                continue;
            }

            if ecs.satisfies::<&Paddle>(entity_a) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallPaddleBounce);
                if let Some(new_direction) = ball_hits_paddle_side(ecs, entity_b, entity_a) {
                    if let Ok((_, physics)) = ecs.query_one_mut::<(&Ball, &mut Physics)>(entity_b) {
                        physics.vel.x = BALL_VEL * new_direction;
                        physics.vel.y = -BALL_VEL;
                    }
                }
                continue;
            }

            if ecs.satisfies::<&BallEater>(entity_a) {
                state.audio_command_buffer.push(AudioCommand::BallDrop);
                state
                    .deletion_events
                    .push(DeletionEvent::Entity { entity: entity_b });
                state
                    .deletion_events
                    .push(DeletionEvent::Physics { entity: entity_b });
                continue;
            }

            if ecs.satisfies::<&Wall>(entity_a) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallWallBounce);
            }
        }
    }
}

#[allow(dead_code)]
pub fn boundary_checking(ecs: &World, _state: &mut State) {
    for (ctransform, shape) in ecs
        .query::<(&mut CTransform, &Shape)>()
        .without::<&FreeToLeavePlayField>()
        .iter()
    {
        if ctransform.pos.x <= 0.0 {
            ctransform.pos.x = 0.0;
        }
        if (ctransform.pos.x + shape.dims.x) >= (DIMS.x as f32 - 1.0) {
            ctransform.pos.x = DIMS.x as f32 - shape.dims.x - 1.0;
        }

        if ctransform.pos.y <= 0.0 {
            ctransform.pos.y = 0.0;
        }
        if (ctransform.pos.y + shape.dims.y) >= (DIMS.y as f32 - 1.0) {
            ctransform.pos.y = DIMS.y as f32 - shape.dims.y - 1.0;
        }
    }
}
