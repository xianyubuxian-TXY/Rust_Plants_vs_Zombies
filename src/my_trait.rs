use std::error::Error;

use ggez::{Context,GameResult};
use crate::resources::ResourceManager;

pub trait Entity {
    fn update_status(&mut self,game_resource_manager:&ResourceManager)->Result<(),Box<dyn Error>>{
        Ok(())
    }
    fn draw(&self,ctx:&mut Context, game_resource_manager:&ResourceManager) -> Result<(),Box<dyn Error>>;
}