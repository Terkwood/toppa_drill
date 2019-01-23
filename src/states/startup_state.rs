use amethyst::{
    assets::{Completion, Handle, ProgressCounter},
    core::timing::Time,
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::VirtualKeyCode,
    shrev::EventChannel,
    ui::{UiCreator, UiLoader, UiPrefab},
};

use crate::{
    components::{for_characters::TagGenerator, IsIngameEntity},
    events::planet_events::ChunkEvent,
    resources::{GameSprites, RenderConfig},
    states::{main_menu, ToppaState},
    ToppaGameData,
};

/// The default state after opening Toppa Drill.
/// It should display a short amethyst logo, and then transist over to the Main Menu State.
pub struct StartupState {
    progress_counter:    ProgressCounter,
    next_states_screen:  Option<Handle<UiPrefab,>,>,
    this_states_screens: Vec<Handle<UiPrefab,>,>,
    current_screen:      Option<Entity,>,
    display_duration:    f32,
    duration:            f32,
    b_screens_loaded:    bool,
}

impl StartupState {
    /// Creates a new StartupState instance with a progress counter for asset-load tracking.
    /// The display duration specifies how long each subscreen should be shown before moving on.
    pub fn new(display_duration: f32) -> Self {
        StartupState {
            progress_counter: ProgressCounter::new(),
            next_states_screen: None,
            this_states_screens: Vec::new(),
            current_screen: None,
            display_duration,
            duration: 0.0,
            b_screens_loaded: false,
        }
    }
}

impl<'a, 'b,> State<ToppaGameData<'a, 'b,>, StateEvent,> for StartupState {
    fn handle_event(
        &mut self,
        data: StateData<'_, ToppaGameData<'_, '_,>,>,
        event: StateEvent,
    ) -> Trans<ToppaGameData<'a, 'b,>, StateEvent,> {
        let StateData {
            world: _,
            data: _,
        } = data;
        match &event {
            StateEvent::Window(wnd_event,) => {
                if is_close_requested(&wnd_event,)
                    || is_key_down(&wnd_event, VirtualKeyCode::Escape,)
                {
                    Trans::Quit
                }
                else {
                    Trans::None
                }
            },
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, ToppaGameData<'a, 'b,>,>,
    ) -> Trans<ToppaGameData<'a, 'b,>, StateEvent,> {
        let StateData {
            mut world,
            data,
        } = data;
        data.update_menu(&world,);

        self.duration += world.read_resource::<Time>().delta_seconds();

        if !self.b_screens_loaded {
            use self::Completion::*;
            match self.progress_counter.complete() {
                Failed => {
                    warn!("Failed to load asset(s).");
                    self.duration = 2.0;

                    let mut trans = Trans::None;

                    for err in self.progress_counter.errors() {
                        error!("Asset type: {}\terror: {}", err.asset_type_name, err.error);

                        if err.asset_name == "Prefabs/ui/MenuScreens/Centre.ron" {
                            error!("Main Menu screen could not be loaded. Closing application.");
                            trans = Trans::Quit
                        }
                    }

                    trans
                },
                Complete => {
                    self.b_screens_loaded = true;
                    self.duration = 0.0;

                    Trans::None
                },
                Loading => Trans::None,
            }
        }
        else if self.duration > self.display_duration {
            self.duration = 0.0;
            if let Some(entity,) = self.current_screen {
                let _ = world.delete_entity(entity,);
            };
            if let Some(ref ui_prefab,) = self.this_states_screens.pop() {
                self.current_screen = Some(world.create_entity().with(ui_prefab.clone(),).build(),);
                Trans::None
            }
            else {
                Trans::Switch(Box::new({
                    if let Some(entity,) = self.current_screen {
                        let _ = world.delete_entity(entity,);
                    };
                    main_menu::CentreState::new(&mut world, self.next_states_screen.clone(),)
                },),)
            }
        }
        else {
            Trans::None
        }
    }

    fn on_start(&mut self, data: StateData<'_, ToppaGameData<'_, '_,>,>,) {
        let StateData {
            world,
            data: _,
        } = data;

        self.current_screen = Some(world.exec(|mut creator: UiCreator<'_,>| {
            creator.create("Prefabs/ui/StartupScreen/PoweredByAmethyst.ron", (),)
        },),);

        let handle = world.exec(|loader: UiLoader<'_,>| {
            loader.load(
                "Prefabs/ui/StartupScreen/DevelopedByTelzhaak.ron",
                &mut self.progress_counter,
            )
        },);
        self.this_states_screens.push(handle,);

        self.next_states_screen = Some(world.exec(|loader: UiLoader<'_,>| {
            loader.load(
                "Prefabs/ui/MenuScreens/Centre.ron",
                &mut self.progress_counter,
            )
        },),);

        let ren_con = RenderConfig::new((128.0, 128.0), 1, (1920*2, 1080*2));

        world.add_resource::<RenderConfig>(ren_con,);
        world.add_resource::<TagGenerator>(TagGenerator::default(),);
        world.add_resource::<GameSprites>(GameSprites::default(),);
        world.add_resource(EventChannel::<ChunkEvent,>::new(),);
        world.register::<IsIngameEntity>();
    }

    // For the sake of completeness:
    fn on_stop(&mut self, data: StateData<'_, ToppaGameData<'a, 'b,>,>,) {
        let StateData {
            world: _,
            data: _,
        } = data;
        // Executed when this game state exits
    }

    fn on_pause(&mut self, data: StateData<'_, ToppaGameData<'a, 'b,>,>,) {
        let StateData {
            world: _,
            data: _,
        } = data;
        // Executed when another game state is pushed onto the stack
    }

    fn on_resume(&mut self, data: StateData<'_, ToppaGameData<'a, 'b,>,>,) {
        let StateData {
            world: _,
            data: _,
        } = data;
        // Executed when the application returns to this game state,
        // after another gamestate was popped from the stack
    }
}
