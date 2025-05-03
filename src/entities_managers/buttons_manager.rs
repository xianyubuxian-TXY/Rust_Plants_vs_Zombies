use ggez::graphics::Image;
use ggez::{Context,GameResult};


use crate::game::GamePage;
use crate::entities::button::Button;
use crate::entities::my_enum::button_enum::{ButtonStatus,ButtonType};


//850.0, 200.0 , 550.0 , 240.0 : start_button
pub struct ButtonManager{
    game_start_button:Button,
    button_be_clicked:ButtonType, //记录被点击的按钮
}

impl ButtonManager{
    pub fn new(ctx:&mut Context)->GameResult<Self>{
        let mut start_button_images=Vec::new();
        start_button_images.push(Image::new(ctx, "/images/buttons/start_button_0.png")?);
        start_button_images.push(Image::new(ctx, "/images/buttons/start_button_1.png")?);
        let start_button=Button::new(ButtonType::GameStart,850.0, 200.0 , 550.0 , 240.0,start_button_images).expect("create start_button failed");

        Ok(ButtonManager{
            game_start_button:start_button,
            button_be_clicked:ButtonType::None,
        })
    }

    pub fn buttons_check_click(&mut self,x:f32,y:f32,game_page:GamePage)
    {
        match game_page {
            GamePage::StartPage=>{
                //if this button be clicked, sete self.button_be_clicked=this button
                if self.game_start_button.check_click(x, y){
                    self.button_be_clicked=ButtonType::GameStart;
                }
            }
            GamePage::PlayPage=>{

            },
        }
    }

    //if have button be clicked, set button up and return type of it, else return ButtonType::None
    pub fn check_set_button_up(&mut self)->ButtonType
    {
        //match which button be clicked and set it up
        let mut be_clicked_button=ButtonType::None;
        match self.button_be_clicked{
            ButtonType::GameStart=>{
                //further judge this button whether be clicked, avoid differ
                if self.game_start_button.be_clicked()
                {
                    self.game_start_button.set_up(); //set button up
                    be_clicked_button=ButtonType::GameStart;
                }
                else {
                    panic!{"logic differ"};
                }
            },
            ButtonType::None=>{},
        }
        //set button_be_clicked  ButtonType::None
        self.button_be_clicked=ButtonType::None;
        be_clicked_button
    }

    pub fn draw_buttons(&mut self,ctx:&mut Context,game_page:GamePage)->GameResult<()>{
        match game_page{
            GamePage::StartPage=>{
                self.game_start_button.draw_image(ctx)?;
            },
            GamePage::PlayPage=>{

            }
        }
        Ok(())
    }
}
