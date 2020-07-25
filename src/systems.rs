use super::components::*;
use ggez::graphics;
use ggez::Context;
use specs::{ReadStorage, System};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);
    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(1.0, 0.0, 0.22, 1.0));


        
        graphics::present(self.context).expect("Error while presenting");
    }
}
