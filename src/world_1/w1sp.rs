/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::scale::use_virtual_resolution;
use crate::modules::map::Map;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::animated_image::AnimatedImage;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String) -> String {
    let mut background1 = StillImage::new(
        "",
        virtual_width/2.0,  // width
        virtual_height, // height
        virtual_width/2.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
        )
        .await;
    background1.set_preload(tm.get_preload("assets/map_files/grass.png").unwrap());
    let mut background2 = StillImage::new(
        "",
        virtual_width/2.0,  // width
        virtual_height, // height
        0.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
    )
    .await;
    background2.set_preload(tm.get_preload("assets/map_files/world1/beach.png").unwrap());
    let mut blue_portal = AnimatedImage::from_gif(
        "", 
        300.0, 100.0,          
        128.0, 128.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/map_files/world1/blueportal.gif") {
        blue_portal.set_preloaded_gif(preloaded, true);
    }
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
    let mut map = Map::new(virtual_width, virtual_height, vec![]).await;
    map.create_map_array(0, 0, 0, vec![]).await;
    loop {
        background1.draw();
        background2.draw();
        use_virtual_resolution(virtual_width, virtual_height);
        blue_portal.draw();
        player.handle_keypresses(pause).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        player.draw();
        next_frame().await;
    }
}