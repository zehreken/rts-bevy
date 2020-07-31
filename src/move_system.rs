use super::components::*;
use specs::join::Join;
use specs::{ReadStorage, System, Write, WriteStorage};

pub struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Selectable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions, selectables) = data;

        if let Some(move_command) = input_queue.move_commands.pop() {
            println!("{:?}", move_command);
            for (position, selectable) in (&mut positions, &selectables).join() {
                if selectable.is_selected {
                    let scale = 4.0;
                    *position = Position {
                        x: move_command.x / super::TILE_WIDTH / scale,
                        y: move_command.y / super::TILE_HEIGHT / scale,
                        z: 11.0,
                    };
                }
            }
        }
    }
}
