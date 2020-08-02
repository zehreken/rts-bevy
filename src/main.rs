use components::*;
use ggez::event::{KeyCode, KeyMods, MouseButton};
use ggez::graphics;
use ggez::graphics::{DrawMode, DrawParam, Rect};
use ggez::{conf, event, Context, GameResult};
use render_system::*;
use specs::{RunNow, World, WorldExt};
use std::path;

mod camera_system;
mod collision_system;
mod components;
mod input_system;
mod map;
mod move_command_system;
mod move_system;
mod render_system;
mod selection_system;
mod texture_atlas;
mod transform_system;

pub const TILE_WIDTH: f32 = 8.0;
pub const TILE_HEIGHT: f32 = 8.0;

struct MainState {
    world: World,
    texture_atlas: texture_atlas::TextureAtlas,
    mouse_init_x: f32,
    mouse_init_y: f32,
    mouse_x: f32,
    mouse_y: f32,
    is_mouse_button_down: bool,
}

impl event::EventHandler for MainState {
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

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.mouse_init_x = x;
            self.mouse_init_y = y;
            self.is_mouse_button_down = true;
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        if button == MouseButton::Left {
            self.is_mouse_button_down = false;
            let mut input_queue = self.world.write_resource::<InputQueue>();
            use ggez::mint::Point2;
            input_queue.selection_command = Some((
                Point2 {
                    x: self.mouse_init_x,
                    y: self.mouse_init_y,
                },
                Point2 {
                    x: self.mouse_x,
                    y: self.mouse_y,
                },
            ));
        }
        if button == MouseButton::Right {
            let mut input_queue = self.world.write_resource::<InputQueue>();
            input_queue.move_commands.push(ggez::mint::Point2 { x, y });
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.mouse_x = x + 1.0;
        self.mouse_y = y + 1.0;
    }

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let mut input_system = input_system::InputSystem {};
        input_system.run_now(&self.world);

        let mut selection_system = selection_system::SelectionSystem {};
        selection_system.run_now(&self.world);

        let mut move_command_system = move_command_system::MoveCommandSystem {};
        move_command_system.run_now(&self.world);

        let mut collision_system = collision_system::CollisionSystem {};
        collision_system.run_now(&self.world);

        let mut move_system = move_system::MoveSystem {};
        move_system.run_now(&self.world);

        // After dynamic entity deletion, a call to World::maintain is necessary
        // in order to make the changes persistent and delete associated components.
        self.world.maintain();
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

        if self.is_mouse_button_down {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::Stroke(graphics::StrokeOptions::default()),
                Rect::new(
                    self.mouse_init_x,
                    self.mouse_init_y,
                    self.mouse_x - self.mouse_init_x,
                    self.mouse_y - self.mouse_init_y,
                ),
                graphics::WHITE,
            )
            .unwrap();

            use ggez::mint::Point2;
            let p1 = Point2 {
                x: self.mouse_init_x,
                y: self.mouse_init_y,
            };
            let p2 = Point2 {
                x: self.mouse_x,
                y: self.mouse_init_y,
            };
            let p3 = Point2 {
                x: self.mouse_x,
                y: self.mouse_y,
            };
            let p4 = Point2 {
                x: self.mouse_init_x,
                y: self.mouse_y,
            };
            let mesh = graphics::Mesh::new_polyline(
                ctx,
                DrawMode::Stroke(graphics::StrokeOptions::default()),
                &[p1, p2, p3, p4, p1],
                graphics::WHITE,
            )
            .unwrap();

            graphics::draw(ctx, &mesh, DrawParam::default()).unwrap();
        }

        graphics::present(ctx).expect("Error while presenting");

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
    let game = &mut MainState {
        world,
        texture_atlas,
        mouse_init_x: 0.0,
        mouse_init_y: 0.0,
        mouse_x: 1.0,
        mouse_y: 1.0,
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
        for j in 0..100 {
            components::create_floor(
                world,
                Position {
                    x: j as f32 * TILE_WIDTH,
                    y: i as f32 * TILE_HEIGHT,
                    z: 0.0,
                },
            );
        }
    }

    for i in 0..10 {
        components::create_actor(
            world,
            Position {
                x: 100.0 + i as f32 * 20.0,
                y: 100.0,
                z: 0.0,
            },
        );
    }

    components::create_tent(
        world,
        Position {
            x: 5.0,
            y: 5.0,
            z: 0.0,
        },
    );
    for i in 0..10 {
        components::create_tree(
            world,
            Position {
                x: 400.0 + i as f32 * 20.0,
                y: 400.0,
                z: 0.0,
            },
        );
    }
}
