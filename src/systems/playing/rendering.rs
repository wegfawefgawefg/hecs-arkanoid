use glam::Vec2;
use hecs::World;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode, Vector2};

use crate::{
    components::{
        Ball, BallEater, Block, CTransform, Health, Paddle, Physics, Shape, StrongBlock, Wall,
    },
    state::State,
    DIMS,
};

fn draw_rect_outline(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    pos: Vec2,
    dims: Vec2,
    color: Color,
) {
    let left = pos.x;
    let top = pos.y;
    let right = pos.x + dims.x - 1.0;
    let bottom = pos.y + dims.y - 1.0;

    d.draw_line_v(Vector2::new(left, top), Vector2::new(right, top), color);
    d.draw_line_v(
        Vector2::new(left, bottom),
        Vector2::new(right, bottom),
        color,
    );
    d.draw_line_v(Vector2::new(left, top), Vector2::new(left, bottom), color);
    d.draw_line_v(Vector2::new(right, top), Vector2::new(right, bottom), color);
    d.draw_pixel(left as i32, top as i32, color);
    d.draw_pixel(right as i32, top as i32, color);
    d.draw_pixel(left as i32, bottom as i32, color);
    d.draw_pixel(right as i32, bottom as i32, color);
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
        d.draw_rectangle(
            ctransform.pos.x as i32,
            ctransform.pos.y as i32,
            shape.dims.x as i32,
            shape.dims.y as i32,
            color,
        );
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
            d.draw_rectangle(
                ctransform.pos.x as i32,
                ctransform.pos.y as i32,
                shape.dims.x as i32,
                shape.dims.y as i32,
                block.color,
            );
        } else {
            draw_rect_outline(d, ctransform.pos, shape.dims, block.color);
            if health.hp > 1 {
                d.draw_line_v(
                    Vector2::new(ctransform.pos.x, ctransform.pos.y),
                    Vector2::new(
                        ctransform.pos.x + shape.dims.x - 1.0,
                        ctransform.pos.y + shape.dims.y - 1.0,
                    ),
                    block.color,
                );
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
