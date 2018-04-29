//! The simplest possible example that does something.
extern crate ggez;

use ggez::{ContextBuilder, Context, GameResult};
use ggez::conf::{WindowSetup};
use ggez::event::{self, EventHandler, MouseState};
use ggez::graphics::{clear, circle, present, DrawMode, Point2};

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
impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx);
        circle(ctx,
                         DrawMode::Fill,
                         Point2::new(self.pos_x, self.pos_y),
                         100.0,
                         0.5)?;
        present(ctx);
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        _x: i32,
        _y: i32,
        _xrel: i32,
        _yrel: i32
    ) {
        self.pos_x = _x as f32;
        self.pos_y = _y as f32;
    }
}
pub fn main() {
    let cb = ContextBuilder::new("rust-game-experiment", "ggez")
        .window_setup(WindowSetup::default().title("My first Rust game"));
    let ctx = &mut cb.build().unwrap();
    println!();
    println!();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
