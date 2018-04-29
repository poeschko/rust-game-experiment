extern crate ggez;
extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics2d;

use ggez::{ContextBuilder, Context, GameResult};
use ggez::conf::{WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{clear, circle, present, DrawMode, Point2};
use ggez::timer;

use na::{Vector2, Translation2};
use ncollide::shape::{Ball};
use nphysics2d::world::World;
use nphysics2d::object::RigidBody;

struct MainState {
    world: World<f32>,
}
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 9.81));
        // let floor = RigidBody::new_static(Plane::new(Vector2::new(-1.0, -1.0)), 0.3, 0.6);
        let mut player = RigidBody::new_dynamic(Ball::new(10.0), 1.0, 0.3, 0.6);
        player.append_translation(&Translation2::new(30.0, 10.0));
        world.add_rigid_body(player);
        let s = MainState { world: world };
        Ok(s)
    }
}
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.world.step(1.0 / DESIRED_FPS as f32);
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx);
        for rb in self.world.rigid_bodies() {
            let pos = rb.borrow().position_center();
            circle(ctx, DrawMode::Fill, Point2::new(pos.x, pos.y), 10.0, 0.5)?;
        }
        present(ctx);
        Ok(())
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
