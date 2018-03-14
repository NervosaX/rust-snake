extern crate ggez;
extern crate rand;
extern crate sdl2;

mod constants;
mod player;
mod node;
mod apple;

use ggez::*;
use ggez::event::*;
use sdl2::video::WindowPos;
use sdl2::VideoSubsystem;
use player::Player;
use apple::Apple;
use constants::DESIRED_FPS;

struct MainState {
    player: Player,
    apple: Apple,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut player = Player::new();

        // Add some starting tails
        for _ in 1..20 {
            player.add_tail();
        }

        let s = MainState {
            player: player,
            apple: Apple::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let dt = 1.0 / (DESIRED_FPS as f32);

            if !self.apple.visible {
                self.apple.set_new_position(
                    ctx.conf.window_mode.width,
                    ctx.conf.window_mode.height,
                    &mut self.player,
                );
            }

            self.player.update_player(ctx, dt)?;

            if self.apple.node.intersects(&self.player.node) {
                self.apple.set_new_position(
                    ctx.conf.window_mode.width,
                    ctx.conf.window_mode.height,
                    &mut self.player,
                );
                self.player.add_tail();
            }

            for tail in &mut self.player.tails {
                if tail.intersects(&self.player.node) {
                    println!("Intersect");
                    // ctx.quit();
                };
            }
        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        self.player.key_down_event(ctx, keycode, keymod, repeat)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.player.draw(ctx)?;
        self.apple.draw(ctx)?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();

    let state = &mut MainState::new().unwrap();

    let display_count = graphics::get_display_count(ctx).unwrap();

    // Hack for multi monitor lol
    if display_count > 1 {
        let video: VideoSubsystem = ctx.sdl_context.video().unwrap();
        let bounds = video.display_bounds(2).unwrap();

        {
            let window = graphics::get_window_mut(ctx);
            window.set_position(
                WindowPos::Positioned(bounds.x() + 100),
                WindowPos::Positioned(100),
            );
        }
    }

    event::run(ctx, state).unwrap();
}
