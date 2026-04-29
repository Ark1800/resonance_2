/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/
use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;

pub async fn run(virtual_width: f32, virtual_height: f32, tm: &TextureManager) -> String {
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLUE);
        next_frame().await;
    }
}
