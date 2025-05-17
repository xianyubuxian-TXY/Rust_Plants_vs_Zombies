use std::sync::mpsc;

use ggez::{graphics::{Image, Rect}, Context, GameResult};
use glam::Vec2;
use crate::{entities::{bullet::Bullet, car::Car,grass::Grass, my_enum::{car_enum::CarStatus, plant_enum::PlantType}, plant::Plant, sunshine::Sunshine, zombie:: SCREEN_WIDTH}, threads::audio_thread::AudioEvent};
use crate::tools::collision;
use crate::my_trait::SunshineAction;
use crate::tools::load_animation;
use super::zombie_manager::ZombieManager;

//top_left_position of first grass 
const TOP_LEFT_POSITION:Vec2=Vec2::new(400.0, 140.0);
// const LOWER_RIGHT_POSITION:Vec2=Vec2::new(1560.0, 955.0);
//row_num
const ROW_NUM:u32=5;
//column_num
const COLUMN_NUM:u32=9;
//row_gap
pub const ROW_GAP:f32=160.0;
//column_gap
pub const COLUMN_GAP:f32=130.0;
//size of bullet_pool
const BUTTETS_POOL_SIZE:u32=400;
//size of sunshine_pool
const SUNSHINES_POOL_SIZE:u32=100;

pub struct MapManager{
    pub grasses:Vec<Vec<Grass>>,//草格-->植物（映射）
    bullets_pool:Vec<Bullet>, //子弹缓存池
    sunshines_pool:Vec<Sunshine>, //阳光缓存池
    cars_pool:Vec<Car>, //"车"缓存池
    plants_ainmation:Vec<Vec<Image>>, //植物的动画-->通过”植物类型“转为”索引“获取对于 动画
    bullets_animation:Vec<Image>,  //子弹动画（只有 2帧）
    sunshines_animation:Vec<Image>, //阳光动画
    cars_animation:Vec<Image>, //”车“动画（只有 1帧）
    // peashooter_animation:Vec<Image>,
    // sunflower_animation:Vec<Image>,
    // wallnut_animation:Vec<Image>,
}

impl MapManager{
    pub fn new(ctx:&mut Context)->GameResult<Self>{
        let mut map_to_plant=Vec::new();
        let start_x=TOP_LEFT_POSITION.x;
        let start_y=TOP_LEFT_POSITION.y;
        //创建“草格”
        for i in 0..ROW_NUM{
            let mut row=Vec::new();
            for j in 0..COLUMN_NUM{
                let rect=Rect::new(start_x+(j as f32)*COLUMN_GAP,start_y +(i as f32)*ROW_GAP,ROW_GAP,COLUMN_GAP);
                let draw_pos=Vec2::new(rect.x+COLUMN_GAP/2.0,rect.y+ROW_GAP/2.0);
                let plant=Plant::new(draw_pos,i);
                let grass=Grass::new(plant);
                row.push(grass);
            }
            map_to_plant.push(row);
        }
        //创建”车缓存池”
        let mut cars_pool=Vec::new();
        for i in 0..ROW_NUM{
            let position=Vec2::new(start_x-COLUMN_GAP/2.0,start_y+(i as f32)*ROW_GAP+ROW_GAP/2.0);
            let car=Car::new(position,i);
            cars_pool.push(car);
        }

        //创建“子弹缓存池”
        let mut bullets_pool=Vec::new();
        for _ in 0..BUTTETS_POOL_SIZE{
            let bullet=Bullet::new();
            bullets_pool.push(bullet);
        }
        //创建“阳光缓存池”
        let mut sunshines_pool=Vec::new();
        for _ in 0..SUNSHINES_POOL_SIZE{
            let sunshine=Sunshine::new();
            sunshines_pool.push(sunshine);
        }
        
        let mut plants_animation=Vec::new();
        //加载“豌豆射手“的动画
        let mut peashooter_animation=Vec::new();
        load_animation(ctx, &mut peashooter_animation, 1, 13,"/images/plants/Peashooter/frame/", ".png")?;
        plants_animation.push(peashooter_animation);

        //加载“向日葵”的动画
        let mut sunflowed_animetion=Vec::new();
        load_animation(ctx, &mut sunflowed_animetion,1, 18,"/images/plants/SunFlower/frame/", ".png")?;
        plants_animation.push(sunflowed_animetion);

        //加载”坚果墙“的动画
        let mut wallnut_animation=Vec::new();
        load_animation(ctx, &mut wallnut_animation,1,15,"/images/plants/WallNut/Wallnut_normal/", ".png")?;
        plants_animation.push(wallnut_animation);

        //加载”子弹“的动画
        let mut bullet_animation=Vec::new();
        let image=Image::new(ctx,"/images/bullets/bullet_normal.png")?;
        bullet_animation.push(image);
        let image=Image::new(ctx,"/images/bullets/bullet_blast.png")?;
        bullet_animation.push(image);

        //加载”阳光“的动画
        let mut sunshines_animation=Vec::new();
        load_animation(ctx,&mut sunshines_animation,1 , 29,"/images/sunshine/frame/",".png")?;

        //加载“车”的动画
        let mut cars_animation=Vec::new();
        load_animation(ctx,&mut cars_animation,0, 0,"/images/cars/car",".png")?;

        Ok(MapManager{
            grasses: map_to_plant,
            bullets_pool:bullets_pool,
            sunshines_pool:sunshines_pool,
            cars_pool:cars_pool,
            plants_ainmation:plants_animation,
            bullets_animation:bullet_animation,
            sunshines_animation:sunshines_animation,
            cars_animation:cars_animation,
        })
    }

    pub fn init(&mut self){
        for row in self.grasses.iter_mut(){
            for grass in row.iter_mut(){
                grass.set_unused(); //grass设置unused时，对于植物也会被设置为unused
            }
        }

        for bullet in self.bullets_pool.iter_mut(){
            bullet.set_unused();
        }

        for sunshine in self.sunshines_pool.iter_mut(){
            sunshine.set_unused();
        }
        //直接重新创建车（不想搞了）
        let mut cars_pool=Vec::new();
        let start_x=TOP_LEFT_POSITION.x;
        let start_y=TOP_LEFT_POSITION.y;
        for i in 0..ROW_NUM{
            let position=Vec2::new(start_x-COLUMN_GAP/2.0,start_y+(i as f32)*ROW_GAP+ROW_GAP/2.0);
            let car=Car::new(position,i);
            cars_pool.push(car);
        }
        self.cars_pool=cars_pool;
    }

    //选择“草格”
    pub fn select_grass(&mut self,x:f32,y:f32)->(Option<usize>,Option<usize>){
        let row=(y-TOP_LEFT_POSITION.y)/ROW_GAP;
        let column=(x-TOP_LEFT_POSITION.x)/COLUMN_GAP;
        if row>=0.0 && column>=0.0{
            let row=row as usize;
            let column=column as usize;
            if row<self.grasses.len() && column<self.grasses[row].len(){
                return (Some(row),Some(column));
            }
        } 
        return (None,None)
    } 

    pub fn grow_plant(&mut self,x:f32,y:f32,plant_be_select:&PlantType,audio_sender:&mpsc::Sender<AudioEvent>)->bool{
        //选择“草格”
        if let(Some(row),Some(column))=self.select_grass(x, y)
        {
            let grass=&mut self.grasses[row][column];
            //“草格”未被使用，则种植
            if !grass.is_used(){
                grass.set_used();
                //播放音效
                match audio_sender.send(AudioEvent::PlaySFX("/audio/grow_plant.mp3".to_string())){
                    Err(e)=>{eprintln!("send audio failed:{}",e);},
                    Ok(_)=>{},
                }
                return grass.grow_plant(plant_be_select.clone());
            }
        }
        false
    }

    pub fn remove_plant(&mut self,x:f32,y:f32,audio_sender:&mpsc::Sender<AudioEvent>){
        //选择“草格”
        if let(Some(row),Some(column))=self.select_grass(x, y){
            let grass=&mut self.grasses[row][column];
            //“草格”上中有“植物”，则“铲除”
            if grass.is_used(){
                grass.set_unused();
                //播放音效
                match audio_sender.send(AudioEvent::PlaySFX("/audio/remove_plant.mp3".to_string())){
                    Err(e)=>{eprintln!("send audio failed:{}",e);},
                    Ok(_)=>{},
                }
            }
        }
    }

    pub fn update_plants_status(&mut self,zombies_manager:&ZombieManager){
        for row in self.grasses.iter_mut(){
            for grass in row.iter_mut(){
                if grass.is_used(){
                   let plant=&mut grass.plant;
                   plant.update_status(); //植物更新状态
                   //能够发送技能
                   if plant.can_activate_skill(){
                        match plant.get_type() {
                            PlantType::Peashooter=>{
                                //找出存活的僵尸的row(且zm.x>plant.x 且 僵尸出现在屏幕中),用于判断哪一行的豌豆射手需要发射子弹
                                if let Some(_)=zombies_manager.zm_pool.iter().find(|zombie| zombie.is_used()&&!zombie.is_dead()
                                &&zombie.get_row()==plant.get_row()&&zombie.get_position().x>plant.get_position().x &&zombie.get_position().x<SCREEN_WIDTH+100.0){
                                    //找到未使用的子弹，发射子弹
                                    if let Some(bullet)=self.bullets_pool.iter_mut().find(|bullet| !bullet.is_used()){
                                        plant.peashooter_shoot_bullet(bullet);
                                    }
                                    //技能冷却
                                    plant.skill_cooldown();
                                }
                            }
                            PlantType::SunFlower=>{
                                //找到第一个未使用的阳光，生成它
                                if let Some(sunshine)=self.sunshines_pool.iter_mut().find(|sunshine| !sunshine.is_used()){
                                    plant.sunflower_create_sunshine(sunshine);
                                }
                                //技能冷却
                                plant.skill_cooldown();
                            }
                            _=>{},
                        }
                   }
                }
            }
        }
    }

    pub fn draw_plants(&self,ctx:&mut Context)->GameResult<()>{
        for row in self.grasses.iter(){
            for grass in row.iter(){
                if grass.is_used(){
                    if let Some(animation_index)=grass.get_plant_type().type_to_index(){
                        grass.draw_plant(ctx, &self.plants_ainmation[animation_index])?;  
                    }
                }
            }
        }
        Ok(())
    }


    pub fn undate_bullets_status(&mut self,zombies_manager:&mut ZombieManager,audio_sender:&mpsc::Sender<AudioEvent>){
        for bullet in self.bullets_pool.iter_mut(){
            if bullet.is_used(){
                //更新子弹状态
                bullet.update_status();
                //子弹处于未死亡状态，能对僵尸造成伤害
                if !bullet.is_dead(){
                    for zombie in zombies_manager.zm_pool.iter_mut(){
                        //僵尸处于使用状态且未死亡状态（死亡后不能再被子弹打中） 且 与子弹处于同一行
                        if zombie.is_used() && !zombie.is_dead() && zombie.get_row()==bullet.get_row(){
                            //检测是否发生碰撞
                            if collision(bullet.get_position(),zombie.get_position()){
                                //播放音效
                                match audio_sender.send(AudioEvent::PlaySFX("/audio/bullet_zombie.mp3".to_string())){
                                    Err(e)=>{eprintln!("send audio failed:{}",e);},
                                    Ok(_)=>{},
                                }
                                //子弹进入死亡状态
                                bullet.become_dead_status();
                                //僵尸受到伤害
                                zombie.be_attacked(bullet.get_damage());
                                break; //退出循环：一个子弹一次只能攻击一只僵尸
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn draw_bullets(&self,ctx:&mut Context)->GameResult<()>{
        for bullet in self.bullets_pool.iter(){
            if bullet.is_used(){
                bullet.draw(ctx, &self.bullets_animation)?;
            }
        }
        Ok(())
    }

    pub fn update_cars_status(&mut self,zombies_manager:&mut ZombieManager){
        for car in self.cars_pool.iter_mut(){
            if car.is_used(){
                //遍历僵尸，看是否与car发生碰撞
                for zombie in zombies_manager.zm_pool.iter_mut(){
                    if zombie.is_used()&&!zombie.is_dead()&&zombie.get_row()==car.get_row(){
                        //检测碰撞
                        if collision(car.get_position(),zombie.get_position()){
                            let car_status=car.get_status();
                            match car_status{
                                CarStatus::Stopping=>{
                                    //从stopping->running
                                    car.become_running();
                                },
                                _=>{},
                            }
                            zombie.be_attacked(1000.0); //设置收到1000点伤害（1000> 所有僵尸的最大血量）
                        }
                    }
                }
                car.update();
            }
        }
    }

    pub fn draw_cars(&self,ctx:&mut Context)->GameResult<()>{
        for car in self.cars_pool.iter(){
            if car.is_used(){
                car.draw(ctx,&self.cars_animation)?;
            }
        }
        Ok(())
    }
}

impl SunshineAction for MapManager {
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