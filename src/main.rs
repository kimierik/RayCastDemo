use std::f64::consts::PI;

use ggez;
use ggez::event::EventHandler;
use ggez::graphics;
use ggez::input::keyboard::{self, KeyCode};
//use ggez::input::keyboard::{self, KeyCode};
mod structs;

const GAME_SCREENW: f32 = 600.0;
const GAME_SCREENY: f32 = 600.0;
const UIX: f32 = 300.0;
const UIY: f32 = 300.0;

const MAP_HEIGTH: usize = 10;


const WALL_H: f32 = GAME_SCREENY / MAP_HEIGTH as f32;
const WALL_W:f32=GAME_SCREENW/MAP_HEIGTH as f32;




//#[derive(Debug,Copy,Clone)]

impl EventHandler<ggez::GameError> for structs::MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.player.move_sideways(1);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.player.move_sideways(-1);
        }

        if !keyboard::is_key_pressed(ctx, KeyCode::A) && !keyboard::is_key_pressed(ctx, KeyCode::D)
        {
            //self.player.velocity[0] = 0.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player.move_forward(1);
        }

        if keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player.move_forward(-1);
        }

        if !keyboard::is_key_pressed(ctx, KeyCode::W) && !keyboard::is_key_pressed(ctx, KeyCode::S)
        {
            //self.player.velocity[1] = 0.0;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::K) {
            self.player.angle += 0.1;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::J) {
            self.player.angle -= 0.1;
        }

        if self.player.angle > PI * 2.0 {
            self.player.angle = 0.0;
        }
        if self.player.angle < 0.0 {
            self.player.angle = PI * 2.0;
        }

        self.player.update_pos();
        self.player.velocity[0] = 0.0;
        self.player.velocity[1] = 0.0;
        //ph we cannot reset vel here prob bad
        //unless we check collision before update_pos
        //
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);
        //^clears the screen with white
        //
        //self.draw_rect( ctx, 0.0, 0.0, GAME_SCREENW + UIX, GAME_SCREENY, graphics::Color::WHITE,)?;
        //
        //self.draw_map(ctx)?;
        //        self.draw_player(ctx)?;
        self.draw_fp(ctx)?;
        //self.show_rays(ctx)?;

        //vv puts everything we just draw to the ctx
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("gametest", "kimierik")
        .window_mode(
            ggez::conf::WindowMode::default().dimensions(GAME_SCREENW + UIX, GAME_SCREENY + UIY),
        )
        .build()
        .expect("cb ERROR");

    let state = structs::MainState::new(&mut ctx);
    ggez::event::run(ctx, event_loop, state)
}
