use amethyst::{
    prelude::*,
    input::{
        is_close_requested, is_key_down
    },
    renderer::{
        VirtualKeyCode,
    },
    assets::{
        ProgressCounter, Completion, Handle,
    },
    ui::{
        UiCreator, UiLoader, UiPrefab,
    },
    ecs::prelude::{
        Entity,
    },
    core::{
        timing::Time,
    }
};

use {
    ToppaGameData,
    states::{
        main_menu,
        ToppaState,
    },
};

/// The default state after opening Toppa Drill.
/// It should display a short amethyst logo, and then transist over to the Main Menu State.
pub struct StartupState{
    progress_counter: ProgressCounter,
    next_states_screen: Option<Handle<UiPrefab>>, 
    this_states_screens: Vec<Handle<UiPrefab>>,
    current_screen: Option<Entity>,
    display_duration: f32,
    duration: f32,
    b_screens_loaded: bool,
}

impl StartupState{
    /// Creates a new StartupState instance with a progress counter for asset-load tracking.
    /// The display duration specifies how long each subscreen should be shown before moving on.
    pub fn new(display_duration: f32) -> Self{
        let progress_counter = ProgressCounter::new();
        info!("ProgressCounter created.");

        StartupState {
            progress_counter,
            next_states_screen: None,
            this_states_screens: Vec::new(),
            current_screen: None,
            display_duration,
            duration: 0.0,
            b_screens_loaded: false,
        }
    }
}

impl<'a, 'b> State<ToppaGameData<'a, 'b>, ()> for StartupState{
    fn handle_event(&mut self, data: StateData<ToppaGameData>, event: StateEvent<()>) 
    -> Trans<ToppaGameData<'a, 'b>, ()>{
        // let StateData {world, data} = data;
        match &event {
            StateEvent::Window(wnd_event) => {
                if is_close_requested(&wnd_event) || is_key_down(&wnd_event, VirtualKeyCode::Escape) {
                    Trans::Quit
                }
                else {
                    Trans::None
                }
            },
            StateEvent::Ui(ui_event) => {
                Trans::None
            },
            _ => {
                Trans::None
            }
        }
    }

    fn update(&mut self, data: StateData<ToppaGameData<'a, 'b>>)
    -> Trans<ToppaGameData<'a, 'b>, ()>{
        let StateData {mut world, mut data} = data;
        data.update_menu(&world);

        self.duration += world.read_resource::<Time>().delta_seconds();
        
        if !self.b_screens_loaded{
            use self::Completion::*;
            match self.progress_counter.complete(){
                Failed =>{
                    warn!("Failed to load asset(s).");
                    self.duration = 2.0;

                    let mut trans = Trans::None;

                    for err in self.progress_counter.errors(){
                        warn!(
                            "Asset type: {}\terror: {}",
                            err.asset_type_name, err.error
                        );

                        if err.asset_name == "resources/ui/StartupScreen/DevelopedByTelzhaak.ron" {
                            error!("Main Menu screen could not be loaded. Closing application.");
                            trans = Trans::Quit
                        }
                    }

                    trans
                }
                Complete =>{
                    self.b_screens_loaded = true;
                    self.duration = 0.0;

                    Trans::None
                }
                Loading => {
                    Trans::None
                }
            }
        }
        else if self.duration > self.display_duration{
            self.duration = 0.0;
            if let Some(entity) = self.current_screen{
                let _ = world.delete_entity(entity);
            };
            if let Some(ref ui_prefab) = self.this_states_screens.pop(){
                self.current_screen = Some(
                    world.create_entity()
                        .with(ui_prefab.clone())
                        .build()
                );
                Trans::None
            }
            else{
                Trans::Switch(
                    Box::new(
                        {
                            if let Some(entity) = self.current_screen{
                                let _ = world.delete_entity(entity);
                            };
                            main_menu::CentreState::new(&mut world, self.next_states_screen.clone())
                        }
                    )
                )
            }            
        }
        else{
            Trans::None
        }
    }

    fn on_start(&mut self, data: StateData<ToppaGameData>) {        
        let StateData {world, data} = data;

        self.current_screen = Some(
            world.exec(|mut creator: UiCreator| {                
                    creator.create("resources/ui/StartupScreen/PoweredByAmethyst.ron", ())
                }
            )
        );
        
        let handle = world.exec(|mut loader: UiLoader| {                
                loader.load("resources/ui/StartupScreen/DevelopedByTelzhaak.ron", &mut self.progress_counter)
            }
        );
        self.this_states_screens.push(handle);

        self.next_states_screen = Some(
            world.exec(|mut loader: UiLoader| {                
                    loader.load("resources/ui/MenuScreens/Centre.ron", &mut self.progress_counter)
                }
            )
        );
    }

// For the sake of completeness: 
    fn on_stop(&mut self, data: StateData<ToppaGameData<'a, 'b>>) {
        let StateData {world, data} = data;
        // Executed when this game state exits
    }

    fn on_pause(&mut self, data: StateData<ToppaGameData<'a, 'b>>) {
        let StateData {world, data} = data;
        // Executed when another game state is pushed onto the stack
    }

    fn on_resume(&mut self, data: StateData<ToppaGameData<'a, 'b>>) {
        let StateData {world, data} = data;
        // Executed when the application returns to this game state, 
        // after another gamestate was popped from the stack
    }
}