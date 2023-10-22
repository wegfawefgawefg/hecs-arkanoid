use std::char::MAX;

use glam::Vec2;
use hecs::{Entity, World};
use raylib::ffi::remove;

use crate::components::{
    Ball, Block, Bouncy, CTransform, FreeToLeavePlayField, HasRigidBody, Health, Paddle, Physics,
    Shape,
};
use crate::physics_engine::m2p;
use crate::state::State;
use crate::DIMS;

const MAX_VEL: f32 = 2.0;
pub fn physics(ecs: &World, state: &mut State) {
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
}

pub fn damage_blocks(ecs: &mut World, state: &mut State) {
    // Go through collision_recv events
    while let Ok(event) = state.physics.collision_recv.try_recv() {
        // Fetch the entities associated with the handles from the event
        state
            .physics
            .collider_set
            .get(event.collider1())
            .and_then(|collider_a| collider_a.parent())
            .and_then(|rigid_body_a| {
                state
                    .physics
                    .get_entity_from_rigid_body_handle(rigid_body_a)
            })
            .and_then(|entity_a| {
                state
                    .physics
                    .collider_set
                    .get(event.collider2())
                    .and_then(|collider_b| collider_b.parent())
                    .and_then(|rigid_body_b| {
                        state
                            .physics
                            .get_entity_from_rigid_body_handle(rigid_body_b)
                            .map(|entity_b| (entity_a, entity_b))
                    })
            })
            .map(|(entity_a, entity_b)| {
                // check if a is a ball
                let mut a_is_ball = false;
                if let Ok(query) = ecs.query_one::<&Ball>(entity_a) {
                    if let Some(ball) = query.get() {
                        a_is_ball = true;
                    }
                }

                // if a is a ball, and b is block, decrement hp, and mark b for removal
                let mut remove_b = false;
                if a_is_ball {
                    if let Ok((block, health)) =
                        ecs.query_one_mut::<(&Block, &mut Health)>(entity_b)
                    {
                        if health.hp > 0 {
                            health.hp -= 1;
                        }
                        if health.hp == 0 {
                            remove_b = true;
                        }
                    }
                }

                // you are removing on contact, this is in the weeds. good luck

                if remove_b {
                    ecs.remove(entity_b);
                    state.physics.remove_rigid_body_mapping(entity_b);
                    state.physics.
                }

                // check if b is a ball
                let mut b_is_ball = false;
                if let Ok(query) = ecs.query_one::<&Ball>(entity_b) {
                    if let Some(ball) = query.get() {
                        b_is_ball = true;
                    }
                }

                // if b is a ball, and a is a block, decrement hp, and mark a for removal
                let mut remove_a = false;
                if b_is_ball {
                    if let Ok((block, health)) =
                        ecs.query_one_mut::<(&Block, &mut Health)>(entity_a)
                    {
                        if health.hp > 0 {
                            health.hp -= 1;
                        }
                        if health.hp == 0 {
                            remove_b = true;
                        }
                    }
                }
            });
    }

    // Remove blocks with 0 or less HP
    let mut to_remove = Vec::new();
    for (entity, health) in ecs.query::<&Health>().with::<&Block>().iter() {
        if health.value <= 0 {
            to_remove.push(entity);
        }
    }

    // Remove physics and ECS entities
    for entity in to_remove {
        // Remove physics bodies and bindings
        if let Some(handle) = state.physics.get_rigid_body_handle(entity) {
            state.physics.rigid_body_set.remove(handle);
        }

        // Remove from ECS
        ecs.despawn(entity);
    }
}

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
