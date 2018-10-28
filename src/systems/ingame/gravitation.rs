use amethyst::{
    ecs::{
        Join, ReadStorage, WriteStorage, System,
    },
    core::{
        transform::components::Transform,
        cgmath::Vector2,
    },
};

use {
    components::physics::{
        PhysicalProperties,
        Dynamics,
    },
    GRAVITATION,
};

#[derive(Default)]
pub struct GravitationSystem;

impl<'s> System<'s> for GravitationSystem{
    type SystemData = (
        WriteStorage<'s, Dynamics>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, PhysicalProperties>,
    );

    fn run(
        &mut self, 
        (mut dynamics, transforms, masses): Self::SystemData,
    ){
        // TODO: Get rotational diff local - world rotation.

        // Transform currently unused, later used for rotational diff against world
        for (mut dynamic, transform, mass) 
        in (&mut dynamics, &transforms, &masses).join(){
            let grav_force2 = (mass.mass * -GRAVITATION * 20.0) 
                * Vector2::new(
                    transform.orientation().up[0] as f64,
                    transform.orientation().up[1] as f64,
                );//* transform.up(); // (magnitude) * direction

            dynamic.force = Vector2::new(0.0, 0.0);//Vector2::new(grav_force2[0],grav_force2[1]); //
            dynamic.torque = 0.0; //always reset
        }
    }
}