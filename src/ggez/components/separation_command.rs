use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct SeparationCommand {
    pub x: f32,
    pub y: f32,
}
