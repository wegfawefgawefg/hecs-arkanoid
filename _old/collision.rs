use glam::Vec2;
use hecs::Entity;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Aabb {
    pub fn new(pos: Vec2, size: Vec2) -> Aabb {
        Aabb { pos, size }
    }

    pub fn get_bounds(&self) -> Tlbr {
        Tlbr {
            tl: self.pos,
            br: self.pos + self.size,
        }
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        let self_tlbr = self.get_bounds();
        let other_tlbr = other.get_bounds();
        self_tlbr.intersects(&other_tlbr)
    }

    pub fn center(&self) -> Vec2 {
        self.pos + self.size / 2.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tlbr {
    pub tl: Vec2,
    pub br: Vec2,
}

impl Tlbr {
    pub fn new(tl: Vec2, br: Vec2) -> Tlbr {
        Tlbr { tl, br }
    }

    pub fn get_aabb(&self) -> Aabb {
        Aabb {
            pos: self.tl,
            size: self.br - self.tl,
        }
    }

    pub fn intersects(&self, other: &Tlbr) -> bool {
        !(self.tl.x > other.br.x
            || self.br.x < other.tl.x
            || self.tl.y > other.br.y
            || self.br.y < other.tl.y)
    }

    pub fn center(&self) -> Vec2 {
        (self.tl + self.br) / 2.0
    }
}

pub struct CollisionResult {
    pub next_pos: Vec2,
    pub next_vel: Vec2,
    pub collision_events: Vec<Collision>,
}

pub struct Collision {
    pub a_id: Entity,
    pub b_id: Entity,
}

pub fn do_impassable_collisions(
    id: Entity,
    shape: Aabb,
    vel: Vec2,
    collidable_shapes: Vec<(Entity, Aabb)>,
) -> CollisionResult {
    // nothing to collide with
    if collidable_shapes.is_empty() {
        return CollisionResult {
            next_pos: shape.pos + vel,
            next_vel: vel,
            collision_events: vec![],
        };
    }

    // only yourself to collide with
    if collidable_shapes.len() == 1 && collidable_shapes[0].0 == id {
        return CollisionResult {
            next_pos: shape.pos + vel,
            next_vel: vel,
            collision_events: vec![],
        };
    }

    let mut collision_result = CollisionResult {
        next_pos: shape.pos,
        next_vel: vel,
        collision_events: vec![],
    };

    // x and y must be checked separately
    //  // afterwhich, if both x and y collisions occured diagonal collision will be checked
    let mut blocked = false;
    let mut blocked_x = false;
    let mut blocked_y = false;

    // ////////////    CHECK X DIRECTION   ////////////
    {
        // get aabb and tlbr of next position
        let just_x_vel: Vec2 = Vec2::new(vel.x, 0.0);
        let next_pos: Vec2 = shape.pos + just_x_vel;
        let next_aabb = Aabb::new(next_pos, shape.size);

        // check collisions
        {
            // check which shapes were overlapped
            let mut overlapped_shapes = vec![];
            for (other_id, other_shape) in &collidable_shapes {
                if id == *other_id {
                    continue;
                }
                if next_aabb.intersects(other_shape) {
                    blocked_x = true;
                    overlapped_shapes.push((*other_id, *other_shape));
                }
            }

            //  put the player at the leftmost edge of the leftmost collided shape
            let x_blocked = !overlapped_shapes.is_empty();
            if x_blocked {
                blocked = true;
                if vel.x > 0.0 {
                    // sort overlapped shapes by left_edge position
                    overlapped_shapes.sort_by(|a, b| a.1.pos.x.partial_cmp(&b.1.pos.x).unwrap());

                    // entity was moving to the right: place it at the leftmost edge of the leftmost collided shape.
                    let leftmost_collided_shape = overlapped_shapes[0];
                    let leftmost_collided_shape_left_edge = leftmost_collided_shape.1.pos.x;
                    collision_result.next_pos.x = leftmost_collided_shape_left_edge - shape.size.x;
                    collision_result.next_vel.x = 0.0;

                    // add collision event for all shapes that share the same left edge as the leftmost collided shape
                    for (other_id, other_shape) in overlapped_shapes[1..].iter() {
                        let other_shape_left_edge = other_shape.pos.x;
                        if other_shape_left_edge == leftmost_collided_shape_left_edge {
                            collision_result.collision_events.push(Collision {
                                a_id: id,
                                b_id: *other_id,
                            })
                        } else {
                            break;
                        }
                    }
                } else if vel.x < 0.0 {
                    // sort overlapped shapes by right_edge position,
                    //  such that the rightmost collided shape is first in the list
                    overlapped_shapes.sort_by(|a, b| {
                        let a_right_edge = a.1.pos.x + a.1.size.x;
                        let b_right_edge = b.1.pos.x + b.1.size.x;
                        b_right_edge.partial_cmp(&a_right_edge).unwrap()
                    });

                    // entity was moving to the left; place it at the rightmost edge of the rightmost collided shape
                    let rightmost_collided_shape = &overlapped_shapes[0];
                    let rightmost_collided_shape_right_edge =
                        rightmost_collided_shape.1.pos.x + rightmost_collided_shape.1.size.x;
                    collision_result.next_pos.x = rightmost_collided_shape_right_edge;
                    collision_result.next_vel.x = 0.0;

                    // add collision event for all shapes that share the same right edge as the rightmost collided shape
                    for (other_id, other_shape) in overlapped_shapes[1..].iter() {
                        let other_shape_right_edge = other_shape.pos.x + other_shape.size.x;
                        if other_shape_right_edge == rightmost_collided_shape_right_edge {
                            collision_result.collision_events.push(Collision {
                                a_id: id,
                                b_id: *other_id,
                            })
                        } else {
                            break;
                        }
                    }
                } else {
                    // entity was not moving... it may have been placed inside another shape or another shape may have been placed inside it
                    // regardless, the collision events must be registered
                    // register all collisions
                    for (other_id, _) in overlapped_shapes {
                        collision_result.collision_events.push(Collision {
                            a_id: id,
                            b_id: other_id,
                        });
                    }
                }
            }
        }
    }

    ////////////    CHECK Y DIRECTION   ////////////
    {
        // get aabb and tlbr of next position
        let just_y_vel: Vec2 = Vec2::new(0.0, vel.y);
        let next_pos: Vec2 = shape.pos + just_y_vel;
        let next_aabb = Aabb::new(next_pos, shape.size);

        // check collisions
        {
            // check which shapes were overlapped
            let mut overlapped_shapes = vec![];
            for (other_id, other_shape) in &collidable_shapes {
                if id == *other_id {
                    continue;
                }
                if next_aabb.intersects(other_shape) {
                    blocked_y = true;
                    overlapped_shapes.push((*other_id, *other_shape));
                }
            }

            //  put the entity at the appropriate edge based on collisions
            let y_blocked = !overlapped_shapes.is_empty();
            if y_blocked {
                blocked = true;
                if vel.y > 0.0 {
                    // sort overlapped shapes by top edge position
                    overlapped_shapes.sort_by(|a, b| a.1.pos.y.partial_cmp(&b.1.pos.y).unwrap());

                    // entity was moving downward: place it at the topmost edge of the topmost collided shape.
                    let topmost_collided_shape = overlapped_shapes[0];
                    let topmost_collided_shape_top_edge = topmost_collided_shape.1.pos.y;
                    collision_result.next_pos.y = topmost_collided_shape_top_edge - shape.size.y;
                    collision_result.next_vel.y = 0.0;

                    // add collision event for all shapes that share the same top edge
                    for (other_id, other_shape) in &overlapped_shapes[1..] {
                        let other_shape_top_edge = other_shape.pos.y;
                        if other_shape_top_edge == topmost_collided_shape_top_edge {
                            collision_result.collision_events.push(Collision {
                                a_id: id,
                                b_id: *other_id,
                            });
                        } else {
                            break;
                        }
                    }
                } else if vel.y < 0.0 {
                    // sort overlapped shapes by bottom edge position, so that the bottom-most collided shape is first
                    overlapped_shapes.sort_by(|a, b| {
                        let a_bottom_edge = a.1.pos.y + a.1.size.y;
                        let b_bottom_edge = b.1.pos.y + b.1.size.y;
                        b_bottom_edge.partial_cmp(&a_bottom_edge).unwrap()
                    });

                    // entity was moving upward: place it at the bottom-most edge of the bottom-most collided shape.
                    let bottommost_collided_shape = overlapped_shapes[0];
                    let bottommost_collided_shape_bottom_edge =
                        bottommost_collided_shape.1.pos.y + bottommost_collided_shape.1.size.y;
                    collision_result.next_pos.y = bottommost_collided_shape_bottom_edge;
                    collision_result.next_vel.y = 0.0;

                    // add collision event for all shapes that share the same bottom edge
                    for (other_id, other_shape) in &overlapped_shapes[1..] {
                        let other_shape_bottom_edge = other_shape.pos.y + other_shape.size.y;
                        if other_shape_bottom_edge == bottommost_collided_shape_bottom_edge {
                            collision_result.collision_events.push(Collision {
                                a_id: id,
                                b_id: *other_id,
                            });
                        } else {
                            break;
                        }
                    }
                } else {
                    // Entity was not moving in Y; still register all collisions
                    for (other_id, _) in &overlapped_shapes {
                        collision_result.collision_events.push(Collision {
                            a_id: id,
                            b_id: *other_id,
                        });
                    }
                }
            }
        }
    }

    // if not blocked in either direction, then check and resolve diagonal collision
    // if !blocked_x && !blocked_y {
    //     // compute next position via full velocity
    //     let next_pos = shape.pos + vel;
    //     let next_aabb = Aabb::new(next_pos, shape.size);

    //     // check which shapes were overlapped
    //     let mut overlapped_shapes = vec![];
    //     for (other_id, other_shape) in &collidable_shapes {
    //         if id == *other_id {
    //             continue;
    //         }
    //         if next_aabb.intersects(other_shape) {
    //             overlapped_shapes.push((other_id, other_shape));
    //         }
    //     }

    //     let diagonal_blocked = !overlapped_shapes.is_empty();
    //     if diagonal_blocked {
    //         blocked = true;
    //         // sort based on the closest overlap
    //         overlapped_shapes.sort_by(|a, b| {
    //             let a_center = a.1.center();
    //             let b_center = b.1.center();
    //             let a_dist = a_center.distance_squared(shape.center());
    //             let b_dist = b_center.distance_squared(shape.center());
    //             a_dist.partial_cmp(&b_dist).unwrap()
    //         });

    //         let closest_shape = overlapped_shapes[0].1;

    //         // determine which direction has greater velocity
    //         let dominant_axis = if vel.x.abs() > vel.y.abs() { 'x' } else { 'y' };

    //         if dominant_axis == 'x' {
    //             // snap to the closest edge horizontally
    //             if vel.x > 0.0 {
    //                 collision_result.next_pos.x = closest_shape.pos.x - shape.size.x;
    //             } else {
    //                 collision_result.next_pos.x = closest_shape.pos.x + closest_shape.size.x;
    //             }
    //         } else {
    //             // snap to the closest edge vertically
    //             if vel.y > 0.0 {
    //                 collision_result.next_pos.y = closest_shape.pos.y - shape.size.y;
    //             } else {
    //                 collision_result.next_pos.y = closest_shape.pos.y + closest_shape.size.y;
    //             }
    //         }

    //         // register the collision
    //         collision_result.collision_events.push(Collision {
    //             a_id: id,
    //             b_id: *overlapped_shapes[0].0,
    //         });
    //     }
    // }

    // if not blocked in either direction, then just move
    if !blocked {
        collision_result.next_pos = shape.pos + vel;
        collision_result.next_vel = vel;
    }

    collision_result
}

pub fn check_for_collisions(
    id: Entity,
    shape: Aabb,
    collidable_shapes: Vec<(Entity, Aabb)>,
) -> Vec<Collision> {
    // nothing to collide with
    if collidable_shapes.is_empty() {
        return vec![];
    }

    // only yourself to collide with
    if collidable_shapes.len() == 1 && collidable_shapes[0].0 == id {
        return vec![];
    }

    // check which shapes were overlapped
    let mut collisions = vec![];
    for (other_id, other_shape) in &collidable_shapes {
        if id == *other_id {
            continue;
        }
        if shape.intersects(other_shape) {
            collisions.push(Collision {
                a_id: id,
                b_id: *other_id,
            });
        }
    }

    collisions
}
