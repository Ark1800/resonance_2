/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::map::Map;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String) -> String {
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Top" {
        player.set_position(virtual_width / 2.0, virtual_height - 80.0);
    } else if last_scene == "down" {
        player.set_position(virtual_width / 2.0, 80.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
    map.create_map_array(0, 4, 0, vec![1, 2, 3, 4]).await;
    loop {         
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        player.handle_keypresses(pause).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        player.draw();
        next_frame().await;
    }
}
