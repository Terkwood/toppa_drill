use amethyst::{
    prelude::*,
    ecs::prelude::*,
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
    core::{
        timing::Time,
        ArcThreadPool,
        SystemBundle,
    },
};

use {
    ToppaGameData,
    systems::DummySystem,
};

/// The default state after opening Toppa Drill.
/// It should display a short amethyst logo, and then transist over to the Main Menu State.
pub struct CentreState<'a, 'b>{
    menu_duration: f32,
    ui_centre: Option<Entity>,

    // TODO: Get rid of the Option<>
    dispatcher: Dispatcher<'a, 'b>,

    progress_counter: ProgressCounter,
    ui_ref_options: Option<Handle<UiPrefab>>,
    ui_ref_new_game: Option<Handle<UiPrefab>>, 
    ui_ref_savegames: Option<Handle<UiPrefab>>,
    ui_ref_credits: Option<Handle<UiPrefab>>,
    b_menu_screens_loaded: bool,
}

impl<'a, 'b> CentreState<'a, 'b>{
    pub fn new(world: &mut World) -> Self{
        let dispatcher = DispatcherBuilder::new()
            .with(DummySystem, "dummy_system", &[])
            .build();

        CentreState{
            menu_duration: 0.0,
            ui_centre: None,
            progress_counter: ProgressCounter::new(),
            ui_ref_options: None,
            ui_ref_new_game: None,
            ui_ref_savegames: None,
            ui_ref_credits: None,
            b_menu_screens_loaded: false,
            dispatcher,
        }
    }
}

impl<'a, 'b> State<ToppaGameData<'a, 'b>, ()> for CentreState<'a, 'b>{
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
        let StateData {world, data} = data;
        //self.dispatch(world);
        self.dispatcher.dispatch(&world.res);
        data.update_menu(&world);
        self.menu_duration += world.read_resource::<Time>().delta_seconds();

        if !self.b_menu_screens_loaded{
            use self::Completion::*;
            match self.progress_counter.complete(){
                Failed =>{
                    warn!("Failed to load menu screen prefab(s).");

                    for err in self.progress_counter.errors(){
                        warn!(
                            "Asset type: {}\terror: {}",
                            err.asset_type_name, err.error
                        );
                    }
                    Trans::None
                }
                Complete =>{
                    info!("Loaded menu screen prefabs successfully.");
                    self.b_menu_screens_loaded = true;
                    Trans::None
                }
                Loading => {
                    Trans::None
                }
            }
        }
        else{
            Trans::None
        }
    }

    fn on_start(&mut self, data: StateData<ToppaGameData<'a, 'b>>) {        
        let StateData {world, data} = data;

        self.ui_ref_options = Some(
            world.exec(|mut loader: UiLoader| {                
                loader.load("resources/ui/MenuScreens/Options.ron", &mut self.progress_counter)
            })
        );
        self.ui_ref_new_game = Some(
            world.exec(|mut loader: UiLoader| {                
                loader.load("resources/ui/MenuScreens/NewGame.ron", &mut self.progress_counter)
            })
        );
        self.ui_ref_savegames = Some(
            world.exec(|mut loader: UiLoader| {                
                loader.load("resources/ui/MenuScreens/Savegames.ron", &mut self.progress_counter)
            })
        );
        self.ui_ref_credits = Some(
            world.exec(|mut loader: UiLoader| {                
                loader.load("resources/ui/MenuScreens/Credits.ron", &mut self.progress_counter)
            })
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