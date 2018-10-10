mod startup_state;

pub mod ingame;
pub mod main_menu;
pub use self::startup_state::StartupState;

use amethyst::{assets::Handle, ecs::prelude::*, ui::UiPrefab};

/// Base functions for a state in this game, as they mostly have their own dispatchers.
pub trait ToppaState: std::marker::Sized {
    fn dispatch(&mut self, world: &World);
    fn enable_dispatcher(&mut self);
    fn disable_dispatcher(&mut self);

    fn new(world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self;

    fn disable_current_screen(&mut self, world: &mut World);
    fn enable_current_screen(&mut self, world: &mut World);
}
