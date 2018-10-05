use std::collections::HashMap;
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
        UiCreator, UiLoader, UiPrefab, UiFinder, UiEventType,
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
    states::ToppaState,
};

#[derive(PartialEq, Eq, Hash, Debug)]
enum CentreButtons{
    NewGame,
    Load,
    Options,
    Credits,
    Exit,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum ReachableScreens{
    NewGame,
    Load,
    Options,
    Credits
}

/// The default state after opening Toppa Drill.
/// It should display a short amethyst logo, and then transist over to the Main Menu State.
pub struct CentreState<'d, 'e>{
    menu_duration: f32,
    dispatcher: Option<Dispatcher<'d, 'e>>,
    progress_counter: ProgressCounter,

    // The displayed Ui Entity, if any.
    current_screen: Option<Entity>,
    // The Handle of the Prefab for the displayed Ui Entity.
    ui_centre: Option<Handle<UiPrefab>>,
    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, CentreButtons>,
    // Map of the PrefabHandles for all reachable states (convenient for the `ToppaState::new()` call on State change)
    ui_screens: HashMap<ReachableScreens, Handle<UiPrefab>>,
    b_menu_screens_loaded: bool,
}

impl<'d, 'e> ToppaState for CentreState<'d, 'e>{
    fn dispatch(&mut self, world: &World){
        if let Some(ref mut dispatcher) = self.dispatcher{
            dispatcher.dispatch(&world.res);
        };
    }

    fn enable_dispatcher(&mut self){
        self.dispatcher = Some(
            DispatcherBuilder::new()
                .with(DummySystem{counter: 0}, "dummy_system", &[])
                .build()
        );
    }

    fn disable_dispatcher(&mut self){
        self.dispatcher = None;
    }

    fn get_current_screen(&self) -> Option<Entity>{
        self.current_screen
    }

    fn disable_current_screen(&mut self, world: &mut World){
        if let Some(entity) = self.current_screen{
            let _ = world.delete_entity(entity);
        };
    }

    fn enable_current_screen(&mut self, world: &mut World){
        if let Some(ref prefab_handle) = self.ui_centre{
            self.current_screen = Some(
                world.create_entity()
                    .with(prefab_handle.clone())
                    .build()
            );
        };
    }

    fn new(world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self{
        CentreState{
            menu_duration: 0.0,
            current_screen: None,
            ui_centre: screen_opt,
            progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::new(),
            ui_screens: HashMap::new(),
            b_menu_screens_loaded: false,
            dispatcher: None,
        }
    }
}

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, ()> for CentreState<'d, 'e>{
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
                use self::UiEventType::*;
                match ui_event.event_type{
                    Click => {
                        self.btn_click(ui_event.target)
                    },
                    _ => Trans::None
                }
            },
            _ => {
                Trans::None
            }
        }
    }

    fn update(&mut self, data: StateData<ToppaGameData>)
    -> Trans<ToppaGameData<'a, 'b>, ()>{
        let StateData {world, data} = data;
        self.dispatch(&world);
        data.update_menu(&world);
        self.menu_duration += world.read_resource::<Time>().delta_seconds();

        if !self.b_menu_screens_loaded{
            use self::Completion::*;
            match self.progress_counter.complete(){
                Failed =>{
                    self.b_menu_screens_loaded = true;
                    warn!("Failed to load menu screen prefab(s).");

                    for err in self.progress_counter.errors(){
                        warn!(
                            "Asset type: {}\terror: {}",
                            err.asset_type_name, err.error
                        );
                        match err.asset_name.as_ref(){
                            "resources/ui/MenuScreens/Options.ron" => {self.ui_screens.remove(&ReachableScreens::Options);},
                            "resources/ui/MenuScreens/Load.ron" => {self.ui_screens.remove(&ReachableScreens::Load);},
                            "resources/ui/MenuScreens/Credits.ron" => {self.ui_screens.remove(&ReachableScreens::Credits);},
                            "resources/ui/MenuScreens/NewGame.ron" => {self.ui_screens.remove(&ReachableScreens::NewGame);},
                            _ => {warn!("Non implemented asset_name detected.");},
                        };
                        for (key, _) in self.ui_screens.iter(){
                            warn!("ui_screens contains: {:?}", key);
                        }
                    }
                    Trans::None
                }
                Complete =>{
                    self.b_menu_screens_loaded = true;
                    info!("Loaded menu screen prefabs successfully.");
                    
                    self.insert_ui_screen(world, ReachableScreens::Options, "resources/ui/MenuScreens/Options.ron");
                    self.insert_ui_screen(world, ReachableScreens::Credits, "resources/ui/MenuScreens/Credits.ron");
                    self.insert_ui_screen(world, ReachableScreens::NewGame, "resources/ui/MenuScreens/NewGame.ron");
                    self.insert_ui_screen(world, ReachableScreens::Load, "resources/ui/MenuScreens/Load.ron");

                    self.insert_ui_button(world, CentreButtons::NewGame, "newgame_button");
                    self.insert_ui_button(world, CentreButtons::Load, "load_button");
                    self.insert_ui_button(world, CentreButtons::Options, "options_button");
                    self.insert_ui_button(world, CentreButtons::Credits, "credits_button");
                    self.insert_ui_button(world, CentreButtons::Exit, "exit_button");
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

    // Executed when this game state runs for the first time.
    fn on_start(&mut self, data: StateData<ToppaGameData>) {        
        let StateData {mut world, data} = data;
        self.enable_dispatcher();
        self.enable_current_screen(&mut world);
    }

    // Executed when this game state gets popped.
    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData {mut world, data} = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when another game state is pushed onto the stack.
    fn on_pause(&mut self, data: StateData<ToppaGameData>) {
        let StateData {mut world, data} = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when the application returns to this game state, 
    // after another gamestate was popped from the stack.
    fn on_resume(&mut self, data: StateData<ToppaGameData>) {
        let StateData {mut world, data} = data;
        self.enable_dispatcher();
        self.enable_current_screen(&mut world);
    }
}

impl<'a, 'b, 'd, 'e> CentreState<'d, 'e>{
    fn insert_ui_screen(&mut self, world: &mut World, screen: ReachableScreens, path: &str){
        let buffer_entity = world.exec(|mut loader: UiLoader| {                
            loader.load(path, &mut self.progress_counter)
        });

        self.ui_screens.insert(
            screen,
            buffer_entity
        );
    }

    fn insert_ui_button(&mut self, world: &mut World, button: CentreButtons, button_name: &str){
        world.exec(|finder: UiFinder| {
            if let Some(entity) = finder.find(button_name){
                info!("Found {}.", button_name);
                self.ui_buttons.insert(
                    entity,
                    button
                );
            }
            else{
                warn!("Couldn't find {}!", button_name);
            }
        });
    }

    fn btn_click(&self, target: Entity) -> Trans<ToppaGameData<'a, 'b>, ()>{
        use self::CentreButtons::*;
        if let Some(button) = self.ui_buttons.get(&target){
            match button{
                NewGame => self.btn_new_game(),
                Load => self.btn_load(),
                Options => self.btn_options(),
                Credits => self.btn_credits(),
                Exit => self.btn_exit(),
                _ => {
                    error!("Non-implemented CentreButton detected!");
                    Trans::None
                },
            }
        }
        else{
            Trans::None
        }
    }

    fn btn_exit(&self) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Shutting down.");
        // TODO: User prompt : Are you sure you want to exit?
        Trans::Quit
    }

    fn btn_credits(&self) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Show credits.");
        // TODO: Credits screen
        /*Trans::Push(
            Box::new(
                main_menu::CentreState::new(&mut world, self.menu_screen.clone())
            )
        )*/
        Trans::None
    }

    fn btn_new_game(&self) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("New game setup screen.");
        Trans::None
    }

    fn btn_load(&self) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Load savegame screen.");
        Trans::None
    }

    fn btn_options(&self) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Options screen.");
        Trans::None
    }
}