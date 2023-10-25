use rapier2d::{
    crossbeam::{self, channel::Receiver},
    na::{OPoint, Vector2},
    prelude::*,
};

use crate::DIMS;

use hecs::Entity;
use rapier2d::dynamics::RigidBodyHandle;
use std::collections::HashMap;

const PIXELS_PER_METER: f32 = 100.0;
const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;

pub fn m2p(m: f32) -> f32 {
    m * PIXELS_PER_METER
}

pub fn p2m(p: f32) -> f32 {
    p * METERS_PER_PIXEL
}

pub struct PhysicsEngine {
    ecs_to_rigid_body: HashMap<Entity, RigidBodyHandle>,
    rigid_body_to_ecs: HashMap<RigidBodyHandle, Entity>,

    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub physics_hooks: (),
    pub collision_recv: Receiver<CollisionEvent>,
    pub contact_force_recv: Receiver<ContactForceEvent>,
    pub event_handler: ChannelEventCollector,

    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        let gravity = vector![0.0, 2.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();

        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();

        // stage physics init
        //  ground is a segment at the bottom of the screen
        // let ground_collider = ColliderBuilder::segment(
        //     Point::new(0.0, DIMS.y as f32),
        //     Point::new(DIMS.x as f32, DIMS.y as f32 - 1.0),
        // )
        // .build();
        // collider_set.insert(ground_collider);

        // // left wall
        // let left_wall_collider =
        //     ColliderBuilder::segment(Point::new(0.0, 0.0), Point::new(0.0, DIMS.y as f32)).build();
        // collider_set.insert(left_wall_collider);

        // // right wall
        // let right_wall_collider = ColliderBuilder::segment(
        //     Point::new(DIMS.x as f32, 0.0),
        //     Point::new(DIMS.x as f32, DIMS.y as f32),
        // )
        // .build();
        // collider_set.insert(right_wall_collider);

        // // top wall
        // let top_wall_collider =
        //     ColliderBuilder::segment(Point::new(0.0, 0.0), Point::new(DIMS.x as f32, 0.0)).build();
        // collider_set.insert(top_wall_collider);

        // let ball_collider = ColliderBuilder::ball(10.0).restitution(0.9).build();
        // let ball_rigid_body = RigidBodyBuilder::dynamic()
        //     .translation(vector![DIMS.x as f32 / 2.0, 10.0])
        //     .build();
        // let ball_body_handle = rigid_body_set.insert(ball_rigid_body);
        // collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);

        // spawn stuff
        // let mut rng = rand::thread_rng();

        // spawn a bunch of balls
        // let num_balls = 100;
        // for _ in 0..num_balls {
        //     let ball_collider = ColliderBuilder::ball(4.0).restitution(0.9).build();
        //     let ball_rigid_body = RigidBodyBuilder::dynamic()
        //         .translation(vector![
        //             rng.gen_range(0.0..DIMS.x as f32),
        //             rng.gen_range(0.0..DIMS.y as f32)
        //         ])
        //         .build();
        //     let ball_body_handle = rigid_body_set.insert(ball_rigid_body);
        //     collider_set.insert_with_parent(ball_collider, ball_body_handle, &mut rigid_body_set);
        // }

        // spawn a bunch of boxes
        // let num_boxes = 500;
        // for _ in 0..num_boxes {
        //     let box_collider = ColliderBuilder::cuboid(2.0, 2.0).restitution(0.9).build();
        //     let box_rigid_body = RigidBodyBuilder::dynamic()
        //         .translation(vector![
        //             rng.gen_range(0.0..DIMS.x as f32),
        //             rng.gen_range(0.0..DIMS.y as f32)
        //         ])
        //         .build();
        //     let box_body_handle = rigid_body_set.insert(box_rigid_body);
        //     collider_set.insert_with_parent(box_collider, box_body_handle, &mut rigid_body_set);
        // }

        Self {
            ecs_to_rigid_body: HashMap::new(),
            rigid_body_to_ecs: HashMap::new(),

            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            physics_hooks: (),
            collision_recv,
            contact_force_recv,
            event_handler,

            rigid_body_set,
            collider_set,
        }
    }

    pub fn step(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &self.physics_hooks,
            &self.event_handler,
        );
    }

    pub fn set_rigid_body_mapping(&mut self, ecs_entity: Entity, physics_handle: RigidBodyHandle) {
        self.ecs_to_rigid_body.insert(ecs_entity, physics_handle);
        self.rigid_body_to_ecs.insert(physics_handle, ecs_entity);
    }

    pub fn remove_rigid_body_mapping(&mut self, ecs_entity: Entity) {
        if let Some(physics_handle) = self.ecs_to_rigid_body.remove(&ecs_entity) {
            self.rigid_body_to_ecs.remove(&physics_handle);
        }
        self.ecs_to_rigid_body.remove(&ecs_entity);
    }

    pub fn get_rigid_body_handle(&self, ecs_entity: Entity) -> Option<RigidBodyHandle> {
        self.ecs_to_rigid_body.get(&ecs_entity).copied()
    }

    pub fn get_entity_from_rigid_body_handle(
        &self,
        physics_handle: RigidBodyHandle,
    ) -> Option<Entity> {
        self.rigid_body_to_ecs.get(&physics_handle).copied()
    }
}
