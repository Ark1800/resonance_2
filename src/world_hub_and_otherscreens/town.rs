/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
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
        virtual_height, // height
        0.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
    )
    .await;
    background.set_preload(tm.get_preload("assets/map_files/town.png").unwrap());

    let get_position = TextButton::new(0.0, 0.0, 200.0, 60.0, "Get position", BLUE, GREEN, 30);
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        map.draw_map(&tm).await;
        player.handle_keypresses(pause).await;
        let old_pos = player.get_oldpos();
        player.move_x();
        let mut new_x = player.get_x();
        let mut new_y = player.get_y();
        let mut collide = false;
        if map.map_collision(&player.view_player()).0
            || (new_x <= 125.0 && new_y <= 300.0)
            || (new_x >= 210.0 && new_x <= 355.0 && new_y >= 210.0 && new_y <= 300.0)
            || (new_x <= 255.0 && new_y <= 210.0)
            || (new_x >= 645.0 && new_y <= 275.0)
            || (new_x > 760.0 && new_y <= 300.0)
            || (new_x >= 560.0 && new_x <= 800.0 && new_y >= 505.0 && new_y <= 660.0)
            || (new_x >= 755.0 && new_y >= 455.0 && new_y <= 605.0)
            || (new_x <= 280.0 && new_y >= 450.0 && new_y <= 600.0)
            || (new_x >= 235.0 && new_x <= 425.0 && new_y >= 500.0 && new_y <= 660.0)

        {
            player.set_x(old_pos.x);
            new_x = old_pos.x;
        }

        player.move_y();
        new_y = player.get_y();
        if map.map_collision(&player.view_player()).0
            || (new_x <= 125.0 && new_y <= 300.0)
            || (new_x >= 210.0 && new_x <= 355.0 && new_y >= 210.0 && new_y <= 300.0)
            || (new_x <= 255.0 && new_y <= 210.0)
            || (new_x >= 645.0 && new_y <= 275.0)
            || (new_x >= 760.0 && new_y <= 300.0)
            || (new_x >= 560.0 && new_x <= 800.0 && new_y >= 505.0 && new_y <= 660.0)
            || (new_x >= 755.0 && new_y >= 455.0 && new_y <= 605.0)
            || (new_x <= 280.0 && new_y >= 450.0 && new_y <= 600.0)
            || (new_x >= 235.0 && new_x <= 425.0 && new_y >= 500.0 && new_y <= 660.0)
        {
            player.set_y(old_pos.y);
        }
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

        if get_position.click() {
            println!("Player position: ({}, {})", player.get_x(), player.get_y());
        }

        background.draw();
        draw_grid(50.0, BLACK);
        player.draw();
        next_frame().await;
    }
}
