use super::components::*;
use specs::join::Join;
use specs::{System, Write, WriteStorage};

pub struct SelectionSystem {}

impl<'a> System<'a> for SelectionSystem {
    type SystemData = (Write<'a, InputQueue>, WriteStorage<'a, Position>);
    // type SystemData = Entities<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, mut positions) = data;

        if let Some(command) = input_queue.selection_command {
            println!("{:?}", command);
            // for position in positions.join() {

            // }
            input_queue.selection_command = None;
        }
    }
}
