use amethyst::{
    assets::Handle,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{UiEventType, UiPrefab},
};
use std::collections::HashMap;

use {states::ToppaState, systems::DummySystem, ToppaGameData};

pub struct GameStartTransitionState<'d, 'e> {
    progress_counter: ProgressCounter,
    current_screen: Option<Entity>,
    current_screen_prefab: Option<Handle<UiPrefab>>,
}

impl<'d, 'e> ToppaState<'d, 'e> for GameStartTransitionState<'d, 'e> {
    type StateButton = GameStartTransitionButtons;

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        GameStartTransitionState {
            main_dispatcher: None,
            shadow_dispatcher: None,

            //progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::new(),
            current_screen: None,
            current_screen_prefab: screen_opt,
            b_buttons_found: false,
        }
    }

    fn get_screen_entity(&self) -> Option<Entity> {
        self.current_screen
    }

    fn set_screen_entity(&mut self, screen_entity: Option<Entity>) {
        self.current_screen = screen_entity;
    }

    fn get_screen_prefab(&self) -> Option<Handle<UiPrefab>> {
        self.current_screen_prefab.clone()
    }

    fn set_screen_prefab(&mut self, screen_prefab: Option<Handle<UiPrefab>>){
        self.current_screen_prefab = screen_prefab;
    }
}

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, StateEvent> for GameStartTransitionState<'d, 'e> {
    fn handle_event(
        &mut self,
        data: StateData<ToppaGameData>,
        event: StateEvent,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, data: _ } = data;
        match &event {
            StateEvent::Window(wnd_event) => {
                if is_close_requested(&wnd_event) || is_key_down(&wnd_event, VirtualKeyCode::Escape)
                {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(ui_event) => {
                use self::UiEventType::*;
                match ui_event.event_type {
                    Click => self.btn_click(&mut world, ui_event.target),
                    _ => Trans::None,
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<ToppaGameData>,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, data: _ } = data;
        self.dispatch(&world);
        //data.update_ingame(&world);

        if !self.b_buttons_found {
            self.b_buttons_found = self.insert_button(
                &mut world,
                GameStartTransitionButtons::Close,
                "ingame_GameStartTransition_close_button",
            );
        }

        Trans::None
    }

    fn shadow_update(&mut self, data: StateData<ToppaGameData>) {
        let StateData { world, data: _ } = data;
        self.shadow_dispatch(&world);
    }

    fn on_start(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_current_screen(&mut world);
        self.enable_dispatcher();
    }

    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }
}

impl<'a, 'b, 'd, 'e> GameStartTransitionState<'d, 'e> {
    fn btn_click(
        &self,
        _world: &mut World,
        target: Entity,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        use self::GameStartTransitionButtons::*;
        if let Some(button) = self.ui_buttons.get(&target) {
            match button {
                Close => self.btn_close(),
            }
        } else {
            Trans::None
        }
    }

    fn btn_close(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        info!("Closing GameStartTransition.");
        Trans::Pop
    }
}
