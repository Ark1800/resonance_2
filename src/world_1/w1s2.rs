/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

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
    musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
) -> String {
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
    background.set_preload(tm.get_preload("assets/map_files/world1/beach2.png").unwrap());
    if *last_scene == "Up" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
    } else if *last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, 80.0);
    } else if *last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if *last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    }
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec![
            "assets/map_files/world1/watertile.png".to_string(),
            "assets/map_files/chest.png".to_string(),
        ],
    )
    .await;
    map.create_map_array(0, 2, 0, vec![3, 1]).await;
    println!("HAI!");
    let mut enemies: Vec<Enemy> = vec![];
    let mut mage = Enemy::new(
        "",
        50.0, // height
        50.0, // width
        70.0, // x
        80.0, // y
        true, // stretching
        1.0,  // zoom level
        20.0,   // health
        10.0,   // damage
        "", // projectile
        "mage", // enemy type
    )
    .await;
    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());
    mage.set_preload(tm.get_preload("assets/fireball.png").unwrap());
    enemies.push(mage);
    for i in 0..2 {
        let mut slime = Enemy::new(
        "",
        25.0, //hieght
        25.0, //width
        70.0, //x
        80.0, //y
        true, //stretching
        1.0, //zoom level
        10.0, //health
        2.0, //damage
        "",
        "slime"//enemy type
    ).await;
        slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
        enemies.push(slime);
    }
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
         if player.get_cleared() == 1 {
                for i in 0..enemies.len() {
                    //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                    if musicdiscfunctions.get_thickofit_active() == false
                        && musicdiscfunctions.get_pandemonium_active() == false
                        && musicdiscfunctions.get_sodapop_active() == false
                    {
                                            match enemies[i].get_enemy_type() {
                        "archer" => {
                            enemies[i].archer_action(tm, player).await;
                            enemies[i].draw_bullet(player);
                        }
                        "slime" => {
                            enemies[i].slime_action(player);
                        }
                        "summoner" => {
                            let (slime1, slime2, slime3, summoned) = enemies[i].summoner_action(tm, player).await;
                            if summoned {
                                enemies.push(slime1);
                                enemies.push(slime2);
                                enemies.push(slime3);
                            }
                        }
                        "mage" => {
                            enemies[i].mage_action(tm, player).await;
                            enemies[i].draw_bullet(player);
                        }
                        "large_slime" => {
                            enemies[i].large_slime_action(tm, player).await;
                        }
                        _ => {}
                    }
                    enemies[i].draw();
                }
            }
         }
        player.handle_inventory();
        player.handle_save_menu().await;
        player.handle_playerdamaging(&enemies);
        player.handle_keypresses(pause, musicdiscfunctions).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        player.handle_player_ui(&mut enemies, musicdiscfunctions).await;
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        player.draw();
        if player.get_y() < 10.0 {
            *last_scene = "Up".to_string();
            return "w1s3".to_string();
        }
        if player.get_y() > virtual_height - 10.0 {
            *last_scene = "Down".to_string();
            return "w1s1".to_string();
        }
        next_frame().await;
    }
}
