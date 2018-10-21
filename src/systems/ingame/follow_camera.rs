//! Makes the camera follow the player.

use amethyst::{
    ecs::{System, WriteStorage, ReadStorage, Join,},
    core::transform::components::Transform,
    renderer::Camera,
};

use components::for_characters::TagPlayer;

/// Makes the camera follow the player.
/// TODO: Multiplayer support (PlayerID and Camera ID?)
pub struct FollowCameraSystem;

impl<'s> System<'s> for FollowCameraSystem{
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TagPlayer>,
        ReadStorage<'s, Camera>,
    );

    fn run(&mut self, (mut transforms, players, camera) : Self::SystemData){
        let mut transform_camera = Transform::default();
        if let Some((transform, player)) = (&transforms, &players).join().next() {
            // TODO: Dont use camera-offset in Player, instead make it a component of the Camera entity?
            transform_camera.translation[0] = transform.translation[0] - player.camera_offset.0;
            transform_camera.translation[1] = transform.translation[1] - player.camera_offset.1;
        }

        if let Some((transform, _)) = (&mut transforms, &camera).join().next() {
            transform.translation[0] = transform_camera.translation[0];
            transform.translation[1] = transform_camera.translation[1];
        }
    }
}