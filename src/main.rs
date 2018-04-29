//! The simplest possible example that does something.
extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Point2};
struct MainState {
    pos_x: f32,
    pos_y: f32,
}
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0, pos_y: 0.0 };
        Ok(s)
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        self.pos_y = self.pos_y % 600.0 + 1.0;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, self.pos_y),
                         100.0,
                         0.5)?;
        graphics::present(ctx);
        Ok(())
    }
}
pub fn main() {
    let cb = ContextBuilder::new("rust-game-experiment", "ggez")
        .window_setup(conf::WindowSetup::default().title("My first Rust game"));
    let ctx = &mut cb.build().unwrap();
    println!();
    println!();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
