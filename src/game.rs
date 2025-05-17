use ggez::input::mouse;

use ggez::{graphics, Context, GameError, GameResult};
use ggez::graphics::{Color, Font, Image, Text};
//EventHandler related
use ggez::event::{EventHandler, KeyCode, KeyMods};
use glam::Vec2;

//sleep but don't block current thread
use async_std::task::{self};
use std::{error, thread};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

use crate::entities::my_enum::button_enum::ButtonType;
use crate::entities::my_enum::card_enum::CardType;
use crate::entities_managers::zombie_manager::ZombieManager;
use crate::my_trait::SunshineAction;
use crate::entities_managers::background_manager::ResourceManager;
use crate::entities_managers::buttons_manager::ButtonManager;
use crate::entities_managers::map_manager::{self, MapManager};
use crate::entities_managers::sunshines_manager::SunshineManager;
use crate::entities_managers::cards_manager::CardManager;
use crate::threads::audio_thread::{load_audio, AudioEvent};
use crate::tools::mydraw;

//僵尸总共的wave数：3
const ZM_WAVES_TOTAL:u32=3;
//失败线
const LOSE_X:f32=325.0;

#[derive(PartialEq)]
pub enum GameStatus {
    Menu,
    Playing,
    Paused,
    GameOver,
    Victory,
}

impl GameStatus {
    pub fn status_to_index(&self)->Option<usize>{
        match self{
            Self::GameOver=>Some(0),
            Self::Victory=>Some(1),
            _=>None,
        }
    }
}

#[derive(PartialEq)]
pub enum GamePage {
    StartPage,
    PlayPage,
}
#[derive(PartialEq)]
pub enum GameMod{
    Common,
    Hard,
}

impl GameMod {
    pub fn mod_to_num(&self)->u32{
        match self{
            GameMod::Common=>1,
            GameMod::Hard=>2,
        }
    }
}


pub struct Game{
    status:GameStatus,
    page:GamePage,
    game_mod:GameMod,
    sunshines_value:u32, //阳光值
    cur_level:u32, //当前游戏进度
    card_be_selected:CardType,  //选择的卡片
    whether_play_bg_audio:bool, //是否播放背景音乐
    cur_bg_audio_page:GamePage, //当前播放的背景音乐对应的背景
    status_images:Vec<Image>, //不同游戏状态的一些图片
    buttons_manager:ButtonManager,
    cards_manager:CardManager,
    resource_manager: Arc<ResourceManager>,
    sunshines_manager:Arc<Mutex<SunshineManager>>,
    map_manager:Arc<Mutex<MapManager>>, //manager grass,plant,cars,tools and so on
    zombies_manager:Arc<Mutex<ZombieManager>>,
    audio_sender:Arc<mpsc::Sender<AudioEvent>>,
}

impl Game {
    pub fn new(ctx:&mut Context,audio_sender: mpsc::Sender<AudioEvent>)->Result<Game, GameError>{
        
        //加载音效资源
        load_audio(ctx,&[
            "/audio/start_bg.MP3", //start_page 背景音乐
            "/audio/play_bg.mp3", //play_page 背景音效
            "/audio/pause.mp3", //暂停
            "/audio/click_button.mp3",//按钮点击音效
            "/audio/click_sunshine.mp3",//阳光点击音效
            "/audio/click_shovel.mp3",//点击铲子
            "/audio/remove_plant.mp3", //铲除植物
            "/audio/grow_plant.mp3",//种植植物
            "/audio/bullet_zombie.mp3",//子弹与僵尸碰撞
            "/audio/zombie_eat.mp3",//僵尸吃植物
            "/audio/first_wave.mp3",//第一波僵尸来临音效
            "/audio/second_wave.mp3",//第二波僵尸来临音效
            "/audio/final_wave.mp3",//第三波僵尸来临音效
            "/audio/win.mp3", //胜利音效
            "/audio/scream.mp3", //尖叫声
            "/audio/lose.mp3",//失败音效
        ])?;
        audio_sender.send(AudioEvent::PlayBGM("/audio/start_bg.MP3".to_string(),true)).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
        //加载图片
        let mut status_images=Vec::new();
        let image=Image::new(ctx,"/images/background/lose.png")?;
        status_images.push(image);
        let image=Image::new(ctx,"/images/background/victory_trophy.png")?;
        status_images.push(image);  

        let game=Game{
            status:GameStatus::Menu,
            page:GamePage::StartPage,
            game_mod:GameMod::Common,
            sunshines_value:50,
            cur_level:1,
            card_be_selected:CardType::NoneCard,
            whether_play_bg_audio:true,
            cur_bg_audio_page:GamePage::StartPage,
            status_images,
            buttons_manager:ButtonManager::new(ctx)?,
            cards_manager:CardManager::new(ctx)?,
            resource_manager:Arc::new(ResourceManager::new(ctx)?),
            sunshines_manager:Arc::new(Mutex::new(SunshineManager::new(ctx)?)),
            map_manager:Arc::new(Mutex::new(MapManager::new(ctx)?)),
            zombies_manager:Arc::new(Mutex::new(ZombieManager::new(ctx)?)),
            audio_sender:Arc::new(audio_sender),
        };
        Ok(game)
    }

    pub fn game_restart(&mut self,page:GamePage,status:GameStatus)->Result<(),GameError>{
        self.page=page;
        self.status=status;
        self.sunshines_value=50;
        self.cur_level=1;
        self.card_be_selected=CardType::NoneCard;
        self.whether_play_bg_audio=true;
        self.buttons_manager.init();
        self.cards_manager.init();
        let mut sunshines_manager=self.sunshines_manager.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
        sunshines_manager.init();
        let mut map_manager=self.map_manager.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
        map_manager.init();
        let mut zombie_manager=self.zombies_manager.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
        zombie_manager.init();
        //停止播放音效 
        match self.page{
            GamePage::PlayPage=>{
                //重新播放背景音乐
                self.audio_sender.send(AudioEvent::StopBGM).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                self.audio_sender.send(AudioEvent::PlayBGM("/audio/play_bg.mp3".to_string(),true)).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
            },
            _=>{},
        }
        Ok(())
    }

    pub fn change_game_status(&mut self,game_status:GameStatus)->Result<(),GameError>{
        match game_status{
            GameStatus::Paused =>{
                self.audio_sender.send(AudioEvent::PlaySFX("/audio/pause.mp3".to_string())).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                 //停止播放音效 
                self.audio_sender.send(AudioEvent::StopBGM).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                self.whether_play_bg_audio=false;
            },
            GameStatus::Playing =>{
                //播放背景音乐
                self.audio_sender.send(AudioEvent::PlayBGM("/audio/play_bg.mp3".to_string(),true)).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                self.whether_play_bg_audio=true;
            },
            GameStatus::Victory=>{
                self.audio_sender.send(AudioEvent::StopBGM).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?; //关闭背景音乐
                self.audio_sender.send(AudioEvent::PlaySFX("/audio/win.mp3".to_string())).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?; //播放胜利音乐
            },
            GameStatus::GameOver=>{
                self.audio_sender.send(AudioEvent::StopBGM).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?; //关闭背景音乐
                // task::block_on(async {
                //     task::sleep(Duration::from_micros(3000)).await;
                //     self.audio_sender.send(AudioEvent::PlaySFX("/audio/scream.mp3".to_string())).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                // });
                self.audio_sender.send(AudioEvent::PlaySFX("/audio/scream.mp3".to_string())).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                std::thread::sleep(Duration::from_secs(1));
                self.audio_sender.send(AudioEvent::PlaySFX("/audio/lose.mp3".to_string())).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?; //播放失败音乐
            },
            _ => {},
        }
        self.status = game_status;
        Ok(())
    }

    pub fn set_game_status(&mut self,stauts:GameStatus){
        self.status=stauts;
    }

    pub fn set_game_page(&mut self,page:GamePage){
        self.page=page;
    }

    pub fn add_sunshine_value(&mut self,sunshine_value:u32) {
        self.sunshines_value+=sunshine_value;
    }

    pub fn write_game_data(&self,ctx:&mut Context)->GameResult<()>{
        //阳光值
        let font = Font::default();
        let sunshine_value_text=Text::new((format!("{}",self.sunshines_value),font,30.0));
        let position=Vec2::new(470.0,90.0);
        let color=Color::from_rgb(0, 0, 0);
        graphics::draw(ctx, &sunshine_value_text,(position,color,))?;
        //关卡进度
        let font = Font::default();
        let levels_text=Text::new((format!("Levels {}—3",self.cur_level),font,60.0));
        let position=Vec2::new(1200.0,950.0);
        let color=Color::from_rgb(168, 44, 0);
        graphics::draw(ctx,&levels_text,(position,color,))?;
        Ok(())
    }



}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
        match self.page{
            GamePage::StartPage=>{
                //播放start_page的背景音乐
                if self.whether_play_bg_audio && self.cur_bg_audio_page!=GamePage::StartPage{
                    self.audio_sender.send(AudioEvent::PlayBGM("/audio/start_bg.MP3".to_string(),true)).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                    self.cur_bg_audio_page=GamePage::StartPage; //更新播放音乐的背景
                }
            },
            GamePage::PlayPage=>{
                //播放play_page的背景音乐
                if self.whether_play_bg_audio && self.cur_bg_audio_page!=GamePage::PlayPage{
                    self.audio_sender.send(AudioEvent::PlayBGM("/audio/play_bg.mp3".to_string(),true)).map_err(|e| ggez::GameError::AudioError(format!("Channel send error: {}", e)))?;
                    self.cur_bg_audio_page=GamePage::PlayPage;//更新播放音乐的背景
                }
            }
        };

        match self.status{
            GameStatus::Playing=>{
            let mut thread_handles=vec![];
            //---------------------------sunshine_thread-------------------------------------------
                //“通道”：用于在“阳光子线程”与“主线程”之间传递 收集的阳光值
                let (tx, rx) = mpsc::channel();
                let sunshine_manager_clone = self.sunshines_manager.clone();
                thread_handles.push(thread::spawn(move ||->Result<(),GameError> {
                    let mut sunshine_manager = sunshine_manager_clone.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                    //随机生成阳光
                    sunshine_manager.create_sunshine();
                    //更新阳光状态
                    let value = sunshine_manager.update_sunshines_status();
                     //将用户收集的阳光值发送到主线程
                     tx.send(value).map_err(|e|GameError::EventLoopError(format!("send sunshine's value failed: {}",e)))?;
                     Ok(())
                }));
            //---------------------------------------------------------------------------------------

            //---------------------------map_thread (include plant)----------------------------------
                let (tx1, rx1) = mpsc::channel();
                let map_manager_clone=self.map_manager.clone();
                let zombie_manager_clone=self.zombies_manager.clone();
                let audio_sender=self.audio_sender.clone();
                thread_handles.push(thread::spawn(move||->Result<(),GameError>{
                    let mut map_manager=map_manager_clone.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                    let mut zombie_manager=zombie_manager_clone.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                    //更新植物状态
                    map_manager.update_plants_status(&zombie_manager);
                    //更新子弹
                    map_manager.undate_bullets_status(&mut zombie_manager,&audio_sender);
                    //更新车状态
                    map_manager.update_cars_status(&mut zombie_manager);
                    //更新阳光状态： 获取用户收取的阳光值，并传给主线程
                    let value=map_manager.update_sunshines_status();
                    tx1.send(value).map_err(|e|GameError::EventLoopError(format!("send sunshine's value failed: {}",e)))?;
                    Ok(())
                }));
            //---------------------------------------------------------------------------------------
            //----------------------------------zombie_thread-----------------------------------------
                let (tx2, rx2) = mpsc::channel();
                let zombie_manager_clone=self.zombies_manager.clone();
                let map_manager_clone=self.map_manager.clone();
                let audio_sender=self.audio_sender.clone();
                thread_handles.push(thread::spawn(move||->Result<(),GameError>{
                    let mut zombie_manager=zombie_manager_clone.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                    let mut map_manager=map_manager_clone.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                    //随机创建僵尸
                    let cur_level=zombie_manager.create_zombie(&audio_sender);
                    //更新僵尸状态
                    let min_x=zombie_manager.update_zombies_status(&mut map_manager,&audio_sender);
                    tx2.send((cur_level,min_x)).map_err(|e|GameError::EventLoopError(format!("send cur_level failed: {}",e)))?; //发送当前关卡到主线程
                    Ok(())
                }));

            //---------------------------------------------------------------------------------------

            //-----------------------------------main_thread -----------------------------------------
                for handle in thread_handles {
                    match handle.join() {
                        Ok(Ok(())) => {}, // 线程成功执行且没有错误
                        Ok(Err(e)) => return Err(e), // 线程返回了错误
                        Err(e) => return Err(GameError::EventLoopError(format!("Thread panicked: {:?}", e))),
                    }
                }
                //接收用户收集的阳光值（随机产生的阳光、植物产生的阳光）
                if let Ok(value) = rx.try_recv() {
                    self.add_sunshine_value(value);
                }
                if let Ok(value)=rx1.try_recv(){
                    self.add_sunshine_value(value);
                }
                //接收当前关卡
                if let Ok((cur_level,min_x))=rx2.recv(){
                    //有僵尸跨过“失败线”，游戏失败
                    if min_x<LOSE_X{    
                        self.change_game_status(GameStatus::GameOver)?;
                    }
                    //关卡不相等，则更新关卡
                    else if cur_level!=self.cur_level{
                        self.cur_level=cur_level;
                        //当cur_level超过僵尸总wave数，则游戏胜利
                        if self.cur_level>ZM_WAVES_TOTAL{
                            self.change_game_status(GameStatus::Victory)?;
                        }
                    }
                }
            }
            _=>{}
        };

        Ok(())
    }

    /*
    DrawParam:scale , dest , color ,src
    */
    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, [0.0, 0.0, 0.0,0.0].into());
        match self.page{

            GamePage::StartPage=>{
                //绘制背景
                self.resource_manager.draw_start_page_background(ctx)?;
                //绘制开始按钮
                self.buttons_manager.draw_buttons(ctx,&self.page)?;
                
            },

            GamePage::PlayPage=>{
                //绘制背景
                self.resource_manager.draw_playing_page_background(ctx)?;
                //绘制按钮
                self.buttons_manager.draw_buttons(ctx,&self.page)?;
                //绘制“卡片”
                self.cards_manager.draw(ctx)?;
                //写 “阳光值”
                self.write_game_data(ctx)?;
                //绘制阳光
                let sunshine_manager=self.sunshines_manager.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                sunshine_manager.draw_sunshines(ctx)?;
                //绘制“用户”选择的“卡片实体”
                match self.card_be_selected{
                    CardType::NoneCard=>{},
                    _=>{
                        self.cards_manager.draw_plant_entity_be_selected_follow_mouse(ctx,mouse::position(ctx))?;
                    }
                }
                //绘制草地上的植物
                let map_manager=self.map_manager.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                map_manager.draw_plants(ctx)?;
                //绘制植物产生的太阳
                map_manager.draw_bullets(ctx)?;
                map_manager.draw_sunshines(ctx)?;
                //绘制车
                map_manager.draw_cars(ctx)?;
                //绘制僵尸
                let zombie_manager=self.zombies_manager.lock().map_err(|e|GameError::EventLoopError(format!("mutex failed:{}",e)))?;
                zombie_manager.draw_zombies(ctx)?;
                
                //绘制失败/胜利画面
                if let Some(index)=self.status.status_to_index(){
                    // task::block_on(async {
                    //     task::sleep(Duration::from_micros(1000)).await;
                    //     mydraw(ctx,&self.status_images[index],600.0,300.0, 400.0, 400.0);
                    // })?;
                    thread::sleep(Duration::from_secs(2));
                    mydraw(ctx,&self.status_images[index],600.0,300.0, 400.0, 400.0)?;
                }
            },
        }
        graphics::present(ctx)

    }
    
    //game will ues it by self
    fn mouse_button_down_event(&mut self,ctx: &mut Context,button: ggez::event::MouseButton,x: f32,y: f32){
        if button==mouse::MouseButton::Left{
            match self.page{
                GamePage::StartPage=>{
                    //检查“按钮”是否被点击，并通过button_manager记录
                    self.buttons_manager.buttons_check_click(x, y,&self.page,&mut self.game_mod);
                    if self.game_mod==GameMod::Hard{
                        // let mut zombie_manager=self.zombies_manager.lock().expect("mutex lock failed");
                        // zombie_manager.set_game_mod(GameMod::Hard);
                        match self.zombies_manager.lock() {
                            Ok(mut zombie_manager) => {
                                zombie_manager.set_game_mod(GameMod::Hard);
                            }
                            Err(e) => {
                                eprintln!("Failed to lock zombies_manager: {}", e);
                            }
                        }
                    }
                }
                GamePage::PlayPage=>{
                    match self.status {
                        GameStatus::Playing=>{
                        //检查是否有按钮被按下，并通过button_manager记录
                        self.buttons_manager.buttons_check_click(x, y,&self.page,&mut self.game_mod);
                        //-----------------------------sunshine---------------------------
                            //检测是否有阳光被“点击”（随机生成的阳光、向日葵生成的阳光）
                            // let mut sunshine_manager=self.sunshines_manager.lock().expect("mutex lock failed");
                            // sunshine_manager.sunshines_check_click(x, y,&self.audio_sender);
                            match self.sunshines_manager.lock(){
                                Ok(mut sunshine_manager)=>{
                                    sunshine_manager.sunshines_check_click(x, y, &self.audio_sender);
                                },
                                Err(e)=>{
                                    eprintln!("lock sunshine_manager failed:{}",e);
                                }
                            }
                            // let mut map_manager=self.map_manager.lock().expect("mutex lock failed");
                            // map_manager.sunshines_check_click(x, y,&self.audio_sender);
                            match self.map_manager.lock(){
                                Ok(mut map_manager)=>{
                                    map_manager.sunshines_check_click(x, y, &self.audio_sender);
                                },
                                Err(e)=>{
                                    eprintln!("lock map_manager failed:{}",e);
                                }
                            }

                        //----------------------------------------------------------------
                        //-----------------------------map (include plant)----------------------------------    
                            //检查用户是否选择”卡片“
                            //1.card_manager会记录选择的卡片，然后”绘制“其跟随鼠标的实体  2.game也会记录其类型，若为”植物卡片“，用于判断”阳光是否足够“、”种植后扣除多少阳光“等
                            let select_card=self.cards_manager.check_select_plant(x, y,&self.audio_sender);
                            //通过”卡片类型“获取”price“，如果”阳光值“足够
                            if self.sunshines_value>=select_card.type_to_price(){
                                match select_card{
                                    //设置game选中的”卡片类型“
                                    CardType::PeashooterCard=>self.card_be_selected=CardType::PeashooterCard,
                                    CardType::SunFlowerCard=>self.card_be_selected=CardType::SunFlowerCard,
                                    CardType::WallnutCard=>self.card_be_selected=CardType::WallnutCard,
                                    CardType::SpadeCard=>self.card_be_selected=CardType::SpadeCard,
                                    CardType::NoneCard=>self.card_be_selected=CardType::NoneCard,
                                }
                            }
                        //----------------------------------------------------------------------------------
                        },
                        _=>{
                            //检查是否有按钮被按下，并通过button_manager记录
                            self.buttons_manager.buttons_check_click(x, y,&self.page,&mut self.game_mod);
                        },
                    }
                }
            }
        }
    }
    
    fn mouse_button_up_event(&mut self,ctx: &mut Context,button: ggez::event::MouseButton,x: f32,y: f32,){
        if button==mouse::MouseButton::Left{
            //通过button_manager 检测是否有按钮处于“被点击”状态，返回“被点击的按钮”
            let be_clicked_button=self.buttons_manager.check_set_button_up(&self.audio_sender); //抬起时才播放音效
            //匹配游戏界面
            match self.page {
            //开始界面
                GamePage::StartPage=>{
                    //匹配“被点击按钮的类型”，并做出相应处理
                    match be_clicked_button{
                        //“开始按钮”被点击
                        ButtonType::GameStart=>{
                            task::block_on(async {
                                task::sleep(Duration::from_micros(500)).await; 
                            });
                            self.set_game_status(GameStatus::Playing);
                            self.set_game_page(GamePage::PlayPage);
                        },
                        _=>{}, //没有按钮被点击
                    }
                }
            //playing page
                GamePage::PlayPage=>{
                    match self.status {
                        GameStatus::Playing=>{
                            //匹配“被点击按钮的类型”，并做出相应处理
                            match be_clicked_button{
                                ButtonType::GamePause=>{
                                    match self.change_game_status(GameStatus::Paused){
                                        Err(e)=>{eprintln!("change_game_status failed:{}",e)},
                                        Ok(_)=>{},
                                    };
                                },
                                ButtonType::GameRestart=>{
                                     //游戏重启
                                    match self.game_restart(GamePage::PlayPage,GameStatus::Playing){
                                        Err(e)=>{eprintln!("game_restart failed:{}",e)},
                                        Ok(_)=>{},
                                    }
                                },
                                ButtonType::GameBack=>{
                                     //游戏重启
                                    match self.game_restart(GamePage::StartPage,GameStatus::Menu){
                                        Err(e)=>{eprintln!("game_restart failed:{}",e)},
                                        Ok(_)=>{},
                                    }
                                     //切换页面
                                    self.set_game_page(GamePage::StartPage);
                                }
                                _=>{},
                            }
                        //--------------------------map (include plant)----------------------------
                            //检查是否有“植物卡片”被选择
                            match self.card_be_selected{
                                CardType::NoneCard=>{},
                                //被选择的是“铲子”
                                CardType::SpadeCard=>{
                                    // let mut map_manager=self.map_manager.lock().expect("locked failed");
                                    // //铲除植物
                                    // map_manager.remove_plant(x, y,&self.audio_sender);
                                    match self.map_manager.lock(){
                                        Ok(mut map_manager)=>{
                                            map_manager.remove_plant(x, y, &self.audio_sender);
                                        },
                                        Err(e)=>{
                                            eprintln!("lock map_manager failed:{}",e);
                                        }
                                    }
                                },
                                //被选择的是“植物”
                                _=>{
                                    // let mut map_manager=self.map_manager.lock().expect("locked failed");
                                    // //将”卡片类型“转为”植物“，如果”转换成功”，则尝试”种植“
                                    // if let Some(plant)=self.card_be_selected.type_to_plant(){
                                    //     if map_manager.grow_plant(x, y, &plant,&self.audio_sender){
                                    //         //种植植物成功，扣除相应相关值（只有在草地上才能成功种植）
                                    //         self.sunshines_value-=self.card_be_selected.type_to_price();
                                    //         self.cards_manager.set_card_in_cool_time(&self.card_be_selected);
                                    //     }
                                    // }
                                    match self.map_manager.lock(){
                                        Ok(mut map_manager)=>{
                                            //将”卡片类型“转为”植物“，如果”转换成功”，则尝试”种植“
                                            if let Some(plant)=self.card_be_selected.type_to_plant(){
                                                if map_manager.grow_plant(x, y, &plant,&self.audio_sender){
                                                    //种植植物成功，扣除相应相关值（只有在草地上才能成功种植）
                                                    self.sunshines_value-=self.card_be_selected.type_to_price();
                                                    self.cards_manager.set_card_in_cool_time(&self.card_be_selected);
                                                }
                                            }
                                        },
                                        Err(e)=>{
                                            eprintln!("lock map_manager failed:{}",e);
                                        }
                                    }
                                    //只要”鼠标左键“抬起，都要”重置“选择的”卡片“
                                    self.card_be_selected=CardType::NoneCard;
                                },
                            }
                        //--------------------------------------------------------------------------
                            //cards_manager也重置”被选择的植物“
                            self.cards_manager.reset_plant_selected();
                        },
                        _=>{
                            match be_clicked_button{
                                ButtonType::GamePlaying=>{
                                    match self.change_game_status(GameStatus::Playing){
                                        Err(e)=>{eprintln!("change_game_status failed:{}",e)},
                                        Ok(_)=>{},
                                    }
                                },
                                ButtonType::GameRestart=>{
                                    match self.game_restart(GamePage::PlayPage,GameStatus::Playing){
                                        Err(e)=>{eprintln!("game_restart failed:{}",e)},
                                        Ok(_)=>{},
                                    }
                                },
                                ButtonType::GameBack=>{
                                    match self.game_restart(GamePage::StartPage,GameStatus::Menu){
                                        Err(e)=>{eprintln!("game_restart failed:{}",e)},
                                        Ok(_)=>{},
                                    }
                                    self.set_game_page(GamePage::StartPage); //切换页面
                                }
                                _=>{},
                            }
                        },
                    }
                },
            }
        }
    }
    
    
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _mods: KeyMods, _repeat: bool) {
        if keycode == KeyCode::Escape {
            match self.status {
                GameStatus::Playing =>{
                    match self.change_game_status(GameStatus::Paused){
                        Err(e)=>{eprintln!("change_game_status failed:{}",e)},
                        Ok(_)=>{},
                    }
                },
                GameStatus::Paused =>{
                    match self.change_game_status(GameStatus::Playing){
                        Err(e)=>{eprintln!("change_game_status failed:{}",e)},
                        Ok(_)=>{},
                    }
                }
                _ => {},
            }
        }
    }
}
