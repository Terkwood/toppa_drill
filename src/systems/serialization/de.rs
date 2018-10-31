use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System},
};

/// TODO: Everything.
#[allow(dead_code)]
pub struct DeSavegameSystem;

impl<'a> System<'a> for DeSavegameSystem {
    type SystemData = (Read<'a, Time>,);

    fn run(&mut self, (_time,): Self::SystemData) {}
}
