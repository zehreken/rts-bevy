use super::components::*;
use ggez::nalgebra::Vector2;
use specs::join::Join;
use specs::{Entities, System, WriteStorage};

pub struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        WriteStorage<'a, MoveCommand>,
        WriteStorage<'a, Position>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut move_commands, mut positions, entities) = data;
        let mut to_remove = vec![];
        for (move_command, position, ent) in (&mut move_commands, &mut positions, &*entities).join()
        {
            let direction: Vector2<f32> =
                Vector2::new(move_command.x - position.x, move_command.y - position.y);
            let speed = 0.3;
            let normalized = direction.normalize();
            position.x += speed * normalized.x;
            position.y += speed * normalized.y;

            println!("There must be one entity {:?}", direction.magnitude());
            if direction.magnitude() < 1.0 {
                // println!("there");
                to_remove.push(ent);
            }
        }

        for ent in to_remove {
            move_commands.remove(ent);
        }
    }
}
