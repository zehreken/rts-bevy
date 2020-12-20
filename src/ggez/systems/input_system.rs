use crate::components::*;
use crate::setup::*;
use ggez::event::KeyCode;
use specs::join::Join;
use specs::{System, Write, WriteStorage};

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (Write<'a, InputQueue>, WriteStorage<'a, Camera>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut cameras) = data;

        for camera in (&mut cameras).join() {
            if let Some(key) = input_queue.keys_pressed.pop() {
                match key {
                    KeyCode::W => camera.y -= 1.0,
                    KeyCode::S => camera.y += 1.0,
                    KeyCode::A => camera.x -= 1.0,
                    KeyCode::D => camera.x += 1.0,
                    _ => (),
                }
            }
        }
    }
}
