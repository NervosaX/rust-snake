use ggez::*;
use ggez::event::*;
use ggez::graphics::Color;
use node::Node;
use constants::{SIZE, SPEED};

pub struct Player {
    pub node: Node,
    pub tails: Vec<Node>,
    velocity_x: f32,
    velocity_y: f32,
}

impl Player {
    pub fn new() -> Self {
        // Can this be done better?
        let color = Color::from_rgb(0, 0, 255);

        Self {
            node: Node::new(300.0, 100.0, color),
            tails: Vec::new(),
            velocity_x: 1.0,
            velocity_y: 0.0,
        }
    }

    fn set_velocity(&mut self, vx: f32, vy: f32) {
        self.velocity_x = vx;
        self.velocity_y = vy;
    }

    fn get_tail_node(&mut self) -> Option<&Node> {
        let count = self.tails.len();

        match count {
            0 => Some(&self.node),
            _ => self.tails.last(),
        }
    }

    fn update_tail_position(&mut self, start_x: f32, start_y: f32) {
        let mut x = start_x;
        let mut y = start_y;

        for tail in &mut self.tails {
            let tvx = tail.x;
            let tvy = tail.y;

            tail.set_position(x, y);

            x = tvx;
            y = tvy;
        }
    }

    pub fn node_intersects(&mut self, node: &Node) -> bool {
        if self.node.intersects(node) {
            return true;
        }

        let mut tail_intersect = false;

        for x in self.tails.clone() {
            if self.node.intersects(&x) {
                tail_intersect = true;
            }
        }

        tail_intersect
    }

    pub fn add_tail(&mut self) {
        let tail = self.get_tail_node().cloned();

        let vx = self.velocity_x * -1.0;
        let vy = self.velocity_y * -1.0;

        match tail {
            Some(node) => {
                self.tails.push(Node::new(
                    node.x + vx * SIZE + vx,
                    node.y + vy * SIZE + vy,
                    Color::from_rgb(0, 255, 0),
                ));
            }
            None => panic!("There's no tail!"),
        };
    }

    pub fn update_player(&mut self, ctx: &mut Context, dt: f32) -> GameResult<()> {
        let last_x = self.node.x;
        let last_y = self.node.y;

        self.node.x += dt * SPEED * self.velocity_x + (SIZE * self.velocity_x);
        self.node.y += dt * SPEED * self.velocity_y + (SIZE * self.velocity_y);

        self.update_tail_position(last_x, last_y);

        for tail in &mut self.tails {
            tail.update(ctx)?;
        }

        let width = ctx.conf.window_mode.width as f32;
        let height = ctx.conf.window_mode.height as f32;
        let x = self.node.x;
        let y = self.node.y;

        if self.node.x < 0.0 {
            self.node.set_position(width, y);
            self.set_velocity(-1.0, 0.0);
        }

        if self.node.x > width {
            self.node.set_position(0.0, y);
            self.set_velocity(1.0, 0.0);
        }

        if self.node.y < -SIZE {
            self.node.set_position(x, height);
            self.set_velocity(0.0, -1.0);
        }

        if self.node.y > height {
            self.node.set_position(x, 0.0);
            self.set_velocity(0.0, 1.0);
        }

        Ok(())
    }
}

impl event::EventHandler for Player {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: Keycode,
        _keymod: Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::Up => {
                if self.velocity_y != 1.0 {
                    self.set_velocity(0.0, -1.0);
                }
            }
            Keycode::Down => {
                if self.velocity_y != -1.0 {
                    self.set_velocity(0.0, 1.0);
                }
            }
            Keycode::Left => {
                if self.velocity_x != 1.0 {
                    self.set_velocity(-1.0, 0.0);
                }
            }
            Keycode::Right => {
                if self.velocity_x != -1.0 {
                    self.set_velocity(1.0, 0.0);
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.node.draw(ctx)?;

        for tail in &mut self.tails {
            tail.draw(ctx)?;
        }

        Ok(())
    }
}
