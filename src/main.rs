extern crate ggez;
extern crate nalgebra as na;
extern crate ncollide;
extern crate nphysics2d;

use ggez::{ContextBuilder, Context, GameResult};
use ggez::conf::{WindowSetup};
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics::{clear, circle, line, present, DrawMode, Point2};
use ggez::timer;

use na::{Vector2, Translation2};
use ncollide::shape::{Ball, Plane};
use nphysics2d::world::World;
use nphysics2d::object::{RigidBody, RigidBodyHandle};

struct MainState {
    world: World<f32>,
    player: RigidBodyHandle<f32>,
    left_down: bool,
    right_down: bool,
}
impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 300.0));
        let mut floor = RigidBody::new_static(Plane::new(Vector2::new(0.0, -1.0)), 0.0, 0.0);
        floor.append_translation(&Translation2::new(0.0, 400.0));
        world.add_rigid_body(floor);
        let mut player = RigidBody::new_dynamic(Ball::new(10.0), 1.0, 0.0, 0.0);
        player.set_inv_mass(1.0);
        player.append_translation(&Translation2::new(300.0, 200.0));
        let player = world.add_rigid_body(player);
        let s = MainState { world, player, left_down: false, right_down: false };
        Ok(s)
    }
    fn refresh_horizontal_vel(&mut self) {
        let mut player: std::cell::RefMut<RigidBody<f32>> = self.player.borrow_mut();
        let current = player.lin_vel();
        let mut dir = 0.0;
        if self.left_down && !self.right_down {
            dir = -1.0;
        } else if self.right_down && !self.left_down{
            dir = 1.0;
        }
        player.set_lin_vel(Vector2::new(dir * 100.0, current.y));
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
        let pos = self.player.borrow().position_center();
        circle(ctx, DrawMode::Fill, Point2::new(pos.x, pos.y), 10.0, 0.5)?;
        line(ctx, &[Point2::new(0.0, 400.0), Point2::new(800.0, 400.0)], 1.0)?;
        present(ctx);
        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            match keycode {
                Keycode::Left => {
                    self.left_down = true;
                }
                Keycode::Right => {
                    self.right_down = true;
                }
                Keycode::Space => {
                    let mut player: std::cell::RefMut<RigidBody<f32>> = self.player.borrow_mut();
                    if player.position_center().y + 10.0 > 399.5 {
                        player.apply_central_impulse(Vector2::new(0.0, -150.0));
                    }
                }
                _ => ()
            }
            self.refresh_horizontal_vel();
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, repeat: bool) {
        if !repeat {
            match keycode {
                Keycode::Left => {
                    self.left_down = false;
                }
                Keycode::Right => {
                    self.right_down = false;
                }
                _ => ()
            }
            self.refresh_horizontal_vel();
        }
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
