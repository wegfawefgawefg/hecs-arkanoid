use glam::Vec2;
use hecs::World;
use raylib::prelude::Color;

use crate::{
    components::ImpactParticleKind,
    entity_archetypes::{spawn_effect_particle, spawn_impact_particle},
    state::State,
    DIMS,
};

const MAX_SHAKE: f32 = 1.5;
const MAX_ZOOM_PULSE: f32 = 0.04;

#[derive(Clone, Copy)]
pub enum BlockBreakEffect {
    Bounce,
    Fireball,
    Laser,
    StrongHit,
}

pub fn update(state: &mut State) {
    state.camera_shake *= 0.82;
    if state.camera_shake < 0.05 {
        state.camera_shake = 0.0;
    }

    state.camera_zoom_pulse *= 0.82;
    if state.camera_zoom_pulse < 0.001 {
        state.camera_zoom_pulse = 0.0;
    }

    state.screen_flash *= 0.74;
    if state.screen_flash < 0.01 {
        state.screen_flash = 0.0;
    }

    state.paddle_pulse *= 0.72;
    if state.paddle_pulse < 0.01 {
        state.paddle_pulse = 0.0;
    }

    state.ball_pulse *= 0.78;
    if state.ball_pulse < 0.01 {
        state.ball_pulse = 0.0;
    }

    state.paddle_recoil *= 0.6;
    if state.paddle_recoil.abs() < 0.01 {
        state.paddle_recoil = 0.0;
    }

    state.camera_impulse *= 0.76;
    if state.camera_impulse.length_squared() < 0.01 {
        state.camera_impulse = Vec2::ZERO;
    }
}

pub fn consume_hitstop(state: &mut State) -> bool {
    if state.hitstop_frames > 0 {
        state.hitstop_frames -= 1;
        return true;
    }
    false
}

pub fn add_hitstop(state: &mut State, frames: u32) {
    state.hitstop_frames = state.hitstop_frames.max(frames);
}

pub fn add_camera_shake(state: &mut State, amount: f32) {
    state.camera_shake = (state.camera_shake + amount).clamp(0.0, MAX_SHAKE);
}

pub fn nudge_camera(state: &mut State, dir: Vec2, amount: f32) {
    let safe_dir = if dir.length_squared() > 0.0 {
        dir.normalize()
    } else {
        Vec2::ZERO
    };
    state.camera_impulse += safe_dir * amount;
    let max = 2.0;
    if state.camera_impulse.length() > max {
        state.camera_impulse = state.camera_impulse.normalize() * max;
    }
}

pub fn add_zoom_pulse(state: &mut State, amount: f32) {
    state.camera_zoom_pulse = (state.camera_zoom_pulse + amount).clamp(0.0, MAX_ZOOM_PULSE);
}

pub fn add_screen_flash(state: &mut State, amount: f32) {
    state.screen_flash = (state.screen_flash + amount).clamp(0.0, 1.0);
}

pub fn pulse_paddle(state: &mut State, amount: f32, recoil: f32) {
    state.paddle_pulse = (state.paddle_pulse + amount).clamp(0.0, 1.0);
    state.paddle_recoil = (state.paddle_recoil + recoil).clamp(-4.0, 4.0);
}

pub fn pulse_ball(state: &mut State, amount: f32) {
    state.ball_pulse = (state.ball_pulse + amount).clamp(0.0, 1.0);
}

pub fn world_offset(state: &State) -> Vec2 {
    let phase = state.t * 0.37;
    let shake = if state.camera_shake > 0.0 {
        Vec2::new((phase * 2.7).sin(), (phase * 3.9).cos()) * state.camera_shake * 0.35
    } else {
        Vec2::ZERO
    };
    state.camera_impulse + shake
}

pub fn world_scale(state: &State) -> f32 {
    1.0 + state.camera_zoom_pulse
}

pub fn world_pos(state: &State, pos: Vec2) -> Vec2 {
    let center = DIMS.as_vec2() * 0.5;
    let scale = world_scale(state);
    (pos - center) * scale + center + world_offset(state)
}

pub fn world_dims(state: &State, dims: Vec2) -> Vec2 {
    dims * world_scale(state)
}

pub fn text_pulse_size(state: &State, base: i32, amplitude: f32) -> i32 {
    let pulse = (state.t * 0.15).sin().abs();
    (base as f32 + pulse * amplitude).round() as i32
}

pub fn spawn_hit_particles(ecs: &mut World, pos: Vec2, color: Color, count: u32, speed: f32) {
    for i in 0..count {
        let t = i as f32 / count.max(1) as f32;
        let angle = t * std::f32::consts::TAU;
        let vel = Vec2::new(angle.cos(), angle.sin()) * speed * (0.7 + t * 0.6);
        let size = if i % 3 == 0 { 2.0 } else { 1.0 };
        spawn_impact_particle(ecs, pos, vel, color, size, 10 + (i % 5));
    }
}

pub fn spawn_fireball_trail(ecs: &mut World, pos: Vec2, vel: Vec2) {
    spawn_effect_particle(
        ecs,
        pos + Vec2::new(0.0, 1.0),
        Vec2::new(-vel.x * 0.02, -10.0 - vel.length() * 0.03),
        Color::new(90, 90, 90, 255),
        Vec2::new(2.0, 2.0),
        16,
        ImpactParticleKind::Smoke,
        -5.0,
        0.93,
        0.12,
    );

    spawn_effect_particle(
        ecs,
        pos + Vec2::new(1.0, 1.0),
        Vec2::new(-vel.x * 0.015, -4.0),
        Color::new(255, 80, 40, 255),
        Vec2::new(1.0, 1.0),
        9,
        ImpactParticleKind::Ember,
        -2.0,
        0.9,
        0.0,
    );
}

pub fn spawn_block_break_effect(
    ecs: &mut World,
    pos: Vec2,
    dims: Vec2,
    color: Color,
    effect: BlockBreakEffect,
) {
    let center = pos + dims * 0.5;
    match effect {
        BlockBreakEffect::Bounce => {
            for i in 0..8 {
                let t = i as f32 / 8.0;
                let angle = t * std::f32::consts::TAU;
                let vel = Vec2::new(angle.cos(), angle.sin()) * (16.0 + t * 8.0);
                spawn_effect_particle(
                    ecs,
                    center,
                    vel,
                    color,
                    Vec2::new(2.0, 2.0),
                    14,
                    ImpactParticleKind::Shard,
                    18.0,
                    0.9,
                    -0.02,
                );
            }
        }
        BlockBreakEffect::Fireball => {
            for i in 0..10 {
                let t = i as f32 / 10.0;
                let angle = t * std::f32::consts::TAU;
                let vel = Vec2::new(angle.cos(), angle.sin()) * (10.0 + t * 10.0);
                let ember_color = if i % 2 == 0 {
                    Color::new(255, 70, 30, 255)
                } else {
                    Color::new(255, 180, 40, 255)
                };
                spawn_effect_particle(
                    ecs,
                    center,
                    vel + Vec2::new(0.0, -8.0),
                    ember_color,
                    Vec2::new(1.0, 1.0),
                    12 + (i % 5),
                    ImpactParticleKind::Ember,
                    -3.0,
                    0.9,
                    0.0,
                );
            }
            for i in 0..5 {
                let x = pos.x + (i as f32 / 4.0) * dims.x;
                spawn_effect_particle(
                    ecs,
                    Vec2::new(x, center.y),
                    Vec2::new((i as f32 - 2.0) * 2.0, -10.0 - i as f32),
                    Color::new(80, 80, 80, 255),
                    Vec2::new(2.0, 2.0),
                    18,
                    ImpactParticleKind::Smoke,
                    -4.0,
                    0.94,
                    0.14,
                );
            }
            spawn_effect_particle(
                ecs,
                center,
                Vec2::new(0.0, 8.0),
                Color::new(255, 120, 20, 255),
                dims * 0.5,
                10,
                ImpactParticleKind::Melt,
                10.0,
                0.9,
                -0.08,
            );
        }
        BlockBreakEffect::Laser => {
            for i in 0..6 {
                let x = pos.x + 2.0 + i as f32 * ((dims.x - 4.0) / 5.0);
                spawn_effect_particle(
                    ecs,
                    Vec2::new(x, pos.y),
                    Vec2::new(0.0, -14.0 - i as f32),
                    Color::new(255, 80, 80, 255),
                    Vec2::new(1.0, dims.y * 0.5),
                    8,
                    ImpactParticleKind::LaserStreak,
                    -4.0,
                    0.92,
                    -0.1,
                );
            }
            for i in 0..6 {
                let t = i as f32 / 6.0;
                let angle = -std::f32::consts::FRAC_PI_2 + (t - 0.5) * 0.8;
                let vel = Vec2::new(angle.cos(), angle.sin()) * 18.0;
                spawn_effect_particle(
                    ecs,
                    center,
                    vel,
                    Color::new(255, 220, 220, 255),
                    Vec2::new(1.0, 2.0),
                    10,
                    ImpactParticleKind::LaserStreak,
                    0.0,
                    0.9,
                    -0.06,
                );
            }
        }
        BlockBreakEffect::StrongHit => {
            for i in 0..5 {
                let t = i as f32 / 5.0;
                let angle = t * std::f32::consts::TAU;
                let vel = Vec2::new(angle.cos(), angle.sin()) * 12.0;
                spawn_effect_particle(
                    ecs,
                    center,
                    vel,
                    Color::GRAY,
                    Vec2::new(1.0, 1.0),
                    10,
                    ImpactParticleKind::Shard,
                    12.0,
                    0.88,
                    0.0,
                );
            }
        }
    }
}
