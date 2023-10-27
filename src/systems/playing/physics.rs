use std::char::MAX;
use std::time::Duration;

use glam::Vec2;
use hecs::{Entity, World};
use nalgebra::{Vector, Vector2};
use rapier2d::prelude::{Real, RigidBody, RigidBodyHandle};
use raylib::ffi::remove;

use crate::audio_playing::{AudioCommand, AudioCommandBuffer};
use crate::components::{
    Ball, Block, Bouncy, CTransform, FreeToLeavePlayField, HasRigidBody, Health, Paddle, Physics,
    Shape,
};
use crate::physics_engine::{m2p, p2m};
use crate::state::{DeletionEvent, State};
use crate::DIMS;

const MAX_VEL: f32 = 200.0;

// pub fn sync_ecs_to_physics(ecs: &World, state: &mut State) {
//     // every ball object that has a rigid body needs to copy its vel over to the physics
//     for (entity, physics) in ecs.query::<&mut Physics>().with::<&HasRigidBody>().iter() {
//         if let Some(body) = state.physics.get_rigid_body_handle(entity) {
//             if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
//                 // copy from physics to rigid body
//                 physics.vel = physics.vel.normalize() * MAX_VEL;
//                 let vel = Vector2::new(p2m(physics.vel.x), p2m(physics.vel.y));
//                 rigid_body.set_linvel(vel, true);
//             }
//         }
//     }
// }
pub fn sync_ecs_to_physics(ecs: &World, state: &mut State) {
    for (entity, physics) in ecs.query::<&mut Physics>().with::<&HasRigidBody>().iter() {
        if let Some(body) = state.physics.get_rigid_body_handle(entity) {
            if let Some(rigid_body) = state.physics.rigid_body_set.get_mut(body) {
                // Update the physics engine
                let vel = Vector2::new(p2m(physics.vel.x), p2m(physics.vel.y));
                rigid_body.set_linvel(vel, true);
            }
        }
    }
}
const ANGLE_45_IN_RAD: f32 = std::f32::consts::PI / 3.0;
pub fn set_ball_to_angle(ecs: &World, state: &mut State) {
    for (entity, physics) in ecs
        .query::<&mut Physics>()
        .with::<(&HasRigidBody, &Ball)>()
        .iter()
    {
        if let Some(_body) = state.physics.get_rigid_body_handle(entity) {
            let x_sign = physics.vel.x.signum();
            let y_sign = physics.vel.y.signum();

            physics.vel.x = ANGLE_45_IN_RAD.cos() * MAX_VEL * x_sign;
            physics.vel.y = ANGLE_45_IN_RAD.sin() * MAX_VEL * y_sign;
        }
    }
}

const ANGLES: [f32; 2] = [30.0, 60.0];
// Angles to snap to
// pub fn set_ball_to_angles(ecs: &World, state: &mut State) {
//     for (entity, physics) in ecs
//         .query::<&mut Physics>()
//         .with::<(&HasRigidBody, &Ball)>()
//         .iter()
//     {
//         if let Some(body) = state.physics.get_rigid_body_handle(entity) {
//             // Compute current angle in degrees
//             let angle_current = physics.vel.y.atan2(physics.vel.x) * (180.0 / std::f32::consts::PI);

//             // Find closest snap angle
//             let closest_angle = ANGLES
//                 .iter()
//                 .copied()
//                 .min_by(|a, b| {
//                     (a - angle_current.abs())
//                         .abs()
//                         .partial_cmp(&(b - angle_current.abs()).abs())
//                         .unwrap()
//                 })
//                 .unwrap_or(angle_current.abs());

//             // Convert closest_angle back to radians
//             let closest_angle_rad = closest_angle.to_radians();

//             // Set new velocity based on closest angle and MAX_VEL
//             physics.vel.x = closest_angle_rad.cos() * MAX_VEL * physics.vel.x.signum();
//             physics.vel.y = closest_angle_rad.sin() * MAX_VEL * physics.vel.y.signum();
//         }
//     }
// }

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
                if let Ok((block, health)) = ecs.query_one_mut::<(&Block, &mut Health)>(entity_a) {
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

// Remove blocks with 0 or less HP
// let mut to_remove = Vec::new();
// for (entity, health) in ecs.query::<&Health>().with::<&Block>().iter() {
//     if health.value <= 0 {
//         to_remove.push(entity);
//     }
// }

// Remove physics and ECS entities
// for entity in to_remove {
//     // Remove physics bodies and bindings
//     if let Some(handle) = state.physics.get_rigid_body_handle(entity) {
//         state.physics.rigid_body_set.remove(handle);
//     }

//     // Remove from ECS
//     ecs.despawn(entity);
// }

pub fn bounce(ecs: &World, state: &mut State) {
    let mut bounce_surfaces: Vec<(Entity, CTransform, Shape)> = ecs
        .query::<(&mut CTransform, &Shape)>()
        .iter()
        .map(|(entity, (ctransform, shape))| (entity, *ctransform, *shape))
        .collect();
}

pub fn boundary_checking(ecs: &World, state: &mut State) {
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

// pub fn physics(ecs: &mut World, state: &mut State) {
// let query = <&mut Physics>::query();
// for physics in query.filter(!component::<velUncapped>()).iter_mut(ecs) {
//     if physics.vel.length() > MAX_VEL {
//         physics.vel = physics.vel.normalize() * MAX_VEL;
//     }
// }
// let mut step_query = <(&mut CTransform, &mut Physics)>::query();
// for (ctransform, physics) in step_query.iter_mut(ecs) {
//     if physics.vel.length() > MAX_VEL {
//         physics.vel = physics.vel.normalize() * MAX_VEL;
//     }
//     ctransform.pos += physics.vel;

//     let rot_matrix = glam::Mat2::from_angle(physics.rot_vel.to_radians() * 0.1);
//     ctransform.rot = (rot_matrix * ctransform.rot).normalize();
// }
// }

// #[system]
// #[write_component(CTransform)]
// #[write_component(Physics)]
// pub fn capture_in_play_field(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
//     let mut query = <(Entity, &mut CTransform)>::query().filter(component::<CaptureInPlayField>());
//     for (entity, ctransform) in query.iter_mut(ecs) {
//         let is_in_play_field = ctransform.pos.x > 0.0
//             && ctransform.pos.x < DIMS.x as f32
//             && ctransform.pos.y > 0.0
//             && (ctransform.pos.y < DIMS.y as f32);
//         if is_in_play_field {
//             cmd.remove_component::<CaptureInPlayField>(*entity);
//         }
//     }
// }
