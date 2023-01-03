
use ggez;
use ggez::graphics::{self, draw};
use crate::utils::*;


use crate::player::Player;



pub struct MainState {
    pub game_map: [String; crate::MAP_HEIGTH],
    pub player: Player,
}

impl MainState {
    pub fn new(_ctx: &mut ggez::Context) -> Self {
        MainState {
            game_map: [
                "wwwwwwwwww".to_string(),
                "w........w".to_string(),
                "w........w".to_string(),
                "w...w....w".to_string(),
                "w........w".to_string(),
                "w........w".to_string(),
                "w........w".to_string(),
                "w....w...w".to_string(),
                "w........w".to_string(),
                "wwwwwwwwww".to_string(),
            ],
            player: Player::new(),
        }
    } //new end
      //
      //

    pub fn draw_rect(
        &self,
        ctx: &mut ggez::Context,
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        c: graphics::Color,
    ) -> ggez::GameResult {
        let game_space = graphics::Rect::new(x, y, w, h);
        let game_space_mesh =
            graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), game_space, c)?;

        graphics::draw(ctx, &game_space_mesh, graphics::DrawParam::default())?;

        Ok(())
    }

    pub fn show_rays(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        for angle_offset in -30..30 {
            for extra in -1..1{

            let radian_offset = (angle_offset as f32+extra as f32 *0.1) * 0.017453;
            self.player.cast_stepped_ray(
                ctx,
                self.player.angle + radian_offset as f64,
                &self.game_map,
            )?;
            }
        }
        Ok(())
    }

    pub fn draw_fp(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let line_y_offset = crate::GAME_SCREENY + crate::UIY / 2.0;

        let fov = 30;
        //350
        let line_y_size = 150.0;
        let line_x_size = 14.0;

        for angle_offset in -fov..fov {
            for extra in -1..1{

            let radian_offset = (angle_offset as f32+extra as f32 *0.1) * 0.017453;

            let dist = self
                .player
                .cast_invi_stepped_ray(self.player.angle + radian_offset as f64, &self.game_map);
            let line_x_offset =
                (crate::GAME_SCREENW + crate::UIX) / 2.0 - angle_offset as f32 * line_x_size;
            if dist != 0.0 {
                draw_line_with_w(
                    ctx,
                    [line_x_offset, line_y_offset - line_y_size / dist],
                    [line_x_offset, line_y_offset + line_y_size / dist],
                    line_x_size,
                )?;
                }
            }
        }

        Ok(())
    }

    pub fn draw_map(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let wall: char = 'w';
        let ite = self.game_map.clone().into_iter();
        for (y, st) in ite.enumerate() {
            for (x, charac) in st.chars().enumerate() {
                if charac == wall {
                    self.draw_rect(
                        ctx,
                        x as f32 * crate::WALL_W,
                        y as f32 * crate::WALL_H,
                        crate::WALL_W - 1.0,
                        crate::WALL_W - 1.0,
                        graphics::Color::RED,
                    )?
                } else {
                    self.draw_rect(
                        ctx,
                        x as f32 * crate::WALL_W,
                        y as f32 * crate::WALL_H,
                        crate::WALL_W - 1.0,
                        crate::WALL_W - 1.0,
                        graphics::Color::BLACK,
                    )?
                }
            }
        }
        self.player.draw_self(ctx)?;

        Ok(())
    }

    /*
        pub fn draw_player(&self,ctx:&mut ggez::Context)->ggez::GameResult{
            self.draw_rect(ctx, self.player.cords[0]*crate::WALL_W, self.player.cords[1]*crate::WALL_H, 10.0, 10.0, graphics::Color::GREEN)? ;
            Ok(())
        }
    */
}
