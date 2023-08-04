use super::{Window, MISSING_TEXTURE};
use raylib::prelude::Image;

const DEFAULT_TITLE: &str = Window::DEFAULT_TITLE;

pub trait ImagePlugin {
    fn from_path(path: &str) -> Image;
    fn from_paths(paths: Vec<&str>) -> Vec<Image>;
}

impl ImagePlugin for Image {
    fn from_path(path: &str) -> Image {
        match Image::load_image(path) {
            Ok(image) => image,
            Err(e) => {
                println!("RAYLIB: Could not load image!");
                println!("{DEFAULT_TITLE}: Loading default image...");
                Image::load_image_from_mem(
                    ".png",
                    &MISSING_TEXTURE.to_vec(),
                    MISSING_TEXTURE.len() as i32,
                )
                .expect("RAYLIB: Could not load default image!")
            }
        }
    }

    fn from_paths(paths: Vec<&str>) -> Vec<Image> {
        let mut images = Vec::<Image>::new();
        for path in paths {
            images.push(match Image::load_image(path) {
                Ok(image) => image,
                Err(e) => {
                    println!("RAYLIB: Could not load image!");
                    println!("{DEFAULT_TITLE}: Loading default image...");
                    Image::load_image_from_mem(
                        ".png",
                        &MISSING_TEXTURE.to_vec(),
                        MISSING_TEXTURE.len() as i32,
                    )
                    .expect("RAYLIB: Could not load default image!")
                }
            });
        }
        images
    }
}
