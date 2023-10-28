use hecs::World;

use crate::state::{DeletionEvent, State};

pub fn process_deletion_events(ecs: &mut World, state: &mut State) {
    for deletion_event in state.deletion_events.iter() {
        match deletion_event {
            DeletionEvent::Entity { entity } => {
                let _ = ecs.take(*entity);
            }
            DeletionEvent::Physics { entity } => {
                if let Some(rigid_body_handle) = state.physics.get_rigid_body_handle(*entity) {
                    state.physics.remove_rigid_body_mapping(*entity);
                    state.physics.rigid_body_set.remove(
                        rigid_body_handle,
                        &mut state.physics.island_manager,
                        &mut state.physics.collider_set,
                        &mut state.physics.impulse_joint_set,
                        &mut state.physics.multibody_joint_set,
                        true, // remove the associated colliders as well
                    );
                }
            }
        }
    }
    state.deletion_events.clear();
}
