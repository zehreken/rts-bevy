use components::*;
use ggez::event::{KeyCode, KeyMods};
use ggez::{conf, event, Context, GameResult};
use specs::{RunNow, World, WorldExt};
use std::path;
use systems::*;

mod camera;
mod components;
mod map;
mod systems;
mod texture_atlas;

pub const TILE_WIDTH: f32 = 8.0;
pub const TILE_HEIGHT: f32 = 8.0;

struct Game {
    world: World,
    camera: camera::Camera,
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
        match keycode {
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let duration = ggez::timer::time_since_start(ctx);

        self.camera.translate(
            ctx,
            ggez::nalgebra::Vector2::new(ggez::timer::delta(ctx).as_secs_f32() * 50.0, 0.0),
        );
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            let mut rs = RenderingSystem {
                context,
                texture_atlas: &mut self.texture_atlas,
            };
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
    let (context, events_loop) = &mut context_builder.build()?;

    // with this you can set scale
    ggez::graphics::set_screen_coordinates(
        context,
        ggez::graphics::Rect::new(0.0, 0.0, 800.0 * 1.0, 600.0 * 1.0),
    )
    .unwrap();
    let camera = camera::Camera::new(800.0, 800.0);

    let texture_atlas =
        texture_atlas::TextureAtlas::new(context, "/images/colored_tilemap_packed.png".to_string());
    let game = &mut Game {
        world,
        camera,
        texture_atlas,
    };

    event::run(context, events_loop, game)
}

fn initialize_level(world: &mut World) {
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
