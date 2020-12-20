use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Collider {
    pub radius: f32,
}
