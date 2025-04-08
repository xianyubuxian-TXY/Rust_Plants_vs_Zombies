use minifb::{Window, WindowOptions};
use image::{open,GenericImageView};
use crate::functionality::my_image::Image;



pub fn start_ui(){
    let mut window = Window::new(
        "Plants vs Zombies - Rust Minifb", 
        1200, 800,                    
        WindowOptions::default(),     
    ).expect("Unable to create window");

    let (buffer_width, buffer_height) = (1200, 800);
    let mut buffer: Vec<u32> = vec![0; buffer_width * buffer_height];
    buffer.fill(0);


    //load background
    // let bck_path = match open("../assets/background/map1.png") {
    //     Ok(path) => path,
    //     Err(e) => {
    //         eprintln!("Error opening the file: {}", e);
    //         return;
    //     }
    // };
    // let bck_path = if let Ok(path) = open("../assets/background/map1.png") {
    //     path
    // } else {
    //     panic()
    // };
    let bck_path=open("../assets/background/map1.png").expect("open map1.png failed");
    let bck_image=Image::new(bck_path,0,0,1200,800);

    let bar_path=open("../assets/bar/bar5.png").expect("open bar5.png failed");
    let (bar_w,bar_y)=bar_path.dimensions();
    let bar_image=Image::new(bar_path,300,0,(bar_w-100) as usize,(bar_y-10) as usize);

    bck_image.draw(&mut buffer, buffer_width, buffer_height);
    bar_image.draw(&mut buffer, buffer_width, buffer_height);
    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, buffer_width, buffer_height).unwrap();
    }
}