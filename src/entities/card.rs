use ggez::{graphics::{Image, Rect}, Context, GameResult};
use glam::Vec2;

use crate::tools::mydraw;
use super::my_enum::card_enum::CardType;

pub struct Card{
    pub card_type:CardType, //卡片类型
    pub name:String, //作为”键“，从hash_map中获取对应“实体图片”
    pub rect:Rect, //卡片范围
    pub card_image:Image, //卡片图像
}

impl Card {
    pub fn new(ctx:&mut Context,card_type:CardType,card_name:String,rect:Rect,card_image_path:&str)->GameResult<Self>{
        let card_image=Image::new(ctx,card_image_path)?;
        Ok(Card{
            card_type:card_type,
            name:card_name,
            rect:rect,
            card_image:card_image,     
        }
        )
    }

    pub fn be_selected(&self,x:f32,y:f32)->bool{
        self.rect.contains(Vec2::new(x,y))
    }

    pub fn get_type(&self)->&CardType{
        &self.card_type
    }

    pub fn get_name(&self)->&str{
        &self.name
    }

    pub fn get_rect(&self)->&Rect{
        &self.rect
    }

    // pub fn get_image(&self)->&Image{
    //     &self.card_image
    // }
    pub fn draw_image(&self,ctx:& mut Context)->GameResult<()>
    {
        mydraw(ctx, &self.card_image, self.rect.x, self.rect.y, self.rect.w, self.rect.h)?;
        Ok(())
    }
}
