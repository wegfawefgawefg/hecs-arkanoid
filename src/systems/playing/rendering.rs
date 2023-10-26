use glam::Vec2;
use hecs::World;
use rand::{rngs::StdRng, Rng};
use rapier2d::prelude::*;
use raylib::prelude::Color;

use crate::{
    components::{Ball, Block, CTransform, Health, Paddle, Physics, Player, Shape, Wall},
    physics_engine::m2p,
    render_commands::RenderCommand,
    state::State,
    DIMS,
};

pub fn render(ecs: &World, state: &mut State) {
    // render_physics(state);

    let mut cursor = Vec2::new(20.0, 20.0);
    for (_, physics) in ecs.query::<&Physics>().with::<&Ball>().iter() {
        state.render_command_buffer.push(RenderCommand::Text {
            pos: cursor,
            text: format!("vel: {}", physics.vel),
            size: 1,
            color: Color::new(255, 255, 255, 10),
        });
        cursor.y += 10.0;
    }

    // render walls
    for (_, (ctransform, shape, wall)) in ecs.query::<(&CTransform, &Shape, &Wall)>().iter() {
        state.render_command_buffer.push(RenderCommand::Line {
            start: ctransform.pos,
            end: ctransform.pos + shape.dims,
            color: wall.color,
        });
    }

    // render every player as a paddle
    for (_, (_, ctransform, shape)) in ecs.query::<(&Paddle, &CTransform, &Shape)>().iter() {
        state.render_command_buffer.push(RenderCommand::Paddle {
            pos: ctransform.pos,
            dims: shape.dims,
            color: Color::RAYWHITE,
        })
    }

    // render every block
    for (_, (block, ctransform, shape, health)) in
        ecs.query::<(&Block, &CTransform, &Shape, &Health)>().iter()
    {
        state.render_command_buffer.push(RenderCommand::Block {
            pos: ctransform.pos,
            dims: shape.dims,
            color: block.color,
            hp: health.hp,
        })
    }

    // render ball
    for (_, (_, ctransform, shape)) in ecs.query::<(&Ball, &CTransform, &Shape)>().iter() {
        state.render_command_buffer.push(RenderCommand::Ball {
            pos: ctransform.pos,
            dims: shape.dims,
        })
    }

    // render the level in the top right
    let mut cursor = Vec2::new(DIMS.x as f32 - 50.0, DIMS.y as f32 - 20.0);
    let size = 1;
    state.render_command_buffer.push(RenderCommand::Text {
        pos: cursor,
        text: format!("Level: {}", state.level),
        size,
        color: Color::WHITE,
    });
}

pub fn render_physics(state: &mut State) {
    // Render colliders
    for (_, collider) in state.physics.collider_set.iter() {
        let center = collider.position().translation.vector;
        let shape = collider.shape();
        let shape_type = shape.shape_type();

        match shape_type {
            ShapeType::Cuboid => {
                let cuboid = shape.as_cuboid().unwrap();
                let tl = center + -cuboid.half_extents;
                let size = cuboid.half_extents * 2.0;

                let ppos = Vec2::new(m2p(tl.x), m2p(tl.y));
                let psize = Vec2::new(m2p(size.x), m2p(size.y));
                state.render_command_buffer.push(RenderCommand::Block {
                    pos: ppos,
                    dims: psize,
                    color: Color::RED, // or any color you prefer for debug
                    hp: 1,
                });
            }
            // Add more shape types here if needed
            _ => {}
        }
    }

    // Render rigid bodies (Optional, if you need to distinguish them)
    for (_, rigid_body) in state.physics.rigid_body_set.iter() {
        let pos = rigid_body.position().translation.vector;
        let rot = rigid_body.position().rotation.angle();

        let ppos = Vec2::new(m2p(pos.x), m2p(pos.y));
        let prot = Vec2::new(rot.cos(), rot.sin());
        state.render_command_buffer.push(RenderCommand::Line {
            start: ppos,
            end: ppos + prot * 10.0,
            color: Color::GREEN, // or any color you prefer for debug
        });
    }
}

// // render system
// /* fetch position and sprite entities, and just blit them with a fixed size with the given position */
// #[system]
// #[read_component(Score)]
// pub fn score_render(ecs: &SubWorld, #[resource] render_command_buffer: &mut RenderCommandBuffer) {
//     let mut cursor = Vec2::new(DIMS.x as f32 * 0.28, DIMS.y as f32 * 0.1);
//     let size = 1;

//     // schedule asteroid rendering
//     <&Score>::query().iter(ecs).for_each(|score| {
//         let text = format!("Score: {}", score.score);

//         render_command_buffer.push(DrawCommand::Text {
//             pos: cursor,
//             text,
//             size,
//             color: Color::WHITE,
//         });

//         cursor.y += 10.0;
//     });
// }

// #[system]
// pub fn render_expiring_messages(
//     #[resource] expiring_messages: &ExpiringMessages,
//     #[resource] render_command_buffer: &mut RenderCommandBuffer,
// ) {
//     // cursor should go up from the bottom left corner
//     let mut cursor = Vec2::new(0.0, DIMS.y as f32 * 0.9);
//     let size = 5;

//     for message in expiring_messages.iter() {
//         render_command_buffer.push(DrawCommand::Text {
//             pos: cursor,
//             text: message.text.clone(),
//             size,
//             color: Color::WHITE,
//         });

//         cursor.y -= size as f32 * 1.5;
//     }
// }
