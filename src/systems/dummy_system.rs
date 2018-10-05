use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System},
};

pub struct DummySystem{
    pub counter: u64,
}

impl<'a> System<'a> for DummySystem {
    type SystemData = (
        Read<'a, Time>
    );

    fn run(&mut self, time: Self::SystemData) {
        if self.counter > 100{
            println!("{}", time.absolute_real_time_seconds());
            self.counter = 0;
        }
        self.counter = self.counter + 1;
    }
}
