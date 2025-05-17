use ggez::{ Context, GameResult};
use ggez::graphics::Image;
use std::collections::HashMap;
use crate::tools::mydraw;


pub fn load_texture(hash_map:&mut HashMap<String,Image>,ctx:&mut Context,texture_path:&str,texture_name: String)->GameResult<()>{
    let texture=Image::new(ctx,texture_path)?;
    hash_map.insert(texture_name, texture);
    Ok(())
}

//manager Image,font,sound..
pub struct ResourceManager {
    texture_res: HashMap<String,Image>,
    // animation_res: HashMap<String,Arc<Vec<Image>>>,
}

impl ResourceManager {
    pub fn new(ctx:&mut Context) -> GameResult<Self> {
        let mut texture_map =HashMap::new();
    //加载游戏开始界面背景
        load_texture(&mut texture_map,ctx,"/images/background/menu.png","menu.png".to_string())?;
    //加载playing界面背景
        load_texture(&mut texture_map,ctx,"/images/background/map1.png","map1.png".to_string())?;
        Ok(ResourceManager{
            texture_res:texture_map,
        })
    }

    //绘制开始界面背景
    pub fn draw_start_page_background(&self,ctx:&mut Context)->GameResult<()>{
        match self.texture_res.get("menu.png"){
            Some(menu)=>{
                mydraw(ctx, &menu, 0.0,0.0, 1600.0,1000.0)?;
            },
            None=>{eprintln!("draw background failed");}
        }
        Ok(())
    }

    //绘制
    pub fn draw_playing_page_background(&self,ctx:&mut Context)->GameResult<()>{
        //draw map
        match self.texture_res.get("map1.png"){
            Some(map1_img)=>{
                mydraw(ctx, &map1_img, 0.0, 0.0, 1600.0, 1000.0)?;
            },
            None=>{eprintln!("draw map1 failed");}
        }
        Ok(())
    }
}

