mod startup_state;

pub mod ingame;
pub mod main_menu;
pub use self::startup_state::StartupState;

use amethyst::{
    assets::Handle, 
    ecs::prelude::*, 
    ui::UiPrefab,
    renderer::HiddenPropagate,
};

/// Base functions for a state in this game, as they mostly have their own dispatchers.
pub trait ToppaState<'g, 'h> {
    // some accessors to make implementation shorter.
    fn get_screen_entity(&self) -> Option<Entity>;
    fn set_screen_entity(&mut self, screen_entity: Option<Entity>);
    fn get_screen_prefab(&self) -> Option<Handle<UiPrefab>>;
    //fn set_screen_prefab(&mut self, screen_prefab: Option<Handle<UiPrefab>>);
    fn get_main_dispatcher(&mut self) -> &mut Option<Dispatcher<'g, 'h>>;
    fn get_shadow_dispatcher(&mut self) -> &mut Option<Dispatcher<'g, 'h>>;
    fn reset_buttons(&mut self);
    
    // --- Actual API --- 
    // Since systems change per state, individual impl necessary
    fn new(world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self;
    fn enable_dispatcher(&mut self);
    fn enable_shadow_dispatcher(&mut self) {}
    // common impl using trait-accessors
    fn dispatch(&mut self, world: &World) {
        if let Some(dispatcher) = self.get_main_dispatcher(){
            dispatcher.dispatch(&world.res);
        }
    }
    fn disable_dispatcher(&mut self){
        *self.get_main_dispatcher() = None;
    }

    fn shadow_dispatch(&mut self, world: &World) {
        if let Some(dispatcher) = self.get_shadow_dispatcher(){
            dispatcher.dispatch(&world.res);
        }
    }
    fn disable_shadow_dispatcher(&mut self){
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
            self.set_screen_entity(Some(world.create_entity().with(prefab_handle.clone()).build()));
        }
        else{
            self.set_screen_entity(None);
            error!("No screen prefab found!");
        }
    }
/*
    fn disable_current_screen(&mut self, world: &mut World) {
        if let Some(entity) = self.get_screen_entity() {
            let mut hidden_component_storage = world.write_storage::<HiddenPropagate>();
            match hidden_component_storage.insert(entity, HiddenPropagate::default()) {
                Ok(_v) => {}
                Err(e) => error!(
                    "Failed to add HiddenPropagateComponent. {:?}",
                    e
                ),
            };
        };
    }

    fn enable_current_screen(&mut self, world: &mut World) {
        if let Some(entity) = self.get_screen_entity() {
            let mut hidden_component_storage = world.write_storage::<HiddenPropagate>();
            hidden_component_storage.remove(entity);
        } else {
            if let Some(ref prefab_handle) = self.get_screen_prefab() {
                self.set_screen_entity(
                    Some(world.create_entity().with(prefab_handle.clone()).build())
                );
            } else {
                error!("No PrefabHandle found.");
            }
        }
    }
*/
}
