use components::*;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::{conf, event, Context, GameResult};
use render_system::*;
use specs::{RunNow, World, WorldExt};
use std::path;

mod camera_system;
mod components;
mod input_system;
mod map;
mod render_system;
mod texture_atlas;
mod transform_system;

pub const TILE_WIDTH: f32 = 8.0;
pub const TILE_HEIGHT: f32 = 8.0;

struct Game {
    world: World,
    texture_atlas: texture_atlas::TextureAtlas,
    mouse_x: f32,
    mouse_y: f32,
    is_mouse_button_down: bool,
}

impl event::EventHandler for Game {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ggez::event::quit(ctx),
            _ => (),
        }

        let mut input_queue = self.world.write_resource::<InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if button == MouseButton::Left {
            self.is_mouse_button_down = true;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        if button == MouseButton::Left {
            self.is_mouse_button_down = false;
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        println!("{:?}", (_x, _y, _dx, _dy));
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut input_system = input_system::InputSystem {};
        input_system.run_now(&self.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        {
            let mut render_system = RenderSystem {
                context: ctx,
                texture_atlas: &mut self.texture_atlas,
            };
            render_system.run_now(&self.world);
        }

        Ok(())
    }
}

fn main() -> GameResult {
    let mut world = World::new();
    components::register_resources(&mut world);
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
        mouse_x: 0.0,
        mouse_y: 0.0,
        is_mouse_button_down: false,
    };

    event::run(context, events_loop, game)
}

fn initialize_level(world: &mut World) {
    components::create_camera(
        world,
        Position {
            x: 0.0,
            y: 0.0,
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
