// mod modules;

// slint::include_modules!();


// fn main() -> Result<(), Box<dyn Error>>  {
//     let ui = AppWindow::new()?;
//     ui.run()?;
//     let base_path: &str = "./images/";
//     let edited_path: &str = "./images/edited/";
//     let image_path: &str = &(base_path.to_string() + "base_image.png");
//     let image = image::open(image_path).unwrap();
//     // grayscale_filter(&image, &(edited_path.to_string() + "grayscale"));
//     // invert_image_filter(&image, &(edited_path.to_string() + "inverted"));
//     modules::functions::pixelation(&image, &(edited_path.to_string() + "pixelated2"));
// }

// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod image_modules;
use std::error::Error;
use native_dialog::{FileDialog};

use image::DynamicImage;
use image_modules::functions::{slint_grayscale_image, slint_invert_image};
use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new().unwrap();
    let undo_image = ui.get_input_image();
    


    ui.on_request_change_image({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let path = FileDialog::new()
                .set_location("~/Desktop")
                .add_filter("PNG Image", &["png"])
                .add_filter("JPEG Image", &["jpg", "jpeg"])
                .show_open_single_file()
                .unwrap();
            
                let result = path.unwrap();
                let opened_image = image::open(&result.as_path()).expect("Error loading cat image").into_rgba8();
                
                let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    opened_image.as_raw(),
                    opened_image.width(),
                    opened_image.height(),
                );

                let final_image = Image::from_rgba8(buffer);
            
            ui.set_input_image(final_image);
        }
    });

    ui.on_request_invert_image({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            
            let image = ui.get_input_image();
            let final_image = slint_invert_image(image);
            ui.set_input_image(final_image);
        }
    });

    // ui.on_request_desaturate_image({
    //     let ui_handle = ui.as_weak();
    //     let final_image = undo_image.clone();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         let image = ui.get_input_image();
    //         println!("{:?}", ui.get_desaturation_value());
    //         let final_image = slint_grayscale_image(image);
    //         ui.set_input_image(final_image);
    //     }
    // });

    ui.on_request_grayscale_image({
        let ui_handle = ui.as_weak();
        let final_image = undo_image.clone();
        move || {
            let ui = ui_handle.unwrap();
            let image = ui.get_input_image();
            let final_image = slint_grayscale_image(image);
            ui.set_input_image(final_image);
        }
    });


    ui.run()?;

    Ok(())
}
