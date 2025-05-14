use rand::Rng;
use ggez::{Context,GameResult};
use ggez::graphics::Image;
use crate::entities::my_enum::sunshine_enum::SunshineType;
use crate::entities::sunshine::Sunshine;
use crate::my_trait::SunshineAction;
use crate::tools::load_animation;

//sunshie_pool_size
const SUNSIHNEPOOLSIZE:i32 =30;
pub struct SunshineManager{
    //when sunshine_timer<=0, create a sunshine
    sunshine_timer:i32,
    sunshines_pool:Vec<Sunshine>,
    sunshines_animation:Vec<Image>,
}

impl SunshineManager {
    pub fn new(ctx:&mut Context)->GameResult<Self>{
        let mut pool=Vec::new();
        for _ in 1..=SUNSIHNEPOOLSIZE{
            pool.push(Sunshine::new());
        }
        let mut animation=Vec::new();
        load_animation(ctx, &mut animation, 1, 29,"/images/sunshine/frame/" , ".png")?;

        Ok(SunshineManager{
            sunshine_timer:600,
            sunshines_pool: pool,
            sunshines_animation:animation
        })
    }

    pub fn init(&mut self){
        self.sunshine_timer=600;
        for sunshine in self.sunshines_pool.iter_mut(){
            sunshine.set_unused();
        }
    }

    pub fn create_sunshine(&mut self){
        self.sunshine_timer-=1;
        if self.sunshine_timer<=0{
            //reset sunshine_timer
            let mut rng=rand::thread_rng();
            self.sunshine_timer=rng.gen_range(1000..1200);
            //create sunshine
            if let Some(sunshine)=self.sunshines_pool.iter_mut().find(|s| !s.is_used()){
                sunshine.init(SunshineType::CommonSunShine);
            }
            // println!("create sunshine success\n");
        }
    }

}

impl SunshineAction for SunshineManager {
    fn get_sunshines_pool_mut(&mut self) -> &mut Vec<Sunshine>{
        &mut self.sunshines_pool
    }
    
    // 返回不可变引用（用于绘制和只读操作）
    fn get_sunshines_pool(&self) -> &Vec<Sunshine>{
        &self.sunshines_pool
    }
    
    fn get_sunshines_animation(&self) -> &Vec<Image>{
        &self.sunshines_animation
    }
}