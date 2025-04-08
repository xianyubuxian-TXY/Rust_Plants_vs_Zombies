use image::{ DynamicImage,imageops::FilterType};

pub struct Image {
    // pub position:(usize,usize),
    pub x: usize,
    pub y: usize,
    pub image_data: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn new(img: DynamicImage, image_x: usize, image_y: usize, image_w: usize, image_h: usize) -> Self {
        let image_data = match load_image_as_u32(img,image_w,image_h) {
            Ok(data) => data,
            Err(e) => {
                panic!("Failed to load image: {}", e);
            }
        };

        let image_data_vec= image_data;

        Image {
            x: image_x,
            y: image_y,
            image_data: image_data_vec,
            width: image_w,
            height: image_h,
        }
    }

    pub fn change(&mut self,new_x: usize, new_y: usize){
        self.x=new_x;
        self.y=new_y;
    }

    pub fn draw(&self, buffer: &mut [u32], buffer_width: usize, buffer_height: usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                let buffer_x = self.x + x;
                let buffer_y = self.y + y;
                if buffer_x < buffer_width && buffer_y < buffer_height {
                    let buffer_idx = buffer_y * buffer_width + buffer_x;
                    let img_idx = y * self.width + x;
                    if self.image_data[img_idx] >> 24 != 0 { 
                        buffer[buffer_idx] = self.image_data[img_idx];
                    }
                }
            }
        }
    }
}


fn load_image_as_u32(img: DynamicImage, target_width: usize, target_height: usize)->Result<Vec<u32>, String> {

    let img_resized = img.resize_exact(target_width as u32, target_height as u32, FilterType::Lanczos3);

    let img_rgba = img_resized.to_rgba8();

    let img_data: Vec<u32> = img_rgba
        .chunks(4)
        .map(|chunk| {
            let a = chunk[3] as u32;
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            (a << 24) | (r << 16) | (g << 8) | b
        })
        .collect();

    Ok(img_data)
}

