use super::components::*;
use specs::{Builder, World, WorldExt};

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
        .with(Collider { radius: 20.0 })
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
        .with(Collider { radius: 4.0 })
        .build();
}
