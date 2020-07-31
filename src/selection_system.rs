use super::components::*;
use specs::join::Join;
use specs::{ReadStorage, System, Write, WriteStorage};

pub struct SelectionSystem {}

impl<'a> System<'a> for SelectionSystem {
    type SystemData = (
        Write<'a, InputQueue>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Selectable>,
    );
    // type SystemData = Entities<'a>;

    fn run(&mut self, data: Self::SystemData) {
        let (mut input_queue, positions, mut selectables) = data;

        if let Some(command) = input_queue.selection_command {
            println!("{:?}", command);
            for (position, selectable) in (&positions, &mut selectables).join() {
                // if selectable is inside the selection
                selectable.is_selected = true;
            }
            input_queue.selection_command = None;
        }
    }
}
