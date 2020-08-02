use specs::{Component, VecStorage};

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

#[derive(Component)]
#[storage(VecStorage)]
pub struct SeparationCommand {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Collider {
    pub radius: f32,
}
