use ggez::event::KeyCode;
use ggez::mint::Point2;
use specs::{Builder, Component, VecStorage, World, WorldExt};

#[derive(Debug, Component, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Renderable {
    pub id: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
// #[storage(NullStorage)] learn this
pub struct Actor {}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Selectable {
    pub is_selected: bool,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct MoveCommand {
    pub x: f32,
    pub y: f32,
}

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
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Actor>();
    world.register::<Selectable>();
}

pub fn create_camera(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Camera {
            x: position.x,
            y: position.y,
            z: position.z,
        })
        .build();
}

pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {
            z: 10.0,
            ..position
        })
        .with(Renderable { id: 1 })
        .with(Wall {})
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5.0, ..position })
        .with(Renderable { id: 15 })
        .build();
}

pub fn create_tent(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {
            z: 10.0,
            ..position
        })
        .with(Renderable { id: 105 })
        .build();
}

pub fn create_tree(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {
            z: 10.0,
            ..position
        })
        .with(Renderable { id: 75 })
        .build();
}

pub fn create_actor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {
            z: 10.0,
            ..position
        })
        .with(Renderable { id: 4 })
        .with(Actor {})
        .with(Selectable { is_selected: false })
        .build();
}
