use components::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{conf, event, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;
use systems::*;

mod camera_system;
mod components;
mod map;
mod systems;
mod texture_atlas;

pub const TILE_WIDTH: f32 = 8.0;
pub const TILE_HEIGHT: f32 = 8.0;

struct Game {
    world: World,
    texture_atlas: texture_atlas::TextureAtlas,
}

impl event::EventHandler for Game {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        // println!("key pressed: {:?}", keycode);
        let mut delta = na::Vector2::new(0.0, 0.0);
        match keycode {
            KeyCode::Escape => ggez::event::quit(ctx),
            KeyCode::W => delta.y -= 1.0,
            KeyCode::A => delta.x -= 1.0,
            KeyCode::S => delta.y += 1.0,
            KeyCode::D => delta.x += 1.0,
            _ => (),
        }

        // move camera with the delta above
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rendering_system = RenderingSystem {
                context,
                texture_atlas: &mut self.texture_atlas,
            };
            rendering_system.run_now(&self.world);
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
    let (context, events_loop) = &mut context_builder.build()?;

    // with this you can set scale
    ggez::graphics::set_screen_coordinates(
        context,
        ggez::graphics::Rect::new(0.0, 0.0, 800.0 * 1.0, 600.0 * 1.0),
    )
    .unwrap();

    let texture_atlas =
        texture_atlas::TextureAtlas::new(context, "/images/colored_tilemap_packed.png".to_string());
    let game = &mut Game {
        world,
        texture_atlas,
    };

    event::run(context, events_loop, game)
}

fn initialize_level(world: &mut World) {
    components::create_camera(
        world,
        Position {
            x: 400.0,
            y: 300.0,
            z: 0.0,
        },
    );
    for i in 0..100 {
        for j in 0..25 {
            components::create_player(
                world,
                Position {
                    x: 0.0 + j as f32 * 4.0,
                    y: i as f32,
                    z: 0.0,
                },
            );
            components::create_box(
                world,
                Position {
                    x: 1.0 + j as f32 * 4.0,
                    y: i as f32,
                    z: 0.0,
                },
            );
            components::create_floor(
                world,
                Position {
                    x: 2.0 + j as f32 * 4.0,
                    y: i as f32,
                    z: 0.0,
                },
            );
            components::create_wall(
                world,
                Position {
                    x: 3.0 + j as f32 * 4.0,
                    y: i as f32,
                    z: 0.0,
                },
            );
        }
    }
}
