/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::grid::draw_grid;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
) -> String {
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    if last_scene == "Right" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Left" {
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
        virtual_height, // height
        0.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
    )
    .await;
    background.set_preload(tm.get_preload("assets/map_files/town.png").unwrap());

    let mut collidable_objects: Vec<StillImage> = vec![
        StillImage::new("", 120.0, 50.0, 0.0, 250.0, true, 1.0).await,
        StillImage::new("", 125.0, 50.0, 250.0, 250.0, true, 1.0).await,
        StillImage::new("", 10.0, 250.0, 250.0, 0.0, true, 1.0).await,
        StillImage::new("", 150.0, 275.0, 675.0, 0.0, true, 1.0).await,
        StillImage::new("", 225.0, 50.0, 800.0, 250.0, true, 1.0).await,
        StillImage::new("", 290.0, 100.0, 0.0, 510.0, true, 1.0).await,
        StillImage::new("", 150.0, 100.0, 260.0, 560.0, true, 1.0).await,
        StillImage::new("", 200.0, 100.0, 600.0, 560.0, true, 1.0).await,
        StillImage::new("", 230.0, 100.0, 790.0, 510.0, true, 1.0).await,
    ];
    for obj in 0..collidable_objects.len() {
        collidable_objects[obj].set_preload(tm.get_preload("assets/map_files/wall.png").unwrap());
    }

    let get_position = TextButton::new(0.0, 0.0, 200.0, 60.0, "Get position", BLUE, GREEN, 30);
    let mut enemies: Vec<Enemy> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        map.draw_map(&tm).await;
        player.handle_keypresses(pause).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &collidable_objects);
        //player.move_player(&map, old_pos, &collidable_objects); //collidable objects breaks player speed
        if player.get_y() > virtual_height - 10.0 {
            *last_scene = "Down".to_string();
            return "wcs1".to_string();
        } else if player.get_x() < 10.0 {
            *last_scene = "Right".to_string();
            return "w1sp".to_string();
        } else if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Left".to_string();
            return "w2s1".to_string();
        } else if player.get_y() < 10.0 {
            *last_scene = "Top".to_string();
            return "w3s1".to_string();
        } else if (player.get_x() > 130.0 && player.get_x() < 200.00) && (player.get_y() > 200.0 && player.get_y() < 270.00) {
            return "shop".to_string();
        }

        if get_position.click() {
            println!("Player position: ({}, {})", player.get_x(), player.get_y());
        }

        background.draw();
        draw_grid(50.0, BLACK);
        player.handle_player_ui(&mut enemies).await;
        player.handle_inventory();
        player.draw();
        next_frame().await;
    }
}
