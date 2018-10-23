mod base_state;
mod inventory;
pub use self::base_state::IngameBaseState;

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum IngameScreens {
    HUD,
    Inventory,
    OreShop,
    PartsShop,
    FuelStation,
    Death,
    Exit,
    Options,
}
