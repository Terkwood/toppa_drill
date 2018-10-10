use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System},
    input::InputHandler,
    renderer::VirtualKeyCode,
};

pub struct DummySystem {
    pub counter: u64,
}

impl<'a> System<'a> for DummySystem {
    type SystemData = (Read<'a, Time>, Read<'a, InputHandler<String, String>>);

    fn run(&mut self, (time, input): Self::SystemData) {
        if self.counter > 100 {
            info!("{}", time.absolute_real_time_seconds());
            self.counter = 0;
        }
        self.counter = self.counter + 1;

        if input.key_is_down(VirtualKeyCode::Space) {
            info!("Space is down.");
        }
        if input.key_is_down(VirtualKeyCode::O) {
            info!("Letter O is down.");
        }
        if let Some(down) = input.action_is_down("shoot") {
            if down {
                info!("Shooting.");
            }
        }
    }
}
