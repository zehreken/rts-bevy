use specs::{Component, VecStorage};

#[derive(Component, Debug, Default, Clone, Copy)]
#[storage(VecStorage)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
