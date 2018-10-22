use std::collections::HashMap;

use amethyst::{
    assets::{Completion, Handle, ProgressCounter},
    core::timing::Time,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{HiddenPropagate, VirtualKeyCode},
    ui::{UiCreator, UiEventType, UiFinder, UiLoader, UiPrefab},
};

use {
    components::for_characters::{TagGenerator, TagPlayer},
    resources::{
        ingame::{planet::Planet, GameSessionData, GameSprites},
        RenderConfig,
    },
    states::{ingame, ToppaState},
    systems::DummySystem,
    ToppaGameData,
};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum NewGameButtons {
    Back,
    CreateNewGame,
}

struct GameInfo {
    pub name: &'static str,
    pub planet_dim: (u64, u64),
    pub chunk_dim: (u64, u64),
}

impl Default for GameInfo{
    fn default() -> Self{
        GameInfo{
            name: "Terra Incognita",
            planet_dim: (16, 16),
            chunk_dim: (16, 32),
        }
    }
}

/// The game creation state, where a new game can be started.
/// TODO: Buttons and TextBoxes etc, to enter GameName, planet and chunk dimensions, ... .
pub struct NewGameState<'d, 'e> {
    menu_duration: f32,
    dispatcher: Option<Dispatcher<'d, 'e>>,
    progress_counter: ProgressCounter,

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

impl<'d, 'e> ToppaState for NewGameState<'d, 'e> {
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
        self.b_buttons_found = false;
        self.ui_buttons.clear();
        if let Some(ref prefab_handle) = self.current_screen_prefab {
            self.current_screen = Some(world.create_entity().with(prefab_handle.clone()).build());
        };
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        NewGameState {
            menu_duration: 0.0,
            current_screen: None,
            current_screen_prefab: screen_opt,
            progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::new(),
            b_buttons_found: false,
            dispatcher: None,
            game_info: GameInfo::default(),
        }
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
            self.insert_button(&mut world, NewGameButtons::Back, "menu_newgame_back_button");
            self.insert_button(
                &mut world,
                NewGameButtons::CreateNewGame,
                "menu_newgame_creategame_button",
            );
            self.b_buttons_found = true;
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
        self.disable_current_screen(&mut world);
    }

    // Executed when the application returns to this game state,
    // after another gamestate was popped from the stack.
    fn on_resume(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_dispatcher();
        self.enable_current_screen(&mut world);
    }
}

impl<'a, 'b, 'd, 'e> NewGameState<'d, 'e> {
    fn insert_button(&mut self, world: &mut World, button: NewGameButtons, button_name: &str) {
        world.exec(|finder: UiFinder| {
            if let Some(entity) = finder.find(button_name) {
                info!("Found {}.", button_name);
                self.ui_buttons.insert(entity, button);
            } else {
                warn!("Couldn't find {}!", button_name);
            }
        });
    }

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
        info!("Returning to CentreState.");
        Trans::Pop
    }

    fn btn_creategame(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        info!("Creating new game.");
        // NOTE: Think about how to do this better
        world.add_resource::<TagGenerator>(TagGenerator::default());
        world.add_resource::<GameSprites>(GameSprites::default());

        let ren_con = &world.read_resource::<RenderConfig>().clone();
        let session_data = GameSessionData::new(
            self.game_info.name,
            self.game_info.planet_dim,
            self.game_info.chunk_dim,
            ren_con,
        );
        world.add_resource::<GameSessionData>(session_data);

        let ingame_ui_prefab_handle =
            Some(world.exec(|loader: UiLoader| loader.load("Prefabs/ui/Ingame/Base.ron", ())));

        Trans::Switch(Box::new({
            ingame::IngameBaseState::new(world, ingame_ui_prefab_handle.clone())
        }))
    }
}
