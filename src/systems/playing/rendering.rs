use glam::Vec2;
use hecs::World;
use rapier2d::prelude::*;
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, BallEater, Block, CTransform, Health, Paddle, Physics, Shape, StrongBlock, Wall,
    },
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
    for (entity, (ctransform, shape)) in ecs.query::<(&CTransform, &Shape)>().with::<&Wall>().iter()
    {
        // white if not a ball eater, red if it is
        let mut color: Color = Color::WHITE;
        if let Ok(mut r) = ecs.query_one::<&BallEater>(entity) {
            if let Some(_) = r.get() {
                color = Color::RED;
            }
        }
        state
            .render_command_buffer
            .push(RenderCommand::SolidRectangle {
                pos: ctransform.pos,
                dims: shape.dims,
                color,
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
    for (entity, (block, ctransform, shape, health)) in
        ecs.query::<(&Block, &CTransform, &Shape, &Health)>().iter()
    {
        let ball_unbreakable = ecs.satisfies::<&StrongBlock>(entity).unwrap_or(false);
        state.render_command_buffer.push(RenderCommand::Block {
            pos: ctransform.pos,
            dims: shape.dims,
            color: block.color,
            hp: health.hp,
            ball_unbreakable,
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
    let cursor = Vec2::new(DIMS.x as f32 - 50.0, DIMS.y as f32 - 20.0);
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

        if let ShapeType::Cuboid = shape_type {
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
                ball_unbreakable: false,
            });
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
