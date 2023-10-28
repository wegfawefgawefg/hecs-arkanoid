use glam::Vec2;
use hecs::World;
use nalgebra::Vector2;
use rapier2d::prelude::RigidBodyHandle;

use crate::audio_playing::AudioCommand;
use crate::components::{
    Ball, Block, CTransform, FreeToLeavePlayField, HasRigidBody, Health, Paddle, Physics,
    PositionManaged, Shape, VelocityManaged,
};
use crate::physics_engine::{m2p, p2m};
use crate::state::{DeletionEvent, State};
use crate::{DIMS, TS_RATIO};

pub fn sync_ecs_to_physics(ecs: &World, state: &mut State) {
    // for (entity, (ctransform, physics)) in ecs
    //     .query::<(&mut CTransform, &mut Physics)>()
    //     .with::<&HasRigidBody>()
    //     .iter()
    // {
    //     if let Some(body) = state.physics.get_rigid_body_handle(entity) {
    //         if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
    //             // Update the physics engine
    //             // let pos = Vector2::new(p2m(ctransform.pos.x), p2m(ctransform.pos.y));
    //             // rigid_body.set_position(pos.into(), true);

    //             let vel = Vector2::new(p2m(physics.vel.x), p2m(physics.vel.y));
    //             rigid_body.set_linvel(vel, true);
    //         }
    //     }
    // }

    // velocity managed
    for (entity, physics) in ecs
        .query::<&mut Physics>()
        .with::<(&HasRigidBody, &VelocityManaged)>()
        .iter()
    {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
                // Update the physics engine
                // let pos = Vector2::new(p2m(ctransform.pos.x), p2m(ctransform.pos.y));
                // rigid_body.set_position(pos.into(), true);

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
                // Update the physics engine
                let center = ctransform.pos + shape.dims / 2.0;
                let pos = Vector2::new(p2m(center.x), p2m(center.y));
                rigid_body.set_position(pos.into(), true);
            }
        }
    }
}
const ANGLE_45_IN_RAD: f32 = std::f32::consts::PI / 3.0;
const BALL_VEL: f32 = 300.0 * (1.0 / TS_RATIO);
pub fn set_ball_to_angle(ecs: &World, state: &mut State) {
    for (entity, physics) in ecs
        .query::<&mut Physics>()
        .with::<(&HasRigidBody, &Ball)>()
        .iter()
    {
        if let Some(_body) = state.physics.get_rigid_body_handle(entity) {
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

#[allow(clippy::option_map_unit_fn)]
pub fn damage_blocks(ecs: &mut World, state: &mut State) {
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
            //////////////// CASE A IS BALL AND B IS BLOCK  ////////////////
            // check if a is a ball
            let a_is_ball = ecs.satisfies::<&Ball>(entity_a).unwrap_or(false);

            // if a is a ball, and b is block, decrement hp, and mark b for removal

            let mut remove_b = false;
            // if a_is_ball {
            //     remove_b = ecs.satisfies::<&Block>(entity_b).unwrap_or(false);
            // }
            if a_is_ball {
                println!("a is a ball");
                if let Ok((_block, health)) = ecs.query_one_mut::<(&Block, &mut Health)>(entity_b) {
                    match health.hp {
                        0 => {}
                        1 => {
                            health.hp -= 1;
                            remove_b = true;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallBlockBounce);
                        }
                        _ => {
                            health.hp -= 1;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallSturdyBlockBounce);
                        }
                    }
                }
            }

            if remove_b {
                state
                    .deletion_events
                    .push(DeletionEvent::Entity { entity: entity_b });
                state
                    .deletion_events
                    .push(DeletionEvent::Physics { entity: entity_b });
                continue;
            }

            // case b was paddle
            if ecs.satisfies::<&Paddle>(entity_b).unwrap_or(false) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallPaddleBounce);
            }

            //////////////// CASE A IS BALL AND B IS BLOCK  ////////////////
            // check if b is a ball
            let b_is_ball = ecs.satisfies::<&Ball>(entity_b).unwrap_or(false);

            // if b is a ball, and a is a block, decrement hp, and mark a for removal
            let mut remove_a = false;
            // if b_is_ball {
            //     remove_a = ecs.satisfies::<&Block>(entity_a).unwrap_or(false);
            // }
            // // let mut remove_a = false;
            if b_is_ball {
                println!("b is a ball");
                if let Ok((_, health)) = ecs.query_one_mut::<(&Block, &mut Health)>(entity_a) {
                    println!("a is a block");
                    match health.hp {
                        0 => {}
                        1 => {
                            health.hp -= 1;
                            remove_a = true;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallBlockBounce);
                        }
                        _ => {
                            health.hp -= 1;
                            state
                                .audio_command_buffer
                                .push(AudioCommand::BallSturdyBlockBounce);
                        }
                    }
                }
            }
            if remove_a {
                state
                    .deletion_events
                    .push(DeletionEvent::Entity { entity: entity_a });
                state
                    .deletion_events
                    .push(DeletionEvent::Physics { entity: entity_a });
                continue;
            }

            // case a was paddle
            if ecs.satisfies::<&Paddle>(entity_a).unwrap_or(false) {
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallPaddleBounce);
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
