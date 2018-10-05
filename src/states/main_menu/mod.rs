mod centre;
mod options;

pub use self::{
    centre::CentreState,
    options::OptionsState,
};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum MenuScreens{
    NewGame,
    Load,
    Options,
    Credits,
    Centre,
}