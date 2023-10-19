// use glam::Vec2;
// use rand::{rngs::StdRng, Rng};
// use raylib::prelude::Color;

// use crate::{
//     message_stream::ExpiringMessages,
//     rendering::{DrawCommand, RenderCommandBuffer},
//     DIMS,
// };

// pub fn entity_render(
//     ecs: &SubWorld,
//     #[resource] rng: &mut StdRng,
//     #[resource] render_command_buffer: &mut RenderCommandBuffer,
// ) {
//     // render GrabZones
//     <(&CTransform, &GrabZone)>::query()
//         .iter(ecs)
//         .for_each(|(transform, grabzone)| {
//             render_command_buffer.push(DrawCommand::Circle {
//                 pos: transform.pos,
//                 radius: grabzone.radius,
//                 color: Color::new(0, 0, 255, 50),
//             })
//         });

//     // render WantsToGoTo
//     <(&CTransform, &WantsToGoTo)>::query()
//         .iter(ecs)
//         .for_each(|(transform, wants_to_go_to)| {
//             render_command_buffer.push(DrawCommand::Line {
//                 start: transform.pos,
//                 end: wants_to_go_to.pos,
//                 color: Color::new(0, 0, 255, 50),
//             })
//         });

//     // schedule asteroid rendering
//     <(&CTransform, &Asteroid)>::query()
//         .iter(ecs)
//         .for_each(|(transform, asteroid)| {
//             render_command_buffer.push(DrawCommand::Asteroid {
//                 pos: transform.pos,
//                 size: asteroid.size,
//                 dir: transform.rot,
//             });
//         });
//     // wrap rendering
//     // <(&CTransform, &Asteroid)>::query()
//     //     .iter(ecs)
//     //     .for_each(|(transform, asteroid)| {
//     //         let mut positions = vec![transform.pos];

//     //         // Check if the object overlaps the right edge
//     //         if transform.pos.x + (asteroid.size as f32) > DIMS.x as f32 {
//     //             positions.push(Vec2::new(transform.pos.x - DIMS.x as f32, transform.pos.y));
//     //         }
//     //         // Check if the object overlaps the left edge
//     //         else if transform.pos.x - (asteroid.size as f32) < 0.0 {
//     //             positions.push(Vec2::new(transform.pos.x + DIMS.x as f32, transform.pos.y));
//     //         }

//     //         // Check if the object overlaps the top edge
//     //         if transform.pos.y + (asteroid.size as f32) > DIMS.y as f32 {
//     //             positions.push(Vec2::new(transform.pos.x, transform.pos.y - DIMS.y as f32));
//     //         }
//     //         // Check if the object overlaps the bottom edge
//     //         else if transform.pos.y - (asteroid.size as f32) < 0.0 {
//     //             positions.push(Vec2::new(transform.pos.x, transform.pos.y + DIMS.y as f32));
//     //         }

//     //         for pos in positions {
//     //             render_command_buffer.push(DrawCommand::Asteroid {
//     //                 pos,
//     //                 size: asteroid.size,
//     //                 dir: transform.rot,
//     //             });
//     //         }
//     //     });

//     // schedule bullet rendering
//     <&CTransform>::query()
//         .filter(component::<Bullet>())
//         .iter(ecs)
//         .for_each(|transform| {
//             render_command_buffer.push(DrawCommand::ColoredSquare {
//                 pos: transform.pos,
//                 color: Color::new(255, rng.gen_range(10..255), 0, 255),
//             });
//         });

//     // schedule player rendering
//     <&CTransform>::query()
//         .filter(component::<Player>())
//         .iter(ecs)
//         .for_each(|transform| {
//             render_command_buffer.push(DrawCommand::Ship {
//                 pos: transform.pos,
//                 dir: transform.rot,
//                 color: Color::GOLD,
//             });
//         });

//     // schedule player rendering
//     <&CTransform>::query()
//         .filter(component::<Enemy>())
//         .iter(ecs)
//         .for_each(|transform| {
//             render_command_buffer.push(DrawCommand::Ship {
//                 pos: transform.pos,
//                 dir: transform.rot,
//                 color: Color::MAROON,
//             });
//         });

//     // schedule player rendering
//     <&CTransform>::query()
//         .filter(component::<Gun>())
//         .iter(ecs)
//         .for_each(|transform| {
//             render_command_buffer.push(DrawCommand::Gun {
//                 pos: transform.pos,
//                 dir: transform.rot,
//             });
//         });

//     // render attachment struts
//     let start_to: Vec<(CTransform, Entity)> = <(&CTransform, &AttachedTo)>::query()
//         .iter(ecs)
//         .map(|(transform, attached_to)| (*transform, attached_to.entity))
//         .collect();

//     for (start, end_entity) in start_to {
//         if let Ok(end) = ecs.entry_ref(end_entity) {
//             if let Ok(end_transform) = end.get_component::<CTransform>() {
//                 // skip if line is too long
//                 if (start.pos - end_transform.pos).length() > 100.0 {
//                     continue;
//                 }

//                 render_command_buffer.push(DrawCommand::Line {
//                     start: start.pos,
//                     end: end_transform.pos,
//                     color: Color::new(255, 255, 255, 100),
//                 });
//             }
//         }
//     }
// }

// // render system
// /* fetch position and sprite entities, and just blit them with a fixed size with the given position */
// #[system]
// #[read_component(Score)]
// pub fn score_render(ecs: &SubWorld, #[resource] render_command_buffer: &mut RenderCommandBuffer) {
//     let mut cursor = Vec2::new(DIMS.x as f32 * 0.28, DIMS.y as f32 * 0.1);
//     let size = 1;

//     // schedule asteroid rendering
//     <&Score>::query().iter(ecs).for_each(|score| {
//         let text = format!("Score: {}", score.score);

//         render_command_buffer.push(DrawCommand::Text {
//             pos: cursor,
//             text,
//             size,
//             color: Color::WHITE,
//         });

//         cursor.y += 10.0;
//     });
// }

// #[system]
// pub fn render_expiring_messages(
//     #[resource] expiring_messages: &ExpiringMessages,
//     #[resource] render_command_buffer: &mut RenderCommandBuffer,
// ) {
//     // cursor should go up from the bottom left corner
//     let mut cursor = Vec2::new(0.0, DIMS.y as f32 * 0.9);
//     let size = 5;

//     for message in expiring_messages.iter() {
//         render_command_buffer.push(DrawCommand::Text {
//             pos: cursor,
//             text: message.text.clone(),
//             size,
//             color: Color::WHITE,
//         });

//         cursor.y -= size as f32 * 1.5;
//     }
// }
