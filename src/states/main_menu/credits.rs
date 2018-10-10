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
        //ProgressCounter, Completion, 
        Handle,
    },
    ui::{
        //UiCreator, UiLoader, 
        UiPrefab, UiFinder, UiEventType,
    },
    core::{
        timing::Time,
        //ArcThreadPool,
        //SystemBundle,
    },
};
use {
    ToppaGameData,
    systems::DummySystem,
    super::super::ToppaState,
};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum CreditsButtons{
    Back,
}

/// The credit menu stage, displaying the credits.
pub struct CreditsState<'d, 'e>{
    menu_duration: f32,
    dispatcher: Option<Dispatcher<'d, 'e>>,
    //progress_counter: ProgressCounter,

    // The displayed Ui Entity, if any.
    current_screen: Option<Entity>,
    // The Handle of the Prefab for the displayed Ui Entity.
    ui_screen: Option<Handle<UiPrefab>>,
    // Map of the Ui Button entities and the corresponding button type.
    ui_buttons: HashMap<Entity, CreditsButtons>,
    b_buttons_found: bool,
}

impl<'d, 'e> ToppaState for CreditsState<'d, 'e>{
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
            let _ = world.delete_entity(entity);
        };
    }

    fn enable_current_screen(&mut self, world: &mut World){
        if let Some(ref prefab_handle) = self.ui_screen{
            self.current_screen = Some(
                world.create_entity()
                    .with(prefab_handle.clone())
                    .build()
            );
        };
    }

    fn new(_world: &mut World, screen_opt: Option<Handle<UiPrefab>>) -> Self{
        CreditsState{
            menu_duration: 0.0,
            current_screen: None,
            ui_screen: screen_opt,
            //progress_counter: ProgressCounter::new(),
            ui_buttons: HashMap::new(),
            b_buttons_found: false,
            dispatcher: None,
        }
    }
}

impl<'a, 'b, 'd, 'e> State<ToppaGameData<'a, 'b>, ()> for CreditsState<'d, 'e>{
    fn handle_event(&mut self, data: StateData<ToppaGameData>, event: StateEvent<()>) 
    -> Trans<ToppaGameData<'a, 'b>, ()>{
        let StateData {mut world, data: _} = data;
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
        let StateData {mut world, data} = data;
        self.dispatch(&world);
        data.update_menu(&world);
        self.menu_duration += world.read_resource::<Time>().delta_seconds();

        if !self.b_buttons_found{
            self.insert_button(&mut world, CreditsButtons::Back, "back_button");
            Trans::None
        }
        else{
            Trans::None
        }
    }

    // Executed when this game state runs for the first time.
    fn on_start(&mut self, data: StateData<ToppaGameData>) {        
        let StateData {mut world, data: _} = data;
        self.enable_current_screen(&mut world);
        self.enable_dispatcher();
    }

    // Executed when this game state gets popped.
    fn on_stop(&mut self, data: StateData<ToppaGameData>) {
        let StateData {mut world, data: _} = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when another game state is pushed onto the stack.
    fn on_pause(&mut self, data: StateData<ToppaGameData>) {
        let StateData {mut world, data: _} = data;
        self.disable_dispatcher();
        self.disable_current_screen(&mut world);
    }

    // Executed when the application returns to this game state, 
    // after another gamestate was popped from the stack.
    fn on_resume(&mut self, data: StateData<ToppaGameData>) {
        let StateData {mut world, data: _} = data;
        self.enable_dispatcher();
        self.enable_current_screen(&mut world);
    }
}

impl<'a, 'b, 'd, 'e> CreditsState<'d, 'e>{
    fn insert_button(&mut self, world: &mut World, button: CreditsButtons, button_name: &str){
        world.exec(|finder: UiFinder| {
            if let Some(entity) = finder.find(button_name){
                info!("Found {}.", button_name);
                self.ui_buttons.insert(
                    entity,
                    button
                );
                self.b_buttons_found = true;
            }
            else{
                warn!("Couldn't find {}!", button_name);
            }
        });
    }

    fn btn_click(&self, _world: &mut World, target: Entity) -> Trans<ToppaGameData<'a, 'b>, ()>{
        use self::CreditsButtons::*;
        if let Some(button) = self.ui_buttons.get(&target){
            match button{
                Back => self.btn_back(),
            }
        }
        else{
            Trans::None
        }
    }

    fn btn_back(&self) -> Trans<ToppaGameData<'a, 'b>, ()>{
        info!("Returning to CentreState.");
        Trans::Pop
    }
}