use ggez::input::mouse;
use ggez::timer::sleep;
use ggez::winit::dpi::Position;
use ggez::{timer, Context, GameError, GameResult};
use ggez::graphics::{self, DrawParam, Image,Color,Font, Text};
use glam::Vec2;
//EventHandler related
use ggez::event::EventHandler;

//sleep but don't block current thread
use async_std::task;
use std::time::Duration;
use std::collections::HashMap;

use crate::entities::sunshine;
use crate::resources::EntityType;
use crate::tools::{mydraw, update_texture_path};
use crate::entities::button::{ Button,ButtonType};
use crate::resources::{ResourceManager,EntityType::SunShine};
use crate::my_trait::Entity;
use crate::spawner::Spawner;

pub enum GameState {
    Menu,
    Playing,
    // Paused,
    // GameOver,
    // Victory,
}

pub enum GamePage {
    StartPage,
    PlayPage,
}


pub struct Game{
    game_state:GameState,
    game_page:GamePage,
    game_buttons:HashMap<String,Button>,
    game_resource_manager: ResourceManager,
    game_spawner:Spawner,
    sunshine_value:u32,
}

impl Game {
    pub fn new(ctx:&mut Context)->Result<Game, GameError>{
        let mut buttons=HashMap::new();
        let start_button=Button::new(ButtonType::GameStart,850.0, 200.0 , 550.0 , 240.0);
        buttons.insert("start_button".to_string(), start_button);

        //create resources_manager and init it
        let mut resources_manager=ResourceManager::new().expect("create resources_manager failed");
        //start_page : load images
        resources_manager.load_texture(ctx,"/images/background/menu.png","menu.png".to_string())?;
        let mut texture_path=String::with_capacity(50);
        for i in 0..=1{
            update_texture_path(&mut texture_path, "/images/buttons/start_button_", i, ".png");
            resources_manager.load_texture(ctx,&texture_path,format!("start_button_{}.png",i))?;
        }
        //play_page
        //load texture
        resources_manager.load_texture(ctx,"/images/background/map1.png","map1.png".to_string())?;
        resources_manager.load_texture(ctx, "/images/background/bar0.png", "bar0.png".to_string())?;
        for i in 0..=2{
            update_texture_path(&mut texture_path,"/images/cards/card" , i, ".png");
            resources_manager.load_texture(ctx, &texture_path,format!("card{}.png",i))?;
        }
        //load animation
        resources_manager.load_animation(ctx, "/images/sunshine/frame/", 29,"sunshine_animation".to_string())?;

        resources_manager.load_entities_pool(EntityType::SunShine, 10,"sunshine_pool".to_string());

        let game=Game{
            game_state:GameState::Menu,
            game_page:GamePage::StartPage,
            game_buttons:buttons,
            game_resource_manager:resources_manager,
            game_spawner:Spawner::new(),
            sunshine_value:50,
        };
        Ok(game)
    }

    pub fn add_sunshine_value(&mut self) {
        self.sunshine_value+=50;
    }

}

impl EventHandler<GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()>{
        match self.game_state{
            GameState::Playing=>{
                self.game_spawner.create_sunshine(&mut self.game_resource_manager).expect("create sunshine failed");
                
                let sunshine_pool=self.game_resource_manager.get_entities_pool("sunshine_pool").expect("get sunshine_pool failed");
                let mut sunshine_pool=sunshine_pool.lock().expect("sunshine_pool lock failed");
                for sunshine in sunshine_pool.iter_mut() {
                    if sunshine.is_used() {
                        sunshine.update_status(self);
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
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0,0.0].into());
        match self.game_page{

            GamePage::StartPage=>{
                //draw menu
                let menu=self.game_resource_manager.get_texture("menu.png").expect("not find Image");
                mydraw(ctx, &menu, 0.0,0.0, 1600.0,1000.0).unwrap();
                //draw start_button
                let start_button=self.game_buttons.get("start_button").unwrap();
                start_button.draw(ctx,&self.game_resource_manager).expect("button draw failed");                
                
            },

            GamePage::PlayPage=>{
                let mut texture_path=String::with_capacity(50);
            //draw start page
                let map1_img=self.game_resource_manager.get_texture("map1.png").expect("not find Image");
                let bar0_img=self.game_resource_manager.get_texture("bar0.png").expect("load img failed");
            //draw playing page
                mydraw(ctx, &map1_img, 0.0, 0.0, 1600.0, 1000.0).expect("draw background failed");
                mydraw(ctx, &bar0_img, 450.0, 0.0, (bar0_img.width()+20) as f32, (bar0_img.height()+20) as f32).expect("draw background failed");
                let mut cards_img;
                for i in 0..=2{
                    update_texture_path(&mut texture_path, "card", i, ".png");
                    cards_img=self.game_resource_manager.get_texture(&texture_path).expect("load image failed");
                    mydraw(ctx, &cards_img, 540.0+(i as f32)*80.0, 5.0, (cards_img.width()+20) as f32,(cards_img.height()+20) as f32).expect("draw cards failed");
                }
                //write sunshine_value
                let font = Font::default();
                let mut text=Text::new((format!("{}",self.sunshine_value),font,30.0));
                let position=Vec2::new(470.0,90.0);
                let color=Color::from_rgb(0, 0, 0);
                graphics::draw(ctx, &text,(position,color,)).expect("write sunshine_value failed");
                // draw entities
                let sunshine_pool=self.game_resource_manager.get_entities_pool("sunshine_pool").expect("get sunshine_pool failed");
                let mut sunshine_pool=sunshine_pool.lock().expect("lock sunshine_pool falied");
                for sunshine in sunshine_pool.iter_mut(){
                    sunshine.draw(ctx, & mut self.game_resource_manager).unwrap();
                }

            },
        }
        graphics::present(ctx)

    }
    
    //game will ues it by self
    fn mouse_button_down_event(&mut self,ctx: &mut Context,button: ggez::event::MouseButton,x: f32,y: f32) {
        if button==mouse::MouseButton::Left{
            match self.game_page{

                GamePage::StartPage=>{
                    let start_button=self.game_buttons.get_mut("start_button").unwrap();
                    if start_button.be_clicked(x, y){
                        start_button.handle_click();
                    }
                }
                GamePage::PlayPage=>{
                    //check sunshine whether be clicked
                    let sunshine_pool=self.game_resource_manager.get_entities_pool("sunshine_pool").expect("get sunshine_pool failed");
                    let mut sunshine_pool=sunshine_pool.lock().expect("lock sunshine_pool failed");
                    for sunshine in sunshine_pool.iter_mut(){
                        if sunshine.is_used(){
                            sunshine.check_clicked(x, y);
                        }
                    }
                }
            }
        }
    }
    
    fn mouse_button_up_event(&mut self,ctx: &mut Context,button: ggez::event::MouseButton,x: f32,y: f32,){
        if button==mouse::MouseButton::Left{
            match self.game_page {

                GamePage::StartPage=>{
                    let start_button=self.game_buttons.get_mut("start_button").unwrap();
                    if start_button.button_is_down{
                        start_button.handle_click();
                        task::block_on(async {
                            task::sleep(Duration::from_micros(500)).await; 
                        });
                        self.game_page=GamePage::PlayPage;
                        self.game_state=GameState::Playing;
                    }
                }
                GamePage::PlayPage=>println!("start game"),
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
    
    // fn key_down_event(
    //     &mut self,
    //     ctx: &mut Context,
    //     input: ggez::input::keyboard::KeyInput,
    //     _repeated: bool,
    // ) -> Result<(), GAmeError> {
    //     if input.keycode == Some(ggez::input::keyboard::KeyCode::Escape) {
    //         ctx.request_quit();
    //     }
    //     Ok(())
    // }
    
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