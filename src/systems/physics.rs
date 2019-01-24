use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage, Write},
    core::Transform,
    input::InputHandler
};
use nphysics2d::world::World;

use crate::pong::{Paddle, Side, RigidBody2D, ARENA_HEIGHT, PADDLE_HEIGHT};

pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, RigidBody2D>,
        Write<'s, World<f32>>
    );

    fn run(&mut self, (mut transforms, rigidBodies, physics_world): Self::SystemData) {
        for (transform, rigidBody) in (&mut transforms, &rigidBodies).join() {
            if let Some(body) = physics_world.rigid_body(rigidBody.handle) {
                let pos = body.position().translation.vector;
//                println!("{:?}", pos);
                transform.set_xyz(pos.x, pos.y, 0.0);
            }
        }
    }
}
