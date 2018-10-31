use amethyst::{
    core::{
        cgmath::{Vector2, Vector3},
        timing::Time,
        transform::components::Transform,
    },
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use components::physics::{Dynamics, PhysicalProperties};

/// TODO: Calculate inertia based on ShipParts' masses and distances
/// --: Combine air-resistance/friction of individual parts
/// --: Rotation, rotation-based horizontal/vertical forces.
/// --: Collision physics here, or in a seperate (afterwards) system?
/// --: handle friction and better (seperately?). Has ground contact, is mid-air, what materials (collision) are involved?
/// --: Should a ship/rock even have friction component, or is that part of a material, or a material-tuple?
/// --: Does a ship/rock have a potential component, or is that its own entity, or a entity-transform-tuple?
#[derive(Default)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Dynamics>,
        ReadStorage<'s, PhysicalProperties>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut dynamics, physical_properties, time): Self::SystemData) {
        let dt = time.delta_seconds();

        for (mut transform, mut dynamic, physical_property) in
            (&mut transforms, &mut dynamics, &physical_properties).join()
        {
            // Current values
            let pos_cur = transform.translation;
            let vel_cur = dynamic.vel;

            // Calculating acceleration based on applied Force,
            // no potential part, since there is currently no spring attached to any entity making it `= 0`.
            let mut accel = Vector2::new(0.0, 0.0);
            match (physical_property.friction, physical_property.air_resistance) {
                (Some(friction), Some(air_resistance)) => {
                    accel = (dynamic.force - 0.5 * (air_resistance + friction) * dynamic.vel)
                        / physical_property.mass;
                }
                (Some(friction), None) => {
                    accel = (dynamic.force - friction * dynamic.vel) / physical_property.mass;
                }
                (None, Some(air_resistance)) => {
                    accel = (dynamic.force - air_resistance * dynamic.vel) / physical_property.mass;
                }
                (None, None) => {
                    /*No acceleration if no dampening mechanism is in place. Otherwise vel of infinity is possible.*/
                }
            }

            dynamic.vel = vel_cur + accel * dt.into();

            transform.translation = Vector3::new(
                0.5 * accel[0] as f32 * dt * dt + dynamic.vel[0] as f32 * dt,
                0.5 * accel[1] as f32 * dt * dt + dynamic.vel[1] as f32 * dt,
                0.0,
            ) + pos_cur;

            // Updating rotation
            /*
            let rot_cur = position.rad;
            let omega_cur = dynamic.omega;
            let delta_omega = Rad(dynamic.torque / inertia.value);
            dynamic.omega = omega_cur + delta_omega * dt;
            position.rad = (delta_omega * dt * dt * 0.5 + dynamic.omega * dt + rot_cur).normalize();*/        }
    }
}
