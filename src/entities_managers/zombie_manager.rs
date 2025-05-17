use std::sync::mpsc;

use ggez::{graphics::Image, Context, GameResult};
use rand::Rng;

use crate::{entities::{my_enum::zombie_enum::ZombieStatus, zombie::{self, Zombie}}, game::{self, GameMod}, threads::audio_thread::AudioEvent, tools::{collision, load_animation}};

use super::map_manager:: MapManager;

//zm_pool 的大小
const ZOMBIEPOOLSIZE:u32=200;
//第一个僵尸创建的时间间隔
// const INTERVAL_CREATE_TIME:i32=1500;
const INTERVAL_CREATE_TIME:i32=1500;
//僵尸吃植物音效播放间隔
const INTERVAL_ZM_EAT_AUDIO:i32=100;
//每一关的僵尸wave数
const ZM_WAVES:u32=5;


fn load_common_zm_animation(ctx:&mut Context,common_zm_animation:&mut Vec<Vec<Image>>)->GameResult<()>{
    //load walk animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation,1, 22,"/images/zm/common_zm/walk/", ".png")?;
    common_zm_animation.push(animation);
    //load eat animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation, 1,21, "/images/zm/common_zm/eat/", ".png")?;
    common_zm_animation.push(animation);
    //load dead animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation, 0, 9,"/images/zm/common_zm/dead/ZombieDie/",".png")?;
    common_zm_animation.push(animation);
    Ok(())
}

fn load_conhead_zm_animation(ctx:&mut Context,conehead_zm_animation:&mut Vec<Vec<Image>>)->GameResult<()>{
    //load walk animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation,1,20,"/images/zm/ConeheadZombie/walk/",".png")?;
    conehead_zm_animation.push(animation);
    //load eat animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation,1,10,"/images/zm/ConeheadZombie/eat/",".png")?;
    conehead_zm_animation.push(animation);
    //load dead animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation, 0, 9,"/images/zm/common_zm/dead/ZombieDie/",".png")?;
    conehead_zm_animation.push(animation);
    Ok(())
}

fn load_polevaulting_zm_animation(ctx:&mut Context,polevaulting_zm_animation:&mut Vec<Vec<Image>>)->GameResult<()>{
    //load walk0 animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation,0,9,"/images/zm/PoleVaultingZombie/walk0/",".png")?;
    polevaulting_zm_animation.push(animation);
    //load eat animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation,0,13,"/images/zm/PoleVaultingZombie/eat/",".png")?;
    polevaulting_zm_animation.push(animation);
    //load dead animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation, 0, 8,"/images/zm/PoleVaultingZombie/dead/",".png")?;
    polevaulting_zm_animation.push(animation);
    //load walk1 animation
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation, 0, 24,"/images/zm/PoleVaultingZombie/walk1/",".png")?;
    polevaulting_zm_animation.push(animation);
    //load jump
    let mut animation=Vec::new();
    load_animation(ctx, &mut animation, 0, 15,"/images/zm/PoleVaultingZombie/jump/",".png")?;
    polevaulting_zm_animation.push(animation);

    Ok(())
}



pub struct ZombieManager{
    //because zm have different status, every status need different animation,so use three dimensional array
    //first index is type, second index is status
    game_mod:GameMod, //游戏模式：普通/困难
    cur_level:u32, //当前level
    zm_waves:u32, //第几波僵尸： 每个level 都有10波僵尸
    zm_timer:i32, //产生僵尸的计时器
    zm_eat_audio_timer:i32, //僵尸吃植物音效播放间隔
    pub zm_pool:Vec<Zombie>,
    animations:Vec<Vec<Vec<Image>>>,
}

impl ZombieManager{
    pub fn new(ctx:&mut Context)->GameResult<Self>{
        //create zombie
        let mut zm_pool=Vec::new();
        for _ in 1..=ZOMBIEPOOLSIZE{
            zm_pool.push(Zombie::new());
        }

        let mut animations=Vec::new();
        //load common_zm's animation
        let mut common_zm_animation=Vec::new(); //type as index
        load_common_zm_animation(ctx, &mut common_zm_animation)?;
        animations.push(common_zm_animation);
        //load conehead_zm's animation
        let mut conehead_zm_animation=Vec::new();
        load_conhead_zm_animation(ctx, &mut conehead_zm_animation)?;
        animations.push(conehead_zm_animation);
        //load polevaulting_zm's animation
        let mut polevaulting_zm_animation=Vec::new();
        load_polevaulting_zm_animation(ctx, &mut polevaulting_zm_animation)?;
        animations.push(polevaulting_zm_animation);

        Ok(ZombieManager{
            game_mod:GameMod::Common,
            cur_level:1,
            zm_waves:0,
            zm_timer:INTERVAL_CREATE_TIME,
            zm_eat_audio_timer:INTERVAL_ZM_EAT_AUDIO,
            zm_pool:zm_pool,
            animations:animations,
         })
    }

    pub fn init(&mut self){
        self.cur_level=1;
        self.zm_waves=0;
        self.zm_timer=INTERVAL_CREATE_TIME;
        for zombie in self.zm_pool.iter_mut(){
            zombie.set_unused();
        }
    }

    pub fn set_game_mod(&mut self,game_mod:GameMod){
        self.game_mod=game_mod;
    }

    //返回当前关卡
    pub fn create_zombie(&mut self,audio_sender:&mpsc::Sender<AudioEvent>)->u32{
        //每关僵尸波数未达"MAX_ZM_WAVES"，则可以继续产生僵尸(当cur_level=4时表示胜利)
        let mod_num=self.game_mod.mod_to_num();
        let cur_level=self.cur_level;
        let cur_wave=self.zm_waves;
        if cur_level<4 && self.zm_waves<(ZM_WAVES+cur_level){
            self.zm_timer-=1;
            if self.zm_timer<=0{
                let mut rng=rand::thread_rng();
                self.zm_waves+=1; //僵尸wave+1
                //每次创建的僵尸数量
                let mut zm_num;
                match cur_level{
                    1=>{
                        zm_num=rng.gen_range(mod_num..=(mod_num+1)); //第一关：每次随机产生1~2只僵尸
                        if self.zm_waves==1{ //第一波僵尸，播放音效
                            match audio_sender.send(AudioEvent::PlaySFX("/audio/first_wave.mp3".to_string())){
                                Err(e)=>{eprintln!("send audio failed:{}",e);},
                                Ok(_)=>{},
                            }
                        }
                    },
                    2=>{
                        zm_num=rng.gen_range(3*mod_num..=(cur_wave+3*mod_num)); //第二关：每次随机产生2~3只僵尸
                        if self.zm_waves==1{ //第一波僵尸，播放音效
                            match audio_sender.send(AudioEvent::PlaySFX("/audio/second_wave.mp3".to_string())){
                                Err(e)=>{eprintln!("send audio failed:{}",e);},
                                Ok(_)=>{},
                            }
                        }
                    }
                    3=>{
                        zm_num=rng.gen_range(5*mod_num..=(cur_wave+5*mod_num));//第三关：每次随机产生3~5只僵尸
                        if self.zm_waves==1{ //第一波僵尸，播放音效
                            match audio_sender.send(AudioEvent::PlaySFX("/audio/final_wave.mp3".to_string())){
                                Err(e)=>{eprintln!("send audio failed:{}",e);},
                                Ok(_)=>{},
                            }
                        }
                    }
                    _=>{
                        zm_num=0;
                    },
                }
                for zombie in self.zm_pool.iter_mut(){
                    if !zombie.is_used(){
                        zombie.init(cur_level);
                        zm_num-=1;
                        if zm_num<=0{
                            break;
                        }
                    }
                }
                self.zm_timer=rng.gen_range(INTERVAL_CREATE_TIME-500..INTERVAL_CREATE_TIME-300); //重置产生僵尸的计时器
            }
        }
        //当僵尸波数超过“MAX_ZM_WAVES”时，停止产生僵尸
        //检查僵尸是否还有剩余的，当没有时-->关卡数增加(通过第3关后胜利)
        else if self.zm_waves>=ZM_WAVES{
            //当前关卡没有僵尸了，增加关卡
            if let Some(zm)=self.zm_pool.iter().find(|zm| zm.is_used()){
                if zm.is_used(){

                }
            }else{
                self.cur_level+=1; //当cur_level=4是表示胜利
                self.zm_waves=0; //僵尸波数清0                
            }

        }
        self.cur_level
    }

    //返回僵尸中最小的position.x-->用于判断是否失败
    pub fn update_zombies_status(&mut self,map_manager:&mut MapManager,audio_sender:&mpsc::Sender<AudioEvent>)->f32{
        let mut min_x=1600.0;
        let mut have_zm_eat=false;
        for zombie in self.zm_pool.iter_mut(){
            //存活的非死亡状态的僵尸
            if zombie.is_used() && !zombie.is_dead(){
                //是否发现攻击对象(用于检查是否需要 从 "eat状态"-->"walk 状态")
                let mut find_attack_target=false;
                //获取僵尸所在行
                let grasses=&mut map_manager.grasses[zombie.get_row() as usize];
                for grass in grasses.iter_mut(){
                    //“使用中”
                    if grass.is_used(){
                        let plant=&mut grass.plant;
                        //"碰到植物"
                        if collision(plant.get_position(),zombie.get_position()){
                            find_attack_target=true;//发现攻击对象 
                            let zombie_status=zombie.get_status();
                            //如果僵尸处于“walk”状态，则需要改变状态
                            if *zombie_status==ZombieStatus::Walk0 || *zombie_status==ZombieStatus::Walk1{
                                //改变僵尸状态
                                zombie.change_status();
                            }
                            //如果僵尸处于eat状态且处于“可攻击”，植物受到攻击 (攻击动画帧每次更新时，才能“真正攻击”)
                            if *zombie.get_status()==ZombieStatus::Eat{
                                have_zm_eat=true;
                                if zombie.can_attack(){
                                    //植物死亡，对于grass设置为unused
                                    plant.be_attacked(zombie.get_damage());
                                    if plant.is_dead(){
                                        grass.set_unused();
                                    }
                                    zombie.attack_cooldown(); //攻击冷却
                                }
                            } 
                        }
                    }
                }
                //如果没有发现攻击目标且处于eat状态-->恢复walk状态（用于植物被僵尸 吃掉）
                if !find_attack_target && *zombie.get_status()==ZombieStatus::Eat{
                    zombie.change_status();
                }
                if zombie.get_position().x<min_x{
                    min_x=zombie.get_position().x;
                }
            }
            //必须在循环外-->因为dead并不代表“真正死亡”，要等”死亡帧“播放完后，才真正死亡（used=false）
            //如果在循环内-->僵尸进入dead就不在update，”死亡帧“永远无法播放完，僵尸永远不会被设置为”uesd=false“
            zombie.update_status();
        }
        if have_zm_eat{
            self.zm_eat_audio_timer-=1;
            if self.zm_eat_audio_timer<=0{
                //播放音效
                match audio_sender.send(AudioEvent::PlaySFX("/audio/zombie_eat.mp3".to_string())){
                    Err(e)=>{eprintln!("send audio failed:{}",e);},
                    Ok(_)=>{},
                }
                self.zm_eat_audio_timer=INTERVAL_ZM_EAT_AUDIO;
            }
        }
        min_x
    }

    pub fn draw_zombies(&self,ctx:&mut Context)->GameResult<()>{
        for zm in self.zm_pool.iter(){
            if zm.is_used(){
                let type_index=zm.get_type().type_to_index();
                let status_index=zm.get_status().status_to_index();
                zm.draw(ctx,&self.animations[type_index][status_index])?;
            }
        }
        Ok(())
    }

}