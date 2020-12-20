use super::components::*;
use ggez::event::KeyCode;
use ggez::mint::Point2;
use specs::{World, WorldExt};

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
    pub selection_command: Option<(Point2<f32>, Point2<f32>)>,
    pub move_commands: Vec<Point2<f32>>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

pub fn register_components(world: &mut World) {
    world.register::<Camera>();
    world.register::<Position>();
    world.register::<MoveCommand>();
    world.register::<Collider>();
    world.register::<SeparationCommand>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Actor>();
    world.register::<Selectable>();
}
