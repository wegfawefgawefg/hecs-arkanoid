use glam::Vec2;
use hecs::World;
use raylib::prelude::Color;

use crate::{
    components::{
        Ball, BallEater, Block, CTransform, Health, Paddle, Physics, Shape, StrongBlock, Wall,
    },
    render_commands::RenderCommand,
    state::State,
    DIMS,
};

pub fn render(ecs: &World, state: &mut State) {
    // render_physics(state);

    let mut cursor = Vec2::new(20.0, 20.0);
    for physics in ecs.query::<&Physics>().with::<&Ball>().iter() {
        state.render_command_buffer.push(RenderCommand::Text {
            pos: cursor,
            text: format!("vel: {}", physics.vel),
            size: 1,
            color: Color::new(255, 255, 255, 10),
        });
        cursor.y += 10.0;
    }

    // render walls
    for (entity, ctransform, shape) in ecs
        .query::<(hecs::Entity, &CTransform, &Shape)>()
        .with::<&Wall>()
        .iter()
    {
        // white if not a ball eater, red if it is
        let mut color: Color = Color::WHITE;
        let mut r = ecs.query_one::<&BallEater>(entity);
        if r.get().is_ok() {
            color = Color::RED;
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
    for (_, ctransform, shape) in ecs.query::<(&Paddle, &CTransform, &Shape)>().iter() {
        state.render_command_buffer.push(RenderCommand::Paddle {
            pos: ctransform.pos,
            dims: shape.dims,
            color: Color::RAYWHITE,
        })
    }

    // render every block
    for (entity, block, ctransform, shape, health) in ecs
        .query::<(hecs::Entity, &Block, &CTransform, &Shape, &Health)>()
        .iter()
    {
        let ball_unbreakable = ecs.satisfies::<&StrongBlock>(entity);
        state.render_command_buffer.push(RenderCommand::Block {
            pos: ctransform.pos,
            dims: shape.dims,
            color: block.color,
            hp: health.hp,
            ball_unbreakable,
        })
    }

    // render ball
    for (_, ctransform, shape) in ecs.query::<(&Ball, &CTransform, &Shape)>().iter() {
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

#[allow(dead_code)]
pub fn render_physics(_state: &mut State) {}
