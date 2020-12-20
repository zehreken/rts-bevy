use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Selectable {
    pub is_selected: bool,
}
