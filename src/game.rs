use ggez::input::mouse;

use ggez::{graphics, Context, GameError, GameResult};
use ggez::graphics::{Color,Font, Text};
//EventHandler related
use ggez::event::{EventHandler, KeyCode, KeyMods};
use glam::Vec2;

//sleep but don't block current thread
use async_std::task;
use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

use crate::entities::my_enum::button_enum::ButtonType;
use crate::entities::my_enum::card_enum::CardType;
use crate::entities_managers::zombie_manager::ZombieManager;
use crate::my_trait::SunshineAction;
use crate::entities_managers::background_manager::ResourceManager;
use crate::entities_managers::buttons_manager::ButtonManager;
use crate::entities_managers::map_manager::MapManager;
use crate::entities_managers::sunshines_manager::SunshineManager;
use crate::entities_managers::cards_manager::CardManager;
use crate::tools::play_mp3;

pub enum GameState {
    Menu,
    Playing,
    Paused,
    // GameOver,
    // Victory,
}

pub enum GamePage {
    StartPage,
    PlayPage,
}


pub struct Game{
    state:GameState,
    page:GamePage,
    buttons_manager:ButtonManager,
    cards_manager:CardManager,
    resource_manager: Arc<ResourceManager>,
    sunshines_manager:Arc<Mutex<SunshineManager>>,
    map_manager:Arc<Mutex<MapManager>>, //manager grass,plant,cars,tools and so on
    zombies_manager:Arc<Mutex<ZombieManager>>,
    sunshines_value:u32,
    card_be_selected:CardType,  //选择的卡片
}

impl Game {
    pub fn new(ctx:&mut Context)->Result<Game, GameError>{
        let game=Game{
            state:GameState::Menu,
            page:GamePage::StartPage,
            buttons_manager:ButtonManager::new(ctx)?,
            cards_manager:CardManager::new(ctx)?,
            resource_manager:Arc::new(ResourceManager::new(ctx)?),
            sunshines_manager:Arc::new(Mutex::new(SunshineManager::new(ctx)?)),
            map_manager:Arc::new(Mutex::new(MapManager::new(ctx)?)),
            zombies_manager:Arc::new(Mutex::new(ZombieManager::new(ctx)?)),
            sunshines_value:50,
            card_be_selected:CardType::NoneCard,
        };
        Ok(game)
    }

    pub fn set_game_status(&mut self,stauts:GameState){
        self.state=stauts;
    }

    pub fn set_game_page(&mut self,page:GamePage){
        self.page=page;
    }

    pub fn add_sunshine_value(&mut self,sunshine_value:u32) {
        self.sunshines_value+=sunshine_value;
    }

    pub fn write_sunshine_value(&self,ctx:&mut Context)->GameResult<()>{
        let font = Font::default();
        let text=Text::new((format!("{}",self.sunshines_value),font,30.0));
        let position=Vec2::new(470.0,90.0);
        let color=Color::from_rgb(0, 0, 0);
        graphics::draw(ctx, &text,(position,color,))?;
        Ok(())
    }
}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
        match self.state{
            GameState::Playing=>{
            let mut thread_handles=vec![];
            //---------------------------sunshine_thread-------------------------------------------
                let (tx, rx) = mpsc::channel();
                let sunshine_manager_clone = self.sunshines_manager.clone();
                thread_handles.push(thread::spawn(move || {
                    let mut sunshine_manager = sunshine_manager_clone.lock().unwrap();
                    //随机生成阳光
                    sunshine_manager.create_sunshine();
                    //更新阳光状态
                    let value = sunshine_manager.update_sunshines_status();
                     //将用户收集的阳光值发送到主线程
                     tx.send(value).expect("send failed");
                }));
            //---------------------------------------------------------------------------------------

            //---------------------------map_thread (include plant)----------------------------------
                let (tx1, rx1) = mpsc::channel();
                let map_manager_clone=self.map_manager.clone();
                let zombie_manager_clone=self.zombies_manager.clone();
                thread_handles.push(thread::spawn(move||{
                    let mut map_manager=map_manager_clone.lock().expect("lock map_manager failed");
                    let mut zombie_manager=zombie_manager_clone.lock().expect("lock zombie_manager failed");
                    //更新植物状态
                    map_manager.update_plants_status(&zombie_manager);
                    //更新子弹、阳光（植物产生）状态 
                    map_manager.undate_bullets_status(&mut zombie_manager);
                    //更新车状态
                    map_manager.update_cars_status(&mut zombie_manager);
                    //获取用户收取的阳光值，并传给主线程
                    let value=map_manager.update_sunshines_status();
                    tx1.send(value).expect("send failed");
                }));
            //---------------------------------------------------------------------------------------
            //----------------------------------zombie_thread-----------------------------------------
                let zombie_manager_clone=self.zombies_manager.clone();
                let map_manager_clone=self.map_manager.clone();
                thread_handles.push(thread::spawn(move||{
                    let mut zombie_manager=zombie_manager_clone.lock().expect("lock zombie_manager failed");
                    let mut map_manager=map_manager_clone.lock().expect("lock map_manager failed");
                    //随机创建僵尸
                    zombie_manager.create_zombie();
                    //更新僵尸状态
                    zombie_manager.update_zombies_status(&mut map_manager);
                }));

            //---------------------------------------------------------------------------------------

            //-----------------------------------main_thread -----------------------------------------
                for handle in thread_handles{
                    handle.join().unwrap();//等待子线程的结束
                }
                //接收用户收集的阳光值（随机产生的阳光、植物产生的阳光）
                if let Ok(value) = rx.try_recv() {
                    self.add_sunshine_value(value);
                }
                if let Ok(value)=rx1.try_recv(){
                    self.add_sunshine_value(value);
                }
            }
            _=>{}
        };

        Ok(())
    }

    /*
    DrawParam:scale , dest , color ,src
    */
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0,0.0].into());
        match self.page{

            GamePage::StartPage=>{
                //绘制背景
                self.resource_manager.draw_start_page_background(ctx).expect("draw start_background failed");
                //绘制开始按钮
                self.buttons_manager.draw_buttons(ctx, GamePage::StartPage).expect("draw button_image failed");
                
            },

            GamePage::PlayPage=>{
                //draw playing_background
                self.resource_manager.draw_playing_page_background(ctx).expect("draw playing_background failed");
                //绘制“卡片”
                self.cards_manager.draw(ctx).expect("draw cards failed");
                //写 “阳光值”
                self.write_sunshine_value(ctx).expect("write sunshine_value failed");
                //绘制阳光
                let sunshine_manager=self.sunshines_manager.lock().expect("lock failed");
                sunshine_manager.draw_sunshines(ctx).expect("draw sunshine failed");
                //绘制“用户”选择的“卡片实体”
                match self.card_be_selected{
                    CardType::NoneCard=>{},
                    _=>{
                        self.cards_manager.draw_plant_entity_be_selected_follow_mouse(ctx,mouse::position(ctx)).expect("draw plant which be selected failed");
                    }
                }
                //绘制草地上的植物
                let map_manager=self.map_manager.lock().expect("lock map_manager failed");
                map_manager.draw_plants(ctx).expect("draw plant in map failed");
                //绘制植物产生的太阳
                map_manager.draw_bullets(ctx).expect("draw bullets failed");
                map_manager.draw_sunshines(ctx).expect("draw sunshines failed");
                //绘制车
                map_manager.draw_cars(ctx).expect("draw cars failed");
                //绘制僵尸
                let zombie_manager=self.zombies_manager.lock().expect("lock failed");
                zombie_manager.draw_zombies(ctx).expect("draw zombies failed");

            },
        }
        graphics::present(ctx)

    }
    
    //game will ues it by self
    fn mouse_button_down_event(&mut self,ctx: &mut Context,button: ggez::event::MouseButton,x: f32,y: f32) {
        if button==mouse::MouseButton::Left{
            match self.page{
                GamePage::StartPage=>{
                    //检查“按钮”是否被点击，并通过button_manager记录
                    self.buttons_manager.buttons_check_click(x, y,GamePage::StartPage);
                }
                GamePage::PlayPage=>{
                    match self.state {
                        GameState::Playing=>{
                        //-----------------------------sunshine---------------------------
                            //检测是否有阳光被“点击”
                            let mut sunshine_manager=self.sunshines_manager.lock().expect("lock failed");
                            sunshine_manager.sunshines_check_click(x, y);
                            let mut map_manager=self.map_manager.lock().expect("lock map_manager failed");
                            map_manager.sunshines_check_click(x, y);
                        //----------------------------------------------------------------
                        
                        //-----------------------------map (include plant)----------------------------------    
                            //检查用户是否选择”卡片“
                            //1.card_manager会记录选择的卡片，然后”绘制“其跟随鼠标的实体  2.game也会记录其类型，若为”植物卡片“，用于判断”阳光是否足够“、”种植后扣除多少阳光“等
                            let select_card=self.cards_manager.check_select_plant(x, y);
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
                        }
                        _=>{},
                    }
                }
            }
        }
    }
    
    fn mouse_button_up_event(&mut self,ctx: &mut Context,button: ggez::event::MouseButton,x: f32,y: f32,){
        if button==mouse::MouseButton::Left{
            //通过button_manager 检测是否有按钮处于“被点击”状态，有的话返回“按钮类型”
            let be_clicked_button=self.buttons_manager.check_set_button_up();
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
                            self.set_game_status(GameState::Playing);
                            self.set_game_page(GamePage::PlayPage);
                            println!("start game!\n");
                        },
                        ButtonType::None=>{}, //没有按钮被点击
                    }
                }
            //playing page
                GamePage::PlayPage=>{
                    match self.state {
                        GameState::Playing=>{
                        //--------------------------map (include plant)----------------------------
                            //检查是否有“植物卡片”被选择
                            match self.card_be_selected{
                                CardType::NoneCard=>{},
                                //被选择的是“铲子”
                                CardType::SpadeCard=>{
                                    let mut map_manager=self.map_manager.lock().expect("locked failed");
                                    //铲除植物
                                    map_manager.remove_plant(x, y);
                                },
                                //被选择的是“植物”
                                _=>{
                                    let mut plant_manager=self.map_manager.lock().expect("locked failed");
                                    //将”卡片类型“转为”植物“，如果”转换成功”，则尝试”种植“
                                    if let Some(plant)=self.card_be_selected.type_to_plant(){
                                        if plant_manager.grow_plant(x, y, &plant){
                                            //种植植物成功，扣除相应相关值（只有在草地上才能成功种植）
                                            self.sunshines_value-=self.card_be_selected.type_to_price();
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
                        _=>{},
                    }
                },
            }
        }
    }
    
    // fn mouse_motion_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _x: f32,
    //     _y: f32,
    //     _dx: f32,
    //     _dy: f32,
    // ) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _mods: KeyMods, _repeat: bool) {
        if keycode == KeyCode::Escape {
            match self.state {
                GameState::Playing => self.state = GameState::Paused,
                GameState::Paused => self.state = GameState::Playing,
                _ => {},
            }
        }
    }
    
    // fn key_up_event(&mut self, _ctx: &mut Context, _input: ggez::input::keyboard::KeyInput) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn text_input_event(&mut self, _ctx: &mut Context, _character: char) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn touch_event(
    //     &mut self,
    //     ctx: &mut Context,
    //     phase: ggez::winit::event::TouchPhase,
    //     x: f64,
    //     y: f64,
    // ) -> Result<(), GAmeError> {
    //     ctx.mouse.handle_move(x as f32, y as f32);
    
    //     match phase {
    //         ggez::winit::event::TouchPhase::Started => {
    //             ctx.mouse.set_button(ggez::event::MouseButton::Left, true);
    //             self.mouse_button_down_event(ctx, ggez::event::MouseButton::Left, x as f32, y as f32)?;
    //         }
    //         ggez::winit::event::TouchPhase::Moved => {
    //             let diff = ctx.mouse.last_delta();
    //             self.mouse_motion_event(ctx, x as f32, y as f32, diff.x, diff.y)?;
    //         }
    //         ggez::winit::event::TouchPhase::Ended | ggez::winit::event::TouchPhase::Cancelled => {
    //             ctx.mouse.set_button(ggez::event::MouseButton::Left, false);
    //             self.mouse_button_up_event(ctx, ggez::event::MouseButton::Left, x as f32, y as f32)?;
    //         }
    //     }
    
    //     Ok(())
    // }
    
    // fn gamepad_button_down_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _btn: ggez::input::gamepad::gilrs::Button,
    //     _id: ggez::event::GamepadId,
    // ) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn gamepad_button_up_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _btn: ggez::input::gamepad::gilrs::Button,
    //     _id: ggez::event::GamepadId,
    // ) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn gamepad_axis_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _axis: ggez::input::gamepad::gilrs::Axis,
    //     _value: f32,
    //     _id: ggez::event::GamepadId,
    // ) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn focus_event(&mut self, _ctx: &mut Context, _gained: bool) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GAmeError> {
    //     debug!("quit_event() callback called, quitting...");
    //     Ok(false)
    // }
    
    // fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) -> Result<(), GAmeError> {
    //     Ok(())
    // }
    
    // fn on_error(&mut self, _ctx: &mut Context, _origin: ggez::event::ErrorOrigin, _e: GAmeError) -> bool {
    //     true
    // }
}








/*
use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use state::GameState;
use threads::ThreadManager;

pub struct Game {
    state: GameState,
    thread_manager: ThreadManager,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let state = GameState::new(ctx)?;
        let thread_manager = ThreadManager::new();
        Ok(Game { state, thread_manager })
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        self.state.update(ctx);

        self.thread_manager.update(&mut self.state);
        Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {

        self.state.draw(ctx)
    }
}

*/