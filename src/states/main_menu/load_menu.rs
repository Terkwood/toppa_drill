use std::{
    collections::HashMap,
    path::PathBuf,
};

use amethyst::{
    assets::Handle,
    core::timing::Time,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{UiEventType, UiLoader, UiPrefab},
};

use states::{
    ToppaState,
    ingame::IngameBaseState,
};
use ToppaGameData;
use resources::{
    ingame::{
        GameSessionData,
        SavegamePaths,
    },
    RenderConfig,
};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum LoadMenuButtons {
    Back,
    Load,
}

/// The load state, form where savegames can be loaded.
pub struct LoadMenuState<'d, 'e> {
    main_dispatcher: Option<Dispatcher<'d, 'e>>,
    current_screen: Option<Entity>,
    current_screen_prefab: Option<Handle<UiPrefab>>,
    ui_buttons: HashMap<Entity, LoadMenuButtons>,
    b_buttons_found: bool,
}

impl<'d, 'e> ToppaState<'d, 'e> for LoadMenuState<'d, 'e> {
    type StateButton = LoadMenuButtons;
    fn enable_dispatcher(&mut self, world: &mut World) {
        self.main_dispatcher = None; /*Some({
            let mut dispatcher = DispatcherBuilder::new()
                .with(DummySystem { counter: 0 }, "dummy_system", &[])
                .build();

            dispatcher.setup(&mut world.res);
            dispatcher
        });*/
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        LoadMenuState {
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

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, StateEvent> for LoadMenuState<'d, 'e> {
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
        }
    }

    fn update(
        &mut self,
        data: StateData<ToppaGameData>,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, data } = data;
        self.dispatch(&world);
        data.update_menu(&world);

        if !self.b_buttons_found {
            self.b_buttons_found =
                self.insert_button(&mut world, LoadMenuButtons::Back, "menu_load_back_button") &&
                self.insert_button(&mut world, LoadMenuButtons::Load, "menu_load_load_button");
        }

        Trans::None
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

impl<'a, 'b, 'd, 'e> LoadMenuState<'d, 'e> {
    fn btn_click(
        &self,
        world: &mut World,
        target: Entity,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        use self::LoadMenuButtons::*;
        if let Some(button) = self.ui_buttons.get(&target) {
            match button {
                Back => self.btn_back(),
                Load => self.btn_load(world),
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

    fn btn_load(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Loading savegame.");

        let mut path = PathBuf::new();
        path.push("./");
        path.push("savegames");
        path.push("Mark");
        path.push("session_data");
        path.set_extension("ron");

        let render_config = {
            world.read_resource::<RenderConfig>().clone()
        };

        match GameSessionData::load(path.clone(), &render_config) {
            Ok(data) => {
                world.add_resource::<GameSessionData>(data);
            },
            Err(e) => {
                error!("Error loading Savegame data at {:?}: {:?}", path, e);
                return Trans::None;
            }
        }

        let ingame_ui_prefab_handle =
            Some(world.exec(|loader: UiLoader| loader.load("Prefabs/ui/Ingame/Base.ron", ())));

        Trans::Switch(Box::new({
            IngameBaseState::new(world, ingame_ui_prefab_handle)
        }))
    }
}
