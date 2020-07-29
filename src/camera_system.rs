use super::components::*;
use ggez::nalgebra::Vector2;
use ggez::Context;
use specs::{ReadStorage, System};

pub struct CameraSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for CameraSystem<'a> {
    type SystemData = (ReadStorage<'a, Camera>);
    fn run(&mut self, data: Self::SystemData) {
        let cameras = data;
    }
}
