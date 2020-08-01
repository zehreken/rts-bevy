use super::components::*;
use specs::join::Join;
use specs::{Entities, ReadStorage, System, Write, WriteStorage};

pub struct MoveCommandSystem {}

impl<'a> System<'a> for MoveCommandSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, MoveCommand>,
        ReadStorage<'a, Selectable>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, mut move_commands, selectables, entities) = data;

        if let Some(move_command) = input_queue.move_commands.pop() {
            println!("{:?}", move_command);
            for (selectable, ent) in (&selectables, &*entities).join() {
                if selectable.is_selected {
                    let scale = 4.0;
                    // *position = Position {
                    //     x: move_command.x / super::TILE_WIDTH / scale,
                    //     y: move_command.y / super::TILE_HEIGHT / scale,
                    //     z: 11.0,
                    // };
                    move_commands
                        .insert(
                            ent,
                            MoveCommand {
                                x: move_command.x / super::TILE_WIDTH / scale,
                                y: move_command.y / super::TILE_HEIGHT / scale,
                            },
                        )
                        .unwrap();
                }
            }
        }
    }
}
