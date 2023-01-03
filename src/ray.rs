use std::f64::consts::PI;


pub struct Ray {
    pub starting_cords: [f32; 2],
    pub now_cords: [f32; 2],
    pub angle: f64,
    pub total_hypotenuse: f32,
    pub ref_cords: [f32; 2],
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
    

    //attempt to start the ray at the first rounded y cordinate    
    pub fn initialize_y(&mut self){
        if self.angle >PI{
            self.now_cords[1]=self.starting_cords[1].ceil();
            //self.now_cords[1]+=0.01;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();
            self.now_cords[0] =
                self.starting_cords[0] - self.ref_cords[1] / self.angle.tan() as f32;
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
        }else{
            self.now_cords[1]=self.starting_cords[1].floor();
            self.now_cords[1]-=0.02;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();
            self.now_cords[0] =
                self.starting_cords[0] + self.ref_cords[1] / self.angle.tan() as f32;
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
        }
        self.total_hypotenuse = (self.ref_cords[0].powf(2.0) + self.ref_cords[1].powf(2.0)).sqrt();
    }
    pub fn initialize_x(&mut self){
        if self.angle> PI/2.0 && self.angle <PI*1.5{
            self.now_cords[0]=self.starting_cords[0].floor();
            self.now_cords[0]-=0.02; 
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
            self.now_cords[1]= self.starting_cords[1] + self.ref_cords[0] * self.angle.tan() as f32;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();
        }else{
            self.now_cords[0]=self.starting_cords[0].ceil();
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
            self.now_cords[1]= self.starting_cords[1] - self.ref_cords[0] * self.angle.tan() as f32;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();

        }
        self.total_hypotenuse = (self.ref_cords[0].powf(2.0) + self.ref_cords[1].powf(2.0)).sqrt();
    }


    pub fn step_y(&mut self) {
        //steps 1 unit in the y direction and appropriate in x
        if self.angle > PI {
            self.now_cords[1] += 1.0;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();
            self.now_cords[0] =
                self.starting_cords[0] - self.ref_cords[1] / self.angle.tan() as f32;
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
        } else {
            self.now_cords[1] -= 1.0;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();
            self.now_cords[0] =
                self.starting_cords[0] + self.ref_cords[1] / self.angle.tan() as f32;
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
        }

        self.total_hypotenuse = (self.ref_cords[0].powf(2.0) + self.ref_cords[1].powf(2.0)).sqrt();
    }

    //this one migth be a little fucked so you need to re check this 
    pub fn step_x(&mut self){
        if self.angle> PI/2.0 && self.angle <PI*1.5{
            self.now_cords[0]-=1.0;
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
            self.now_cords[1]= self.starting_cords[1] + self.ref_cords[0] * self.angle.tan() as f32;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();
        }else{
            self.now_cords[0]+=1.0;
            self.ref_cords[0] = (self.now_cords[0] - self.starting_cords[0]).abs();
            self.now_cords[1]= self.starting_cords[1] - self.ref_cords[0] * self.angle.tan() as f32;
            self.ref_cords[1] = (self.now_cords[1] - self.starting_cords[1]).abs();
        }
        self.total_hypotenuse = (self.ref_cords[0].powf(2.0) + self.ref_cords[1].powf(2.0)).sqrt();
    }


    pub fn step(&mut self) {
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
