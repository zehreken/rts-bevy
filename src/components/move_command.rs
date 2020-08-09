use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct MoveCommand {
    pub x: f32,
    pub y: f32,
}
