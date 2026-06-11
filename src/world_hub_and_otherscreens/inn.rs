/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::map::Map;
use crate::modules::musicdisc::Musicdisc;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    musicdiscfunctions: &mut Musicdisc,
    records: &Vec<DatabaseTable>,
    client: &DatabaseClient
) -> String {
    player.set_currentscreen("inn".to_string());
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;

    let mut collidable_objects: Vec<StillImage> = vec![
        StillImage::new("", 0.0, 0.0, virtual_width, 10.0, true, 1.0).await,
        StillImage::new("", 0.0, 0.0, 10.0, virtual_height, true, 1.0).await,
        StillImage::new("", virtual_width, 0.0, 50.0, virtual_height, true, 1.0).await,
        StillImage::new("", 300.0, virtual_height, virtual_width, 50.0, true, 1.0).await,
    ];
    for obj in 0..collidable_objects.len() {
        collidable_objects[obj].set_preload(tm.get_preload("assets/map_files/wall.png").unwrap());
    }
    
    if last_scene == "Top" {
        player.set_position(50.0, virtual_height - 50.0);
    } else {
        player.set_position(225.0, 140.0);
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
    background.set_preload(tm.get_preload("assets/map_files/tavern.png").unwrap());
    let mut enemies: Vec<Enemy> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        map.draw_map(&tm).await;
        player.handle_keypresses(pause, musicdiscfunctions).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &collidable_objects);
        if player.get_y() > virtual_height - 10.0 {
            *last_scene = "Inn".to_string();
            return "town".to_string();
        }
        
        background.draw();
        player.handle_player_ui(&mut enemies, musicdiscfunctions).await;
        player.handle_inventory();
        let (save, exit) = player.handle_save_menu().await;
        if save {
            println!("Saving game...");
            player.update_save_data(records, client, last_scene).await;
        } if exit {
            return "title_screen".to_string();
        }
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        player.draw();
        next_frame().await;
    }
}
