use std::error::Error;
use rand::Rng;
use crate::entities::sunshine;
use crate::resources::ResourceManager;

pub struct Spawner{
    //when sunshine_timer<=0, create a sunshine
    sunshine_timer:i32,
}

impl Spawner {
    pub fn new()->Spawner{
        Spawner{
            sunshine_timer:600,
        }
    }

    pub fn create_sunshine(&mut self,game_resource_manager:&mut ResourceManager)->Result<(),Box<dyn Error>> {
        self.sunshine_timer-=1;
        if self.sunshine_timer<=0{
            //reset sunshine_timer
            let mut rng=rand::thread_rng();
            self.sunshine_timer=rng.gen_range(600..1200);
            //create sunshine
            let sunshine_pool=game_resource_manager.get_entities_pool("sunshine_pool").expect("get sunsine_pool failed");
            let mut pool=sunshine_pool.lock().unwrap();
            if let Some(sunshine)=pool.iter_mut().find(|s| !s.is_used()){
                sunshine.init();
                sunshine.set_used();
            }
            // println!("create sunshine success\n");
        }
        Ok(())
    }
}