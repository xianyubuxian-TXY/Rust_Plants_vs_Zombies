use std::error::Error;

use ggez::graphics::{draw, Image, Rect};
use ggez::{Context,GameResult};
use glam::Vec2;
use crate::game::{self, Game, GamePage, GameState};
use crate::my_trait::Entity;
use crate::tools::mydraw;
use crate::resources::ResourceManager;
pub enum ButtonType {
    GameStart,
}


//850.0, 200.0 , 550.0 , 240.0 : start_button

pub struct Button {
    button_frame:u32,
    rect: Rect,
    pub button_type: ButtonType,
    pub button_is_down: bool,
}

impl Button{
    pub fn new(btn_type:ButtonType,x:f32,y:f32,w:f32,h:f32) -> Button {
        Button {
            button_frame:0,
            rect: Rect::new(x,y,w,h),
            button_type:btn_type,
            button_is_down:false,
        }
    }

    pub fn be_clicked(&mut self,x:f32,y:f32)->bool
    {
        self.rect.contains(Vec2::new(x,y),)
    } 

    pub fn handle_click(&mut self) {
        match self.button_type{
            ButtonType::GameStart=>{
                if !self.button_is_down{
                    // self.button_image=;
                    println!("button down, change image");
                    self.button_is_down=true;
                }else{
                    println!("button up");
                    self.button_is_down=false;
                }
            }
        }
    }
}

impl Entity for Button {
    fn draw(&self,ctx:&mut Context, game_resource_manager:&ResourceManager)->Result<(),Box<dyn Error>> {
        match self.button_type{
            ButtonType::GameStart=>{
                let button_img;
                if self.button_is_down{
                    button_img=game_resource_manager.get_texture("start_button_1.png").unwrap();
                }else{
                    button_img=game_resource_manager.get_texture("start_button_0.png").unwrap();
                }
                mydraw(ctx, &button_img, self.rect.x, self.rect.y, self.rect.w, self.rect.h)?;
            }
        }
        Ok(())
    }
}