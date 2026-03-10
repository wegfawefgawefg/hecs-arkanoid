use hecs::World;
use raylib::prelude::Color;

use crate::{
    components::{Ball, CTransform, ImpactParticle, Physics, ScorePopup, Shape},
    entity_archetypes::spawn_impact_particle,
    state::{DeletionEvent, State, FRAMES_PER_SECOND},
};

pub fn step(ecs: &mut World, state: &mut State) {
    spawn_ball_trails(ecs, state);
    let dt = 1.0 / FRAMES_PER_SECOND as f32;
    let entities: Vec<_> = ecs
        .query::<(hecs::Entity, &ImpactParticle)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    for entity in entities {
        let Ok((ctransform, physics, particle)) =
            ecs.query_one_mut::<(&mut CTransform, &mut Physics, &mut ImpactParticle)>(entity)
        else {
            continue;
        };

        ctransform.pos += physics.vel * dt;
        physics.vel *= 0.88;
        particle.frames_left = particle.frames_left.saturating_sub(1);

        if particle.frames_left == 0 {
            state.deletion_events.push(DeletionEvent::Entity { entity });
        }
    }

    let popup_entities: Vec<_> = ecs
        .query::<(hecs::Entity, &ScorePopup)>()
        .iter()
        .map(|(entity, _)| entity)
        .collect();

    for entity in popup_entities {
        let Ok((ctransform, physics, popup)) =
            ecs.query_one_mut::<(&mut CTransform, &mut Physics, &mut ScorePopup)>(entity)
        else {
            continue;
        };

        ctransform.pos += physics.vel * dt;
        physics.vel *= 0.95;
        popup.frames_left = popup.frames_left.saturating_sub(1);

        if popup.frames_left == 0 {
            state.deletion_events.push(DeletionEvent::Entity { entity });
        }
    }
}

fn spawn_ball_trails(ecs: &mut World, state: &State) {
    if (state.t as u32) % 2 != 0 {
        return;
    }

    let balls: Vec<_> = ecs
        .query::<(&Ball, &CTransform, &Physics, &Shape)>()
        .iter()
        .map(|(_, ctransform, physics, shape)| (ctransform.pos, physics.vel, shape.dims))
        .collect();

    for (pos, vel, dims) in balls {
        let speed = vel.length();
        if speed < 70.0 {
            continue;
        }

        let life = if speed > 120.0 { 10 } else { 7 };
        let size = if speed > 120.0 { 2.0 } else { 1.0 };
        spawn_impact_particle(
            ecs,
            pos + dims * 0.25,
            -vel * 0.03,
            Color::new(220, 220, 220, 255),
            size,
            life,
        );
    }
}
