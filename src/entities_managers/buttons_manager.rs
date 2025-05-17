use std::sync::mpsc;

use ggez::graphics::Image;
use ggez::{Context, GameResult};


use crate::game::{GameMod, GamePage};
use crate::entities::button::Button;
use crate::entities::my_enum::button_enum::ButtonType;
use crate::threads::audio_thread::AudioEvent;


//850.0, 200.0 , 550.0 , 240.0 : start_button
pub struct ButtonManager{
    game_start_button:Button,
    game_hard_mod_button:Button,
    game_pause_button:Button,
    game_run_button:Button,
    game_restart_button:Button,
    game_back_button:Button,
    button_be_clicked:ButtonType, //记录被点击的按钮
}

impl ButtonManager{
    pub fn new(ctx:&mut Context)->GameResult<Self>{
    //游戏开始按钮
        let mut start_button_images=Vec::new();
        start_button_images.push(Image::new(ctx, "/images/buttons/start_button_0.png")?); //没被按下时的图片
        start_button_images.push(Image::new(ctx, "/images/buttons/start_button_1.png")?); //被按下时的图片
        let start_button=Button::new(ButtonType::GameStart,850.0, 200.0 , 550.0 , 240.0,start_button_images)?;
    //加载困难模式按钮
        let mut game_hard_mod_button_images=Vec::new();
        game_hard_mod_button_images.push(Image::new(ctx,"/images/buttons/hard_mod_button_0.png")?);
        game_hard_mod_button_images.push(Image::new(ctx,"/images/buttons/hard_mod_button_1.png")?);
        let hard_mod_button=Button::new(ButtonType::GameStart,850.0,450.0,550.0,240.0,game_hard_mod_button_images)?;
    //游戏暂停按钮
        let mut pause_button_images=Vec::new();
        //按下和不按下图片就一样吧
        pause_button_images.push(Image::new(ctx,"/images/buttons/pause_button.png")?);
        pause_button_images.push(Image::new(ctx,"/images/buttons/pause_button.png")?);
        let pause_button=Button::new(ButtonType::GamePause, 0.0, 0.0, 100.0, 100.0, pause_button_images)?;
    //游戏继续运行按钮
        let mut run_button_images=Vec::new();
        //按下和不按下图片就一样吧
        run_button_images.push(Image::new(ctx,"/images/buttons/run_button.png")?);
        run_button_images.push(Image::new(ctx,"/images/buttons/run_button.png")?);
        let run_button=Button::new(ButtonType::GamePlaying, 120.0, 0.0, 100.0, 100.0, run_button_images)?;
    //游戏重新开始按钮
        //按下和不按下图片就一样吧
        let mut restart_button_images=Vec::new();
        restart_button_images.push(Image::new(ctx,"/images/buttons/restart_button.png")?);
        restart_button_images.push(Image::new(ctx,"/images/buttons/restart_button.png")?);
        let restart_button=Button::new(ButtonType::GamePlaying, 0.0, 120.0, 100.0, 100.0, restart_button_images)?;
    //游戏返回按钮
        let mut back_button_images=Vec::new();
        back_button_images.push(Image::new(ctx,"/images/buttons/back_button.png")?);
        back_button_images.push(Image::new(ctx,"/images/buttons/back_button.png")?);
        let back_button=Button::new(ButtonType::GameBack,120.0,120.0,100.0,100.0,back_button_images)?;
        Ok(ButtonManager{
            game_start_button:start_button,
            game_hard_mod_button:hard_mod_button,
            game_pause_button:pause_button,
            game_run_button:run_button,
            game_restart_button:restart_button,
            game_back_button:back_button,
            button_be_clicked:ButtonType::None,
        })
    }

    pub fn init(&mut self){
        self.game_pause_button.init();
        self.game_run_button.init();
        self.game_restart_button.init();
    }

    pub fn buttons_check_click(&mut self,x:f32,y:f32,game_page:&GamePage,game_mod:&mut GameMod)
    {
        match game_page {
            GamePage::StartPage=>{
                //if this button be clicked, sete self.button_be_clicked=this button
                if self.game_start_button.check_click(x, y){
                    self.button_be_clicked=ButtonType::GameStart;
                    *game_mod=GameMod::Common;
                }
                else if self.game_hard_mod_button.check_click(x, y){
                    self.button_be_clicked=ButtonType::GameStart;
                    *game_mod=GameMod::Hard;
                }
            }
            GamePage::PlayPage=>{
                if self.game_pause_button.check_click(x, y){
                    self.button_be_clicked=ButtonType::GamePause;
                }
                else if self.game_run_button.check_click(x, y){
                    self.button_be_clicked=ButtonType::GamePlaying;
                }
                else if self.game_restart_button.check_click(x, y){
                    self.button_be_clicked=ButtonType::GameRestart;
                }
                else if self.game_back_button.check_click(x, y){
                    self.button_be_clicked=ButtonType::GameBack;
                }
            },
        }
    }

    //if have button be clicked, set button up and return type of it, else return ButtonType::None
    pub fn check_set_button_up(&mut self,audio_sender:&mpsc::Sender<AudioEvent>)->ButtonType
    {
        let mut be_clicked_button=ButtonType::None;
        //看是否有按钮被点击
        match self.button_be_clicked{
            ButtonType::GameStart=>{
                //开始按钮被点击
                self.game_start_button.set_up(); //set button up
                self.game_hard_mod_button.set_up();
                be_clicked_button=ButtonType::GameStart;
            },
            ButtonType::GamePause=>{
                self.game_pause_button.set_up();
                be_clicked_button=ButtonType::GamePause;
            },
            ButtonType::GamePlaying=>{
                self.game_run_button.set_up();
                be_clicked_button=ButtonType::GamePlaying;
            },
            ButtonType::GameRestart=>{
                self.game_restart_button.set_up();
                be_clicked_button=ButtonType::GameRestart;
            },
            ButtonType::GameBack=>{
                self.game_back_button.set_up();
                be_clicked_button=ButtonType::GameBack;
            }
            ButtonType::None=>{},
        }
        //有按钮被点击，播放音效
        if be_clicked_button!=ButtonType::None{
            match audio_sender.send(AudioEvent::PlaySFX("/audio/click_button.mp3".to_string())){
                Err(e)=>{eprintln!("send audio failed:{}",e);},
                Ok(_)=>{},
            }
        }
        //重置被点击的按钮为ButtonType::None
        self.button_be_clicked=ButtonType::None;
        be_clicked_button
    }

    pub fn draw_buttons(&mut self,ctx:&mut Context,game_page:&GamePage)->GameResult<()>{
        match game_page{
            GamePage::StartPage=>{
                self.game_start_button.draw(ctx)?;
                self.game_hard_mod_button.draw(ctx)?;
            },
            GamePage::PlayPage=>{
                self.game_pause_button.draw(ctx)?;
                self.game_run_button.draw(ctx)?;
                self.game_restart_button.draw(ctx)?;
                self.game_back_button.draw(ctx)?;
            }
        }
        Ok(())
    }
}
