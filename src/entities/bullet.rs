use ggez::{graphics::Image, Context, GameResult};
use glam::Vec2;

use crate::tools::{collision, mydraw};
//row_gap and column_gap
use crate::entities_managers::map_manager::ROW_GAP;

use super::zombie::Zombie;

//damage
const DAMAGE:f32=50.0;
//move speed
const SPEED:f32=2.0;
//width and height
const WIDTH:f32=35.0;
const HEIGHT:f32=35.0;
//dead_delay
const DEADDELAY:i32=10;

pub struct Bullet{
    position:Vec2,
    damage:f32,
    speed:f32,
    row:u32, //used when check collision with zm
    used:bool,
    frame:usize, //only two image
    dead:bool,
    dead_delay:i32, //“死亡”前留下点时间绘制死亡画面
}

impl Bullet{
    pub fn new()->Bullet {
        Bullet {  
            position:Vec2::new(0.0, 0.0),
            damage:DAMAGE,
            speed:SPEED,
            row:0,
            used:false,
            frame:0,
            dead:false,
            dead_delay:0,
        }
    }

    pub fn init(&mut self,position:&Vec2,row:u32){
        self.position.x=position.x;
        self.position.y=position.y;
        self.used=true;
        self.row=row;
        self.frame=0;
        self.dead=false;
        self.dead_delay=DEADDELAY;
    }

    pub fn is_used(&self)->bool{
        self.used
    }

    pub fn is_dead(&self)->bool{
        self.dead
    }

    pub fn get_position(&self)->&Vec2{
        &self.position
    }

    pub fn get_row(&self)->u32{
        self.row
    }

    pub fn get_damage(&self)->f32{
        self.damage
    }

    pub fn update_status(&mut self){
        if self.used{
            self.position.x+=self.speed;
            if self.dead{ //死亡状态
                self.dead_delay-=1;
                if self.dead_delay<=0{
                    self.used=false; //彻底死亡
                }
            }
        }
    }

    pub fn become_dead_status(&mut self){
        self.dead=true;
        self.frame=1;
    }

    pub fn draw(&self,ctx:&mut Context,animation:&Vec<Image>)->GameResult<()>{
        if self.used{
            mydraw(ctx, &animation[self.frame], self.position.x, self.position.y-ROW_GAP/3.0, WIDTH, HEIGHT)?;
        }

        Ok(())
    }
}