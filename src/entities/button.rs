use ggez::graphics::{Image, Rect};
use ggez::{Context,GameResult};
use glam::Vec2;
use crate::tools::mydraw;
use super::my_enum::button_enum::{ButtonStatus,ButtonType};

pub struct Button {
    frame:usize,
    rect: Rect,
    button_type: ButtonType,
    pub button_status:ButtonStatus,
    images:Vec<Image>,
}

impl Button{
    pub fn new(btn_type:ButtonType,x:f32,y:f32,w:f32,h:f32,image:Vec<Image>) -> GameResult<Self> {
        Ok(Button {
            frame:0,
            rect: Rect::new(x,y,w,h),
            button_type:btn_type,
            button_status:ButtonStatus::ButtonUp,
            images:image,
        })
    }

    pub fn check_click(&mut self,x:f32,y:f32)->bool
    {
        let clicked= self.rect.contains(Vec2::new(x,y),);
        if clicked
        {
            self.button_status=ButtonStatus::ButtonDown;
            self.frame=ButtonStatus::ButtonDown as usize;
        }
        clicked
    }

    pub fn set_up(&mut self){
        self.button_status=ButtonStatus::ButtonUp;
        self.frame=ButtonStatus::ButtonUp as usize;
    }

    pub fn be_clicked(&self)->bool{
        match self.button_status {
            ButtonStatus::ButtonDown=>true,
            ButtonStatus::ButtonUp=>false,
        }
    }
    pub fn draw_image(&self,ctx:&mut Context)->GameResult<()> {
        let image=&self.images[self.frame];
        mydraw(ctx, &image, self.rect.x, self.rect.y, self.rect.w, self.rect.h)?;
        Ok(())
    }

}

