use std::{collections::HashMap,};

use ggez::{graphics::{Image, Rect}, mint::Point2, Context, GameResult};

use crate::entities::{card::Card, my_enum::card_enum::CardType};
use crate::tools::mydraw;

//“铲子”的Rect
pub const SPADE_RECT:Rect=Rect::new(1080.0, 0.0, 85.0, 120.0);
//"植物卡槽"的Rect
const PLANT_BAR_RECT:Rect=Rect::new(450.0,0.0,632.0,122.0);
//卡片间隔
const CARDS_GRAP:f32=87.0;
//卡片的width和height
const CARDS_WIDTH:f32=85.0;
const CARDS_HEIGHT:f32=110.0;

fn get_rect(bar_x:f32,card_id:u32)->Rect{
    Rect::new(bar_x+CARDS_GRAP*(card_id as f32+1.0),5.0,CARDS_WIDTH,CARDS_HEIGHT)
}

fn load_card(ctx:&mut Context,plants_cards:&mut Vec<Card>,cards_hash_map:&mut HashMap<String,Image>,card_type:CardType,card_name:String,image_rect:Rect,card_image_path:&str,card_entity_image_path:&str)->GameResult<()>{
    
    //because use card_name tow times,  clone it in first time
    let card=Card::new(ctx,card_type,card_name.clone(),image_rect,card_image_path)?;
    let card_entity_image=Image::new(ctx,card_entity_image_path)?;
    plants_cards.push(card);
    cards_hash_map.insert(card_name,card_entity_image);
    Ok(())
}

pub struct CardManager{
    cards:Vec<Card>, //卡片集合
    plants_cards_bar:Image, //“植物卡片槽”图片
    spade_slot:Image, //“铲子槽”图片
    cards_entities_images:HashMap<String,Image>, //卡片”对应实体“的图片，通过“卡片名”从hash_map中获取
    card_be_select:CardType, //被选择的”卡片类型“，通过”卡片类型“作为下标从cards”卡片集合“中获取对于卡片，进而获取”卡片名“
}

impl CardManager{
    pub fn new(ctx:&mut Context)->GameResult<Self>
    {
        let mut cards_hash_map=HashMap::new();
        let mut plants_cards=Vec::new();
        //加载“植物卡片槽”图片
        let bar_image=Image::new(ctx,"/images/background/plant_bar.png")?;
        //加载“铲子槽”图片
        let spade_image=Image::new(ctx,"/images/background/shovelSlot.png")?;
//加载卡片
        //加载“铲子卡片”
        let spade_rect=SPADE_RECT.clone();
        load_card(ctx, &mut plants_cards, &mut cards_hash_map,CardType::SpadeCard,"spade_card".to_string(), spade_rect,"/images/cards/spade.png", "/images/cards/spade.png")?;
    
    //加载“植物卡片”到“植物卡槽”中
        let bar_x=PLANT_BAR_RECT.x;
        let mut card_id:u32=0; //order of cards
        //加载”豌豆射手卡片”
        let peashooter_rect=get_rect(bar_x, card_id);
        load_card(ctx, &mut plants_cards,&mut cards_hash_map, CardType::PeashooterCard, "peashooter_card".to_string(),peashooter_rect,"/images/cards/peashooter.png", "/images/plants/Peashooter/frame/1.png")?;
        card_id+=1;
        //加载”向日葵卡片“
        let sunshine_rect=get_rect(bar_x, card_id);
        load_card(ctx, &mut plants_cards, &mut cards_hash_map, CardType::SunFlowerCard,"sunshine_card".to_string(), sunshine_rect,"/images/cards/sunflower.png","/images/plants/SunFlower/frame/1.png")?;
        card_id+=1;
        //加载“见过墙卡片”
        let wallnut_rect=get_rect(bar_x, card_id);
        load_card(ctx, &mut plants_cards, &mut cards_hash_map, CardType::WallnutCard,"wallnut_card".to_string(), wallnut_rect,"/images/cards/wallnut.png", "/images/plants/WallNut/Wallnut_normal/0.png")?;
        

        Ok(
            CardManager{
                cards:plants_cards,
                plants_cards_bar:bar_image,
                spade_slot:spade_image,
                cards_entities_images:cards_hash_map,
                card_be_select:CardType::NoneCard,
            }
        )
    }
    //检查是否选择“植物”，返回选中的“植物类型”
    pub fn check_select_plant(&mut self,x:f32,y:f32)->&CardType{
        for card in self.cards.iter_mut(){
            if card.be_selected(x, y){
                //设置被选中的“植物类型”
                match card.get_type() {
                    CardType::PeashooterCard=>self.card_be_select=CardType::PeashooterCard,
                    CardType::SunFlowerCard=>self.card_be_select=CardType::SunFlowerCard,
                    CardType::WallnutCard=>self.card_be_select=CardType::WallnutCard,
                    CardType::SpadeCard=>self.card_be_select=CardType::SpadeCard,
                    CardType::NoneCard=>self.card_be_select=CardType::NoneCard,
                }
                //返回选择的”卡片类型“
                return &self.card_be_select;
            }
        }
        self.card_be_select=CardType::NoneCard;
        &self.card_be_select
    }

    //重置选中的"卡片类型"
    pub fn reset_plant_selected(&mut self){
        self.card_be_select=CardType::NoneCard;
    }

    //绘制被选择的卡片的“实体”图片-->图片跟随“鼠标”移动
    pub fn draw_plant_entity_be_selected_follow_mouse(&self,ctx:&mut Context,mouse_position:Point2<f32>)->GameResult<()>{
        //通过“卡片类型”获取“下标”，进而获取“卡片”
        if let Some(index)=self.card_be_select.type_to_index(){
            //获取“卡片名”
            let plant_name=self.cards[index].get_name();
            //通过“卡片名”为“键”，从hash_map中获取“卡片实体图片”
            let image=self.cards_entities_images.get(plant_name).expect("get plant failed");
            //设置绘制位置
            let x=mouse_position.x-(image.width()/2) as f32;
            let y=mouse_position.y-(image.height()/2) as f32; 
            //绘制跟随鼠标的”卡片实体“
            mydraw(ctx, image,x, y, image.width() as f32, image.height() as f32)?;
        }
        Ok(())
    }

    pub fn draw(&self,ctx:&mut Context)->GameResult<()>{
        //绘制“卡片槽”
        mydraw(ctx, &self.plants_cards_bar, PLANT_BAR_RECT.x, PLANT_BAR_RECT.y, PLANT_BAR_RECT.w, PLANT_BAR_RECT.h)?;
        //绘制“铲子槽”
        mydraw(ctx, &self.spade_slot, SPADE_RECT.x, SPADE_RECT.y, SPADE_RECT.w, SPADE_RECT.h)?;
        //绘制卡片
        for card in self.cards.iter(){
            let rect=card.get_rect();
            //“铲子”被选择时，不在绘制“铲子”的图片（跳过）
            if self.card_be_select==CardType::SpadeCard && card.card_type==CardType::SpadeCard{
                continue;
            }
            mydraw(ctx, card.get_image(), rect.x, rect.y, rect.w, rect.h)?;
        }
        Ok(())
    }
}