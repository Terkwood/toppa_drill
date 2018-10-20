use amethyst::{
    assets::{Handle, ProgressCounter},
    core::timing::Time,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{
        UiEventType,
        UiFinder,
        //UiCreator, UiLoader,
        UiPrefab,
    },
};
use std::collections::HashMap;
use {states::ToppaState, systems::DummySystem, ToppaGameData};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum BaseStateButtons {
    Inventory,
    Options,
    Exit,
    Save,
    Mute,
}

/// The game creation state, where a new game can be started.
pub struct IngameBaseState<'d, 'e> {
    menu_duration: f32,
    dispatcher: Option<Dispatcher<'d, 'e>>,
    progress_counter: ProgressCounter,

    // The displayed Ui Entity, if any.
    current_screen: Option<Entity>,
    // The Handle of the Prefab for the displayed Ui Entity.
    ui_screen: Option<Handle<UiPrefab>>,
    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, BaseStateButtons>,
    b_buttons_found: bool,
}

impl<'d, 'e> ToppaState for IngameBaseState<'d, 'e> {
    fn dispatch(&mut self, world: &World) {
        if let Some(ref mut dispatcher) = self.dispatcher {
            dispatcher.dispatch(&world.res);
        };
    }

    fn enable_dispatcher(&mut self) {
        self.dispatcher = Some(
            DispatcherBuilder::new()
                .with(DummySystem { counter: 0 }, "dummy_system", &[])
                .build(),
        );
    }

    fn disable_dispatcher(&mut self) {
        self.dispatcher = None;
    }

    fn disable_current_screen(&mut self, world: &mut World) {
        if let Some(entity) = self.current_screen {
            let _ = world.delete_entity(entity);
        };
    }

    fn enable_current_screen(&mut self, world: &mut World) {
        if let Some(ref prefab_handle) = self.ui_screen {
            self.current_screen = Some(world.create_entity().with(prefab_handle.clone()).build());
            warn!("Building ingame ui.");
        };
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        IngameBaseState {
            menu_duration: 0.0,
            current_screen: None,
            ui_screen: screen_opt,
            progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::new(),
            b_buttons_found: false,
            dispatcher: None,
        }
    }
}

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, StateEvent> for IngameBaseState<'d, 'e> {
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

    fn update(&mut self, data: StateData<ToppaGameData>) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, data } = data;
        self.dispatch(&world);
        data.update_ingame(&world);

        if !self.b_buttons_found{
            self.b_buttons_found =
                self.insert_button(&mut world, BaseStateButtons::Inventory, "ingame_base_inventory_button") &&
                self.insert_button(&mut world, BaseStateButtons::Exit, "ingame_base_exit_button") &&
                self.insert_button(&mut world, BaseStateButtons::Save, "ingame_base_save_button") &&
                self.insert_button(&mut world, BaseStateButtons::Mute, "ingame_base_mute_button") &&
                self.insert_button(&mut world, BaseStateButtons::Options, "ingame_base_options_button");
        }

        Trans::None
    }

    // Executed when this game state runs for the first time.
    fn on_start(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_current_screen(&mut world);
        self.enable_dispatcher();
    }

    // Executed when this game state gets popped.
    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when another game state is pushed onto the stack.
    fn on_pause(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.disable_dispatcher();
    }

    // Executed when the application returns to this game state,
    // after another gamestate was popped from the stack.
    fn on_resume(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_dispatcher();
    }
}

impl<'a, 'b, 'd, 'e> IngameBaseState<'d, 'e> {
    fn insert_button(&mut self, world: &mut World, button: BaseStateButtons, button_name: &str) -> bool{
        world.exec(|finder: UiFinder| {
            if let Some(entity) = finder.find(button_name) {
                info!("Found {}.", button_name);
                self.ui_buttons.insert(entity, button);
                true
            } else {
                warn!("Couldn't find {}!", button_name);
                false
            }
        })
    }

    fn btn_click(&self, world: &mut World, target: Entity) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        use self::BaseStateButtons::*;
        if let Some(button) = self.ui_buttons.get(&target) {
            match button {
                Inventory => self.btn_inventory(),
                Options => self.btn_options(),
                Exit => self.btn_exit(),
                Save => self.btn_save(world),
                Mute => self.btn_mute(),
            }
        } else {
            Trans::None
        }
    }

    fn btn_inventory(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        info!("Opening Inventory not implemented yet.");
        Trans::None
    }

    fn btn_options(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        info!("Opening Options not implemented yet.");
        Trans::None
    }

    fn btn_exit(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        info!("Exiting to main menu.");
        Trans::Pop
    }

    fn btn_save(&self, world: &World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        info!("Saving game.");
        use systems::serialization::SerSavegameSystem;
        SerSavegameSystem.run_now(&world.res);

        Trans::None
    }

    fn btn_mute(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        info!("Muting game not implemented yet..");
        Trans::None
    }
}
