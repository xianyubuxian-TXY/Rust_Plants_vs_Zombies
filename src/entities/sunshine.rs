use std::error::Error;

use ggez::graphics::Image;
use ggez::{graphics::Rect, Context,GameResult};
use glam::Vec2;
use rand::Rng;
use crate::game;
use crate::entities_managers::background_manager::ResourceManager;
use crate::tools::mydraw;
use crate::game::Game;

use super::my_enum::sunshine_enum::SunshineType;

//mov offset after be clicked
const  OFFSET:f32=6.0;
//collection rect
const COLLECTION_RECT:Rect=Rect::new(450.0,0.0,20.0,20.0);
//image's width and height
const WIDTH:f32=100.0;
const HEIGHT:f32=100.0;
//the interval of update frame
const INTERVAL_UPDATE_FRAME:u32=15;
//the speed of sunshine's descent_speed
const DESCENT_SPEED:f32=1.0;
//lifetime of sunshine after stop descent
const LIFETIME:i32=500;



pub struct Sunshine{
    pub position: Vec2,
    width:f32,
    height:f32,
    frame_index: usize, // animation frame
    delay:u32, //the delay of draw animation 
    pub dest_y: f32,
    used: bool,//used=true: liveable  used=false:dead
    pub be_clicked:bool,//whether be clicked by mouse
    timer: i32, //lifetime
    descent_speed: f32,
    sunshine_type:SunshineType
}

impl Sunshine {
    pub fn new()->Sunshine{
        Sunshine { 
            position: Vec2::new(0.0,0.0),
            width:WIDTH,
            height:HEIGHT,
            frame_index: 0,
            delay:0,
            dest_y: 0.0,
            used:false,
            be_clicked:false,
            descent_speed:0.0,
            timer:0,
            sunshine_type:SunshineType::CommonSunShine
        }
    }

    pub fn is_used(&self)->bool{
        self.used
    }

    // pub fn set_used(&mut self){
    //     self.used=true;
    // }

    pub fn check_clicked(&mut self,x:f32,y:f32){
        if !self.be_clicked {
            let m_x=self.position.x;
            let m_y=self.position.y;
            let m_w=self.width;
            let m_h=self.height;
            if x>=m_x && x<=(m_x+m_w) && y>=m_y && y<=(m_y+m_h){
                self.be_clicked=true;
                println!("be clicked\n");
            }
        }
    }

    pub fn be_clikced(&self)->bool{
        self.be_clicked
    }

    pub fn init(&mut self,sunshine_type:SunshineType){
        let mut rng=rand::thread_rng(); //get random seed
        self.position.x=rng.gen_range(400.0..1450.0);
        self.position.y=40.0;
        self.frame_index=rng.gen_range(0..29);
        self.delay=INTERVAL_UPDATE_FRAME;
        self.dest_y=rng.gen_range(200.0..850.0);
        self.descent_speed=DESCENT_SPEED;
        self.be_clicked=false; 
        self.used=true;  
        self.timer=LIFETIME;
        self.sunshine_type=sunshine_type;
    }
    //return sunshine_value be added
    pub fn update_status(&mut self){
        if self.used{
            //if be clicked
            if self.be_clicked {
                //if sunshine be collected, set unused and sunshine_vaue+50
                if COLLECTION_RECT.contains(self.position) || self.position.y<=0.0{ //still have some offset when sunshine position.x=400, so add condition of self.position.y<=0.0
                    self.used=false;
                }
                else {
                    //if be clicked but not be collected, mov to collection
                    let my_x=self.position.x;
                    let my_y=self.position.y;
                    let des_x=COLLECTION_RECT.x;
                    let des_y=COLLECTION_RECT.y;
                    let atan=((my_y-des_y).abs()/(des_x-my_x).abs()).atan();
                    if my_x>des_x {
                        self.position.x-=OFFSET*atan.cos();
                    }else{
                        self.position.x+=OFFSET*atan.cos();
                    }
                    self.position.y-=OFFSET*atan.sin();
                }
            }
            //if not be clicked and reach the dest_y, stop     
            else if self.position.y>=self.dest_y{ 
                self.timer-=1;
                //if lifetime be used out of,set unused
                if self.timer<=0{
                    self.used=false;
                }
            }
            else {
                self.position.y+=self.descent_speed;
            }
            self.delay-=1;
            if self.delay<=0 {
                self.frame_index=(self.frame_index+1)%29;
                self.delay=INTERVAL_UPDATE_FRAME;
            }
        }
    }

    pub fn draw(&self,ctx:&mut Context,animation:&Vec<Image>)->GameResult<()> {
        if self.used{
            let frame=&animation[self.frame_index];
            mydraw(ctx, &frame, self.position.x,self.position.y, self.width, self.height)?;
        }
        Ok(())
    }
}