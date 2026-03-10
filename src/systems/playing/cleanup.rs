use hecs::World;

use crate::state::{DeletionEvent, State};

pub fn process_deletion_events(ecs: &mut World, state: &mut State) {
    for deletion_event in state.deletion_events.iter() {
        match deletion_event {
            DeletionEvent::Entity { entity } => {
                let _ = ecs.take(*entity);
            }
        }
    }
    state.deletion_events.clear();
}
