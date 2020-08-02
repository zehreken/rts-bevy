use super::components::*;
use specs::join::Join;
use specs::{Entities, ReadStorage, System, Write, WriteStorage};
use super::setup::*;

pub struct MoveCommandSystem {}

impl<'a> System<'a> for MoveCommandSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, MoveCommand>,
        ReadStorage<'a, Selectable>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut move_commands, selectables, entities) = data;

        if let Some(move_command) = input_queue.move_commands.pop() {
            println!("{:?}", move_command);
            for (selectable, ent) in (&selectables, &*entities).join() {
                if selectable.is_selected {
                    let _scale = 1.0;
                    move_commands
                        .insert(
                            ent,
                            MoveCommand {
                                x: move_command.x,
                                y: move_command.y,
                            },
                        )
                        .unwrap();
                }
            }
        }
    }
}
