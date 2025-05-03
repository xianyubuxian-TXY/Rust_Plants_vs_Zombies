use std::collections::btree_map::Range;

use ggez::{graphics::Image, Context, GameResult};
use rand::Rng;

use crate::{entities::{my_enum::zombie_enum::ZombieStatus, zombie::{self, Zombie}}, tools::{collision, load_animation, update_texture_path}};

use super::map_manager::{self, MapManager};

//zm_pool_size
const ZOMBIEPOOLSIZE:u32=30;
//interval_create_time of zm
const INTERVAL_CREATE_TIME:i32=1500;


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
    zm_timer:i32,
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
            zm_timer:INTERVAL_CREATE_TIME,
            zm_pool:zm_pool,
            animations:animations,
         })
    }

    pub fn create_zombie(&mut self){
        self.zm_timer-=1;
        if self.zm_timer<=0{
            let mut rng=rand::thread_rng();
            self.zm_timer=rng.gen_range(INTERVAL_CREATE_TIME..INTERVAL_CREATE_TIME+500);
            //the num of zm be create every time, no more than three
            //select zms which is unused
            let mut zm_num=rng.gen_range(1..=2); 
            for zombie in self.zm_pool.iter_mut().filter(|zm|!zm.is_used()){
                zombie.init();
                zm_num-=1;
                if zm_num<=0{
                    break;
                }
            }
        }
    }

    pub fn update_zombies_status(&mut self,map_manager:&mut MapManager){
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
                                println!("change status");
                                zombie.change_status();
                            }
                            //如果僵尸处于eat状态且处于“可攻击”，植物受到攻击 (攻击动画帧每次更新时，才能“真正攻击”)
                            if *zombie.get_status()==ZombieStatus::Eat && zombie.can_attack(){
                                plant.be_attacked(zombie.get_damage());
                                //植物死亡，对于grass设置为unused
                                if plant.is_dead(){
                                    grass.set_unused();
                                }
                                zombie.attack_cooldown(); //攻击冷却
                            } 
                        }
                    }
                }
                //如果没有发现攻击目标且处于eat状态-->恢复walk状态（用于植物被僵尸 吃掉）
                if !find_attack_target && *zombie.get_status()==ZombieStatus::Eat{
                    zombie.change_status();
                }
            }
            zombie.update_status();
        }
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