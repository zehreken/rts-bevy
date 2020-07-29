use ggez::event::KeyCode;
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
pub struct Player {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {}

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
}

pub fn register_components(world: &mut World) {
    world.register::<Camera>();
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Wall>();
    world.register::<Player>();
    world.register::<Box>();
    world.register::<BoxSpot>();
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

pub fn create_box(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {
            z: 10.0,
            ..position
        })
        .with(Renderable { id: 105 })
        .with(Box {})
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position {
            z: 10.0,
            ..position
        })
        .with(Renderable { id: 4 })
        .with(Player {})
        .build();
}
