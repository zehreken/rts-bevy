use super::components::*;
use ggez::event::KeyCode;
use specs::{System, Write, WriteStorage};

use specs::join::Join;

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (Write<'a, InputQueue>, WriteStorage<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {
        // let (mut input_queue,&mutcameras) = data;
        let (mut input_queue, mut cameras) = data;

        for camera in (&mut cameras).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                match key {
                    KeyCode::Up => camera.y -= 1.0,
                    KeyCode::Down => camera.y += 1.0,
                    KeyCode::Left => camera.x -= 1.0,
                    KeyCode::Right => camera.x += 1.0,
                    _ => (),
                }
            }
        }
    }
}
