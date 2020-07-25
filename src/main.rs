use components::*;
use ggez::{conf, event, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;
use systems::*;

mod components;
mod systems;

pub const TILE_WIDTH: f32 = 32.0;
pub const TILE_HEIGHT: f32 = 32.0;

struct Game {
    world: World,
}

impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world);
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);
    initialize_level(&mut world);

    let context_builder = ggez::ContextBuilder::new("rts", "zehreken")
        .window_setup(conf::WindowSetup::default().title("rts"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = &mut context_builder.build()?;
    let game = &mut Game { world };

    event::run(context, event_loop, game)
}

fn initialize_level(world: &mut World) {
    components::create_player(world, Position { x: 0, y: 0, z: 0 });
    components::create_box(world, Position { x: 1, y: 0, z: 0 });
    components::create_box_spot(world, Position { x: 2, y: 0, z: 0 });
    components::create_floor(world, Position { x: 3, y: 0, z: 0 });
    components::create_wall(world, Position { x: 4, y: 0, z: 0 });
}
