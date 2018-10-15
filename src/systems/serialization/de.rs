use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System},
};

pub struct DeChunkSystem;

impl<'a> System<'a> for DeChunkSystem {
    type SystemData = (Read<'a, Time>,);

    fn run(&mut self, (time,): Self::SystemData) {}
}

pub struct DeSavegameSystem;

impl<'a> System<'a> for DeSavegameSystem {
    type SystemData = (Read<'a, Time>,);

    fn run(&mut self, (time,): Self::SystemData) {}
}

pub struct DePlayerSystem;

impl<'a> System<'a> for DePlayerSystem {
    type SystemData = (Read<'a, Time>,);

    fn run(&mut self, (time,): Self::SystemData) {}
}
