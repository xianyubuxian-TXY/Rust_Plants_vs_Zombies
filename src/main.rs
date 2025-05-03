mod game;
mod threads;
mod entities;
mod tools;
mod my_trait;
mod entities_managers;

use std::{env, path};

//context related 
use ggez::{event, ContextBuilder, GameResult};
//color related
//EventHandler related
use game::Game;

fn main() -> GameResult {

    //设置资源文件路径
    let resource_dir=if let Ok(manifest_dir)=env::var("CARGO_MAINIFEST_DIR"){
        let mut path=path::PathBuf::from(manifest_dir);
        path.push("assets");
        path
    }else{
        path::PathBuf::from("./assets")
    };

    //ctx: game context
    let (mut ctx, event_loop) = ContextBuilder::new("PlantsVsZombies", "tangxianyu")
        .window_setup(ggez::conf::WindowSetup {
            title: "Plants Vs Zombies".into(),
            vsync: true,
            ..Default::default()
        })
        .window_mode(ggez::conf::WindowMode {
            width: 1600.0,
            height: 1000.0,
            resizable: false,
            maximized:false,
            fullscreen_type:ggez::conf::FullscreenType::Windowed,
            ..Default::default()
        })
        .add_resource_path(resource_dir)
        .build()
        .expect("create ctx error");
    
    
    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}

