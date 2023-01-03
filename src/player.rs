
use std::f64::consts::PI;


use ggez::graphics;
use crate::utils::*;
use crate::ray::Ray;


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
                    self.cords[0] * crate::WALL_W + (self.angle.cos() * 5.0) as f32,
                    self.cords[1] * crate::WALL_H - (self.angle.sin() * 5.0) as f32,
                ],
            ],
            2.0,
            graphics::Color::GREEN,
        )?;
        //magick number
        //draw_line(ctx, world_to_screen(self.cords), [ self.cords[0] * crate::WALL_W + ((self.angle+PI/2.0).cos() * 15.0) as f32, self.cords[1] * crate::WALL_H - ((self.angle+PI/2.0).sin() * 15.0) as f32, ],)?;

        graphics::draw(ctx, &line_mesh, graphics::DrawParam::default())?;
        graphics::draw(ctx, &elipse_mesh, graphics::DrawParam::default())?;

        Ok(())
    }

    pub fn cast_invi_stepped_ray(
        &self,
        ang: f64,
        map: &[String; crate::MAP_HEIGTH],
    ) -> f32{
        let mut x:Ray=Ray::new(self.cords, ang);
        let mut y: Ray = Ray::new(self.cords, ang);

        x.initialize_x();
        y.initialize_y();

        let mut x_hit:bool=false;
        let mut y_hit:bool=false;

        let mut dof: i32 = 0;
        let maxdof: i32 = 8;
        let mut dist:f32=0.0;

        if check_map(x.now_cords, map) {
            x_hit=true;
            dof = maxdof;
        }

        if check_map(y.now_cords, map) {
            y_hit=true;
            dof = maxdof;
        }

        while dof < maxdof {

            x.step_x();
            if check_map(x.now_cords, map) {
                x_hit=true;
                dof = maxdof;
            }
            y.step_y();
            if check_map(y.now_cords, map) {
                y_hit=true;
                dof = maxdof;
            }
            

            dof += 1;
        }
        
        //omg so spagetti
        if x_hit {
            dist=x.total_hypotenuse;
        }
        if y_hit{
            dist=y.total_hypotenuse;
        }
        if y_hit && x_hit {
            //witch has bigger hypotenuse
            if x.total_hypotenuse>y.total_hypotenuse{
                dist=y.total_hypotenuse;
            }else{
                dist=x.total_hypotenuse;
            }
        }



        return dist;
    }

    //check if a ray hit a wall then we need to see which one has a smaller hypotenuse
    //put to list? if list has 1 then it is that if it has 2 then smaller hypo
    pub fn cast_stepped_ray(
        &self,
        ctx: &mut ggez::Context,
        ang: f64,
        map: &[String; crate::MAP_HEIGTH],
    ) -> ggez::GameResult {
        let mut x:Ray=Ray::new(self.cords, ang);
        let mut y: Ray = Ray::new(self.cords, ang);
        let mut color : graphics::Color= graphics::Color::GREEN; 

        x.initialize_x();
        y.initialize_y();

        let mut x_hit:bool=false;
        let mut y_hit:bool=false;

        let mut dof: i32 = 0;
        let maxdof: i32 = 8;
        let mut end_cords: [f32; 2] = [0.0, 0.0];

        if check_map(x.now_cords, map) {
            x_hit=true;
            dof = maxdof;
        }

        if check_map(y.now_cords, map) {
            y_hit=true;
            dof = maxdof;
        }

        while dof < maxdof {

            x.step_x();
            if check_map(x.now_cords, map) {
                x_hit=true;
                dof = maxdof;
            }
            y.step_y();
            if check_map(y.now_cords, map) {
                y_hit=true;
                dof = maxdof;
            }

            dof += 1;
        }
        
        //omg so spagetti
        if x_hit {
            end_cords=x.now_cords;
            color=graphics::Color::RED;
        }
        if y_hit{
            end_cords=y.now_cords;
            color=graphics::Color::BLUE;
        }
        if y_hit && x_hit {
            //witch has bigger hypotenuse
            if x.total_hypotenuse>y.total_hypotenuse{
                end_cords=y.now_cords;
                color=graphics::Color::BLUE;
            }else{
                end_cords=x.now_cords;
                color=graphics::Color::RED;
            }
        }



        draw_line_w_col(ctx, world_to_screen(self.cords), world_to_screen(end_cords),color)?;
        //println!("x:{:?} , y:{:?} ",x_hit,y_hit);
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
