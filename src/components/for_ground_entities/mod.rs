/// An enumaration of all ground tile types.
#[allow(dead_code)]
#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Hash, Debug)]
pub enum TileTypes {
    /// Nothing drilleable here.
    Empty,
    /// Some dirt blocking your vision, worthless.
    Dirt,
    /// Bed rock, indestructible.
    BedRock,

    /// Plain old rock. Worthless but hard,
    /// and can fall on your head.
    Rock,
    /// Gassy rock, explodes when it gets ignited, careful with your dynamite.
    Gas,
    /// Hot rock, ouch.
    Lava,

    /// A treasure, can be sold for a premium.
    TreasureChest,
    /// An artifact, maybe some museum is interested?
    Skeleton,
    /// An artifact, there should be a museum interested in this.
    Fossile,
    /// An artifact, every museum wants this!
    MeteoriteShard,

    /// `Ag.2 S` for production of silver
    Acanthite,
    /// `Ba S O.4` for production of barium
    Barite,
    /// `Al (O H.3) + Al O O H` for production of aluminium
    Bauxite,
    /// `Cu.5 Fe S.4` for production of copper
    Bornite,
    /// `Sn O.2` for production of tin
    Cassiterite,
    /// `Cu.2 S` for production of copper
    Chalcocite,
    /// `(Fe, Mg) Cr.2 O.4` for production of chrome
    Chromite,
    /// `Hg S` for production of mercury
    Cinnabar,
    /// `Pb S` for production of lead
    Galena,
    /// `Au`, native gold
    Gold,
    /// `Fe.2 O.3` for production of iron
    Hematite,
    /// `Fe.3 O.4` for production of iron
    Magnetite,
    /// `Mo S.2` for production of molybdenum
    Molybdenite,
    /// `Mn O.2` for production of manganese
    Pyrolusite,
    /// `Pt As.2` for production of platinum
    Sperrylite,
    /// `Zn S` for production of zinc
    Sphalerite,
}
