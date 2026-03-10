use glam::Vec2;
use hecs::World;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    components::{
        Ball, BallEater, Block, CTransform, Health, Paddle, Physics, Shape, StrongBlock, Wall,
    },
    state::State,
    DIMS,
};

fn snap_rect(pos: Vec2, dims: Vec2) -> (i32, i32, i32, i32) {
    let left = pos.x.round() as i32;
    let top = pos.y.round() as i32;
    let width = dims.x.round().max(1.0) as i32;
    let height = dims.y.round().max(1.0) as i32;
    (left, top, width, height)
}

fn draw_rect_outline(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    pos: Vec2,
    dims: Vec2,
    color: Color,
) {
    let (left, top, width, height) = snap_rect(pos, dims);
    let right = left + width - 1;
    let bottom = top + height - 1;

    d.draw_rectangle(left, top, width, 1, color);
    d.draw_rectangle(left, bottom, width, 1, color);
    d.draw_rectangle(left, top, 1, height, color);
    d.draw_rectangle(right, top, 1, height, color);
}

pub fn render(ecs: &World, state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    // render_physics(state);

    let mut cursor = Vec2::new(20.0, 20.0);
    for physics in ecs.query::<&Physics>().with::<&Ball>().iter() {
        d.draw_text(
            format!("vel: {}", physics.vel).as_str(),
            cursor.x as i32,
            cursor.y as i32,
            1,
            Color::new(255, 255, 255, 10),
        );
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
        let (left, top, width, height) = snap_rect(ctransform.pos, shape.dims);
        d.draw_rectangle(left, top, width, height, color);
    }

    // render every player as a paddle
    for (_, ctransform, shape) in ecs.query::<(&Paddle, &CTransform, &Shape)>().iter() {
        draw_rect_outline(d, ctransform.pos, shape.dims, Color::RAYWHITE);
    }

    // render every block
    for (entity, block, ctransform, shape, health) in ecs
        .query::<(hecs::Entity, &Block, &CTransform, &Shape, &Health)>()
        .iter()
    {
        let ball_unbreakable = ecs.satisfies::<&StrongBlock>(entity);
        if ball_unbreakable {
            let (left, top, width, height) = snap_rect(ctransform.pos, shape.dims);
            d.draw_rectangle(left, top, width, height, block.color);
        } else {
            draw_rect_outline(d, ctransform.pos, shape.dims, block.color);
            if health.hp > 1 {
                let (left, top, width, height) = snap_rect(ctransform.pos, shape.dims);
                d.draw_line(left, top, left + width - 1, top + height - 1, block.color);
            }
        }
    }

    // render ball
    for (_, ctransform, shape) in ecs.query::<(&Ball, &CTransform, &Shape)>().iter() {
        draw_rect_outline(d, ctransform.pos, shape.dims, Color::RAYWHITE);
    }

    // render the level in the top right
    let cursor = Vec2::new(DIMS.x as f32 - 50.0, DIMS.y as f32 - 20.0);
    d.draw_text(
        format!("Level: {}", state.level).as_str(),
        cursor.x as i32,
        cursor.y as i32,
        1,
        Color::WHITE,
    );
}

#[allow(dead_code)]
pub fn render_physics(_state: &State) {}
