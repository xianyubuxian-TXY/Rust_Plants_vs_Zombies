use std::error::Error;

use ggez::graphics::draw;
use ggez::{graphics::Rect, Context,GameResult};
use glam::Vec2;
use rand::Rng;
use crate::game;
use crate::my_trait::Entity;
use crate::resources::ResourceManager;
use crate::tools::mydraw;
use crate::game::Game;

//offset after sunshine be clicked
const  OFFSET:f32=3.0;
//rect: collected sunshine, where sunshine's destination after being clicked
const COLLECTION_RECT:Rect=Rect::new(450.0,0.0,20.0,20.0);
//sunshinee's width and height
const WIDTH:f32=80.0;
const HEIGHT:f32=80.0;
const INTERVAL_UPDATE_FRAME:u32=15;

pub struct SunShine{
    position: Vec2,
    width:f32,
    height:f32,
    frame_index: usize, //animation frame
    counter:u32, //delay time of update frame
    dest_y: f32,
    used: bool,//used=true: liveable  used=false:dead
    be_clicked:bool,//whether be clicked by mouse
    timer: i32, //lifetime
    move_speed: f32,
}

impl SunShine {
    pub fn new()->SunShine{
        SunShine { 
            position: Vec2::new(0.0,0.0),
            width:0.0,
            height:0.0,
            frame_index: 0,
            counter:0,
            dest_y: 0.0,
            used:false,
            be_clicked:false,
            move_speed:0.0,
            timer:0,
        }
    }
    //init SunShine's member
    pub fn init(&mut self){
        let mut rng=rand::thread_rng(); //get random number generator of current thread
        self.position.x=rng.gen_range(400.0..1450.0);
        self.position.y=40.0;
        self.width=WIDTH;
        self.height=HEIGHT;
        self.frame_index=rng.gen_range(0..29);
        self.counter=INTERVAL_UPDATE_FRAME;
        self.dest_y=rng.gen_range(200.0..850.0);
        self.move_speed=0.5;
        self.be_clicked=false; 
        self.used=true;  
        self.timer=500;
    }
    pub fn update_status(&mut self,my_game:&mut Game){
        if self.used{
            //be clicked ,will mov to position of collection
            if self.be_clicked {
                //be collected, set used=false
                if COLLECTION_RECT.contains(self.position){
                    self.used=false;
                    my_game.add_sunshine_value();
                }
                else {
                    //calculate x_off and y_off , destination become collection rect
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
            //reach dest_y, timer start            
            else if self.position.y>=self.dest_y{ 
                self.timer-=1;
                if self.timer<=0{
                    self.used=false; //run out time, set used
                }
            }
            else {
                self.position.y+=self.move_speed;
            }
            self.counter-=1;
            if self.counter<=0 {
                self.frame_index=(self.frame_index+1)%29;
                self.counter=INTERVAL_UPDATE_FRAME;
            }
        }
    }

    pub fn is_used(&self)->bool{
        self.used
    }

    pub fn set_used(&mut self){
        self.used=true;
    }

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
}

impl Entity for SunShine {
    fn draw(&self,ctx:&mut Context, game_resource_manager:&ResourceManager) -> Result<(),Box<dyn Error>>{
        if self.used{
            let animation=game_resource_manager.get_animation("sunshine_animation").expect("get sunshine_animation failed");
            let frame=&animation[self.frame_index];
            mydraw(ctx, &frame, self.position.x,self.position.y, self.width, self.height)?;
            // println!("draw sunshine\n");
        }
        Ok(())
    }

}