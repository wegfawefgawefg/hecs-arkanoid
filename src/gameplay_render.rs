use glam::Vec2;
use hecs::World;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    components::{
        Ball, BallEater, Block, CTransform, Health, LaserShot, Paddle, Physics, PowerUp,
        PowerUpDrop, PowerUpType, Shape, StrongBlock, Wall,
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

fn powerup_color(power_up_type: PowerUpType) -> Color {
    match power_up_type {
        PowerUpType::Enlarge => Color::GREEN,
        PowerUpType::Shrink => Color::PINK,
        PowerUpType::SpeedUp => Color::ORANGE,
        PowerUpType::SlowDown => Color::SKYBLUE,
        PowerUpType::BallSplit => Color::YELLOW,
        PowerUpType::Catch => Color::LIME,
        PowerUpType::ExtraLife => Color::GOLD,
        PowerUpType::Lasers => Color::RED,
        PowerUpType::BombBall => Color::MAGENTA,
    }
}

fn draw_powerup_symbol(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    pos: Vec2,
    power_up_type: PowerUpType,
    color: Color,
) {
    let x = pos.x.round() as i32;
    let y = pos.y.round() as i32;
    match power_up_type {
        PowerUpType::Enlarge => {
            d.draw_line(x + 1, y + 4, x + 6, y + 4, color);
            d.draw_line(x + 1, y + 4, x + 2, y + 3, color);
            d.draw_line(x + 1, y + 4, x + 2, y + 5, color);
            d.draw_line(x + 6, y + 4, x + 5, y + 3, color);
            d.draw_line(x + 6, y + 4, x + 5, y + 5, color);
        }
        PowerUpType::Shrink => {
            d.draw_line(x + 1, y + 4, x + 6, y + 4, color);
            d.draw_line(x + 2, y + 3, x + 3, y + 4, color);
            d.draw_line(x + 2, y + 5, x + 3, y + 4, color);
            d.draw_line(x + 5, y + 3, x + 4, y + 4, color);
            d.draw_line(x + 5, y + 5, x + 4, y + 4, color);
        }
        PowerUpType::SpeedUp => {
            d.draw_text(">>", x, y + 1, 6, color);
        }
        PowerUpType::SlowDown => {
            d.draw_text("<<", x, y + 1, 6, color);
        }
        PowerUpType::BallSplit => {
            d.draw_circle(x + 2, y + 2, 1.0, color);
            d.draw_circle(x + 5, y + 2, 1.0, color);
            d.draw_circle(x + 4, y + 5, 1.0, color);
        }
        PowerUpType::Catch => {
            d.draw_line(x + 2, y + 1, x + 2, y + 6, color);
            d.draw_line(x + 5, y + 1, x + 5, y + 6, color);
            d.draw_line(x + 2, y + 6, x + 5, y + 6, color);
        }
        PowerUpType::ExtraLife => {
            d.draw_line(x + 4, y + 1, x + 4, y + 6, color);
            d.draw_line(x + 2, y + 3, x + 6, y + 3, color);
        }
        PowerUpType::Lasers => {
            d.draw_rectangle(x + 2, y + 1, 1, 6, color);
            d.draw_rectangle(x + 5, y + 1, 1, 6, color);
        }
        PowerUpType::BombBall => {
            d.draw_circle_lines(x + 3, y + 4, 2.0, color);
            d.draw_line(x + 5, y + 2, x + 6, y + 1, color);
            d.draw_pixel(x + 6, y + 1, Color::YELLOW);
        }
    }
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

    for (_, ctransform, shape) in ecs.query::<(&LaserShot, &CTransform, &Shape)>().iter() {
        let (left, top, width, height) = snap_rect(ctransform.pos, shape.dims);
        d.draw_rectangle(left, top, width, height, Color::RED);
    }

    for (_, ctransform, shape, power_up) in ecs
        .query::<(&PowerUpDrop, &CTransform, &Shape, &PowerUp)>()
        .iter()
    {
        let color = powerup_color(power_up.power_up_type);
        draw_rect_outline(d, ctransform.pos, shape.dims, color);
        draw_powerup_symbol(d, ctransform.pos, power_up.power_up_type, color);
    }

    d.draw_text(
        format!("Score {}", state.score).as_str(),
        6,
        146,
        10,
        Color::WHITE,
    );
    d.draw_text(
        format!("Lives {}", state.lives).as_str(),
        92,
        146,
        10,
        Color::WHITE,
    );
    d.draw_text(
        format!("Level {}", state.level).as_str(),
        DIMS.x as i32 - 58,
        146,
        10,
        Color::WHITE,
    );

    let mut active_y = 6;
    if state.paddle_width_scale > 1.01 {
        let kind = PowerUpType::Enlarge;
        let color = powerup_color(kind);
        draw_rect_outline(
            d,
            Vec2::new(6.0, active_y as f32),
            Vec2::new(8.0, 8.0),
            color,
        );
        draw_powerup_symbol(d, Vec2::new(6.0, active_y as f32), kind, color);
        d.draw_text("Wide", 18, active_y, 8, Color::WHITE);
        active_y += 10;
    } else if state.paddle_width_scale < 0.99 {
        let kind = PowerUpType::Shrink;
        let color = powerup_color(kind);
        draw_rect_outline(
            d,
            Vec2::new(6.0, active_y as f32),
            Vec2::new(8.0, 8.0),
            color,
        );
        draw_powerup_symbol(d, Vec2::new(6.0, active_y as f32), kind, color);
        d.draw_text("Narrow", 18, active_y, 8, Color::WHITE);
        active_y += 10;
    }
    if state.ball_speed_scale > 1.01 {
        let kind = PowerUpType::SpeedUp;
        let color = powerup_color(kind);
        draw_rect_outline(
            d,
            Vec2::new(6.0, active_y as f32),
            Vec2::new(8.0, 8.0),
            color,
        );
        draw_powerup_symbol(d, Vec2::new(6.0, active_y as f32), kind, color);
        d.draw_text("Fast", 18, active_y, 8, Color::WHITE);
        active_y += 10;
    } else if state.ball_speed_scale < 0.99 {
        let kind = PowerUpType::SlowDown;
        let color = powerup_color(kind);
        draw_rect_outline(
            d,
            Vec2::new(6.0, active_y as f32),
            Vec2::new(8.0, 8.0),
            color,
        );
        draw_powerup_symbol(d, Vec2::new(6.0, active_y as f32), kind, color);
        d.draw_text("Slow", 18, active_y, 8, Color::WHITE);
        active_y += 10;
    }
    if state.laser_mode {
        let kind = PowerUpType::Lasers;
        let color = powerup_color(kind);
        draw_rect_outline(
            d,
            Vec2::new(6.0, active_y as f32),
            Vec2::new(8.0, 8.0),
            color,
        );
        draw_powerup_symbol(d, Vec2::new(6.0, active_y as f32), kind, color);
        d.draw_text("Laser", 18, active_y, 8, Color::WHITE);
        active_y += 10;
    }
    if state.sticky_mode {
        let kind = PowerUpType::Catch;
        let color = powerup_color(kind);
        draw_rect_outline(
            d,
            Vec2::new(6.0, active_y as f32),
            Vec2::new(8.0, 8.0),
            color,
        );
        draw_powerup_symbol(d, Vec2::new(6.0, active_y as f32), kind, color);
        d.draw_text("Catch", 18, active_y, 8, Color::WHITE);
        active_y += 10;
    }
    if state.fireball_mode {
        let kind = PowerUpType::BombBall;
        let color = powerup_color(kind);
        draw_rect_outline(
            d,
            Vec2::new(6.0, active_y as f32),
            Vec2::new(8.0, 8.0),
            color,
        );
        draw_powerup_symbol(d, Vec2::new(6.0, active_y as f32), kind, color);
        d.draw_text("Fire", 18, active_y, 8, Color::WHITE);
    }
}

#[allow(dead_code)]
pub fn render_physics(_state: &State) {}
