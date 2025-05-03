use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use rodio::{Decoder, OutputStream, Sink};

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawParam, Image, Rect};
use glam::Vec2;


//blood_width
const BLOODWIDTH:f32=120.0;
//blood_height
const  BLOODHEIGHT:f32=10.0;
//collision's width and height
const COLLISIONWIDTH:f32=10.0;
const COLLISIONHEIGHT:f32=10.0;

#[inline]
pub fn mydraw(ctx:&mut Context,image:&Image,x:f32,y:f32,width:f32,height:f32)->GameResult<()>{
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

#[inline]
pub fn load_animation(ctx:&mut Context,animation:&mut Vec<Image>,first_frame_id:u32,end_frame_id:u32,path_front_part:&str,path_end_part:&str)->GameResult<()>{
    let mut path=String::with_capacity(50);
    for i in first_frame_id..=end_frame_id{
        update_texture_path(&mut path, path_front_part, i,path_end_part);
        let image=Image::new(ctx, &path)?;
        animation.push(image);
    }
    Ok(())
}

#[inline]
pub fn draw_blood_bar(ctx:&mut Context,pos_x:f32,pos_y:f32,cur_blood:f32,max_blood:f32)->GameResult<()>{
    
    let background_color = Color::from_rgb(255, 0, 0); //red
    let foreground_color = Color::from_rgb(0, 255, 0); //green

    //current blood percentage
    let blood_percentage=cur_blood/max_blood;

    //draw part of read
    let background=graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(pos_x, pos_y,BLOODWIDTH, BLOODHEIGHT),
        background_color,
    )?;
    graphics::draw(ctx,&background,DrawParam::default())?;
    //draw part of green
    let foreground=graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        Rect::new(pos_x,pos_y,BLOODWIDTH*blood_percentage,BLOODHEIGHT),
        foreground_color,
    )?;
    graphics::draw(ctx,&foreground,DrawParam::default())?;

    Ok(())
}

#[inline]
pub fn draw_position(ctx:&mut Context,position:Vec2)->GameResult<()>{
    let center = Vec2::new(position.x, position.y);
    
    let radius = 5.0;

    let color = Color::from_rgb(255, 0, 0);//red

    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        center,
        radius,
        0.1,
        color,
    )?;
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    
    Ok(())
}

//检查position1是否与position2碰撞
#[inline]
pub fn collision(position1:&Vec2,position2:&Vec2)->bool{
    if (position1.x-position2.x).abs()<COLLISIONWIDTH && (position1.y-position2.y).abs()<COLLISIONHEIGHT{
        return true
    }
    false
}

#[inline]
pub fn play_mp3(mp3_path: impl AsRef<Path>) -> Result<(), String> {
    // 1. 打开 MP3 文件
    let file = File::open(mp3_path)
        .map_err(|e| format!("无法打开 MP3 文件: {}", e))?;
    
    // 2. 创建音频输出流
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| format!("无法创建音频流: {}", e))?;
    
    // 3. 创建音频接收器 (Sink)
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| format!("无法创建音频接收器: {}", e))?;
    
    // 4. 解码 MP3 文件
    let source = Decoder::new(BufReader::new(file))
        .map_err(|e| format!("无法解码 MP3 文件: {}", e))?;
    
    // 5. 播放音频
    sink.append(source);
    
    // 6. 阻塞直到播放完成（可选）
    // 注意：此行会阻塞当前线程，适合短音效
    // 如果是背景音乐，可移除此行避免阻塞
    // sink.sleep_until_end();
    
    Ok(())
}
