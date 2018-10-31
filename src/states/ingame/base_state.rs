use amethyst::{
    assets::{Handle, ProgressCounter},
    core::{
        transform::components::Transform,
        cgmath::Vector3,
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    ui::{UiEventType, UiPrefab},
};

use entities;
use resources::ingame::{GameSessionData, SavegamePaths};
use states::ToppaState;
use std::collections::HashMap;
use systems::{
    ingame::{EngineForceSystem, GravitationSystem, MovementSystem, PlayerPositionSystem},
    serialization::HotChunkSystem,
    DummySystem,
};
use ToppaGameData;

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum BaseStateButtons {
    Inventory,
    Options,
    Exit,
    Save,
    Mute,
}

/// The game creation state, where a new game can be started.
pub struct IngameBaseState<'d, 'e> {
    main_dispatcher: Option<Dispatcher<'d, 'e>>,
    shadow_dispatcher: Option<Dispatcher<'d, 'e>>,
    progress_counter: ProgressCounter,

    // The displayed Ui Entity, if any.
    current_screen: Option<Entity>,
    // The Handle of the Prefab for the displayed Ui Entity.
    current_screen_prefab: Option<Handle<UiPrefab>>,
    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, BaseStateButtons>,
    b_buttons_found: bool,
}

impl<'d, 'e> ToppaState<'d, 'e> for IngameBaseState<'d, 'e> {
    type StateButton = BaseStateButtons;
    fn enable_dispatcher(&mut self, world: &mut World) {
        self.main_dispatcher = Some({
            let mut dispatcher = DispatcherBuilder::new()
                .with(DummySystem::default(), "dummy_system", &[])
                .with(GravitationSystem, "gravitation_system", &[])
                .with(
                    EngineForceSystem,
                    "engine_force_system",
                    &["gravitation_system"],
                )
                .with(MovementSystem, "movement_system", &["engine_force_system"])
                .with(
                    PlayerPositionSystem::default(),
                    "player_position_system",
                    &[],
                )
                .with(
                    HotChunkSystem::new(),
                    "hotchunk_system",
                    &["player_position_system"],
                )
                .build();

            dispatcher.setup(&mut world.res);

            dispatcher
        });
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self {
        IngameBaseState {
            current_screen: None,
            current_screen_prefab: screen_opt,
            progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::new(),
            b_buttons_found: false,
            main_dispatcher: None,
            shadow_dispatcher: None,
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
        }
    }

    fn update(
        &mut self,
        data: StateData<ToppaGameData>,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, data } = data;
        self.dispatch(&world);
        data.update_ingame(&world);

        if !self.b_buttons_found {
            self.b_buttons_found = self.insert_button(
                &mut world,
                BaseStateButtons::Inventory,
                "ingame_base_inventory_button",
            ) && self.insert_button(
                &mut world,
                BaseStateButtons::Exit,
                "ingame_base_exit_button",
            ) && self.insert_button(
                &mut world,
                BaseStateButtons::Save,
                "ingame_base_save_button",
            ) && self.insert_button(
                &mut world,
                BaseStateButtons::Mute,
                "ingame_base_mute_button",
            ) && self.insert_button(
                &mut world,
                BaseStateButtons::Options,
                "ingame_base_options_button",
            );
        }

        Trans::None
    }

    fn shadow_update(&mut self, data: StateData<ToppaGameData>) {
        let StateData { world, data: _ } = data;
        self.shadow_dispatch(&world);
    }

    // Executed when this game state runs for the first time.
    fn on_start(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_current_screen(&mut world);
        self.enable_dispatcher(&mut world);

        // TODO: Get rid.
        entities::tile::prepare_spritesheet(world, Some(&mut self.progress_counter));
        {
            let game_name = world.read_resource::<GameSessionData>().game_name;
            world.add_resource(SavegamePaths::init("./", game_name));
        }

        entities::player_parts::init_player(world, None);

        let mut transform = Transform::default();
        transform.translation = Vector3::new(
            500.0,
            400.0,
            40.0
        );
        if let Err(e) = entities::player_parts::new_player(
            world,
            &transform,
            entities::player_parts::ShipTypes::NotImplemented,
        ) {
            error!("Error creating new player: {:?}", e);
        };
    }

    // Executed when this game state gets popped.
    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when another game state is pushed onto the stack.
    fn on_pause(&mut self, data: StateData<ToppaGameData>) {
        let StateData { world: _, data: _ } = data;
        self.disable_dispatcher();
    }

    // Executed when the application returns to this game state,
    // after another gamestate was popped from the stack.
    fn on_resume(&mut self, data: StateData<ToppaGameData>) {
        let StateData { mut world, data: _ } = data;
        self.enable_dispatcher(&mut world);
    }
}

impl<'a, 'b, 'd, 'e> IngameBaseState<'d, 'e> {
    fn btn_click(
        &self,
        world: &mut World,
        target: Entity,
    ) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
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
        #[cfg(feature = "debug")]
        debug!("Opening Inventory not implemented yet.");
        Trans::None
    }

    fn btn_options(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Opening Options not implemented yet.");
        Trans::None
    }

    fn btn_exit(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Exiting to main menu.");
        Trans::Pop
    }

    fn btn_save(&self, world: &World) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Saving game.");
        use systems::serialization::SerSavegameSystem;
        SerSavegameSystem.run_now(&world.res);

        Trans::None
    }

    fn btn_mute(&self) -> Trans<ToppaGameData<'a, 'b>, StateEvent> {
        #[cfg(feature = "debug")]
        debug!("Muting game not implemented yet..");
        Trans::None
    }
}
