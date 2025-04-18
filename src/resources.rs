// use ggez::{graphics, Context, GameResult};
// use ggez::graphics::Image;
// use ggez::audio::Source;
// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::path::Path;
// use crate::entities::{plant::Plant, zombie::Zombie, bullet::Bullet};

// #[derive(Debug)]
// pub enum Assets {
//     Texture(Arc<Image>),
//     Animation(Arc<Vec<Image>>),
//     Sound(Arc<Source>),
// }

// pub enum Entity {
//     Plant(Plant),
//     Zombie(Zombie),
//     Bullet(Bullet),
// }

// pub struct EntityPool {
//     pub pool: Vec<Entity>,
// }

// pub struct ResourceManager {
//     assets_res: HashMap<String, Arc<Assets>>,
//     entities_res: HashMap<String, Arc<Mutex<EntityPool>>>,
// }

// impl ResourceManager {
//     pub fn new() -> Self {
//         ResourceManager {
//             assets_res: HashMap::new(),
//             entities_res: HashMap::new(),
//         }
//     }

//     pub fn add_image(&mut self, ctx: &mut Context, texture: Image, image_name: &str) -> GameResult<()> {
//         let img = Arc::new(Assets::Texture(Arc::new(texture)));
//         self.assets_res.insert(image_name.to_string(), img);
//         Ok(())
//     }

//     pub fn add_animation(&mut self, ctx: &mut Context, animation: Vec<Image>, animation_name: &str) -> GameResult<()> {
//         let amn = Arc::new(Assets::Animation(Arc::new(animation)));
//         self.assets_res.insert(animation_name.to_string(), amn);
//         Ok(())
//     }

//     pub fn add_sound(&mut self, ctx: &mut Context, sound: Source, sound_name: &str) -> GameResult<()> {
//         let snd = Arc::new(Assets::Sound(Arc::new(sound)));
//         self.assets_res.insert(sound_name.to_string(), snd);
//         Ok(())
//     }

//     pub fn get_assets(&self, resource_name: &str) -> Option<&Arc<Assets>> {
//         self.assets_res.get(resource_name)
//     }

//     pub fn get_texture(&self, resource_name: &str) -> Option<Arc<Image>> {
//         match self.get_assets(resource_name) {
//             Some(arc_assets) => match arc_assets.as_ref() {
//                 Assets::Texture(image) => Some(Arc::clone(image)),
//                 _ => None,
//             },
//             None => None,
//         }
//     }

//     pub fn get_animation(&self, resource_name: &str) -> Option<Arc<Vec<Image>>> {
//         match self.get_assets(resource_name) {
//             Some(arc_assets) => match arc_assets.as_ref() {
//                 Assets::Animation(animation) => Some(Arc::clone(animation)),
//                 _ => None,
//             },
//             None => None,
//         }
//     }

//     pub fn get_sound(&self, resource_name: &str) -> Option<Arc<Source>> {
//         match self.get_assets(resource_name) {
//             Some(arc_assets) => match arc_assets.as_ref() {
//                 Assets::Sound(source) => Some(Arc::clone(source)),
//                 _ => None,
//             },
//             None => None,
//         }
//     }
//     // pub fn clear(&mut self) {
//     //     self.assets_res.clear();
//     // }
// }


use ggez::{graphics, Context, GameResult};
use ggez::graphics::Image;
use ggez::audio::Source;
use glam::u32;
use std::collections::HashMap;
use std::os::windows::raw::HANDLE;
use std::sync::{Arc, Mutex};
use std::path::Path;
use crate::entities::{self, sunshine};
use crate::entities::{sunshine::SunShine,plant::Plant, zombie::Zombie, bullet::Bullet};
use crate::tools::update_texture_path;

pub enum EntityType{
    SunShine,
}
pub struct ResourceManager {
    texture_res: HashMap<String,Arc<Image>>,
    animation_res: HashMap<String,Arc<Vec<Image>>>,
    entities_pools:HashMap<String,Arc<Mutex<Vec<SunShine>>>>,
}

impl ResourceManager {
    pub fn new() -> GameResult<Self> {
        let texture_map =HashMap::new();
        let animation_map=HashMap::new();
        let entities_pools_map=HashMap::new(); 

        Ok(ResourceManager{
            texture_res:texture_map,
            animation_res:animation_map,
            entities_pools:entities_pools_map,
        })
    }
    //load texture
    pub fn load_texture(&mut self,ctx:&mut Context,texture_path:&str,texture_name: String)->GameResult<()>{
        let texture=Arc::new(Image::new(ctx,texture_path)?);
        self.texture_res.insert(texture_name, texture);
        Ok(())
    }

    pub fn load_animation(&mut self,ctx:&mut Context,frame_dir:&str,frame_num: u32,animation_name: String)->GameResult<()>{   
        let mut animation=Vec::new();
        let mut path=String::with_capacity(30);
        for i in 1..=frame_num{
            update_texture_path(&mut path,frame_dir, i,".png");
            let texture=Image::new(ctx,&path)?;
            animation.push(texture);
        }
        let animation=Arc::new(animation);
        self.animation_res.insert(animation_name, animation);
        Ok(())
    }

    pub fn load_entities_pool(&mut self,entities_type:EntityType,num:u32,pool_name:String)->GameResult<()>{
        let mut pool=Vec::new();
        match entities_type{
            EntityType::SunShine=>{
                for i in 1..=num{
                    pool.push(SunShine::new());
                }
            },
        };
        let pool=Arc::new(Mutex::new(pool));
        self.entities_pools.insert(pool_name, pool);
        Ok(())
    }

    pub fn get_animation(&self,animation_name: &str)->Option<Arc<Vec<Image>>>{
        self.animation_res.get(animation_name).cloned()
    }

    //get texture
    pub fn get_texture(&self, texture_name: &str) -> Option<Arc<Image>> {
        self.texture_res.get(texture_name).cloned()
    }

    pub fn get_entities_pool(&mut self,pool_name:&str)->Option<Arc<Mutex<Vec<SunShine>>>>{
        self.entities_pools.get_mut(pool_name).cloned()
    }
}

