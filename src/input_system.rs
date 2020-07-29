use super::components::*;
use ggez::event::KeyCode;
use specs::{ReadStorage, System, Write, WriteStorage};

use specs::join::Join;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Camera>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, cameras) = data;

        for (position, _camera) in (&mut positions, &cameras).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                println!("{:?}", key);
                match key {
                    KeyCode::Up => position.y -= 1.0,
                    KeyCode::Down => position.y += 1.0,
                    KeyCode::Left => position.x -= 1.0,
                    KeyCode::Right => position.x += 1.0,
                    _ => (),
                }
            }
            println!("camera position: {:?}", position);
        }
    }
}
