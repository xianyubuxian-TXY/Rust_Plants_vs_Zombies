use ggez::{graphics::{Image, Rect}, Context, GameResult};
use glam::Vec2;

use super::{my_enum::plant_enum::PlantType, plant::Plant};

pub struct Grass{
    used:bool,
    pub plant:Plant,
    row:u32,
    // column:u32,
    // rect:Rect,
}

impl Grass{
    pub fn new(plant:Plant,row:u32)->Grass{
        Grass{
            used:false,
            plant:plant,
            row:row,
            // column:column
            // rect:rect,
        }
    }

    pub fn set_used(&mut self){
        self.used=true;
    }

    pub fn set_unused(&mut self){
        self.used=false;
        self.plant.set_unused();
    }

    pub fn is_used(&self)->bool{
        self.used
    }

    pub fn get_plant_type(&self)->&PlantType{
        self.plant.get_type()
    }

    //return grow plant whether is succeed
    pub fn grow_plant(&mut self,plant_type:PlantType)->bool{
        if !self.plant.is_used(){
            self.plant.init(plant_type);
            self.plant.set_used();
            return true;
        }
        else{
            panic!("plant is unused,but grass is still used, the logic is not consistent\n");
        }
        false
    }

    pub fn draw_plant(&self,ctx:&mut Context,animation:&Vec<Image>)->GameResult<()>{
        if self.plant.is_used(){
            self.plant.draw(ctx, animation)?;
        }
        Ok(())
    }

}