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
    /// Specifies the enum for the buttons used to identify each button-entity.
    type StateButton;
    // some accessors to make implementation shorter.
    /// May return the `ToppaState`'s `current_screen`-entity, if it has one.
    fn get_screen_entity(&self) -> Option<Entity> {None}
    /// Implement to set the `ToppaState`'s `current_screen`-entity, if it has this field.
    fn set_screen_entity(&mut self, screen_entity: Option<Entity>) {}
    
    /// Implement to set the `ToppaState`'s `current_screen_prefab`, if it has this field.
    fn get_screen_prefab(&self) -> Option<Handle<UiPrefab>> {None}
    /// May return the `ToppaState`'s `current_screen_prefab`, if it has one.
    fn set_screen_prefab(&mut self, screen_prefab: Option<Handle<UiPrefab>>) {}

    /// May return the `ToppaState`'s `main_dispatcher`, if it has one.
    fn get_main_dispatcher(&mut self) -> Option<&mut Option<Dispatcher<'g, 'h>>> {None}
    /// May return the `ToppaState`'s `shadow_dispatcher`, if it has one.
    fn get_shadow_dispatcher(&mut self) -> Option<&mut Option<Dispatcher<'g, 'h>>> {None}
    
    /// Implement to clear the `ToppaState`'s `ui_buttons`-field and set the
    /// `b_buttons_found`-field to false, causing the update to search for the buttons again.
    fn reset_buttons(&mut self) {}
    /// May return the `ToppaState`'s `ui_buttons`-field, if it has one.
    fn get_buttons(&mut self) -> Option<&mut HashMap<Entity, Self::StateButton>> {None}

    // --- Actual API ---
    // Since systems change per state, individual impl necessary
    // Implementation mandatory:
    fn new(world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self;
    // Implementation optional:
    /// Implement this function to create a custom dispatcher for this `State`,
    /// which should be dispatched every frame, as long as it is the active `State`.
    fn enable_dispatcher(&mut self, world: &mut World) {}
    /// Implement this function to create a custom dispatcher for this `State`,
    /// which should be dispatched every frame, even when it is not the active `State`.
    fn enable_shadow_dispatcher(&mut self, world: &mut World) {}

    /// Call this function from your `State`'s `.update()` function,
    /// to dispatch the dispatcher built in [`enable_dispatcher()`](trait.ToppaState.html#method.enable_dispatcher).
    fn dispatch(&mut self, world: &World) {
        if let Some(Some(dispatcher)) = self.get_main_dispatcher() {
            dispatcher.dispatch(&world.res);
        }
    }
    /// Call this function to destroy the dispatcher built in [`enable_dispatcher()`](trait.ToppaState.html#method.enable_dispatcher).
    /// Currently necessary, since non-dispatched `System`'s `ReaderID`'s would otherwise cause the buffer to grow forever.
    fn disable_dispatcher(&mut self) {
        if let Some(dispatcher_opt) = self.get_main_dispatcher(){
            *dispatcher_opt = None;
        }
    }

    /// Call this function from your `State`'s `.shadow_update()` function,
    /// to dispatch the dispatcher built in [`enable_shadow_dispatcher()`](trait.ToppaState.html#method.enable_shadow_dispatcher).
    fn shadow_dispatch(&mut self, world: &World) {
        if let Some(Some(dispatcher)) = self.get_shadow_dispatcher() {
            dispatcher.dispatch(&world.res);
        }
    }
    /// Call this function to destroy the dispatcher built in [`enable__shadow_dispatcher()`](trait.ToppaState.html#method.enable_shadow_dispatcher).
    /// Currently necessary, since non-dispatched `System`'s `ReaderID`'s would otherwise cause the buffer to grow forever.
    fn disable_shadow_dispatcher(&mut self) {
        if let Some(dispatcher_opt) = self.get_shadow_dispatcher(){
            *dispatcher_opt = None;
        }
    }

    /// Deletes the ui-entities associated with this `State`.
    fn disable_current_screen(&mut self, world: &mut World) {
        if let Some(entity) = self.get_screen_entity() {
            let _ = world.delete_entity(entity);
        };
        self.reset_buttons();
    }
    /// Creates the ui-entities associated with this `State`,
    /// while also resetting the `ui_button` field of the `State`.
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

    /// Inserts buttons into the `ToppaState`'s `ui_buttons`-field,
    /// that most ToppaStates have.
    fn insert_button(
        &mut self,
        world: &mut World,
        button: Self::StateButton,
        button_name: &str,
    ) -> bool {
        world.exec(|finder: UiFinder| {
            if let Some(entity) = finder.find(button_name) {
                if let Some(buttons) = self.get_buttons(){
                    {/*turn back to debug later*/}warn!("Adding button {}.", button_name);
                    buttons.insert(entity, button);
                }
                true
            } else {
                warn!("Couldn't find {}!", button_name);
                false
            }
        })
    }
}
