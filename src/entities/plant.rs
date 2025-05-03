use ggez::{graphics::Image, Context, GameResult};
use glam::Vec2;

use crate::tools::{draw_blood_bar, draw_position, mydraw};
use super::{bullet::Bullet, my_enum::plant_enum::PlantType, sunshine::Sunshine};
//row_gap and column_gap
use crate::entities_managers::map_manager::{ROW_GAP,COLUMN_GAP};
//interval_update_frame
const INTERVAL_UPDATE_FRAME:i32=15;
//plant Image's width and height
const WIDTH:f32=110.0;
const HEIGHT:f32=110.0;



/// price of plants
/// sunshine:50   Peashooter:100 WallNut:50

pub struct Plant{
    position:Vec2, //in the middle of grass
    width:f32,
    height:f32,
    plant_type:PlantType,
    frame_index:usize,
    delay:i32, // delay of draw frame
    used:bool,
    max_blood:f32,
    cur_blood:f32,
    row:u32, 
    skill_time:i32, //when skill_time<=0, activate the skill
    can_activate_skill:bool,
}

impl Plant {
    pub fn new(draw_pos:Vec2,row:u32)->Plant{
        Plant{
            position: draw_pos,
            width:WIDTH,
            height:HEIGHT,
            plant_type:PlantType::NonePlant,
            frame_index:0,
            delay:0,
            used:false,
            max_blood:0.0,
            cur_blood:0.0,
            row:row,
            skill_time:0,
            can_activate_skill:false,
        }
    }

    pub fn init(&mut self,plant_type:PlantType){
        self.plant_type=plant_type;
        self.frame_index=0;
        self.delay=INTERVAL_UPDATE_FRAME;
        self.used=true;
        self.max_blood=self.plant_type.type_to_blood();
        self.cur_blood=self.max_blood;
        self.skill_time=self.plant_type.type_to_skill_time();
        self.can_activate_skill=false;
    }

    pub fn set_used(&mut self){
        self.used=true;
    }

    pub fn set_unused(&mut self){
        self.used=false;
    }

    pub fn is_used(&self)->bool{
        self.used
    }

    pub fn get_type(&self)->&PlantType{
        &self.plant_type
    }

    pub fn get_position(&self)->&Vec2{
        &self.position
    }

    pub fn get_row(&self)->u32{
        self.row
    }

    pub fn can_activate_skill(&self)->bool{
        self.can_activate_skill
    }

    pub fn is_dead(&self)->bool{
        self.cur_blood<=0.0
    }

    pub fn be_attacked(&mut self,damage:f32){
        self.cur_blood-=damage;
        if self.cur_blood<=0.0{
            self.used=false;//死亡
        }
    }

}

impl Plant {
    //技能冷却
    pub fn skill_cooldown(&mut self){
        self.can_activate_skill=false;
        self.skill_time=self.plant_type.type_to_skill_time(); 
    }

    //豌豆射杀的技能： 发射子弹
    pub fn peashooter_shoot_bullet(&mut self,bullet:&mut Bullet){
        bullet.init(&self.position, self.row);
        //重置技能时间
    }

    //太阳花的技能：生成太阳
    pub fn sunflower_create_sunshine(&mut self,sunshine:&mut Sunshine){
        sunshine.init(super::my_enum::sunshine_enum::SunshineType::CommonSunShine);
        sunshine.position.x=self.position.x-COLUMN_GAP/2.0;
        sunshine.position.y=self.position.y-ROW_GAP/2.0;
        sunshine.dest_y=sunshine.position.y+ROW_GAP/2.0;
    }

    //更新状态
    pub fn update_status(&mut self){
        if self.used
        {
            //动画帧的更新延迟
            self.delay-=1;
            if self.delay<=0 
            {
                self.frame_index=(self.frame_index+1)%(self.plant_type.type_to_frame_num());
                self.delay=INTERVAL_UPDATE_FRAME;
            }
            //技能时间减少（只有当skill_time>0时才减少，当skill_time=0时，相当于存储技能，等待”发动技能后重新进入冷却“）
            if self.skill_time>0{
                self.skill_time-=1;
                if self.skill_time<=0{
                    self.can_activate_skill=true; //设置能够使用技能 
                }
            }
        }
    }

    pub fn draw(&self,ctx:&mut Context,animation:&Vec<Image>)->GameResult<()>{
        if self.used{
            let frame=&animation[self.frame_index];
            mydraw(ctx, &frame, self.position.x-COLUMN_GAP/2.0,self.position.y-ROW_GAP/2.0+20.0, self.width, self.height)?;
            draw_blood_bar(ctx,self.position.x-COLUMN_GAP/2.0,self.position.y-ROW_GAP/2.0, self.cur_blood,self.max_blood)?;
        }
        Ok(())
    }
}

