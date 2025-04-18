// use std::thread;
// use ggez::{Context, GameResult};
// use std::sync::{Arc, Mutex};

// pub fn game_update_thread(ctx: Arc<Mutex<Context>>, game_data: Arc<Mutex<GameData>>) {
//     thread::spawn(move || {
//         loop {
//             let mut ctx = ctx.lock().unwrap();
//             let mut game_data = game_data.lock().unwrap();
//             game_data.update(&mut ctx);
//             thread::sleep(std::time::Duration::from_millis(16));
//         }
//     });
// }
