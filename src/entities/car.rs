use ggez::{graphics::Image, Context, GameResult};
use glam::Vec2;

use super::my_enum::car_enum::CarStatus;
use crate::{entities::zombie::SCREEN_WIDTH, tools::mydraw};
use crate::entities_managers::map_manager::{ROW_GAP,COLUMN_GAP};

//mov_speed
const MOV_SPEED:f32=3.0;
//image's width and height
const WIDTH:f32=110.0;
const HEIGHT:f32=110.0;

pub struct Car{
    position:Vec2,
    width:f32,
    height:f32,
    mov_speed:f32,
    row:u32,
    status:CarStatus,
    used:bool,
    frame:usize,
}

impl Car{
    pub fn new(position:Vec2,row:u32)->Car{
        Car{
            position:position,
            width:WIDTH,
            height:HEIGHT,
            used:true,
            mov_speed:MOV_SPEED,
            status:CarStatus::Stopping,
            row:row,
            frame:0,
        }
    }

    pub fn is_used(&self)->bool{
        self.used
    }

    pub fn get_row(&self)->u32{
        self.row
    }

    pub fn get_position(&self)->&Vec2{
        &self.position
    }

    pub fn get_status(&self)->&CarStatus{
        &self.status
    }

    pub fn become_running(&mut self){
        self.status=CarStatus::Running;
    }

    pub fn update(&mut self){
        match self.status{
            CarStatus::Running=>{
                self.position.x+=self.mov_speed;
                //出了屏幕，设置used=false
                if self.position.x>SCREEN_WIDTH{
                    self.used=false;
                }
            },
            CarStatus::Stopping=>{}
        }
    }

    pub fn draw(&self,ctx:&mut Context,animation:&Vec<Image>)->GameResult<()>{
        if self.used{
            mydraw(ctx, &animation[self.frame], self.position.x-COLUMN_GAP/2.0, self.position.y-ROW_GAP/2.0, self.width, self.height)?;
        }
        Ok(())
    }
}