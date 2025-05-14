use ggez::{graphics::Image, Context, GameResult};
use glam::Vec2;
use rand::Rng;

use crate::tools::{draw_blood_bar, draw_position, mydraw};

use super::my_enum::zombie_enum::{ZombieStatus, ZombieType};
//row_gap and column_gap
use crate::entities_managers::map_manager::{ROW_GAP,COLUMN_GAP};
//first_row's y
const FIRST_ROW_Y:f32=140.0;
//init_x=screen's width
pub const SCREEN_WIDTH:f32=1600.0;
//interval_update_frame
const INTERVAL_UPDATE_FRAME:i32=15;
//move_speed
const MOVE_SPEED:f32=0.3;
//伤害
const DAMAGE:f32=2.0;

pub struct Zombie{
    position:Vec2,
    width:f32,
    height:f32,
    used:bool,
    row:u32,
    zm_type:ZombieType,
    zm_status:ZombieStatus,
    frame_index:usize,
    delay:i32,
    mov_speed:f32,
    max_blood:f32,
    cur_blood:f32,
    dead:bool,
    can_attack:bool,
    damage:f32,
}

impl Zombie {
    pub fn new()->Zombie {
        Zombie{
            position:Vec2::new(0.0,0.0),
            width:0.0,
            height:0.0,
            used:false,
            row:0,
            zm_type:ZombieType::CommonZM,
            zm_status:ZombieStatus::Walk0,
            frame_index:0,
            delay:INTERVAL_UPDATE_FRAME,
            mov_speed:MOVE_SPEED,
            max_blood:0.0,
            cur_blood:0.0,
            dead:false,
            can_attack:false,
            damage:DAMAGE,
        }
    }

    //关卡数越高，僵尸种类越多
    pub fn init(&mut self,cur_levels:u32){
        //随机中字
        let mut rng=rand::thread_rng(); //get random seed
        //初始话僵尸的位置、所在行
        let row=rng.gen_range(0..=4);
        self.position.x=SCREEN_WIDTH+rng.gen_range(COLUMN_GAP/2.0..COLUMN_GAP);
        self.position.y=FIRST_ROW_Y+ROW_GAP*(row as f32)+ROW_GAP/2.0;
        self.row=row;
        //初始化速度
        self.mov_speed=MOVE_SPEED;
        //初始化僵尸类型
        let mut type_num=rng.gen_range(1..=7) as usize;
        match cur_levels{
            1=>{
                //第一关，全为“普通僵尸”
                type_num=0;
            },
            2=>{
                //第二关：普通僵尸：路障僵尸=4:3
                match type_num{
                    1..=4=>{
                        type_num=0;
                    },
                    _=>{
                        type_num=1;
                    },
                }
            },
            3=>{
                //第三关：普通僵尸：路障僵尸：撑杆僵尸=3:2:2
                match type_num{
                    1..=3=>{
                        type_num=0;
                    },
                    4..5=>{
                        type_num=1;
                    },
                    _=>{
                        type_num=2;
                    }

                }
            }
            _=>{},
        }
        //通过 “数值”->"僵尸类型"： 0：普通僵尸  1：路障僵尸  2：撑杆僵尸
        match ZombieType::try_from(type_num){
            Ok(zombie)=>self.zm_type=zombie,
            Err(e)=>println!("Error:{}",e),
        };
        //初始化僵尸状态
        self.zm_status=ZombieStatus::Walk0;
        //初始化width与height
        (self.width,self.height)=self.zm_type.type_to_width_height();
        self.used=true;
        self.frame_index=0;
        //初始化血量
        self.max_blood=self.zm_type.type_to_blood();
        self.cur_blood=self.max_blood;
        //初始化dead
        self.dead=false;
        // self.dead_dalay=DEADDELAY;
    }

    pub fn is_used(&self)->bool{
        self.used
    }

    pub fn set_unused(&mut self){
        self.used=false;
    }

    pub fn is_dead(&self)->bool{
        self.dead
    }

    pub fn get_type(&self)->&ZombieType{
        &self.zm_type
    }

    pub fn get_status(&self)->&ZombieStatus{
        &self.zm_status
    }

    pub fn get_position(&self)->&Vec2{
        &self.position
    }

    pub fn get_row(&self)->u32{
        self.row
    }

    pub fn get_damage(&self)->f32{
        self.damage
    }

    pub fn can_attack(&self)->bool{
        self.can_attack
    }

    pub fn attack_cooldown(&mut self){
        self.can_attack=false;
    }

    pub fn be_attacked(&mut self,damage:f32){
        self.cur_blood-=damage;
        if self.cur_blood<=0.0{
            //进入死亡状态
            self.mov_speed=0.0; //速度降为0
            self.frame_index=0;
            self.dead=true;
            self.zm_status=ZombieStatus::Dead;
        }
    }


} 

impl Zombie{
    pub fn become_walk0_status(&mut self){
        self.frame_index=0; //动画帧清0
        self.mov_speed=MOVE_SPEED; //速度恢复
        self.zm_status=ZombieStatus::Walk0;
    }

    pub fn become_eat_status(&mut self){
        self.frame_index=0;//动画帧清0
        self.mov_speed=0.0; //停止
        self.zm_status=ZombieStatus::Eat; //进入eat状态
    }

    pub fn become_jump_status(&mut self){
        self.frame_index=0;//动画帧清0
        self.mov_speed=0.0; //速度清理，“位移”在播放“跳跃帧”时发送
        self.zm_status=ZombieStatus::Jump;
    }

    pub fn become_walk1_status(&mut self){
        self.frame_index=0;//动画帧清0
        self.mov_speed=MOVE_SPEED; //速度恢复
        self.zm_status=ZombieStatus::Walk1;
    }

    pub fn change_status(&mut self){
        match self.zm_type{
            //撑杆僵尸
            ZombieType::PoleVaultingZM=>{
                match self.zm_status{
                    ZombieStatus::Walk0=>{ //walk0->jump
                        self.become_jump_status();
                    },
                    ZombieStatus::Jump=>{
                        self.become_walk1_status();
                    },
                    ZombieStatus::Walk1=>{
                        self.become_eat_status(); //walk1->eat
                    },
                    ZombieStatus::Eat=>{
                        self.become_walk1_status(); //eat->walk1
                    },
                    _=>{},
                }
            },
            //普通僵尸和路障僵尸
            _=>{
                match self.zm_status {
                    ZombieStatus::Walk0=>{ //walk0->eat
                        self.become_eat_status();
                    },
                    ZombieStatus::Eat=>{
                        self.become_walk0_status(); //eat->walk0
                    },
                    _=>{},
                }
            },
        }
    }

    pub fn update_status(&mut self){
        if self.used{
            self.position.x-=self.mov_speed;
            self.delay-=1;
            if self.delay<=0{
                //status_to_frame_num: 根据僵尸状态返回frame数
                self.frame_index=(self.frame_index+1)%(self.zm_status.status_to_frame_num(&self.zm_type));
                //处于死亡状态
                if self.zm_status==ZombieStatus::Dead{
                    //当“死亡帧”再次为0，死亡动画播放结束
                    if self.frame_index==0{
                        self.used=false; //真正死亡
                    }
                }
                //处于eat状态，每帧播放后 才能“攻击”
                else if self.zm_status==ZombieStatus::Eat{
                    self.can_attack=true;
                }
                //处于跳跃状态
                else if self.zm_status==ZombieStatus::Jump{
                    //总共跳“一格”距离，每一帧 “移动一点”
                    self.position.x-=(COLUMN_GAP)/(self.zm_status.status_to_frame_num(&self.zm_type) as f32);
                    //跳跃帧再次为0，跳跃帧结束
                    if self.frame_index==0{
                        self.change_status();
                    }
                }
                self.delay=INTERVAL_UPDATE_FRAME;
            }
        }   
    }

    pub fn draw(&self,ctx:&mut Context,animation:&Vec<Image>)->GameResult<()>{
        if self.used{
            mydraw(ctx,&animation[self.frame_index], self.position.x-COLUMN_GAP, self.position.y-ROW_GAP/2.0-75.0, self.width, self.height)?;
            if !self.dead{
                //死亡状态就不画血条了
                draw_blood_bar(ctx,self.position.x-COLUMN_GAP/2.0, self.position.y-ROW_GAP/2.0-50.0, self.cur_blood, self.max_blood)?;
            }
            draw_position(ctx, self.position)?;
        }
        Ok(())
    }
}