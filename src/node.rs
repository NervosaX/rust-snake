use ggez::*;
use ggez::graphics::{Color, DrawMode, Rect, WHITE};
use constants::SIZE;

#[derive(Debug, Clone)]
pub struct Node {
    pub x: f32,
    pub y: f32,
    color: Color,
}

impl Node {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Node {
            x: x,
            y: y,
            color: color,
        }
    }

    pub fn intersects(&mut self, node: &Node) -> bool {
        let rect1 = Rect::new(self.x, self.y, SIZE, SIZE);
        let rect2 = Rect::new(node.x, node.y, SIZE, SIZE);

        rect1.overlaps(&rect2)
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}

impl event::EventHandler for Node {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, self.color)?;
        graphics::rectangle(ctx, DrawMode::Fill, Rect::new(self.x, self.y, SIZE, SIZE))?;
        graphics::set_color(ctx, WHITE)?;
        Ok(())
    }
}
