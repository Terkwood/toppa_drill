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
    states::main_menu,
};

/// The default state after opening Toppa Drill.
/// It should display a short amethyst logo, and then transist over to the Main Menu State.
pub struct StartupState{
    progress_counter: ProgressCounter,
    menu_screen: Option<Handle<UiPrefab>>, 
    ls_devby: Option<Handle<UiPrefab>>,
    ls_poweredbyamethyst: Option<Entity>,
    display_duration: f32,
    duration: f32,
    b_displaying_devs: bool,
}

impl StartupState{
    /// Creates a new StartupState instance with a progress counter for asset-load tracking.
    /// The display duration specifies how long each subscreen should be shown before moving on.
    pub fn new(display_duration: f32) -> Self{
        let progress_counter = ProgressCounter::new();
        info!("ProgressCounter created.");

        StartupState {
            progress_counter,
            menu_screen: None,
            ls_devby: None,
            ls_poweredbyamethyst: None,
            display_duration,
            duration: 0.0,
            b_displaying_devs: false,
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
        
        if self.duration > self.display_duration{
            // If developers haven't been displayed yet, display them (if possible),
            // otherwise trans to main menu.
            if !self.b_displaying_devs{
                use self::Completion::*;
                match self.progress_counter.complete(){
                    Failed =>{
                        warn!("Failed to load asset(s).");
                        self.b_displaying_devs = true;
                        self.duration = 2.0;

                        let mut trans = Trans::None;

                        for err in self.progress_counter.errors(){
                            warn!(
                                "Asset type: {}\terror: {}",
                                err.asset_type_name, err.error
                            );

                            if err.asset_name == "resources/ui/StartupScreen/DevelopedByTelzhaak.ron" {
                                warn!("Skipping *Developed By* screen.");
                                trans = Trans::None
                            }
                            else{
                                error!("Main Menu screen could not be loaded. Closing application.");
                                trans = Trans::Quit
                            }
                        }

                        trans
                    }
                    Complete =>{
                        info!("Loaded DevelopedByTelzhaak.ron successfully.");
                        // Removing "Powered By Amethyst" from screen.
                        if let Some(entity) = self.ls_poweredbyamethyst{
                            let _ = world.delete_entity(entity);
                        };
                        if let Some(ref ui_prefab) = self.ls_devby{
                            world.create_entity()
                                .with(ui_prefab.clone())
                                .build();
                        };

                        self.b_displaying_devs = true;
                        self.duration = 0.0;

                        Trans::None
                    }
                    Loading => {
                        Trans::None
                    }
                }
            }
            else{
                Trans::Switch(
                    Box::new(
                        main_menu::CentreState::new(&mut world)
                    )
                )

                //Trans::None
            }
        }
        else{
            Trans::None
        }
    }

    fn on_start(&mut self, data: StateData<ToppaGameData>) {        
        let StateData {world, data} = data;

        self.ls_poweredbyamethyst = Some(
            world.exec(|mut creator: UiCreator| {                
                    creator.create("resources/ui/StartupScreen/PoweredByAmethyst.ron", ())
                }
            )
        );
        self.ls_devby = Some(
            world.exec(|mut loader: UiLoader| {                
                    loader.load("resources/ui/StartupScreen/DevelopedByTelzhaak.ron", &mut self.progress_counter)
                }
            )
        );
        self.menu_screen = Some(
            world.exec(|mut loader: UiLoader| {                
                    loader.load("resources/ui/MenuScreens/Centre.ron", &mut self.progress_counter)
                }
            )
        );
    }

// For the sake of completeness: 
    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData {world, data} = data;
        // Executed when this game state exits
    }

    fn on_pause(&mut self, data: StateData<ToppaGameData>) {
        let StateData {world, data} = data;
        // Executed when another game state is pushed onto the stack
    }

    fn on_resume(&mut self, data: StateData<ToppaGameData>) {
        let StateData {world, data} = data;
        // Executed when the application returns to this game state, 
        // after another gamestate was popped from the stack
    }
}