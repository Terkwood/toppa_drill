use amethyst::{
    prelude::*,
    input::{is_close_requested, is_key_down},
    renderer::{
        VirtualKeyCode,
    },
    assets::{
        ProgressCounter,
    }
};

use entities::{
    player
};

use ToppaGameData;

/// The default state after opening Toppa Drill.
/// It should display a short amethyst logo, and then transist over to the Main Menu State.
pub struct StartupState{
    progress_counter: ProgressCounter,
}

impl StartupState{
    /// Creates a new StartupState instance with a progress counter for asset-load tracking.
    pub fn new() -> Self{
        let ps = ProgressCounter::new();
        info!("ProgressCounter created.");

        StartupState {
            progress_counter: ps,
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

    fn update(&mut self, data: StateData<ToppaGameData>)
    -> Trans<ToppaGameData<'a, 'b>, ()>{
        let StateData {world, data} = data;

        data.update_menu(&world);

        Trans::None
    }

    fn on_start(&mut self, data: StateData<ToppaGameData>) {        
        let StateData {world, data} = data;

        player::init(world, &mut self.progress_counter);
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