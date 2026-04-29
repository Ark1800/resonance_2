/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager) -> String {
    let mut map = Map::new().await;
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        player.handle_keypresses().await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos);
        player.draw();
        next_frame().await;
    }
}
