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
use amethyst::renderer::Hidden;
use {
    ToppaGameData,
    systems::DummySystem,
    super::super::ToppaState,
    super::*,
};

static mut Loaded_visible: bool = true;

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum CentreButtons{
    NewGame,
    Load,
    Options,
    Credits,
    Exit,
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
    ui_screen: Option<Handle<UiPrefab>>,
    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, CentreButtons>,
    // Map of the PrefabHandles for all reachable states (convenient for the `ToppaState::new()` call on State change)
    ui_screens: HashMap<super::MenuScreens, Handle<UiPrefab>>,
    b_screens_loaded: bool,
    b_buttons_found: bool,
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

    fn disable_current_screen(&mut self, world: &mut World){
        if let Some(entity) = self.current_screen{
            let mut hidden_component_storage = world.write_storage::<Hidden>();
            match hidden_component_storage.insert(entity, Hidden::default()){
                Ok(v) => {},
                Err(e) => error!("Failed to add HiddenComponent to CentreState Ui. {:?}", e),
            };
        };
    }

    fn enable_current_screen(&mut self, world: &mut World){
        self.b_buttons_found = false;        
        match self.current_screen{
            None => {
                if let Some(ref prefab_handle) = self.ui_screen{
                    self.current_screen = Some(
                        world.create_entity()
                            .with(prefab_handle.clone())
                            .build()
                    );
                }
            },
            Some(entity) => {
                let mut hidden_component_storage = world.write_storage::<Hidden>();
                hidden_component_storage.remove(entity);
            }
        };
        
    }

    fn new(world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self{
        CentreState{
            menu_duration: 0.0,
            current_screen: None,
            ui_screen: screen_opt.clone(),
            progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::new(),
            ui_screens: HashMap::new(),
            b_screens_loaded: false,
            b_buttons_found: false,
            dispatcher: None,
        }
    }
}

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, ()> for CentreState<'d, 'e>{
    fn handle_event(&mut self, data: StateData<ToppaGameData>, event: StateEvent<()>) 
    -> Trans<ToppaGameData<'a, 'b>, ()>{
        let StateData {mut world, data} = data;
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
                        self.btn_click(&mut world, ui_event.target)
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

        if !self.b_buttons_found{
            error!("Buttons not found!");
            self.insert_button(world, CentreButtons::NewGame, "newgame_button");
            self.insert_button(world, CentreButtons::Load, "load_button");
            self.insert_button(world, CentreButtons::Options, "options_button");
            self.insert_button(world, CentreButtons::Credits, "credits_button");
            self.insert_button(world, CentreButtons::Exit, "exit_button");
            self.b_buttons_found = true;
        }

        if !self.b_screens_loaded{
            use self::Completion::*;
            match self.progress_counter.complete(){
                Failed =>{
                    self.b_screens_loaded = true;
                    warn!("Failed to load menu screen prefab(s).");

                    for err in self.progress_counter.errors(){
                        warn!(
                            "Asset type: {}\terror: {}",
                            err.asset_type_name, err.error
                        );
                        match err.asset_name.as_ref(){
                            "resources/ui/MenuScreens/Options.ron" => {self.ui_screens.remove(&MenuScreens::Options);},
                            "resources/ui/MenuScreens/Load.ron" => {self.ui_screens.remove(&MenuScreens::LoadGame);},
                            "resources/ui/MenuScreens/Credits.ron" => {self.ui_screens.remove(&MenuScreens::Credits);},
                            "resources/ui/MenuScreens/NewGame.ron" => {self.ui_screens.remove(&MenuScreens::NewGame);},
                            _ => {warn!("Non implemented asset_name detected.");},
                        };
                        for (key, _) in self.ui_screens.iter(){
                            warn!("ui_screens contains: {:?}", key);
                        }
                    }
                    Trans::None
                }
                Complete =>{
                    self.b_screens_loaded = true;
                    info!("Loaded menu screen prefabs successfully.");
                    
                    self.insert_reachable_menu(world, MenuScreens::Options, "resources/ui/MenuScreens/Options.ron");
                    self.insert_reachable_menu(world, MenuScreens::Credits, "resources/ui/MenuScreens/Credits.ron");
                    self.insert_reachable_menu(world, MenuScreens::NewGame, "resources/ui/MenuScreens/NewGame.ron");
                    self.insert_reachable_menu(world, MenuScreens::LoadGame, "resources/ui/MenuScreens/Load.ron");
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
    fn insert_reachable_menu(&mut self, world: &mut World, screen: MenuScreens, path: &str){
        let prefab_handle = world.exec(|mut loader: UiLoader| {                
            loader.load(path, &mut self.progress_counter)
        });

        self.ui_screens.insert(
            screen,
            prefab_handle
        );
    }

    fn insert_button(&mut self, world: &mut World, button: CentreButtons, button_name: &str){
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

    fn btn_click(&self, world: &mut World, target: Entity) -> Trans<ToppaGameData<'a, 'b>, ()>{
        use self::CentreButtons::*;
        let entity = target.clone();
        if let Some(button) = self.ui_buttons.get(&target){
            match button{
                NewGame => self.btn_new_game(world),
                Load => self.btn_load(world),
                Options => self.btn_options(world),
                Credits => self.btn_credits(world),
                Exit => self.btn_exit(world),
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

    fn btn_exit(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Shutting down.");
        // TODO: User prompt : Are you sure you want to exit?
        Trans::Quit
    }

    fn btn_credits(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Credits screen.");
        Trans::Push(
            Box::new(
                {
                    if let Some(ref handle) = self.ui_screens.get(&MenuScreens::Credits){
                        CreditsState::new(world, Some({*handle}.clone()))
                    }
                    else{
                        CreditsState::new(world, None)
                    }
                }
            )
        )
    }

    fn btn_new_game(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("NewGame screen.");
        Trans::Push(
            Box::new(
                {
                    if let Some(ref handle) = self.ui_screens.get(&MenuScreens::NewGame){
                        NewGameState::new(world, Some({*handle}.clone()))
                    }
                    else{
                        NewGameState::new(world, None)
                    }
                }
            )
        )
    }

    fn btn_load(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("LoadGame screen.");
        Trans::Push(
            Box::new(
                {
                    if let Some(ref handle) = self.ui_screens.get(&MenuScreens::LoadGame){
                        LoadMenuState::new(world, Some({*handle}.clone()))
                    }
                    else{
                        LoadMenuState::new(world, None)
                    }
                }
            )
        )
    }

    fn btn_options(&self, world: &mut World) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Options screen.");
        Trans::Push(
            Box::new(
                {
                    if let Some(ref handle) = self.ui_screens.get(&MenuScreens::Options){
                        OptionsState::new(world, Some({*handle}.clone()))
                    }
                    else{
                        OptionsState::new(world, None)
                    }
                }
            )
        )
    }
}
