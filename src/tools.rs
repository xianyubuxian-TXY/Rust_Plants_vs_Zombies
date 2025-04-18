use std::error::Error;
use ggez::Context;
use ggez::graphics::{self, DrawParam, Image};
use glam::Vec2;

#[inline]
pub fn mydraw(ctx:&mut Context,image:&Image,x:f32,y:f32,width:f32,height:f32)->Result<(),Box<dyn Error>>{
    let rect=image.dimensions();
    let scale_w=width/rect.w;
    let scale_h=height/rect.h;
    let position=Vec2::new(x,y);
    let params=DrawParam::default().dest(position).scale([scale_w,scale_h]);
    graphics::draw(ctx, image, params)?;
    Ok(())
}


#[inline]
pub fn update_texture_path(texture_path:&mut String,front_part:&str,i:u32,end_part:&str)
{
    texture_path.clear();
    texture_path.push_str(front_part);
    texture_path.push_str(&i.to_string());
    texture_path.push_str(end_part);
}