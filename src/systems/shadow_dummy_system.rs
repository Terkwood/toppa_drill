use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System},
    input::InputHandler,
    renderer::VirtualKeyCode,
};

#[derive(Default)]
pub struct ShadowDummySystem {
    pub counter: u64,
}

impl<'a> System<'a> for ShadowDummySystem {
    type SystemData = (Read<'a, Time>, Read<'a, InputHandler<String, String>>);

    fn run(&mut self, (time, input): Self::SystemData) {
        if self.counter > 130 {
            info!("Shadow update {}", time.absolute_real_time_seconds());
            self.counter = 0;
        }
        self.counter = self.counter + 1;

        if input.key_is_down(VirtualKeyCode::O) {
            trace!("Letter O is down. Shadow update");
        }
    }
}
