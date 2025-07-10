use super::MISSING_TEXTURE;
use raylib::prelude::{Image, RaylibHandle, RaylibThread, Texture2D};

pub trait Texture2DPlugin {
    fn from_path(raylib: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Texture2D;
    fn from_paths(
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
        paths: Vec<&str>,
    ) -> Vec<Texture2D>;
}

impl Texture2DPlugin for Texture2D {
    fn from_path(raylib: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Texture2D {
        match raylib.load_texture(thread, path) {
            Ok(texture) => texture,
            Err(_) => {
                println!("RAYLIB: Could not load texture!");
                println!("Engine: Loading default texture.");
                raylib
                    .load_texture_from_image(
                        thread,
                        &Image::load_image_from_mem(
                            ".png",
                            &MISSING_TEXTURE.to_vec(),
                            MISSING_TEXTURE.len() as i32,
                        )
                        .expect("RAYLIB: Could not load default image!"),
                    )
                    .expect("RAYLIB: Could not load default texture!")
            }
        }
    }

    fn from_paths(
        raylib: &mut RaylibHandle,
        thread: &RaylibThread,
        paths: Vec<&str>,
    ) -> Vec<Texture2D> {
        let mut textures = Vec::<Texture2D>::new();
        for path in paths {
            textures.push(match raylib.load_texture(thread, path) {
                Ok(texture) => texture,
                Err(_) => {
                    println!("RAYLIB: Could not load texture!");
                    println!("Engine: Loading default texture.");
                    raylib
                        .load_texture_from_image(
                            thread,
                            &Image::load_image_from_mem(
                                ".png",
                                &MISSING_TEXTURE.to_vec(),
                                MISSING_TEXTURE.len() as i32,
                            )
                            .expect("RAYLIB: Could not load default image!"),
                        )
                        .expect("RAYLIB: Could not load default texture!")
                }
            });
        }
        textures
    }
}
