use glam::Vec2;
use hecs::World;
use nalgebra::Vector2;
use rapier2d::prelude::RigidBodyHandle;

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
    // velocity managed
    for (entity, physics) in ecs
        .query::<&mut Physics>()
        .with::<(&HasRigidBody, &VelocityManaged)>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
                let vel = Vector2::new(p2m(physics.vel.x), p2m(physics.vel.y));
                rigid_body.set_linvel(vel, true);
            }
        }
    }

    // position managed
    for (entity, (ctransform, shape)) in ecs
        .query::<(&mut CTransform, &Shape)>()
        .with::<(&HasRigidBody, &PositionManaged)>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
                let center = ctransform.pos + shape.dims / 2.0;
                let pos = Vector2::new(p2m(center.x), p2m(center.y));
                rigid_body.set_position(pos.into(), true);
            }
        }
    }
}
const ANGLE_45_IN_RAD: f32 = std::f32::consts::PI / 3.0;
const BALL_VEL: f32 = 200.0 * (1.0 / TS_RATIO);
pub fn set_ball_to_angle(ecs: &World, state: &mut State) {
    for (entity, physics) in ecs
        .query::<&mut Physics>()
        .with::<(&HasRigidBody, &Ball)>()
        .iter()
    {
        if let Some(_body) = state.physics.get_rigid_body_handle(entity) {
            // physics.vel = physics.vel.normalize() * BALL_VEL;

            let x_sign = physics.vel.x.signum();
            let y_sign = physics.vel.y.signum();

            physics.vel.x = ANGLE_45_IN_RAD.cos() * BALL_VEL * x_sign;
            physics.vel.y = ANGLE_45_IN_RAD.sin() * BALL_VEL * y_sign;
        }
    }
}

/// Collision events are emptied here so dont check collisions in step before this is called
pub fn step_physics(ecs: &World, state: &mut State) {
    state.physics.step();

    //////////////////////////////////////////////////////////////////
    // now copy the physics engine's state back into the ecs
    //////////////////////////////////////////////////////////////////

    // first for positions
    for (entity, (ctransform, shape)) in ecs
        .query::<(&mut CTransform, &Shape)>()
        .with::<&HasRigidBody>()
        .without::<&PositionManaged>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                let center = rigid_body.position().translation.vector;
                let rot = rigid_body.position().rotation.angle();
                let pos = Vec2::new(
                    m2p(center.x) - shape.dims.x / 2.0,
                    m2p(center.y) - shape.dims.y / 2.0,
                );
                ctransform.pos = pos;
                ctransform.rot = Vec2::new(rot.cos(), rot.sin());
            }
        }
    }

    // now for velocities
    for (entity, physics) in ecs.query::<&mut Physics>().with::<&HasRigidBody>().iter() {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                let vel = *rigid_body.linvel();
                physics.vel = Vec2::new(m2p(vel.x), m2p(vel.y));
            }
        }
    }

    // paddle specifically
    for (entity, (ctransform, shape)) in ecs
        .query::<(&mut CTransform, &Shape)>()
        .with::<&Paddle>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                let center = rigid_body.position().translation.vector;
                ctransform.pos.x = m2p(center.x) - shape.dims.x / 2.0;
                ctransform.pos.y = m2p(center.y) - shape.dims.y / 2.0;
            }
        }
    }

    // store all collision events for later iteration
    state.physics.collision_events.clear();
    while let Ok(event) = state.physics.collision_recv.try_recv() {
        state.physics.collision_events.push(event);
    }
}

pub fn constantly_resize_paddle(ecs: &mut World, state: &mut State) {
    let new_shape = Vec2::new(
        BASE_PADDLE_SHAPE.x * (1.0 + (state.t * 0.1).sin() / 2.0) + 10.0,
        BASE_PADDLE_SHAPE.y,
    );
    println!("new shape: {:?}", new_shape);
    for (entity, shape) in ecs.query::<&mut Shape>().with::<&Paddle>().iter() {
        shape.dims = new_shape;

        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get(body) {
                for collider_handle in rigid_body.colliders().iter() {
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

#[allow(clippy::option_map_unit_fn)]
pub fn respond_to_collisions(ecs: &mut World, state: &mut State) {
    let collision_events = state.physics.collision_events.clone();
    for event in collision_events {
        if event.started() {
            continue;
        }
        // Fetch the entities associated with the colliders involved in this collision event
        // also fetch the rigid body handles
        let mut rigid_body_handle_a: Option<RigidBodyHandle> = None;
        let mut rigid_body_handle_b: Option<RigidBodyHandle> = None;

        let entity_a = state
            .physics
            .collider_set
            .get(event.collider1())
            .and_then(|collider_a| collider_a.parent())
            .and_then(|rigid_body_a| {
                rigid_body_handle_a = Some(rigid_body_a);
                state
                    .physics
                    .get_entity_from_rigid_body_handle(rigid_body_a)
            });

        let entity_b = state
            .physics
            .collider_set
            .get(event.collider2())
            .and_then(|collider_b| collider_b.parent())
            .and_then(|rigid_body_b| {
                rigid_body_handle_b = Some(rigid_body_b);
                state
                    .physics
                    .get_entity_from_rigid_body_handle(rigid_body_b)
            });

        // if there are entities associated with those colliders
        // respond to the collision depending on the entity types and properties
        if let (Some(entity_a), Some(entity_b)) = (entity_a, entity_b) {
            println!("Collision between {:?} and {:?}", entity_a, entity_b);
            // case: a is ball and b is block
            if ecs.satisfies::<&Ball>(entity_a).unwrap_or(false) {
                println!("a is a ball");
                // check if b is block and BallUnbreakable
                if ecs
                    .satisfies::<(&Block, &StrongBlock)>(entity_b)
                    .unwrap_or(false)
                {
                    state
                        .audio_command_buffer
                        .push(AudioCommand::BallBlockBounce);
                    continue;
                }

                // reduce block health on hit
                if let Ok((_block, health)) = ecs.query_one_mut::<(&Block, &mut Health)>(entity_b) {
                    match health.hp {
                        0 => {}
                        1 => {
                            health.hp -= 1;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallBlockBounce);
                            state
                                .deletion_events
                                .push(DeletionEvent::Entity { entity: entity_b });
                            state
                                .deletion_events
                                .push(DeletionEvent::Physics { entity: entity_b });
                            continue;
                        }
                        _ => {
                            health.hp -= 1;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallSturdyBlockBounce);
                            continue;
                        }
                    }
                }
            }

            // case: a is ball and b is paddle
            if ecs.satisfies::<&Paddle>(entity_b).unwrap_or(false) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallPaddleBounce);

                let mut ball_new_direction: Option<f32> = None;

                // determine if the ball hit the left, middle or right of the paddle
                // if hit left, set ball velocity to -
                // if hit right, set ball velocity to +
                if let Ok(mut res) = ecs.query_one::<(&Paddle, &CTransform, &Shape)>(entity_b) {
                    if let Some((_, ctransform, shape)) = res.get() {
                        let paddle_start = ctransform.pos.x;
                        let paddle_end = paddle_start + shape.dims.x;

                        // get ball position
                        if let Ok(mut res) = ecs.query_one::<(&Ball, &CTransform, &Shape)>(entity_a)
                        {
                            if let Some((_, ctransform, shape)) = res.get() {
                                let ball_center = ctransform.pos.x + shape.dims.x / 2.0;

                                // if ball_pos is in the left 3rd, set ball velocity to -
                                let paddle_left_third_end =
                                    paddle_start + (paddle_end - paddle_start) / 3.0;
                                if ball_center > paddle_start && ball_center < paddle_left_third_end
                                {
                                    ball_new_direction = Some(-1.0);
                                }
                                let paddle_right_third_start =
                                    paddle_end - (paddle_end - paddle_start) / 3.0;
                                if ball_center > paddle_right_third_start
                                    && ball_center < paddle_end
                                {
                                    ball_new_direction = Some(1.0);
                                }
                            }
                        }
                    }
                }

                if let Some(new_direction) = ball_new_direction {
                    if let Ok((_, physics)) = ecs.query_one_mut::<(&Ball, &mut Physics)>(entity_a) {
                        physics.vel.x = BALL_VEL * new_direction;
                        physics.vel.y = -BALL_VEL;
                    }
                }

                continue;
            }

            // case: a is ball and b is balleater
            if ecs.satisfies::<&BallEater>(entity_b).unwrap_or(false) {
                state.audio_command_buffer.push(AudioCommand::BallDrop);
                state
                    .deletion_events
                    .push(DeletionEvent::Entity { entity: entity_a });
                state
                    .deletion_events
                    .push(DeletionEvent::Physics { entity: entity_a });
                continue;
            }

            // case: a is ball and b is wall
            if ecs.satisfies::<&Wall>(entity_b).unwrap_or(false) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallWallBounce);
                continue;
            }

            ////////////////    MIRRORED CASES    ////////////////
            // case: b is ball and a is block
            if ecs.satisfies::<&Ball>(entity_b).unwrap_or(false) {
                println!("b is a ball");
                // check if a is block and BallUnbreakable
                if ecs
                    .satisfies::<(&Block, &StrongBlock)>(entity_a)
                    .unwrap_or(false)
                {
                    state
                        .audio_command_buffer
                        .push(AudioCommand::BallBlockBounce);
                    continue;
                }

                // reduce block health on hit
                if let Ok((_, health)) = ecs.query_one_mut::<(&Block, &mut Health)>(entity_a) {
                    println!("a is a block");

                    // reduce block health on hit
                    match health.hp {
                        0 => {}
                        1 => {
                            health.hp -= 1;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallBlockBounce);
                            state
                                .deletion_events
                                .push(DeletionEvent::Entity { entity: entity_a });
                            state
                                .deletion_events
                                .push(DeletionEvent::Physics { entity: entity_a });
                            continue;
                        }
                        _ => {
                            health.hp -= 1;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallSturdyBlockBounce);
                            continue;
                        }
                    }
                }
            }

            // case: b is ball and a is paddle
            if ecs.satisfies::<&Paddle>(entity_a).unwrap_or(false) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallPaddleBounce);

                let mut ball_new_direction: Option<f32> = None;

                // determine if the ball hit the left, middle or right of the paddle
                // if hit left, set ball velocity to -
                // if hit right, set ball velocity to +
                if let Ok(mut res) = ecs.query_one::<(&Paddle, &CTransform, &Shape)>(entity_a) {
                    if let Some((_, ctransform, shape)) = res.get() {
                        let paddle_start = ctransform.pos.x;
                        let paddle_end = paddle_start + shape.dims.x;

                        // get ball position
                        if let Ok(mut res) = ecs.query_one::<(&Ball, &CTransform, &Shape)>(entity_b)
                        {
                            if let Some((_, ctransform, shape)) = res.get() {
                                let ball_center = ctransform.pos.x + shape.dims.x / 2.0;

                                // if ball_pos is in the left 3rd, set ball velocity to -
                                let paddle_left_third_end =
                                    paddle_start + (paddle_end - paddle_start) / 3.0;
                                if ball_center > paddle_start && ball_center < paddle_left_third_end
                                {
                                    ball_new_direction = Some(-1.0);
                                }
                                let paddle_right_third_start =
                                    paddle_end - (paddle_end - paddle_start) / 3.0;
                                if ball_center > paddle_right_third_start
                                    && ball_center < paddle_end
                                {
                                    ball_new_direction = Some(1.0);
                                }
                            }
                        }
                    }
                }

                if let Some(new_direction) = ball_new_direction {
                    if let Ok((_, physics)) = ecs.query_one_mut::<(&Ball, &mut Physics)>(entity_b) {
                        physics.vel.x = BALL_VEL * new_direction;
                        physics.vel.y = -BALL_VEL;
                    }
                }

                continue;
            }

            // case: b is ball and a is balleater
            if ecs.satisfies::<&BallEater>(entity_a).unwrap_or(false) {
                state.audio_command_buffer.push(AudioCommand::BallDrop);
                state
                    .deletion_events
                    .push(DeletionEvent::Entity { entity: entity_b });
                state
                    .deletion_events
                    .push(DeletionEvent::Physics { entity: entity_b });
                continue;
            }

            // case: b is ball and a is wall
            if ecs.satisfies::<&Wall>(entity_a).unwrap_or(false) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallWallBounce);
                continue;
            }
        };
    }
}

pub fn boundary_checking(ecs: &World, _state: &mut State) {
    for (_, (ctransform, shape)) in ecs
        .query::<(&mut CTransform, &Shape)>()
        .without::<&FreeToLeavePlayField>()
        .iter()
    {
        if ctransform.pos.x <= 0.0 {
            ctransform.pos.x = 0.0
        }
        if (ctransform.pos.x + shape.dims.x) >= (DIMS.x as f32 - 1.0) {
            ctransform.pos.x = DIMS.x as f32 - shape.dims.x - 1.0;
        }

        if ctransform.pos.y <= 0.0 {
            ctransform.pos.y = 0.0
        }
        if (ctransform.pos.y + shape.dims.y) >= (DIMS.y as f32 - 1.0) {
            ctransform.pos.y = DIMS.y as f32 - shape.dims.y - 1.0;
        }
    }
}
