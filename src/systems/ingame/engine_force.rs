use amethyst::{
    core::{
        timing::Time,
        transform::components::Transform,
        cgmath::{
            Vector2, prelude::*, dot,
        },
    },
    input::InputHandler,
    ecs::{
        Join, Read, ReadStorage, WriteStorage, System
    },
};

use {
    components::{
        for_characters::{
            FuelTank, Engine,
        },
        physics::{
            Dynamics,
            PhysicalProperties,
        },
    },
};

#[derive(Default)]
pub struct EngineForceSystem;

impl<'s> System<'s> for EngineForceSystem{
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Dynamics>,
        WriteStorage<'s, FuelTank>,
        ReadStorage<'s, Engine>,
        Read<'s, Time>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(
        &mut self, 
        (
            transforms, mut dynamics, mut fuel_tanks, engines,
            time, input
        ) : Self::SystemData,
    ){
        let dt = time.delta_seconds();

        for (
            transform, mut dynamic, mut tank, engine,
        )
        in (
            &transforms, &mut dynamics,&mut fuel_tanks, &engines,
        ).join(){
            // Input gathering ( !! not multiplayer friendly, add playerID's, e.g. in ship_base !! )
            let mut engine_scaling = Vector2::new(0.0, 0.0);
            {
                let engine_scaling_x = input.axis_value("right");
                let engine_scaling_y = input.axis_value("up");
                
                if let Some(engine_scaling_x_temp) = engine_scaling_x {
                    engine_scaling[0] = engine_scaling_x_temp as f64;
                };
                if let Some(engine_scaling_y_temp) = engine_scaling_y {
                    engine_scaling[1] = engine_scaling_y_temp as f64;
                };
            }

            let mut engine_force_vec = engine.max_force.mul_element_wise(engine_scaling);
            let engine_force_attempt = engine_force_vec.magnitude();
            {
                let fuel_consumption = (engine_force_attempt * engine.consumption * dt as f64) / engine.efficiency;
                if fuel_consumption > tank.fuel_level {
                    // Provide as much force as the fuel allows and set the tank empty.
                    let engine_force_actual = engine.efficiency * tank.fuel_level / (engine.consumption * dt as f64);
                    tank.fuel_level = 0.0;

                    let scaling = engine_force_actual / engine_force_attempt;
                    engine_force_vec *= scaling;
                }
                else{
                    // If enough fuel is present, only reduce fuel level.
                    tank.fuel_level -= fuel_consumption;
                }
                //#[cfg(feature = "debug")]
                //warn!("Fuel level: {}", tank.fuel_level);
            }
            // Add engine force (player input) to *natural*/physical forces, e.g. gravitational force.
            let world_force_x = dot(engine_force_vec, Vector2::new(transform.orientation().right[0] as f64, transform.orientation().right[1] as f64));
            let world_force_y = dot(engine_force_vec, Vector2::new(transform.orientation().up[0] as f64, transform.orientation().up[1] as f64));

            let world_force_vec = Vector2::new(world_force_x, world_force_y);

            dynamic.force += world_force_vec;
        }
    }
}