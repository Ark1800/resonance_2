/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::collision::check_collision;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::map::Map;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::animated_image::AnimatedImage;
use crate::modules::enemy::Enemy;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String, _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc, records: &Vec<DatabaseTable>,
    client: &DatabaseClient) -> String {
    player.set_currentscreen("w3sp".to_string());
    let mut background1 = StillImage::new(
        "",
        virtual_width,  // width
        virtual_height, // height
        0.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
        )
        .await;
    background1.set_preload(tm.get_preload("assets/map_files/magma_floor.png").unwrap());
    let mut red_portal = AnimatedImage::from_gif(
        "", 
        100.0, 100.0,          
        498.0, 498.0,          
        true                   
    ).await;
    let portal_hitbox = StillImage::new(
        "assets/map_files/wall.png",
        100.0,  // width
        300.0,  // height
        300.0, // x position
        200.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    ).await;
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/magma.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
    map.create_map_array(0, 1, 0, vec![3]).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/map_files/world1/red_portal.gif") {
        red_portal.set_preloaded_gif(preloaded, true);
    }
    if last_scene == "Down" {
        player.set_position((virtual_width / 2.0)-20.0, 80.0);
    } else if last_scene == "Top" {
        player.set_position((virtual_width / 2.0)-20.0, virtual_height - 10.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
    //COLLIDABLES
    let mut collidable_objects: Vec<StillImage> = vec![
        StillImage::new(
            "",
            250.0,  // width
            175.0,  // height
            0.0, // x position
            600.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
        StillImage::new(
            "",
            150.0,  // width
            200.0,  // height
            0.0, // x position
            450.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
        StillImage::new(
            "",
            200.0,  // width
            150.0,  // height
            0.0, // x position
            300.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
        StillImage::new(
            "",
            250.0,  // width
            350.0,  // height
            0.0, // x position
            0.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
    ];
    for obj in 0..collidable_objects.len() {
        collidable_objects[obj].set_preload(tm.get_preload("assets/map_files/magma.png").unwrap());
    }
    let mut world_numeral = StillImage::new(
        "",
        200.0, // width
        200.0, // height
        (virtual_width / 2.0) - 100.0, // x position
        (virtual_height / 2.0) - 100.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    world_numeral.set_preload(tm.get_preload("assets/map_files/3_rn.png").unwrap());
    loop {
        // set virtual resolution and clear frame
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        let mut enemies: Vec<Enemy> = vec![];
        //backgrounds
        background1.draw();
        map.draw_map(tm).await;
        if player.get_cleared() >= 12 {
        red_portal.draw();
        }
        world_numeral.draw();
        //game 
        if player.get_y() > virtual_width - 10.0 {
            *last_scene = "Down".to_string();
            return "town".to_string();
        }
        
        if check_collision(player.view_player(), &portal_hitbox, 1) && player.get_cleared() >= 12 {
            *last_scene = "Top".to_string(); 
            return "w3s1".to_string();
        }
        //player
        player.handle_keypresses(pause, _musicdiscfunctions).await;
        player.handle_player_ui(&mut enemies, _musicdiscfunctions).await;
        player.handle_inventory();      
        let (save, exit) = player.handle_save_menu().await;
        if save {
            println!("Saving game...");
            player.update_save_data(records, client, last_scene).await;
        } if exit {
            return "title_screen".to_string();
        }
        let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &collidable_objects); 
        player.draw();
        next_frame().await;
    }
}