/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::grid::draw_grid;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String) -> String {
    let mut map = Map::new(virtual_width, virtual_height).await;
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
    let mut background = StillImage::new(
        "",
        virtual_width,  // width
        virtual_height,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
        background.set_preload(tm.get_preload("assets/map_files/town.png").unwrap());
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        player.handle_keypresses(pause).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        if player.get_y() > virtual_height - 10.0 {
            *last_scene = "Down".to_string();
            return "wcs1".to_string();
        } else if player.get_x() < 10.0 {
            *last_scene = "Left".to_string();
            return "w1s1".to_string();
        } else if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "w2s1".to_string();
        } else if player.get_y() < 10.0 {
            *last_scene = "Top".to_string();
            return "w3s1".to_string();
        } else if (player.get_x() > 130.0 && player.get_x() < 200.00) && (player.get_y() > 200.0 && player.get_y() < 270.00) {
            return "shop".to_string();
            
        }

        draw_grid(50.0, BROWN);
        player.handle_inventory();
        player.handle_player_ui();
        player.draw();
        next_frame().await;
    }
}
