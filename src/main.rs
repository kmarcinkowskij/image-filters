use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};

/*#[derive(Debug, Clone)]
struct PixelInfo {
    pixel_color:Vec<u8>,
    brightness: f32
}
*/


fn save_from_buffer(_image_buffer: &RgbaImage, _name: &str) {
    match _image_buffer.save(_name.to_owned() + ".png") {
        Ok(()) => {
            println!("Sucessfully saved file")
        }, 
        Err(error) => {
            println!("Problems while saving file!, {}", error);
        }
    }
}

fn grayscale_filter(_image: &DynamicImage, _name: &str) {
    let mut image_buffer: RgbaImage = ImageBuffer::new(_image.dimensions().0, _image.dimensions().1);
    let mut current_pixel_brightness: u8;
    for pixel in _image.pixels() {
        current_pixel_brightness = (0.2126 * pixel.2.0[0] as f32 + 0.7152*pixel.2.0[1] as f32 + 0.0722*pixel.2.0[2] as f32) as u8;
        image_buffer[(pixel.0, pixel.1)] = Rgba([current_pixel_brightness, current_pixel_brightness, current_pixel_brightness, 255]);
    }
    save_from_buffer(&image_buffer, &_name)
}

fn invert_image_filter(_image: &DynamicImage, _name: &str) {
    let mut image_buffer: RgbaImage = ImageBuffer::new(_image.dimensions().0, _image.dimensions().1);
    for pixel in _image.pixels() {
        image_buffer[(pixel.0, pixel.1)] = Rgba([pixel.2[3] - pixel.2[0], pixel.2[3] - pixel.2[1], pixel.2[3] - pixel.2[2], pixel.2[3]]);
    }
    save_from_buffer(&image_buffer, &_name)
}
fn main() {
    let base_path: &str = "./images/";
    let edited_path: &str = "./images/edited/";
    let image_path: &str = &(base_path.to_string() + "base_image.png");
    let image = image::open(image_path).unwrap();
    grayscale_filter(&image, &(edited_path.to_string() + "grayscale"));
    invert_image_filter(&image, &(edited_path.to_string() + "inverted"));
}
