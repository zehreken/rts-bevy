use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
// #[storage(NullStorage)] // Learn about this
pub struct Actor {}
