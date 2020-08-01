use super::components::*;
use specs::join::Join;
use specs::{ReadStorage, System, WriteStorage};

pub struct CollisionSystem {}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (ReadStorage<'a, Actor>, ReadStorage<'a, Collider>);

    fn run(&mut self, data: Self::SystemData) {
        let (actors, colliders) = data;

        for (actor, collider) in (&actors, &colliders).join() {}
    }
}
