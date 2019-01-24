use amethyst::{
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    core::Transform,
    input::InputHandler
};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = movement(&paddle.side, &input)
                .filter(|mv_amount| *mv_amount != 0.0);

            if let Some(amount) = movement {
                let y = transform.translation().y;
                transform.set_y(
                    (y + amount as f32)
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        .max(PADDLE_HEIGHT * 0.5));
            }
        }
    }
}

fn movement(side: &Side, input: &Read<'_, InputHandler<String, String>>) -> Option<f64> {
    match side {
        Side::Left => input.axis_value("left_paddle"),
        Side::Right => input.axis_value("right_paddle"),
    }
}