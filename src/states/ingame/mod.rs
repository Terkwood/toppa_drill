/*mod base_state;
pub use self::{
    base_state::IngameBaseState,
}*/

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum IngameScreens{
    HUD,
    Inventory,
    OreShop,
    PartsShop,
    FuelStation,
    Death,
    Exit,
    Options,
}