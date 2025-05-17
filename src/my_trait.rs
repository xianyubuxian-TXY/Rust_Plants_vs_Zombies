use std::sync::mpsc;
use ggez::graphics::Image;
use ggez::{Context, GameResult};

use crate::entities::sunshine::Sunshine;
use crate::threads::audio_thread::AudioEvent;

pub trait SunshineAction {
    // 返回可变引用（用于修改阳光）
    fn get_sunshines_pool_mut(&mut self) -> &mut Vec<Sunshine>;
    
    // 返回不可变引用（用于绘制和只读操作）
    fn get_sunshines_pool(&self) -> &Vec<Sunshine>;
    
    fn get_sunshines_animation(&self) -> &Vec<Image>;

    // 检查阳光点击（需要修改阳光状态）
    fn sunshines_check_click(&mut self, x: f32, y: f32,audio_send:&mpsc::Sender<AudioEvent>){
        for sunshine in self.get_sunshines_pool_mut().iter_mut() {
            if sunshine.is_used() {
                //被点击，播放音效
                if sunshine.check_clicked(x, y){
                    match audio_send.send(AudioEvent::PlaySFX("/audio/click_sunshine.mp3".to_string())){
                        Err(e)=>{eprintln!("send audio sunshine.mp3 failed:{}",e);},
                        Ok(_)=>{},
                    }
                }
            }
        }
    }

    // 更新阳光状态（需要修改阳光状态）
    fn update_sunshines_status(&mut self) -> u32 {
        let mut sunshine_value = 0;
        for sunshine in self.get_sunshines_pool_mut().iter_mut() {
            if sunshine.is_used() {
                sunshine.update_status();
                if !sunshine.is_used() && sunshine.be_clicked {
                    sunshine_value += 50;
                }
            }
        }
        sunshine_value
    }

    // 绘制阳光（只需要读取数据）
    fn draw_sunshines(&self, ctx: &mut Context) -> GameResult<()> {
        for sunshine in self.get_sunshines_pool().iter() {
            if sunshine.is_used() {
                sunshine.draw(ctx, self.get_sunshines_animation())?;
            }
        }
        Ok(())
    }
}


