use hecs::World;

use crate::{
    components::{CTransform, ImpactParticle, Physics},
    state::{DeletionEvent, State, FRAMES_PER_SECOND},
};

pub fn step(ecs: &mut World, state: &mut State) {
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
}
