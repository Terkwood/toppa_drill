use amethyst::ecs::{prelude::ParallelIterator, Entities, ParJoin, ReadStorage, System};

use crate::components::IsIngameEntity;

/// Marks all Entities with an [`IsIngameEntity`](struct.IsIngameEntity.html)-component for deletion.
#[derive(Default)]
pub struct CleanupOnCloseSystem;

impl<'s,> System<'s,> for CleanupOnCloseSystem {
    type SystemData = (Entities<'s,>, ReadStorage<'s, IsIngameEntity,>,);

    fn run(&mut self, (entities, ingame_components,): Self::SystemData,) {
        /*
        for (entity, _) in (&*entities, &ingame_components).join() {
            if let Err(e) = entities.delete(entity){
                error!("Error deleting ingame entity: {:?}", e);
            }
        }
        */

        (&*entities, &ingame_components,)
            .par_join()
            .for_each(|(entity, _,)| {
                if let Err(e,) = entities.delete(entity,) {
                    error!("Error deleting ingame entity: {:?}", e);
                }
            },);
    }
}
