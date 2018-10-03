use amethyst::{
    ecs::prelude::*,
    core::{
        ArcThreadPool,
        SystemBundle,
    },
    Error,
    DataInit,
    Result,
};

pub trait ToppaState<'a, 'b>{
    fn dispatch(&mut self, world: &World);

    fn new() -> ToppaStateBuilder<'a, 'b, Self>
    where
        Self: std::marker::Sized;

    fn build(&mut self, dispatcher: Option<Dispatcher<'a, 'b>>);
}

pub struct ToppaStateBuilder<'a, 'b, T>
where
        T: ToppaState<'a, 'b> + 'static,
{
    state: T,
    dispatcher_builder: DispatcherBuilder<'a, 'b>,
    b_any_systems_added: bool,
}

impl<'a, 'b, T> ToppaStateBuilder<'a, 'b, T>
where
        T: ToppaState<'a, 'b> + 'static,
{
    pub fn new(toppa_state: T) -> Self
    {
        ToppaStateBuilder{
            state: toppa_state,
            dispatcher_builder: DispatcherBuilder::new(),
            b_any_systems_added: false,
        }
    }

    pub fn with_system<S>(
        mut self,
        system: S,
        name: &str,
        dependencies: &[&str],
    ) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.b_any_systems_added = true;
        self.dispatcher_builder.add(system, name, dependencies);
        self
    }

    pub fn with_bundle<B>(mut self, bundle: B) -> Result<Self>
    where
        B: SystemBundle<'a, 'b>,
    {
        self.b_any_systems_added = true;
        bundle
            .build(&mut self.dispatcher_builder)
            .map_err(|err| Error::Core(err))?;
        Ok(self)
    }

    pub fn build(mut self, world: &mut World) -> T{
         #[cfg(not(no_threading))]
        let pool = world.read_resource::<ArcThreadPool>().clone();

        #[cfg(not(no_threading))]
        let mut dispatcher = self.dispatcher_builder.with_pool(pool.clone()).build();
        #[cfg(no_threading)]
        let mut dispatcher = self.dispatcher_builder.build();
        dispatcher.setup(&mut world.res);

        if self.b_any_systems_added{
            self.state.build(Some(dispatcher));
        }
        else{
            self.state.build(None);
        }

        self.state
    }
}
