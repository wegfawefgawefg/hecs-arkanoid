use glam::Vec2;
use hecs::World;
use raylib::prelude::Color;

use crate::audio_playing::AudioCommand;
use crate::components::{
    AttachedTo, Ball, Block, CTransform, Health, Paddle, Physics, Shape, StrongBlock,
};
use crate::entity_archetypes::spawn_score_popup;
use crate::game_mode_transitions::BASE_PADDLE_SHAPE;
use crate::juice;
use crate::powerups::maybe_spawn_powerup_drop;
use crate::state::{DeletionEvent, State, FRAMES_PER_SECOND};
use crate::DIMS;

const BALL_SPEED: f32 = 100.0;
const PHYSICS_SUBSTEPS: usize = 4;

#[derive(Clone, Copy)]
struct Rect {
    pos: Vec2,
    dims: Vec2,
}

impl Rect {
    fn right(self) -> f32 {
        self.pos.x + self.dims.x
    }

    fn bottom(self) -> f32 {
        self.pos.y + self.dims.y
    }

    fn overlaps(self, other: Self) -> bool {
        self.pos.x < other.right()
            && self.right() > other.pos.x
            && self.pos.y < other.bottom()
            && self.bottom() > other.pos.y
    }
}

fn rect_for(transform: &CTransform, shape: &Shape) -> Rect {
    Rect {
        pos: transform.pos,
        dims: shape.dims,
    }
}

fn bounce_feedback(ecs: &mut World, state: &mut State, pos: Vec2, color: Color) {
    juice::add_hitstop(state, 1);
    juice::add_camera_shake(state, 0.6);
    juice::pulse_ball(state, 0.4);
    juice::spawn_hit_particles(ecs, pos, color, 4, 16.0);
}

fn resolve_rect_collision(ball_rect: Rect, previous_rect: Rect, other_rect: Rect) -> (Vec2, bool) {
    let mut corrected_pos = ball_rect.pos;
    let mut hit_horizontal = false;

    if previous_rect.bottom() <= other_rect.pos.y {
        corrected_pos.y = other_rect.pos.y - ball_rect.dims.y;
    } else if previous_rect.pos.y >= other_rect.bottom() {
        corrected_pos.y = other_rect.bottom();
    } else if previous_rect.right() <= other_rect.pos.x {
        corrected_pos.x = other_rect.pos.x - ball_rect.dims.x;
        hit_horizontal = true;
    } else if previous_rect.pos.x >= other_rect.right() {
        corrected_pos.x = other_rect.right();
        hit_horizontal = true;
    } else {
        let overlap_left = ball_rect.right() - other_rect.pos.x;
        let overlap_right = other_rect.right() - ball_rect.pos.x;
        let overlap_top = ball_rect.bottom() - other_rect.pos.y;
        let overlap_bottom = other_rect.bottom() - ball_rect.pos.y;

        let min_x = overlap_left.min(overlap_right);
        let min_y = overlap_top.min(overlap_bottom);

        if min_x < min_y {
            hit_horizontal = true;
            corrected_pos.x = if overlap_left < overlap_right {
                other_rect.pos.x - ball_rect.dims.x
            } else {
                other_rect.right()
            };
        } else {
            corrected_pos.y = if overlap_top < overlap_bottom {
                other_rect.pos.y - ball_rect.dims.y
            } else {
                other_rect.bottom()
            };
        }
    }

    (corrected_pos, hit_horizontal)
}

fn ball_hits_paddle_side(
    ecs: &World,
    ball_entity: hecs::Entity,
    paddle_entity: hecs::Entity,
) -> Option<f32> {
    let mut paddle_query = ecs.query_one::<(&Paddle, &CTransform, &Shape)>(paddle_entity);
    let (_, paddle_transform, paddle_shape) = paddle_query.get().ok()?;
    let paddle_start = paddle_transform.pos.x;
    let paddle_end = paddle_start + paddle_shape.dims.x;

    let mut ball_query = ecs.query_one::<(&Ball, &CTransform, &Shape)>(ball_entity);
    let (_, ball_transform, ball_shape) = ball_query.get().ok()?;
    let ball_center = ball_transform.pos.x + ball_shape.dims.x / 2.0;

    let left_third_end = paddle_start + (paddle_end - paddle_start) / 3.0;
    if ball_center > paddle_start && ball_center < left_third_end {
        return Some(-1.0);
    }

    let right_third_start = paddle_end - (paddle_end - paddle_start) / 3.0;
    if ball_center > right_third_start && ball_center < paddle_end {
        return Some(1.0);
    }

    None
}

pub fn sync_ecs_to_physics(_ecs: &World, _state: &mut State) {}

pub fn set_ball_to_angle(ecs: &World, state: &mut State) {
    for physics in ecs.query::<&mut Physics>().with::<&Ball>().iter() {
        let x_sign = if physics.vel.x == 0.0 {
            1.0
        } else {
            physics.vel.x.signum()
        };
        let y_sign = if physics.vel.y == 0.0 {
            -1.0
        } else {
            physics.vel.y.signum()
        };
        let angle = std::f32::consts::PI / 3.0;
        physics.vel.x = angle.cos() * BALL_SPEED * state.ball_speed_scale * x_sign;
        physics.vel.y = angle.sin() * BALL_SPEED * state.ball_speed_scale * y_sign;
    }
}

pub fn step_physics(ecs: &mut World, state: &mut State) {
    let dt = 1.0 / FRAMES_PER_SECOND as f32 / PHYSICS_SUBSTEPS as f32;
    update_attached_balls(ecs, state);
    let paddle = ecs
        .query::<(hecs::Entity, &Paddle, &CTransform, &Shape)>()
        .iter()
        .next()
        .map(|(entity, _, transform, shape)| (entity, rect_for(transform, shape)));

    let ball_entities: Vec<_> = ecs
        .query::<(hecs::Entity, &Ball)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    for ball_entity in ball_entities {
        let mut dropped = false;

        for _ in 0..PHYSICS_SUBSTEPS {
            if ecs.satisfies::<&AttachedTo>(ball_entity) {
                break;
            }

            let Some((mut next_pos, mut next_vel, previous_rect, mut ball_rect, ball_dims)) =
                (|| {
                    let (ball_transform, ball_physics, ball_shape) = ecs
                        .query_one_mut::<(&CTransform, &Physics, &Shape)>(ball_entity)
                        .ok()?;
                    let next_pos = ball_transform.pos + ball_physics.vel * dt;
                    let next_vel = ball_physics.vel;
                    let previous_rect = rect_for(ball_transform, ball_shape);
                    let ball_rect = Rect {
                        pos: next_pos,
                        dims: ball_shape.dims,
                    };
                    Some((
                        next_pos,
                        next_vel,
                        previous_rect,
                        ball_rect,
                        ball_shape.dims,
                    ))
                })()
            else {
                break;
            };

            if ball_rect.pos.x <= 0.0 {
                ball_rect.pos.x = 0.0;
                next_pos.x = 0.0;
                next_vel.x = next_vel.x.abs();
                bounce_feedback(ecs, state, next_pos + ball_dims * 0.5, Color::WHITE);
                juice::nudge_camera(state, Vec2::new(1.0, 0.0), 1.0);
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallWallBounce);
            } else if ball_rect.right() >= DIMS.x as f32 - 1.0 {
                ball_rect.pos.x = DIMS.x as f32 - 1.0 - ball_rect.dims.x;
                next_pos.x = ball_rect.pos.x;
                next_vel.x = -next_vel.x.abs();
                bounce_feedback(ecs, state, next_pos + ball_dims * 0.5, Color::WHITE);
                juice::nudge_camera(state, Vec2::new(-1.0, 0.0), 1.0);
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallWallBounce);
            }

            if ball_rect.pos.y <= 0.0 {
                ball_rect.pos.y = 0.0;
                next_pos.y = 0.0;
                next_vel.y = next_vel.y.abs();
                bounce_feedback(ecs, state, next_pos + ball_dims * 0.5, Color::WHITE);
                juice::nudge_camera(state, Vec2::new(0.0, 1.0), 1.0);
                state
                    .audio_command_buffer
                    .push(AudioCommand::BallWallBounce);
            } else if ball_rect.bottom() >= DIMS.y as f32 - 1.0 {
                juice::add_hitstop(state, 5);
                juice::add_camera_shake(state, 2.0);
                juice::nudge_camera(state, Vec2::new(0.0, -1.0), 1.6);
                juice::add_zoom_pulse(state, 0.015);
                juice::add_screen_flash(state, 0.18);
                juice::spawn_hit_particles(ecs, next_pos + ball_dims * 0.5, Color::RED, 10, 26.0);
                state.audio_command_buffer.push(AudioCommand::BallDrop);
                state.deletion_events.push(DeletionEvent::Entity {
                    entity: ball_entity,
                });
                dropped = true;
                break;
            }

            if let Some((paddle_entity, paddle_rect)) = paddle {
                if ball_rect.overlaps(paddle_rect) {
                    let (corrected_pos, hit_horizontal) =
                        resolve_rect_collision(ball_rect, previous_rect, paddle_rect);
                    next_pos = corrected_pos;

                    if hit_horizontal {
                        if previous_rect.right() <= paddle_rect.pos.x {
                            next_vel.x = -next_vel.x.abs();
                        } else if previous_rect.pos.x >= paddle_rect.right() {
                            next_vel.x = next_vel.x.abs();
                        } else {
                            next_vel.x = -next_vel.x;
                        }
                    } else {
                        if previous_rect.bottom() <= paddle_rect.pos.y {
                            if state.sticky_mode {
                                let x_offset = next_pos.x - paddle_rect.pos.x + ball_dims.x / 2.0;
                                next_pos.y = paddle_rect.pos.y - ball_dims.y - 1.0;
                                next_vel = Vec2::ZERO;
                                let _ = ecs.insert_one(
                                    ball_entity,
                                    AttachedTo {
                                        entity: paddle_entity,
                                        offset: Vec2::new(x_offset, -ball_dims.y - 1.0),
                                    },
                                );
                            } else {
                                next_vel.y = -next_vel.y.abs();
                                if let Some(new_direction) =
                                    ball_hits_paddle_side(ecs, ball_entity, paddle_entity)
                                {
                                    next_vel.x =
                                        BALL_SPEED * state.ball_speed_scale * new_direction;
                                }
                            }
                        } else {
                            next_vel.y = next_vel.y.abs();
                        }
                    }

                    juice::add_hitstop(state, 1);
                    juice::add_camera_shake(state, 0.8);
                    juice::pulse_paddle(state, 0.7, 1.25);
                    juice::pulse_ball(state, 0.35);
                    juice::spawn_hit_particles(
                        ecs,
                        next_pos + ball_dims * 0.5,
                        Color::RAYWHITE,
                        5,
                        18.0,
                    );
                    state
                        .audio_command_buffer
                        .push(AudioCommand::BallPaddleBounce);
                    if let Ok((ball_transform, ball_physics)) =
                        ecs.query_one_mut::<(&mut CTransform, &mut Physics)>(ball_entity)
                    {
                        ball_transform.pos = next_pos;
                        ball_physics.vel = next_vel;
                    }
                    continue;
                }
            }

            let mut hit_block = None;
            for (block_entity, _, block_transform, block_shape) in ecs
                .query::<(hecs::Entity, &Block, &CTransform, &Shape)>()
                .iter()
            {
                let block_rect = rect_for(block_transform, block_shape);
                if ball_rect.overlaps(block_rect) {
                    hit_block = Some((block_entity, block_rect));
                    break;
                }
            }

            if let Some((block_entity, block_rect)) = hit_block {
                let strong_block = ecs.satisfies::<&StrongBlock>(block_entity);
                if !state.fireball_mode || strong_block {
                    let (corrected_pos, hit_horizontal) =
                        resolve_rect_collision(ball_rect, previous_rect, block_rect);
                    next_pos = corrected_pos;
                    if hit_horizontal {
                        next_vel.x = -next_vel.x;
                    } else {
                        next_vel.y = -next_vel.y;
                    }
                }

                if strong_block {
                    state.score = state.score.saturating_add(5);
                    spawn_score_popup(ecs, block_rect.pos + block_rect.dims * 0.5, 5, Color::GRAY);
                    juice::add_hitstop(state, 1);
                    juice::add_camera_shake(state, 0.7);
                    juice::pulse_ball(state, 0.3);
                    juice::spawn_hit_particles(
                        ecs,
                        block_rect.pos + block_rect.dims * 0.5,
                        Color::GRAY,
                        4,
                        14.0,
                    );
                    state
                        .audio_command_buffer
                        .push(AudioCommand::BallSturdyBlockBounce);
                } else if let Ok((block, health)) =
                    ecs.query_one_mut::<(&Block, &mut Health)>(block_entity)
                {
                    let block_color = block.color;
                    let hit_pos = block_rect.pos + block_rect.dims * 0.5;
                    state.score = state.score.saturating_add(10);
                    if health.hp > 0 {
                        health.hp -= 1;
                    }
                    if health.hp == 0 {
                        state.score = state.score.saturating_add(90);
                        let _ = health;
                        let _ = block;
                        spawn_score_popup(ecs, hit_pos, 10, block_color);
                        spawn_score_popup(ecs, hit_pos, 90, block_color);
                        juice::add_hitstop(state, 3);
                        juice::add_camera_shake(state, 1.6);
                        juice::add_zoom_pulse(state, 0.02);
                        juice::add_screen_flash(state, 0.12);
                        juice::pulse_ball(state, 0.5);
                        juice::spawn_hit_particles(ecs, hit_pos, block_color, 10, 24.0);
                        maybe_spawn_powerup_drop(
                            ecs,
                            state,
                            block_rect.pos + block_rect.dims / 2.0,
                        );
                        state
                            .audio_command_buffer
                            .push(AudioCommand::BallBlockBounce);
                        state.deletion_events.push(DeletionEvent::Entity {
                            entity: block_entity,
                        });
                    } else {
                        let _ = health;
                        let _ = block;
                        spawn_score_popup(ecs, hit_pos, 10, block_color);
                        juice::add_hitstop(state, 1);
                        juice::add_camera_shake(state, 0.65);
                        juice::pulse_ball(state, 0.3);
                        juice::spawn_hit_particles(ecs, hit_pos, block_color, 5, 16.0);
                        state
                            .audio_command_buffer
                            .push(AudioCommand::BallSturdyBlockBounce);
                    }
                }
            }

            if let Ok((ball_transform, ball_physics)) =
                ecs.query_one_mut::<(&mut CTransform, &mut Physics)>(ball_entity)
            {
                ball_transform.pos = next_pos;
                ball_physics.vel = next_vel;
            }
        }

        if dropped {
            continue;
        }
    }
}

fn update_attached_balls(ecs: &mut World, state: &mut State) {
    let attached_balls: Vec<_> = ecs
        .query::<(hecs::Entity, &AttachedTo, &Shape)>()
        .with::<&Ball>()
        .iter()
        .map(|(entity, attached_to, shape)| {
            (entity, attached_to.entity, attached_to.offset, shape.dims)
        })
        .collect();

    for (ball_entity, owner_entity, offset, dims) in attached_balls {
        let owner_pos = {
            let mut query = ecs.query_one::<&CTransform>(owner_entity);
            query.get().ok().map(|transform| transform.pos)
        };

        let Some(owner_pos) = owner_pos else {
            continue;
        };

        if let Ok((ball_transform, ball_physics)) =
            ecs.query_one_mut::<(&mut CTransform, &mut Physics)>(ball_entity)
        {
            ball_transform.pos = owner_pos + offset;
            ball_physics.vel = Vec2::ZERO;
        }

        if state.playing_inputs.shoot {
            let launch_x = if offset.x < BASE_PADDLE_SHAPE.x / 3.0 {
                -1.0
            } else if offset.x > BASE_PADDLE_SHAPE.x * 2.0 / 3.0 {
                1.0
            } else {
                0.0
            };
            if let Ok((ball_transform, ball_physics)) =
                ecs.query_one_mut::<(&mut CTransform, &mut Physics)>(ball_entity)
            {
                ball_transform.pos.y = owner_pos.y - dims.y - 1.0;
                ball_physics.vel = Vec2::new(
                    BALL_SPEED * state.ball_speed_scale * launch_x,
                    -BALL_SPEED * state.ball_speed_scale,
                );
            }
            let _ = ecs.remove_one::<AttachedTo>(ball_entity);
        }
    }
}

#[allow(dead_code)]
pub fn constantly_resize_paddle(ecs: &mut World, state: &mut State) {
    let new_shape = Vec2::new(
        BASE_PADDLE_SHAPE.x * (1.0 + (state.t * 0.1).sin() / 2.0) + 10.0,
        BASE_PADDLE_SHAPE.y,
    );
    for shape in ecs.query::<&mut Shape>().with::<&Paddle>().iter() {
        shape.dims = new_shape;
    }
}

pub fn respond_to_collisions(_ecs: &mut World, _state: &mut State) {}

#[allow(dead_code)]
pub fn boundary_checking(_ecs: &World, _state: &mut State) {}
