use glam::Vec2;
use hecs::World;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use crate::{
    components::{
        Ball, BallEater, Block, CTransform, Health, ImpactParticle, ImpactParticleKind, LaserShot,
        Paddle, Physics, PowerUp, PowerUpDrop, PowerUpType, ScorePopup, Shape, StrongBlock, Wall,
    },
    juice,
    render_helpers::{draw_powerup_symbol, draw_rect_outline, powerup_color, snap_rect},
    state::State,
    DIMS,
};

fn world_rect(state: &State, pos: Vec2, dims: Vec2) -> (Vec2, Vec2) {
    (juice::world_pos(state, pos), juice::world_dims(state, dims))
}

fn draw_world_outline(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    state: &State,
    pos: Vec2,
    dims: Vec2,
    color: Color,
) {
    let (pos, dims) = world_rect(state, pos, dims);
    draw_rect_outline(d, pos, dims, color);
}

fn fade_color(color: Color, alpha: f32) -> Color {
    Color::new(
        color.r,
        color.g,
        color.b,
        (alpha.clamp(0.0, 1.0) * 255.0) as u8,
    )
}

fn draw_active_powerup(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    active_y: &mut i32,
    kind: PowerUpType,
    label: &str,
) {
    let color = powerup_color(kind);
    let pos = Vec2::new(6.0, *active_y as f32);
    draw_rect_outline(d, pos, Vec2::new(8.0, 8.0), color);
    draw_powerup_symbol(d, pos, kind, color);
    d.draw_text(label, 18, *active_y, 8, Color::WHITE);
    *active_y += 10;
}

fn draw_paddle_mode(
    d: &mut RaylibTextureMode<RaylibDrawHandle>,
    state: &State,
    pos: Vec2,
    dims: Vec2,
) {
    let (pos, dims) = world_rect(state, pos, dims);
    let (left, top, width, _height) = snap_rect(pos, dims);

    if state.sticky_mode {
        let cup_y = top - 1;
        d.draw_line(left + 3, cup_y, left + 3, cup_y + 3, Color::LIME);
        d.draw_line(
            left + width - 4,
            cup_y,
            left + width - 4,
            cup_y + 3,
            Color::LIME,
        );
        d.draw_line(
            left + 3,
            cup_y + 3,
            left + width - 4,
            cup_y + 3,
            Color::LIME,
        );
    }

    if state.laser_mode {
        d.draw_rectangle(left + 2, top - 2, 1, 3, Color::RED);
        d.draw_rectangle(left + width - 3, top - 2, 1, 3, Color::RED);
        d.draw_pixel(left + 2, top - 3, Color::WHITE);
        d.draw_pixel(left + width - 3, top - 3, Color::WHITE);
    }

    if state.fireball_mode {
        d.draw_line(
            left + width / 2 - 2,
            top - 2,
            left + width / 2,
            top - 4,
            Color::ORANGE,
        );
        d.draw_line(
            left + width / 2,
            top - 4,
            left + width / 2 + 2,
            top - 2,
            Color::YELLOW,
        );
    }
}

pub fn render(ecs: &World, state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    for physics in ecs.query::<&Physics>().with::<&Ball>().iter() {
        d.draw_text(
            format!("vel: {}", physics.vel).as_str(),
            20,
            20,
            1,
            Color::new(255, 255, 255, 10),
        );
    }

    for (entity, ctransform, shape) in ecs
        .query::<(hecs::Entity, &CTransform, &Shape)>()
        .with::<&Wall>()
        .iter()
    {
        let mut color = Color::WHITE;
        let mut r = ecs.query_one::<&BallEater>(entity);
        if r.get().is_ok() {
            color = Color::RED;
        }
        let (pos, dims) = world_rect(state, ctransform.pos, shape.dims);
        let (left, top, width, height) = snap_rect(pos, dims);
        d.draw_rectangle(left, top, width, height, color);
    }

    for (_, ctransform, shape) in ecs.query::<(&Paddle, &CTransform, &Shape)>().iter() {
        let mut dims = shape.dims;
        dims.x *= 1.0 + state.paddle_pulse * 0.08;
        dims.y *= 1.0 - state.paddle_pulse * 0.18;
        let pos = ctransform.pos + Vec2::new(0.0, state.paddle_recoil);
        draw_world_outline(d, state, pos, dims, Color::RAYWHITE);
        draw_paddle_mode(d, state, pos, dims);
    }

    for (entity, block, ctransform, shape, health) in ecs
        .query::<(hecs::Entity, &Block, &CTransform, &Shape, &Health)>()
        .iter()
    {
        let color = block.color;
        let (pos, dims) = world_rect(state, ctransform.pos, shape.dims);
        let strong = ecs.satisfies::<&StrongBlock>(entity);
        if strong {
            let (left, top, width, height) = snap_rect(pos, dims);
            d.draw_rectangle(left, top, width, height, color);
        } else {
            draw_rect_outline(d, pos, dims, color);
            if health.hp > 1 {
                let (left, top, width, height) = snap_rect(pos, dims);
                d.draw_line(left, top, left + width - 1, top + height - 1, color);
            }
        }
    }

    for (_, ctransform, shape) in ecs.query::<(&Ball, &CTransform, &Shape)>().iter() {
        let mut dims = shape.dims;
        dims *= 1.0 + state.ball_pulse * 0.12;
        let ball_color = if state.fireball_mode {
            Color::new(255, 80, 40, 255)
        } else {
            Color::RAYWHITE
        };
        draw_world_outline(d, state, ctransform.pos, dims, ball_color);
        if state.fireball_mode {
            let (pos, dims) = world_rect(
                state,
                ctransform.pos - Vec2::ONE,
                shape.dims + Vec2::splat(2.0),
            );
            let (left, top, width, height) = snap_rect(pos, dims);
            d.draw_rectangle_lines(left, top, width, height, Color::ORANGE);
            d.draw_pixel(left + width / 2, top - 1, Color::YELLOW);
        }
    }

    for (_, ctransform, shape) in ecs.query::<(&LaserShot, &CTransform, &Shape)>().iter() {
        let (pos, dims) = world_rect(state, ctransform.pos, shape.dims);
        let (left, top, width, height) = snap_rect(pos, dims);
        d.draw_rectangle(left, top, width, height, Color::RED);
        d.draw_rectangle(left, top, width.max(1), 1, Color::WHITE);
    }

    for (_, ctransform, shape, power_up) in ecs
        .query::<(&PowerUpDrop, &CTransform, &Shape, &PowerUp)>()
        .iter()
    {
        let bob = (state.t * 0.18 + ctransform.pos.x * 0.1).sin() * 0.8;
        let color = powerup_color(power_up.power_up_type);
        let pos = ctransform.pos + Vec2::new(0.0, bob);
        let (screen_pos, screen_dims) = world_rect(state, pos, shape.dims);
        draw_rect_outline(d, screen_pos, screen_dims, color);
        draw_powerup_symbol(d, screen_pos, power_up.power_up_type, color);
    }

    for (particle, ctransform, shape) in
        ecs.query::<(&ImpactParticle, &CTransform, &Shape)>().iter()
    {
        let alpha = particle.frames_left as f32 / particle.max_frames.max(1) as f32;
        let color = fade_color(particle.color, alpha);
        let (pos, dims) = world_rect(state, ctransform.pos, shape.dims);
        let (left, top, width, height) = snap_rect(pos, dims);
        match particle.kind {
            ImpactParticleKind::Square | ImpactParticleKind::Shard => {
                d.draw_rectangle(left, top, width, height, color);
            }
            ImpactParticleKind::Smoke => {
                d.draw_rectangle(left, top, width, height, color);
                if width > 1 && height > 1 {
                    d.draw_rectangle(
                        left + 1,
                        top + 1,
                        (width - 1).max(1),
                        (height - 1).max(1),
                        fade_color(Color::BLACK, alpha * 0.35),
                    );
                }
            }
            ImpactParticleKind::Ember => {
                d.draw_rectangle(left, top, width.max(1), height.max(1), color);
                d.draw_pixel(left, top, Color::YELLOW);
            }
            ImpactParticleKind::LaserStreak => {
                d.draw_rectangle(left, top, width.max(1), height.max(1), color);
                d.draw_rectangle(left, top, width.max(1), 1, Color::WHITE);
            }
            ImpactParticleKind::Melt => {
                d.draw_rectangle(left, top, width.max(1), height.max(1), color);
                d.draw_line(
                    left + width / 2,
                    top,
                    left + width / 2,
                    top + height.max(1) - 1,
                    Color::YELLOW,
                );
            }
        }
    }

    for (popup, ctransform) in ecs.query::<(&ScorePopup, &CTransform)>().iter() {
        let alpha = popup.frames_left as f32 / popup.max_frames.max(1) as f32;
        let color = fade_color(popup.color, alpha);
        let pos = juice::world_pos(state, ctransform.pos);
        d.draw_text(
            format!("+{}", popup.value).as_str(),
            pos.x.round() as i32,
            pos.y.round() as i32,
            8,
            color,
        );
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
        draw_active_powerup(d, &mut active_y, PowerUpType::Enlarge, "Wide");
    } else if state.paddle_width_scale < 0.99 {
        draw_active_powerup(d, &mut active_y, PowerUpType::Shrink, "Narrow");
    }
    if state.ball_speed_scale > 1.01 {
        draw_active_powerup(d, &mut active_y, PowerUpType::SpeedUp, "Fast");
    } else if state.ball_speed_scale < 0.99 {
        draw_active_powerup(d, &mut active_y, PowerUpType::SlowDown, "Slow");
    }
    if state.laser_mode {
        draw_active_powerup(d, &mut active_y, PowerUpType::Lasers, "Laser");
    }
    if state.sticky_mode {
        draw_active_powerup(d, &mut active_y, PowerUpType::Catch, "Catch");
    }
    if state.fireball_mode {
        draw_active_powerup(d, &mut active_y, PowerUpType::BombBall, "Fire");
    }
}

#[allow(dead_code)]
pub fn render_physics(_state: &State) {}
