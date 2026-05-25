/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::map::Map;
use crate::modules::still_image::StillImage;
use crate::modules::enemy::Enemy;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String) -> String {
    let mut background = StillImage::new(
        "",
        virtual_width,  // width
        virtual_height,  // height
        0.0,  // x position
        0.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    ).await;
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/world1/watertile.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
        background.set_preload(tm.get_preload("assets/map_files/world1/beach2.png").unwrap());
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    map.create_map_array(0, 1, 0, vec![4]).await;
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    map.create_map_array(0, 1, 0, vec![2]).await;
    } else if last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
    map.create_map_array(0, 1, 0, vec![3]).await;
    } else if last_scene == "Top" {
        player.set_position((virtual_width / 2.0) - 20.0, 80.0);
    map.create_map_array(0, 1, 0, vec![1]).await;
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    map.create_map_array(0, 0, 0, vec![]).await;
    }
    let map_row_len = map.get_map_array().len();
    let map_column_len = map.get_map_columns().len();
    let mut wall_places: Vec<Vec<i32>> = vec![];
    let mut walls: Vec<i32> = vec![];

    for x in 0..map_row_len {
        for y in 0..map_column_len {
            if y != 4 && x != 0 && x != map_row_len - 1 && y != 0 && y != map_column_len - 1 && y != 5 {
                wall_places.push(vec![x as i32, y as i32]);
                walls.push(1);
            }
        }
    }
    map.change_map(walls, wall_places);
    let mut enemies: Vec<Enemy> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        player.handle_keypresses(pause).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        player.handle_player_ui(&mut enemies).await;
        player.draw();
        if player.get_x() < 10.0 {
            *last_scene = "Left".to_string();
            return "w1sb".to_string();
        }
        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "w1s3".to_string();
        }
        next_frame().await;
    }
}
