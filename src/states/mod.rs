mod startup_state;

pub mod ingame;
pub mod main_menu;
pub use self::startup_state::StartupState;

use amethyst::{
    assets::Handle,
    ecs::prelude::*,
    prelude::*,
    ui::{UiFinder, UiPrefab},
};
use std::collections::HashMap;

/// Base functions for a state in this game, as they mostly have their own dispatchers.
pub trait ToppaState<'g, 'h> {
    type StateButton;
    // some accessors to make implementation shorter.
    fn get_screen_entity(&self) -> Option<Entity>;
    fn set_screen_entity(&mut self, screen_entity: Option<Entity>);
    fn get_screen_prefab(&self) -> Option<Handle<UiPrefab>>;
    //fn set_screen_prefab(&mut self, screen_prefab: Option<Handle<UiPrefab>>);
    fn get_main_dispatcher(&mut self) -> &mut Option<Dispatcher<'g, 'h>>;
    fn get_shadow_dispatcher(&mut self) -> &mut Option<Dispatcher<'g, 'h>>;
    fn reset_buttons(&mut self);
    fn get_buttons(&mut self) -> &mut HashMap<Entity, Self::StateButton>;

    // --- Actual API ---
    // Since systems change per state, individual impl necessary
    fn new(world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self;
    fn enable_dispatcher(&mut self);
    fn enable_shadow_dispatcher(&mut self) {}
    // common impl using trait-accessors
    fn dispatch(&mut self, world: &World) {
        if let Some(dispatcher) = self.get_main_dispatcher() {
            dispatcher.dispatch(&world.res);
        }
    }
    fn disable_dispatcher(&mut self) {
        *self.get_main_dispatcher() = None;
    }

    fn shadow_dispatch(&mut self, world: &World) {
        if let Some(dispatcher) = self.get_shadow_dispatcher() {
            dispatcher.dispatch(&world.res);
        }
    }
    fn disable_shadow_dispatcher(&mut self) {
        *self.get_shadow_dispatcher() = None;
    }

    fn disable_current_screen(&mut self, world: &mut World) {
        if let Some(entity) = self.get_screen_entity() {
            let _ = world.delete_entity(entity);
        };
    }

    fn enable_current_screen(&mut self, world: &mut World) {
        self.reset_buttons();
        if let Some(ref prefab_handle) = self.get_screen_prefab() {
            self.set_screen_entity(Some(
                world.create_entity().with(prefab_handle.clone()).build(),
            ));
        } else {
            self.set_screen_entity(None);
            error!("No screen prefab found!");
        }
    }

    fn insert_button(
        &mut self,
        world: &mut World,
        button: Self::StateButton,
        button_name: &str,
    ) -> bool {
        world.exec(|finder: UiFinder| {
            if let Some(entity) = finder.find(button_name) {
                info!("Found {}.", button_name);
                self.get_buttons().insert(entity, button);
                true
            } else {
                warn!("Couldn't find {}!", button_name);
                false
            }
        })
    }
}
