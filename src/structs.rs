use std::f64::consts::PI;

use ggez;
use ggez::graphics::{self, draw};

fn draw_line(
    ctx: &mut ggez::Context,
    start_cords: [f32; 2],
    end_cords: [f32; 2],
) -> ggez::GameResult {
    if start_cords[1].round() == end_cords[1].round()
        && start_cords[0].round() == end_cords[0].round()
    {
    } else {
        let line_mesh =
            graphics::Mesh::new_line(ctx, &[start_cords, end_cords], 2.0, graphics::Color::GREEN)?;
        //magick number
        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
    }
    Ok(())
}

fn draw_line_with_w(
    ctx: &mut ggez::Context,
    start_cords: [f32; 2],
    end_cords: [f32; 2],
    width: f32,
) -> ggez::GameResult {
    if start_cords[1].round() == end_cords[1].round()
        && start_cords[0].round() == end_cords[0].round()
    {
    } else {
        let line_mesh = graphics::Mesh::new_line(
            ctx,
            &[start_cords, end_cords],
            width,
            graphics::Color::GREEN,
        )?;
        //magick number
        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn world_to_screen(cords: [f32; 2]) -> [f32; 2] {
    let ret = [cords[0] * crate::WALL_W, cords[1] * crate::WALL_H];
    ret
}

pub struct Ray {
    starting_cords: [f32; 2],
    pub now_cords: [f32; 2],
    angle: f64,
    total_hypotenuse: f32,
    ref_cords: [f32; 2],
}

impl Ray {
    pub fn new(cords: [f32; 2], ang: f64) -> Self {
        Ray {
            starting_cords: cords,
            now_cords: cords,
            total_hypotenuse: 0.0,
            angle: ang,
            ref_cords: cords,
        }
    }

    fn step(&mut self) {
        //ref cords are absolute cords of the end of ray
        //if you were toj
        self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
        self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();

        //here is where you implement something better
        self.now_cords[1] -= self.angle.sin() as f32 * 0.1;
        self.now_cords[0] += self.angle.cos() as f32 * 0.1;

        self.total_hypotenuse = (self.ref_cords[0].powf(2.0) + self.ref_cords[1].powf(2.0)).sqrt();
    }
}

pub struct Player {
    pub cords: [f32; 2],
    pub velocity: [f32; 2],
    pub angle: f64,
}

impl Player {
    pub fn new() -> Self {
        Player {
            cords: [5.0, 5.0],
            velocity: [0.0, 0.0],
            angle: 0.0,
        }
    }

    pub fn update_pos(&mut self) {
        self.cords[0] += self.velocity[0];
        self.cords[1] += self.velocity[1];
    }

    pub fn move_forward(&mut self, dir: i32) {
        //magick number movement speed
        self.velocity[0] += dir as f32 * (self.angle.cos()) as f32 * 0.1;
        self.velocity[1] += dir as f32 * (-self.angle.sin()) as f32 * 0.1;
    }

    pub fn move_sideways(&mut self, dir: i32) {
        //magick number movement speed
        //do the math this is so bad rigth now

        self.velocity[0] += dir as f32 * ((self.angle + PI / 2.0).cos()) as f32 * 0.1;
        self.velocity[1] += dir as f32 * (-(self.angle + PI / 2.0).sin()) as f32 * 0.1;
    }

    pub fn draw_self(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let elipse_mesh = ggez::graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            world_to_screen(self.cords),
            10.0,
            1.0,
            graphics::Color::GREEN,
        )?;

        let line_mesh = graphics::Mesh::new_line(
            ctx,
            &[
                world_to_screen(self.cords),
                [
                    self.cords[0] * crate::WALL_W + (self.angle.cos() * 15.0) as f32,
                    self.cords[1] * crate::WALL_H - (self.angle.sin() * 15.0) as f32,
                ],
            ],
            2.0,
            graphics::Color::GREEN,
        )?;
        //magick number
        draw_line(ctx, world_to_screen(self.cords), 
                [
                    self.cords[0] * crate::WALL_W + ((self.angle+PI/2.0).cos() * 15.0) as f32,
                    self.cords[1] * crate::WALL_H - ((self.angle+PI/2.0).sin() * 15.0) as f32,
                ],)?;

        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &elipse_mesh, graphics::DrawParam::default())?;

        Ok(())
    }

    //ray cast no draw returns length of the ray or 0 if out of range
    pub fn cast_invi_ray(&self, ang: f64, map: &[String; crate::MAP_HEIGTH]) -> f32 {
        //same idea as the one bellow

        let mut dof: i32 = 0;
        let maxdof: i32 = 80;

        let mut r = Ray::new(self.cords, ang);
        if ang == 0.0 || ang == PI {
        } else {
            while dof < maxdof {
                r.step();
                if r.now_cords[0].floor() < crate::MAP_HEIGTH as f32
                    && r.now_cords[1].floor() < crate::MAP_HEIGTH as f32
                {
                    if (map[(r.now_cords[1].floor()) as usize].as_bytes()
                        [(r.now_cords[0].floor()) as usize] as char
                        == 'w')
                    {
                        dof = maxdof;
                        return r.total_hypotenuse;
                    }
                }
                dof += 1;
            }
        }
        return 0.0;
    }

    pub fn cast_ray(
        &self,
        ctx: &mut ggez::Context,
        ang: f64,
        map: &[String; crate::MAP_HEIGTH],
    ) -> ggez::GameResult {
        //same idea as the one bellow

        let mut dof: i32 = 0;
        let maxdof: i32 = 50;

        let mut r = Ray::new(self.cords, ang);
        if ang == 0.0 || ang == PI {
        } else {
            while dof < maxdof {
                r.step();
                if r.now_cords[0].floor() < crate::MAP_HEIGTH as f32
                    && r.now_cords[1].floor() < crate::MAP_HEIGTH as f32
                {
                    draw_line(
                        ctx,
                        world_to_screen(self.cords),
                        world_to_screen(r.now_cords),
                    )?;
                    if (map[(r.now_cords[1].floor()) as usize].as_bytes()
                        [(r.now_cords[0].floor()) as usize] as char
                        == 'w')
                    {
                        dof = maxdof;
                    }
                }
                dof += 1;
            }
        }

        Ok(())
    }
    /*
        pub fn cast_ray(&self,ctx:&mut ggez::Context,ang:f64,map:&[String;crate::MAP_HEIGTH])->ggez::GameResult{
            let a_tan:f32=ang.atan()as f32;
            let mut ray_y:f32=0.1;
            let mut ray_x:f32=0.1;
            let mut y_offset:f32=0.0;
            let mut x_offset:f32=0.0;
            let mut dof:i32=0;

            println!("{:?}",ang);

            if ang>PI{
                ray_y=self.cords[1].ceil()  as f32;
            }

            if ang<PI{
                ray_y=self.cords[1].floor()as  f32 ;//+1.0;//crate::WALL_W;
            }

            ray_x=(self.cords[1]-ray_y)*ang.cos()as f32+self.cords[0];
            y_offset=-ang.sin()as f32;//-1.0;//crate::WALL_H;
            x_offset=ang.cos()as f32;
            if ang==0.0||ang==PI*2.0{
                ray_x=self.cords[0];
                ray_y=self.cords[1];
                dof=8;
            }
            while dof<8{
            println!("{:?} {:?}",self.cords,[ray_x,ray_y]);
                //get map x and map y from the rayx and ray y_offset
                //then see it it is a w to see that it is infact a wall
                if (map[(ray_y)as usize].as_bytes()[(ray_x)as usize]as char=='w'){
                    println!("hitst wal");
                    dof=8;
                }else{
                    ray_x+=x_offset/5.0;
                    ray_y+=y_offset/5.0;
                    dof*=1;
                }
            }
            draw_line(ctx, [self.cords[0]*WALL_W,self.cords[1]*crate::WALL_H], [ray_x*crate::WALL_W,ray_y*crate::WALL_H])?;

            Ok(())
        }
    */
} //player impol end

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
            let radian_offset = angle_offset as f32 * 0.017453;
            self.player.cast_ray(
                ctx,
                self.player.angle + radian_offset as f64,
                &self.game_map,
            )?;
        }
        Ok(())
    }

    pub fn draw_fp(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let line_y_offset = (crate::GAME_SCREENY + crate::UIY) / 2.0;

        let fov = 30;

        let line_y_size = 350.0;
        let line_x_size = 10.0;

        for angle_offset in -fov..fov {
            let radian_offset = angle_offset as f32 * 0.017453;
            let dist = self
                .player
                .cast_invi_ray(self.player.angle + radian_offset as f64, &self.game_map);
            let line_x_offset =
                (crate::GAME_SCREENW + crate::UIX) / 2.0 + angle_offset as f32 * line_x_size;
            if dist != 0.0 {
                draw_line_with_w(
                    ctx,
                    [line_x_offset, line_y_offset - line_y_size / dist],
                    [line_x_offset, line_y_offset + line_y_size / dist],
                    line_x_size,
                )?;
            }
        }

        Ok(())
    }

    pub fn draw_map(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.player.draw_self(ctx)?;

        let wall: char = 'w';
        let ite = self.game_map.clone().into_iter();
        for (y, st) in ite.enumerate() {
            for (x, charac) in st.chars().enumerate() {
                if charac == wall {
                    self.draw_rect(
                        ctx,
                        x as f32 * crate::WALL_W,
                        y as f32 * crate::WALL_H,
                        crate::WALL_W,
                        crate::WALL_W,
                        graphics::Color::RED,
                    )?
                }
            }
        }
        Ok(())
    }

    /*
        pub fn draw_player(&self,ctx:&mut ggez::Context)->ggez::GameResult{
            self.draw_rect(ctx, self.player.cords[0]*crate::WALL_W, self.player.cords[1]*crate::WALL_H, 10.0, 10.0, graphics::Color::GREEN)? ;
            Ok(())
        }
    */
}
