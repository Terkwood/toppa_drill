use super::{CreditsState, LoadMenuState, MenuScreens, NewGameState, OptionsState};
use amethyst::{
    assets::{Completion, Handle, ProgressCounter},
    core::timing::Time,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{HiddenPropagate, VirtualKeyCode},
    ui::{UiEventType, UiLoader, UiPrefab},
};
use states::ToppaState;
use std::collections::HashMap;
use systems::{DummySystem, ShadowDummySystem};
use ToppaGameData;

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum CentreButtons {
    NewGame,
    Load,
    Options,
    Credits,
    Exit,
}

/// The main menu state, from which the other menu states can be reached.
pub struct CentreState<'d, 'e> {
    menu_duration: f32,
    main_dispatcher: Option<Dispatcher<'d, 'e>>,
    shadow_dispatcher: Option<Dispatcher<'d, 'e>>,
    progress_counter: ProgressCounter,

    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, CentreButtons>,
    // Map of the PrefabHandles for all reachable states (convenient for the `ToppaState::new()` call on State change)
    screen_prefabs: HashMap<super::MenuScreens, Handle<UiPrefab>>,
    // Map of the Entities for all reachable states. Entities are only created after their prefab is loaded successfully.
    screen_entities: HashMap<super::MenuScreens, Entity>,

    b_screens_loaded: bool,
    b_buttons_found: bool,
}

impl<'d, 'e> ToppaState<'d, 'e> for CentreState<'d, 'e> {
    type StateButton = CentreButtons;
    fn enable_dispatcher(&mut self, world: &mut World) {
        self.main_dispatcher = Some({
            let mut dispatcher = DispatcherBuilder::new()
                .with(DummySystem { counter: 0 }, "dummy_system", &[])
                .build();

            dispatcher.setup(&mut world.res);
            dispatcher
        });
    }

    fn enable_shadow_dispatcher(&mut self, world: &mut World) {
        self.shadow_dispatcher = Some({
            let mut dispatcher = DispatcherBuilder::new()
                .with(ShadowDummySystem { counter: 0 }, "shadow_dummy_system", &[])
                .build();

            dispatcher.setup(&mut world.res);
            dispatcher
        });
    }

    fn disable_current_screen(&mut self, world: &mut World) {
        if let Some(entity) = self.screen_entities.get(&MenuScreens::Centre) {
            let mut hidden_component_storage = world.write_storage::<HiddenPropagate>();
            match hidden_component_storage.insert(*entity, HiddenPropagate::default()) {
                Ok(_v) => {}
                Err(e) => error!(
                    "Failed to add HiddenPropagateComponent to CentreState Ui. {:?}",
                    e
                ),
            };
        };
    }

    fn enable_current_screen(&mut self, world: &mut World) {
        if self.screen_entities.contains_key(&MenuScreens::Centre) {
            if let Some(entity) = self.screen_entities.get(&MenuScreens::Centre) {
                let mut hidden_component_storage = world.write_storage::<HiddenPropagate>();
                hidden_component_storage.remove(*entity);
            } else {
                error!("No Entity found for Main Menu even though the screen_entities-HashMap contains the key !?");
            }
        } else {
            if self.screen_prefabs.contains_key(&MenuScreens::Centre) {
                let mut handle = None;
                if let Some(prefab_handle) = self.screen_prefabs.get(&MenuScreens::Centre) {
                    handle = Some(prefab_handle.clone());
                } else {
                    error!("No PrefabHandle found for Main Menu even though the screen_prefabs-HashMap contains the key !?");
                }

                if let Some(prefab_handle) = handle {
                    self.reset_buttons();
                    self.screen_entities.insert(MenuScreens::Centre, {
                        world.create_entity().with(prefab_handle.clone()).build()
                    });
                }
            } else {
                error!("No Prefab Handle found for Main Menu screen!");
            }
        }
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        let btn_count = 5;
        let prefab_count = 5;

        let mut rv = CentreState {
            menu_duration: 0.0,
            main_dispatcher: None,
            shadow_dispatcher: None,
            progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::with_capacity(btn_count),
            screen_prefabs: HashMap::with_capacity(prefab_count),
            screen_entities: HashMap::with_capacity(prefab_count),
            b_screens_loaded: false,
            b_buttons_found: false,
        };

        if let Some(screen_prefab) = screen_opt {
            rv.screen_prefabs
                .insert(MenuScreens::Centre, screen_prefab.clone());
        } else {
            error!("No Prefab Handle provided for Main Menu screen!");
        }
        rv
    }

    fn get_screen_entity(&self) -> Option<Entity> {
        if let Some(entity) = self.screen_entities.get(&MenuScreens::Centre) {
            Some(*entity)
        } else {
            None
        }
    }

    fn set_screen_entity(&mut self, screen_entity: Option<Entity>) {
        if let Some(entity) = screen_entity {
            self.screen_entities.insert(MenuScreens::Centre, entity);
        };
    }

    fn get_screen_prefab(&self) -> Option<Handle<UiPrefab>> {
        if let Some(prefab) = self.screen_prefabs.get(&MenuScreens::Centre) {
            Some(prefab.clone())
        } else {
            None
        }
    }

    fn set_screen_prefab(&mut self, screen_prefab: Option<Handle<UiPrefab>>) {
        if let Some(screen_prefab) = screen_prefab {
            self.screen_prefabs
                .insert(MenuScreens::Centre, screen_prefab.clone());
        };
    }

    fn get_main_dispatcher(&mut self) -> Option<&mut Option<Dispatcher<'d, 'e>>> {
        Some(&mut self.main_dispatcher)
    }

    fn get_shadow_dispatcher(&mut self) -> Option<&mut Option<Dispatcher<'d, 'e>>> {
        Some(&mut self.shadow_dispatcher)
    }

    fn reset_buttons(&mut self) {
        self.b_buttons_found = false;
        self.ui_buttons.clear();
    }

    fn get_buttons(&mut self) -> Option<&mut HashMap<Entity, Self::StateButton>> {
        Some(&mut self.ui_buttons)
    }
}

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, StateEvent> for CentreState<'d, 'e> {
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
        let StateData { world, data } = data;
        self.dispatch(&world);
        data.update_menu(&world);
        self.menu_duration += world.read_resource::<Time>().delta_seconds();

        if !self.b_buttons_found {
            self.b_buttons_found =
                self.insert_button(world, CentreButtons::NewGame, "menu_centre_newgame_button")
                    && self.insert_button(world, CentreButtons::Load, "menu_centre_load_button")
                    && self.insert_button(
                        world,
                        CentreButtons::Options,
                        "menu_centre_options_button",
                    )
                    && self.insert_button(
                        world,
                        CentreButtons::Credits,
                        "menu_centre_credits_button",
                    )
                    && self.insert_button(world, CentreButtons::Exit, "menu_centre_exit_button");
        }

        if !self.b_screens_loaded {
            use self::Completion::*;
            match self.progress_counter.complete() {
                Failed => {
                    self.b_screens_loaded = true;
                    warn!("Failed to load menu screen prefab(s).");

                    for err in self.progress_counter.errors() {
                        warn!("Asset type: {}\terror: {}", err.asset_type_name, err.error);
                        match err.asset_name.as_ref() {
                            "Prefabs/ui/MenuScreens/Options.ron" => {
                                self.screen_prefabs.remove(&MenuScreens::Options);
                            }
                            "Prefabs/ui/MenuScreens/Load.ron" => {
                                self.screen_prefabs.remove(&MenuScreens::LoadGame);
                            }
                            "Prefabs/ui/MenuScreens/Credits.ron" => {
                                self.screen_prefabs.remove(&MenuScreens::Credits);
                            }
                            "Prefabs/ui/MenuScreens/NewGame.ron" => {
                                self.screen_prefabs.remove(&MenuScreens::NewGame);
                            }
                            _ => {
                                warn!("Non implemented asset_name detected.");
                            }
                        };
                        for (key, _) in self.screen_prefabs.iter() {
                            warn!("screen_prefabs contains: {:?}", key);
                        }
                    }
                    Trans::None
                }
                Complete => {
                    self.b_screens_loaded = true;
                    #[cfg(feature = "debug")]
                    debug!("Loaded menu screen prefabs successfully.");

                    Trans::None
                }
                Loading => Trans::None,
            }
        } else {
            Trans::None
        }
    }

    fn shadow_update(&mut self, data: StateData<ToppaGameData>) {
        let StateData { world, data: _ } = data;
        self.shadow_dispatch(&world);
    }

    // Executed when this game state runs for the first time.
    fn on_start(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_dispatcher(&mut world);
        self.enable_shadow_dispatcher(&mut world);
        self.enable_current_screen(&mut world);

        self.insert_reachable_menu(
            world,
            MenuScreens::Options,
            "Prefabs/ui/MenuScreens/Options.ron",
        );
        self.insert_reachable_menu(
            world,
            MenuScreens::Credits,
            "Prefabs/ui/MenuScreens/Credits.ron",
        );
        self.insert_reachable_menu(
            world,
            MenuScreens::NewGame,
            "Prefabs/ui/MenuScreens/NewGame.ron",
        );
        self.insert_reachable_menu(
            world,
            MenuScreens::LoadGame,
            "Prefabs/ui/MenuScreens/Load.ron",
        );
    }

    // Executed when this game state gets popped or switched from.
    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.disable_dispatcher();
        self.disable_shadow_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when another game state is pushed onto the stack.
    fn on_pause(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when the application returns to this game state,
    // after another gamestate was popped from the stack.
    fn on_resume(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_dispatcher(&mut world);
        self.enable_current_screen(&mut world);
    }
}

impl<'a, 'b, 'd, 'e, 'f, 'g> CentreState<'d, 'e> {
    fn insert_reachable_menu(&mut self, world: &mut World, screen: MenuScreens, path: &str) {
        let prefab_handle =
            world.exec(|loader: UiLoader| loader.load(path, &mut self.progress_counter));

        self.screen_prefabs.insert(screen, prefab_handle);
    }

    fn btn_click(
        &self,
        world: &mut World,
        target: Entity,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        use self::CentreButtons::*;
        if let Some(button) = self.ui_buttons.get(&target) {
            match button {
                NewGame => self.btn_new_game(world),
                Load => self.btn_load(world),
                Options => self.btn_options(world),
                Credits => self.btn_credits(world),
                Exit => self.btn_exit(),
            }
        } else {
            Trans::None
        }
    }

    fn btn_exit(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Shutting down.");
        // TODO: User prompt : Are you sure you want to exit?
        Trans::Quit
    }

    fn btn_credits(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Credits screen.");
        Trans::Push(Box::new({
            if let Some(ref handle) = self.screen_prefabs.get(&MenuScreens::Credits) {
                CreditsState::new(world, Some({ *handle }.clone()))
            } else {
                CreditsState::new(world, None)
            }
        }))
    }

    fn btn_new_game(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("NewGame screen.");
        Trans::Push(Box::new({
            if let Some(ref handle) = self.screen_prefabs.get(&MenuScreens::NewGame) {
                NewGameState::new(world, Some({ *handle }.clone()))
            } else {
                NewGameState::new(world, None)
            }
        }))
    }

    fn btn_load(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("LoadGame screen.");
        Trans::Push(Box::new({
            if let Some(ref handle) = self.screen_prefabs.get(&MenuScreens::LoadGame) {
                LoadMenuState::new(world, Some({ *handle }.clone()))
            } else {
                LoadMenuState::new(world, None)
            }
        }))
    }

    fn btn_options(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Options screen.");
        Trans::Push(Box::new({
            if let Some(ref handle) = self.screen_prefabs.get(&MenuScreens::Options) {
                OptionsState::new(world, Some({ *handle }.clone()))
            } else {
                OptionsState::new(world, None)
            }
        }))
    }
}
