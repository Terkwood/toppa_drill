use amethyst::{
    ecs::prelude::*,
    core::{
        ArcThreadPool,
        SystemBundle,
    },
    assets::{
        Handle,
    },
    ui::{
        UiPrefab,
    },
    Error,
    DataInit,
    Result,
};

pub trait ToppaState: std::marker::Sized{
    fn dispatch(&mut self, world: &World);
    fn enable_dispatcher(&mut self);
    fn disable_dispatcher(&mut self);

    fn new(world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self;

    fn get_current_screen(&self) -> Option<Entity>;
    fn disable_current_screen(&mut self, world: &mut World);
    fn enable_current_screen(&mut self, world: &mut World);
}
