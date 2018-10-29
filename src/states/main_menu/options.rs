use amethyst::{
    assets::Handle,
    core::timing::Time,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{UiEventType, UiPrefab},
};
use std::collections::HashMap;
use {states::ToppaState, systems::DummySystem, ToppaGameData};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum OptionsButtons {
    Back,
}

/// The options menu state, similar to ingame options, but offers more customisability.
pub struct OptionsState<'d, 'e> {
    menu_duration: f32,
    main_dispatcher: Option<Dispatcher<'d, 'e>>,

    // The displayed Ui Entity, if any.
    current_screen: Option<Entity>,
    // The Handle of the Prefab for the displayed Ui Entity.
    current_screen_prefab: Option<Handle<UiPrefab>>,
    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, OptionsButtons>,
    b_buttons_found: bool,
}

impl<'a, 'b, 'd, 'e> ToppaState<'d, 'e> for OptionsState<'d, 'e> {
    type StateButton = OptionsButtons;
    fn enable_dispatcher(&mut self, world: &mut World) {
        self.main_dispatcher = Some(
            DispatcherBuilder::new()
                .with(DummySystem { counter: 0 }, "dummy_system", &[])
                .build(),
        );
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        OptionsState {
            menu_duration: 0.0,
            current_screen: None,
            current_screen_prefab: screen_opt,
            ui_buttons: HashMap::new(),
            b_buttons_found: false,
            main_dispatcher: None,
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

    fn set_screen_prefab(&mut self, screen_prefab: Option<Handle<UiPrefab>>) {
        self.current_screen_prefab = screen_prefab.clone();
    }

    fn get_main_dispatcher(&mut self) -> Option<&mut Option<Dispatcher<'d, 'e>>> {
        Some(&mut self.main_dispatcher)
    }

    fn reset_buttons(&mut self) {
        self.b_buttons_found = false;
        self.ui_buttons.clear();
    }

    fn get_buttons(&mut self) -> Option<&mut HashMap<Entity, Self::StateButton>> {
        Some(&mut self.ui_buttons)
    }
}

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, StateEvent> for OptionsState<'d, 'e> {
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
        let StateData { mut world, data } = data;
        self.dispatch(&world);
        data.update_menu(&world);
        self.menu_duration += world.read_resource::<Time>().delta_seconds();

        if !self.b_buttons_found {
            self.b_buttons_found =
                self.insert_button(&mut world, OptionsButtons::Back, "menu_options_back_button");
            Trans::None
        } else {
            Trans::None
        }
    }

    // Executed when this game state runs for the first time.
    fn on_start(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_current_screen(&mut world);
        self.enable_dispatcher(&mut world);
    }

    // Executed when this game state gets popped.
    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }
}

impl<'a, 'b, 'd, 'e> OptionsState<'d, 'e> {
    fn btn_click(
        &self,
        _world: &mut World,
        target: Entity,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        use self::OptionsButtons::*;
        if let Some(button) = self.ui_buttons.get(&target) {
            match button {
                Back => self.btn_back(),
            }
        } else {
            Trans::None
        }
    }

    fn btn_back(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Returning to CentreState.");
        Trans::Pop
    }
}
