use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System},
};

pub struct DummySystem;

impl<'a> System<'a> for DummySystem {
    type SystemData = (
        Read<'a, Time>
    );

    fn run(&mut self, time: Self::SystemData) {
        println!("{}", time.absolute_real_time_seconds());
    }
}
