/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;

pub async fn run(virtual_height: f32, virtual_width: f32, player: &mut crate::modules::player::Player) -> String {
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        player.handle_keypresses().await;
        player.move_player();
        player.draw();
        next_frame().await;
    }
}
