/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details: portal entry scene w1sp, leads to w1s1.
*/

use crate::modules::animated_image::AnimatedImage;
use crate::modules::collision::check_collision;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::enemy::Enemy;
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
    records: &Vec<DatabaseTable>,
    client: &DatabaseClient,
) -> String {
    player.set_currentscreen("w1sp".to_string());
    let mut background1 = StillImage::new(
        "",
        virtual_width / 2.0, // width
        virtual_height,      // height
        virtual_width / 2.0, // x position
        0.0,
        true,
        1.0,
    )
    .await;
    background1.set_preload(tm.get_preload("assets/map_files/grass.png").unwrap());
    let mut background2 = StillImage::new(
        "",
        virtual_width / 2.0, // width
        virtual_height,      // height
        0.0,                 // x position
        0.0,
        true,
        1.0,
    )
    .await;
    background2.set_preload(tm.get_preload("assets/map_files/world1/beach.png").unwrap());
    let mut blue_portal = AnimatedImage::from_gif("", 100.0, 100.0, 498.0, 498.0, true).await;
    let mut portal_hitbox = StillImage::new(
        "", 100.0, // width
        300.0, // height
        300.0, // x position
        200.0, true, 1.0,
    )
    .await;
    portal_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap());
    let mut world_numeral = StillImage::new(
        "",
        200.0,                         // width
        200.0,                         // height
        (virtual_width / 2.0) + 100.0, // x position
        (virtual_height / 2.0) - 100.0,
        true,
        1.0,
    )
    .await;
    world_numeral.set_preload(tm.get_preload("assets/map_files/1_rn.png").unwrap());
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/map_files/world1/blueportal.gif") {
        blue_portal.set_preloaded_gif(preloaded, true);
    }

    let mut map = Map::new(virtual_width, virtual_height, vec![]).await;
    map.create_map_array(0, 1, 0, vec![2]).await;
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Right" {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    } else if last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
    } else if last_scene == "Up" {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
    //COLLIDABLES
    let mut collidable_objects: Vec<StillImage> = vec![
        StillImage::new("", 250.0, 175.0, 0.0, 600.0, true, 1.0).await,
        StillImage::new("", 150.0, 200.0, 0.0, 450.0, true, 1.0).await,
        StillImage::new("", 200.0, 150.0, 0.0, 300.0, true, 1.0).await,
        StillImage::new("", 250.0, 350.0, 0.0, 0.0, true, 1.0).await,
        StillImage::new("", virtual_width, 100.0, 0.0, -80.0, true, 1.0).await,
        StillImage::new("", virtual_width, 100.0, 0.0, virtual_height - 20.0, true, 1.0).await,
    ];
    for obj in 0..collidable_objects.len() {
        collidable_objects[obj].set_preload(tm.get_preload("assets/map_files/wall.png").unwrap());
    }
    loop {
        // set virtual resolution and clear frame
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        let mut enemies: Vec<Enemy> = vec![];
        //backgrounds
        background1.draw();
        background2.draw();
        blue_portal.draw();
        world_numeral.draw();
        player.handle_keypresses(pause, _musicdiscfunctions).await;
        if *pause == false {
            player.draw();
            //game
            if player.get_x() > virtual_width - 10.0 {
                *last_scene = "Right".to_string();
                return "town".to_string();
            }
            if check_collision(player.view_player(), &portal_hitbox, 1) {
                *last_scene = "Left".to_string();
                return "w1s1".to_string();
            }
            //player
            player.handle_player_ui(&mut enemies, _musicdiscfunctions).await;
            let old_pos = player.get_oldpos();
            #[allow(unused)]
            player.move_player(&map, old_pos, &collidable_objects);
        }
        player.handle_inventory();
        let (save, exit) = player.handle_save_menu().await;
        if save {
            *pause = false;
            player.update_save_data(records, client, last_scene).await;
        }
        if exit {
            return "title_screen".to_string();
        }
        if last_scene == "null" {
            player.show_player_messagebox();
            *last_scene = "".to_string();
        }
        player.draw_player_messagebox();
        next_frame().await;
    }
}
