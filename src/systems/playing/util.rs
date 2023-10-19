use glam::Vec2;
use rand::{rngs::StdRng, Rng};

use crate::DIMS;

// #[system]
// #[write_component(LifeSpan)]
// pub fn step_lifespan(ecs: &mut SubWorld, cmd: &mut CommandBuffer) {
//     let mut query = <(Entity, &mut LifeSpan)>::query();
//     for (entity, lifespan) in query.iter_mut(ecs) {
//         lifespan.frames_left -= 1;
//         if lifespan.frames_left == 0 {
//             cmd.remove(*entity);
//         }
//     }
// }

// #[system]
// pub fn step_alerts(#[resource] expiring_messages: &mut ExpiringMessages) {
//     for message in expiring_messages.iter_mut() {
//         message.lifetime -= 1;
//     }
//     expiring_messages.retain(|message| message.lifetime > 0);
// }

pub fn get_random_pos_in_play_area(rng: &mut StdRng) -> Vec2 {
    Vec2::new(
        rng.gen_range(0.0..DIMS.x as f32),
        rng.gen_range(0.0..DIMS.y as f32),
    )
}

pub fn get_position_outside_play_area(rng: &mut StdRng) -> Vec2 {
    get_padded_position_outside_play_area(rng, 0.0)
}

/** gives a random position outside the viewable area, also can account for padded sizes */
pub fn get_padded_position_outside_play_area(rng: &mut StdRng, padded_size: f32) -> Vec2 {
    // position needs to be outside of the screen
    // there are 8 zones, first pick a zone
    let zone = rng.gen_range(0..8);
    let position = match zone {
        0 => Vec2::new(
            // top left
            rng.gen_range(-padded_size * 2.0..-padded_size),
            rng.gen_range(-padded_size * 2.0..-padded_size),
        ),
        1 => Vec2::new(
            // top right
            rng.gen_range(DIMS.x as f32 + padded_size..DIMS.x as f32 + padded_size * 2.0),
            rng.gen_range(-padded_size * 2.0..-padded_size),
        ),
        2 => Vec2::new(
            // bottom right
            rng.gen_range(DIMS.x as f32 + padded_size..DIMS.x as f32 + padded_size * 2.0),
            rng.gen_range(DIMS.y as f32 + padded_size..DIMS.y as f32 + padded_size * 2.0),
        ),
        3 => Vec2::new(
            // bottom left
            rng.gen_range(-padded_size * 2.0..-padded_size),
            rng.gen_range(DIMS.y as f32 + padded_size..DIMS.y as f32 + padded_size * 2.0),
        ),
        4 => Vec2::new(
            // top
            rng.gen_range(0.0..DIMS.x as f32),
            rng.gen_range(-padded_size * 2.0..-padded_size),
        ),
        5 => Vec2::new(
            // bottom
            rng.gen_range(0.0..DIMS.x as f32),
            rng.gen_range(DIMS.y as f32 + padded_size..DIMS.y as f32 + padded_size * 2.0),
        ),
        6 => Vec2::new(
            // left
            rng.gen_range(-padded_size * 2.0..-padded_size),
            rng.gen_range(0.0..DIMS.y as f32),
        ),
        7 => Vec2::new(
            // right
            rng.gen_range(DIMS.x as f32 + padded_size..DIMS.x as f32 + padded_size * 2.0),
            rng.gen_range(0.0..DIMS.y as f32),
        ),
        _ => panic!("Unexpected zone"), // This shouldn't happen with rng.gen_range(0..8)
    };
    position
}

pub fn is_in_play_area(pos: Vec2) -> bool {
    if pos.x < 0.0 || pos.x > DIMS.x as f32 {
        return false;
    }
    if pos.y < 0.0 || pos.y > DIMS.y as f32 {
        return false;
    }
    true
}
