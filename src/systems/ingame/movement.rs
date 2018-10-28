use amethyst::{
    core::{
        timing::Time,
        transform::components::Transform,
    },
    ecs::{
        Join, Read, ReadStorage, WriteStorage, System,
    },
    core::cgmath::{
        Rad, Angle, Vector3,
    },
};

use {
    components::physics::{
        Dynamics,
        PhysicalProperties,
    },
};

#[derive(Default)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem{
    type SystemData = ( 
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Dynamics>,
        ReadStorage<'s, PhysicalProperties>,

        Read<'s, Time>,
    );

    fn run(
        &mut self, 
        (
            mut transforms, mut dynamics, physical_properties, time
        ): Self::SystemData,
    ){
        let dt = time.delta_seconds();

        // TODO: Collision physics here, or in a seperate (afterwards) system?

        // potentials unused, since currently nothing is bound to a spring or something like that
        for (mut transform, mut dynamic, physical_property) 
        in (&mut transforms, &mut dynamics, &physical_properties).join(){
            // TODO: handle friction and potential forces better (seperately?). Has ground contact, is mid-air, what materials (collision) are involved?
            // TODO: Should a ship/rock even have friction component, or is that part of a material, or a material-tuple?
            // TODO: Does a ship/rock have a potential component, or is that its own entity, or a entity-transform-tuple?

            // Current values
            let pos_cur = transform.translation;
            let vel_cur = dynamic.vel;

            // Calculating acceleration based on applied Force, 
            // no potential part, since there is currently no spring attached to any entity making it `= 0`.
            let accel = (dynamic.force - physical_property.fric * dynamic.vel) / physical_property.mass;

            dynamic.vel = vel_cur + accel * dt.into();

            transform.translation = Vector3::new(
                0.5 * accel[0] as f32 * dt * dt + dynamic.vel[0] as f32 * dt,
                0.5 * accel[1] as f32 * dt * dt + dynamic.vel[1] as f32 * dt,
                0.0
            ) + pos_cur;
            
            // Updating rotation
            /*
            let rot_cur = position.rad;
            let omega_cur = dynamic.omega;
            let delta_omega = Rad(dynamic.torque / inertia.value);
            dynamic.omega = omega_cur + delta_omega * dt;
            position.rad = (delta_omega * dt * dt * 0.5 + dynamic.omega * dt + rot_cur).normalize();*/
        };
    }
}