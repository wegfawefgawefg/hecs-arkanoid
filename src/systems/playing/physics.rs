use std::char::MAX;

use hecs::{Without, World};

use crate::components::{CTransform, Paddle, Physics};
use crate::state::State;

const MAX_VEL: f32 = 2.0;
pub fn physics(ecs: &World, state: &mut State) {
    for (_, (ctransform, physics)) in ecs.query::<(&mut CTransform, &mut Physics)>().iter() {
        if physics.vel.length() > MAX_VEL {
            physics.vel = physics.vel.normalize() * MAX_VEL;
        }

        ctransform.pos += physics.vel;

        let rot_matrix = glam::Mat2::from_angle(physics.rot_vel.to_radians() * 0.1);
        ctransform.rot = (rot_matrix * ctransform.rot).normalize();
    }
}

// pub fn physics(ecs: &mut World, state: &mut State) {
// let query = <&mut Physics>::query();
// for physics in query.filter(!component::<VelocityUncapped>()).iter_mut(ecs) {
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
