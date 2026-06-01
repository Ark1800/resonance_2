/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::animated_image::AnimatedImage;
use crate::modules::collision::check_collision;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
) -> String {
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/tree.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    map.create_map_array(0, 1, 0, vec![2]).await;
    if last_scene == "Left" {
        player.set_position((virtual_width / 2.0) + 120.0, (virtual_height / 2.0) - 100.0);
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
    } else if last_scene == "Up" {
        player.set_position((virtual_width / 2.0) - 20.0, 80.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }

    let mut green_portal = AnimatedImage::from_gif("", 700.0, 100.0, 100.0, 300.0, true).await;
    let _portal_hitbox = StillImage::new(
        "assets/map_files/wall.png",
        100.0, // width
        300.0, // height
        300.0, // x position
        200.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/map_files/green_portal.gif") {
        green_portal.set_preloaded_gif(preloaded, true);
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
    let portal_hitbox = StillImage::new(
        "assets/map_files/wall.png",
        100.0, // width
        300.0, // height
        700.0, // x position
        100.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    background.set_preload(tm.get_preload("assets/map_files/world2_start.png").unwrap());
    let mut enemies: Vec<crate::modules::enemy::Enemy> = vec![];
    loop {
        if player.get_x() < 10.0 {
            *last_scene = "Left".to_string();
            return "town".to_string();
        }

        if check_collision(player.view_player(), &portal_hitbox, 1) {
            *last_scene = "Right".to_string();
            return "w2s1".to_string();
        }
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        background.draw();
        player.handle_keypresses(pause, _musicdiscfunctions).await;
        let old_pos = player.get_oldpos();

        player.move_player(&map, old_pos, &vec![]);
        let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);

        player.handle_inventory();
        player.handle_save_menu().await;
        green_portal.draw();
        player.draw();
        map.draw_map(&tm).await;
        next_frame().await;
    }
}
