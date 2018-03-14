use ggez::*;
use rand;
use ggez::graphics::Color;
use node::Node;
use constants::SIZE;
use rand::Rng;
use player::Player;

pub struct Apple {
    pub node: Node,
    pub visible: bool,
}

impl Apple {
    pub fn new() -> Self {
        // Can this be done better?
        let color = Color::from_rgb(255, 0, 0);

        Self {
            node: Node::new(300.0, 100.0, color),
            visible: false,
        }
    }

    fn generate_pos(&self, max_width: u32, max_height: u32) -> (f32, f32) {
        let x_positions = max_width / SIZE as u32;
        let y_positions = max_height / SIZE as u32;

        let x = rand::thread_rng().gen_range(0, x_positions) as f32;
        let y = rand::thread_rng().gen_range(0, y_positions) as f32;

        (x * SIZE, y * SIZE)
    }

    pub fn set_new_position(&mut self, max_width: u32, max_height: u32, player: &mut Player) {
        loop {
            let (x, y) = self.generate_pos(max_width, max_height);
            self.node.set_position(x, y);

            if !player.node_intersects(&self.node) {
                break;
            }
        }

        self.visible = true;
    }
}

impl event::EventHandler for Apple {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.visible {
            self.node.draw(ctx)?;
        }

        Ok(())
    }
}
