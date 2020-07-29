use super::components::*;
use ggez::nalgebra::Vector2;
use ggez::Context;
use specs::{ReadStorage, System};

pub struct CameraSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for CameraSystem<'a> {
    type SystemData = (ReadStorage<'a, Camera>, ReadStorage<'a, Position>);
    fn run(&mut self, data: Self::SystemData) {
        let (camera, position) = data;
    }
}
