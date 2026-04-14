/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details: 
*/

use macroquad::{color, prelude::*};
use crate::{VIRTUAL_HEIGHT, modules::scale::use_virtual_resolution};

pub async fn run(virtual_height: f32, virtual_width: f32) -> String {
    use_virtual_resolution(virtual_height, virtual_width);
    loop {
        clear_background(RED);
        next_frame().await;
    }
}