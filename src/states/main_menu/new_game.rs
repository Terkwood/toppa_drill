use std::{collections::HashMap, u64};

use amethyst::{
    assets::Handle,
    core::timing::Time,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{UiEventType, UiLoader, UiPrefab},
};

use components::for_characters::TagGenerator;
use resources::{
    ingame::{GameSessionData, GameSprites},
    RenderConfig,
};
use states::{ingame, ToppaState};
use systems::DummySystem;
use ToppaGameData;

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum NewGameButtons {
    Back,
    CreateNewGame,
}

struct GameInfo {
    pub name: &'static str,
    pub planet_dim: (u64, u64),
    pub chunk_dim: (u64, u64),
}

impl Default for GameInfo {
    fn default() -> Self {
        GameInfo {
            name: "Terra Incognita",
            planet_dim: (16, 16),
            chunk_dim: (16, 32),
        }
    }
}

impl GameInfo {
    pub fn new(name: &'static str, planet_dim: (u64, u64), chunk_dim: (u64, u64)) -> Self {
        GameInfo {
            name,
            planet_dim,
            chunk_dim,
        }
    }
}

/// The game creation state, where a new game can be started.
/// TODO: Buttons and TextBoxes etc, to enter GameName, planet and chunk dimensions, ... .
pub struct NewGameState<'d, 'e> {
    menu_duration: f32,
    main_dispatcher: Option<Dispatcher<'d, 'e>>,

    // The displayed Ui Entity, if any.
    current_screen: Option<Entity>,
    // The Handle of the Prefab for the displayed Ui Entity.
    current_screen_prefab: Option<Handle<UiPrefab>>,
    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, NewGameButtons>,
    b_buttons_found: bool,

    // Info specific to the game about to be created.
    // e.g. the player count, names, etc...
    game_info: GameInfo,
}

impl<'d, 'e> ToppaState<'d, 'e> for NewGameState<'d, 'e> {
    type StateButton = NewGameButtons;
    fn enable_dispatcher(&mut self, world: &mut World) {
        self.main_dispatcher = Some({
            let mut dispatcher = DispatcherBuilder::new()
                .with(DummySystem { counter: 0 }, "dummy_system", &[])
                .build();

            dispatcher.setup(&mut world.res);
            dispatcher
        });
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        NewGameState {
            menu_duration: 0.0,
            current_screen: None,
            current_screen_prefab: screen_opt,
            ui_buttons: HashMap::new(),
            b_buttons_found: false,
            main_dispatcher: None,
            game_info: GameInfo::new("Mark", (128, 128), (32, 32)), //GameInfo::new("Trumpet", (2, 3), (3, 4)),
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

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, StateEvent> for NewGameState<'d, 'e> {
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
        self.menu_duration += world.read_resource::<Time>().delta_seconds();

        if !self.b_buttons_found {
            self.b_buttons_found =
                self.insert_button(&mut world, NewGameButtons::Back, "menu_newgame_back_button")
                    && self.insert_button(
                        &mut world,
                        NewGameButtons::CreateNewGame,
                        "menu_newgame_creategame_button",
                    );
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

impl<'a, 'b, 'd, 'e> NewGameState<'d, 'e> {
    fn btn_click(
        &self,
        world: &mut World,
        target: Entity,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        use self::NewGameButtons::*;
        if let Some(button) = self.ui_buttons.get(&target) {
            match button {
                Back => self.btn_back(),
                CreateNewGame => self.btn_creategame(world),
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

    fn btn_creategame(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Creating new game.");
        // NOTE: Think about how to do this better
        world.add_resource::<TagGenerator>(TagGenerator::default());
        world.add_resource::<GameSprites>(GameSprites::default());

        let ren_con = &world.read_resource::<RenderConfig>().clone();
        let session_data = GameSessionData::new(
            self.game_info.name.to_string(),
            self.game_info.planet_dim,
            self.game_info.chunk_dim,
            ren_con,
        );

        // TODO: Get rid
        use amethyst::shrev::EventChannel;
        use components::for_characters::{player::Position, PlayerBase};
        use events::planet_events::ChunkEvent;
        world.register::<PlayerBase>();
        world.register::<Position>();
        world.add_resource(EventChannel::<ChunkEvent>::new());
        world.add_resource::<GameSessionData>(session_data);
        // end: get rid

        let ingame_ui_prefab_handle =
            Some(world.exec(|loader: UiLoader| loader.load("Prefabs/ui/Ingame/Base.ron", ())));

        Trans::Switch(Box::new({
            ingame::IngameBaseState::new(world, ingame_ui_prefab_handle)
        }))
    }
}
