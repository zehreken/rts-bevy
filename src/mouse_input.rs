use super::movement::MoveCommand;
use super::Actor;
use bevy::{
    input::{mouse::*, ElementState},
    prelude::*,
    window::CursorMoved,
};

pub struct MouseInputPlugin;

fn mouse_input_system(
    mut commands: Commands,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut query: Query<(Entity, &Actor)>,
) {
    for (button_event, move_event) in mouse_button_input_events
        .iter()
        .zip(cursor_moved_events.iter())
    {
        match (button_event.button, button_event.state) {
            (MouseButton::Left, ElementState::Pressed) => {}
            (MouseButton::Left, ElementState::Released) => {
                for (entity, _) in query.iter_mut() {
                    commands.entity(entity).insert(MoveCommand {
                        position: Vec3::new(move_event.position.x, move_event.position.y, 0.0),
                    });
                }
            }
            (MouseButton::Right, ElementState::Pressed) => {}
            (MouseButton::Right, ElementState::Released) => {}
            _ => {}
        }
    }
}

impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(mouse_input_system);
    }
}
