use amethyst::{
    core::{ArcThreadPool, SystemBundle},
    ecs::prelude::*,
    DataInit, Error, Result,
};

/// Contains the `core` Dispatcher the game needs to run at all,
/// responsible for rendering and input gathering.
/// Furthermore holds the `ingame-core` and `main_menu_core` for the base functionality of each major state,
/// and a `credits` dispatcher.
pub struct ToppaGameData<'a, 'b> {
    /// `Core` dispatcher responsible for input gathering, rendering and audio.
    pub core: Dispatcher<'a, 'b>,
    /// Dispatches the base functionalities of a running game session.
    /// This includes for example total play time tracking and background music.
    pub ingame_core: Dispatcher<'a, 'b>,
    /// Dispatches the base functionalities of the main menu.
    /// This includes for example the menu theme.
    pub main_menu_core: Dispatcher<'a, 'b>,
}

impl<'a, 'b> ToppaGameData<'a, 'b> {
    /// Updates the `ingame_core` dispatchers, which might i.e. include
    /// - `TotalPlaytimeSystem`,
    /// - `PlayerChatSystem`,
    /// - ...
    pub fn update_ingame(&mut self, world: &World) {
        self.ingame_core.dispatch(&world.res);
        self.core.dispatch(&world.res);
    }

    /// Updates the `main_menu_core` dispatchers, which might i.e. include
    /// - `BackgroundFogAnimationSystem` (like in Skyrim's menu)
    /// - `LazyAssetLoadSystem` (to preload assets needed by any game session anyways, like the tile spritesheets)
    /// - ...
    pub fn update_menu(&mut self, world: &World) {
        self.main_menu_core.dispatch(&world.res);
        self.core.dispatch(&world.res);
    }
}

/// Responsible for building the dispatchers in the [ToppaGameData](struct.ToppaGameData.html).
/// Allows adding systems and bundles to each dispatcher individually.
pub struct ToppaGameDataBuilder<'a, 'b> {
    core: DispatcherBuilder<'a, 'b>,
    ingame_core: DispatcherBuilder<'a, 'b>,
    main_menu_core: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for ToppaGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        ToppaGameDataBuilder::new()
    }
}

impl<'a, 'b> ToppaGameDataBuilder<'a, 'b> {
    /// Creates a new game data builder with a [DispatcherBuilder](https://www.amethyst.rs/doc/master/doc/amethyst/ecs/prelude/struct.DispatcherBuilder.html) for each dispatcher.
    /// The Link might be outdated, as it is not locked to the commit used by this game, but refers to the development documentation of amethyst.
    pub fn new() -> Self {
        ToppaGameDataBuilder {
            core: DispatcherBuilder::new(),
            ingame_core: DispatcherBuilder::new(),
            main_menu_core: DispatcherBuilder::new(),
        }
    }
}

impl<'a, 'b> ToppaGameDataBuilder<'a, 'b> {
    /// Add a system to the `core` dispatcher.
    pub fn with_core_system<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.core.add(system, name, dependencies);
        self
    }
    /// Add a system to the `ingame_core` dispatcher.
    pub fn with_ingame_core_sytem<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.ingame_core.add(system, name, dependencies);
        self
    }
    /// Add a system to the `main_menu_core` dispatcher.
    pub fn with_menu_core_system<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.main_menu_core.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> ToppaGameDataBuilder<'a, 'b> {
    /// Add a system bundle to the `core` dispatcher.
    pub fn with_core_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.core)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }
    /// Add a system bundle to the `ingame_core` dispatcher.
    pub fn with_ingame_core_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.ingame_core)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }
    /// Add a system bundle to the `main_menu_core` dispatcher.
    pub fn with_menu_core_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        bundle
            .build(&mut self.main_menu_core)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }
}

impl<'a, 'b> DataInit<ToppaGameData<'a, 'b>> for ToppaGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> ToppaGameData<'a, 'b> {
        #[cfg(not(no_threading))]
        let pool = world.read_resource::<ArcThreadPool>().clone();

        #[cfg(not(no_threading))]
        let mut core = self.core.with_pool(pool.clone()).build();
        #[cfg(no_threading)]
        let mut core = self.core.build();
        core.setup(&mut world.res);

        #[cfg(not(no_threading))]
        let mut ingame_core = self.ingame_core.with_pool(pool.clone()).build();
        #[cfg(no_threading)]
        let mut ingame_core = self.ingame_core.build();
        ingame_core.setup(&mut world.res);

        #[cfg(not(no_threading))]
        let mut main_menu_core = self.main_menu_core.with_pool(pool.clone()).build();
        #[cfg(no_threading)]
        let mut main_menu_core = self.main_menu_core.build();
        main_menu_core.setup(&mut world.res);

        ToppaGameData {
            core,
            ingame_core,
            main_menu_core,
        }
    }
}
