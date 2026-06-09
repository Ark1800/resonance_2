/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::item;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
    records: &Vec<DatabaseTable>,
    client: &DatabaseClient
) -> String {
    player.set_currentscreen("w1s3".to_string());
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
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec![
            "assets/map_files/world1/watertile.png".to_string(),
            "assets/map_files/chest.png".to_string(),
        ],
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
    map.create_map_array(0, 1, 0, vec![3]).await;
    let mut enemies: Vec<Enemy> = vec![];
    let mut large_slime = Enemy::new(
    "",
    75.0, //hieght
    75.0, //width
    70.0, //x
    80.0, //y
    true, //stretching
    1.0, //zoom level
    20.0, //health
    8.0, //damage
    "",
    "large_slime"//enemy type
        ).await;
    large_slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
        let mut summoner = Enemy::new(
    "",
    50.0, //hieght
    50.0, //width
    70.0, //x
    80.0, //y
    true, //stretching
    1.0, //zoom level
    20.0, //health
    10.0, //damage
    "",
    "summoner"//enemy type
    ).await;
    summoner.set_preload(tm.get_preload("assets/summoner_files/summoner_standL.png").unwrap());
    let mut mage = Enemy::new(
    "",
    50.0, //hieght
    50.0, //width
    70.0, //x
    80.0, //y
    true, //stretching
    1.0, //zoom level
    20.0, //health
    10.0, //damage
    "", //projectile
    "mage"//enemy type
    ).await;
    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());
    mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
    enemies.push(large_slime);
    enemies.push(summoner);
    enemies.push(mage);
    let mut choose_open = false;
    let mut item_valid = false;
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        if player.get_cleared() <= 5 {
                for i in 0..enemies.len() {
                    //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                    if musicdiscfunctions.get_thickofit_active() == false
                        && musicdiscfunctions.get_pandemonium_active() == false
                        && musicdiscfunctions.get_sodapop_active() == false
                    {
                                            match enemies[i].get_enemy_type() {
                        "archer" => {
                            enemies[i].archer_action(tm, player, musicdiscfunctions).await;
                            enemies[i].draw_bullet(player, musicdiscfunctions);
                        }
                        "slime" => {
                            enemies[i].slime_action(player, musicdiscfunctions);
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
                            enemies[i].mage_action(tm, player, musicdiscfunctions).await;
                            enemies[i].draw_bullet(player, musicdiscfunctions);
                        }
                        "large_slime" => {
                            enemies[i].large_slime_action(tm, player, musicdiscfunctions).await;
                        }
                        _ => {}
                    }
                    enemies[i].draw();
                }
            }
         }
        map.draw_map(&tm).await;
        player.handle_keypresses(pause, musicdiscfunctions).await;
        let old_pos = player.get_oldpos();
        player.handle_inventory();
        let (save, exit) = player.handle_save_menu().await;
        if save {
            println!("Saving game...");
            player.update_save_data(records, client, last_scene).await;
        } if exit {
            return "title_screen".to_string();
        }
        let (restart, quit) = player.handle_death_screen(pause, musicdiscfunctions).await;
        if restart {
            *last_scene = "None".to_string();
            return "inn".to_string();
        } if quit {
            return "main_screen".to_string();
        }
        player.move_player(&map, old_pos, &vec![]);
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        if mlehit {
            enemies[index].dmg_enemy(player.get_meleedmg());
            if enemies[index].get_health() <= 0.0 {
                if enemies[index].get_enemy_type() == "large_slime" {
                    let (slime1, slime2, split) = enemies[index].large_slime_action(tm, player, musicdiscfunctions).await;
                    if split {
                        enemies.push(slime1);
                        enemies.push(slime2);
                    }
                }
                enemies[index].add_gold(player);
                enemies.remove(index);
            }
        }
        if rnghit {
            enemies[index].dmg_enemy(player.get_rngdmg());
            if enemies[index].get_health() <= 0.0 {
                if enemies[index].get_enemy_type() == "large_slime" {
                    let (slime1, slime2, split) = enemies[index].large_slime_action(tm, player, musicdiscfunctions).await;
                    if split {
                        enemies.push(slime1);
                        enemies.push(slime2);
                    }
                }
                enemies[index].add_gold(player);
                enemies.remove(index);
            }
        }
        if enemies.is_empty() && player.get_cleared() <= 5 {
            player.add_cleared();
            item_valid = true;
            choose_open = true;
            map.change_map(vec![0, 0], vec![vec![0, 4], vec![0, 5]]);
            player.add_health(30.0);
        }
        player.draw();
        if player.get_x() < 10.0 {
            *last_scene = "Left".to_string();
            return "w1s4".to_string();
        }
        if player.get_y() > virtual_height - 10.0 {
            *last_scene = "Down".to_string();
            return "w1s2".to_string();
        }
        (choose_open, item_valid) = player.handle_choose_item(&mut choose_open, &mut item_valid);
        next_frame().await;
    }
}
