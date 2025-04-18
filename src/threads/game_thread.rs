// use std::thread;
// use ggez::{Context, graphics, GameResult};
// use std::sync::{Arc, Mutex};

// pub fn load_image_async(ctx: &mut Context, img_path: &str, cache: Arc<Mutex<ResourceCache>>) {
//     thread::spawn(move || {
//         let image = graphics::Image::new(ctx, img_path);
//         if let Ok(image) = image {
//             let mut cache = cache.lock().unwrap();
//             cache.add_image(img_path.to_string(), image);
//         } else {
//             eprintln!("Failed to load image: {}", img_path);
//         }
//     });
// }