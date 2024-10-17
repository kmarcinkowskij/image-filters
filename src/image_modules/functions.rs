
use std::{fmt::format, fs::{File, OpenOptions}, io::Write, iter};

use image::{DynamicImage, EncodableLayout, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};



// pub fn pixelation(_image: &RgbaImage) -> RgbaImage {
//     let mut image_buffer: RgbaImage =
//         ImageBuffer::new(_image.dimensions().0, _image.dimensions().1);
//     let mut current_color: Rgba<u8>;
//     let kernel_percent: f32 = 0.1;
//     let kernel_size_horizontal = (_image.dimensions().0 as f32 * kernel_percent) as u32;
//     let kernel_size_vertical = (_image.dimensions().1 as f32 * kernel_percent) as u32;

//     for vertical in 0.._image.dimensions().1 / kernel_size_vertical {
//         for horizontal in 0.._image.dimensions().0 / kernel_size_horizontal {
//             current_color = *_image.get_pixel(
//                 horizontal * kernel_size_horizontal + kernel_size_horizontal / 2,
//                 vertical * kernel_size_vertical + kernel_size_vertical / 2,
//             );
//             for i in horizontal * kernel_size_horizontal..(horizontal + 1) * kernel_size_horizontal
//             {
//                 for y in vertical * kernel_size_vertical..(vertical + 1) * kernel_size_vertical {
//                     image_buffer[(i, y)] = current_color;
//                 }
//             }
//         }
//     }
//     // save_from_buffer(&image_buffer, &_name)
//     image_buffer
// }

// pub fn slint_pixelate_image(_image: Image) -> Image {
    // let mut image_buffer = *_image.to_rgba8_premultiplied().unwrap().as_slice();
    // image_buffer[0] = Rgba8Pixel::new(255, 255, 255, 255);
    // println!("{:?}",image_buffer[0]);
    // Image::from_rgba8(image_buffer)
// }

pub fn slint_invert_image(_image: Image) -> slint::Image {
    let mut image_buffer:  SharedPixelBuffer<Rgba8Pixel> = _image.to_rgba8().unwrap();
    for pixel in image_buffer.make_mut_slice() {

        *pixel = Rgba8Pixel::new(255 - pixel.r, 255 - pixel.g, 255 - pixel.b, pixel.a);
    }
    

    Image::from_rgba8(image_buffer)
}

pub fn slint_grayscale_image(_image: Image) -> Image {
    let mut image_buffer:  SharedPixelBuffer<Rgba8Pixel> = _image.to_rgba8().unwrap();

    let mut current_pixel_brightness: u8;
    for pixel in image_buffer.make_mut_slice() {
        //pixel: (X, Y, [R, G, B, A]);
        //calculate pixel brightness, use it as color value to get grayscale effect
        current_pixel_brightness = (0.2126 * pixel.r as f32
            + 0.7152 * pixel.g as f32
            + 0.0722 * pixel.b as f32) as u8;
        *pixel = Rgba8Pixel::new(
            current_pixel_brightness,
            current_pixel_brightness,
            current_pixel_brightness,
            pixel.a);
    }
    Image::from_rgba8(image_buffer)
}

// fn save_slint_image(_buffer: SharedPixelBuffer<Rgba8Pixel>) {
//     let image = Image::from_rgba8(_buffer);
//     image.
// }