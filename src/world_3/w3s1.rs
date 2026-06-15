/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::enemy::Enemy;
use crate::modules::map::Map;
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use miniquad::date;

use macroquad::prelude::*;
pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
    records: &Vec<DatabaseTable>,
    client: &DatabaseClient,
) -> String {
    rand::srand(date::now() as u64);
    player.set_currentscreen("w3s1".to_string());
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
    background.set_preload(tm.get_preload("assets/map_files/magma_floor.png").unwrap());
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/magma.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    // If cleared
    if player.get_cleared() == 12 {
        map.create_map_array(0, 1, 0, vec![3]).await;
    } else {
        map.create_map_array(0, 2, 0, vec![3, 4]).await;
    }
    println!("Last scene: {}", last_scene);
    let mut enemies: Vec<Enemy> = vec![];
    for _i in 0..2 {
        let mut archer = Enemy::new(
            "",
            50.0,
            50.0,
            rand::gen_range(100.0, virtual_width - 100.0),
            rand::gen_range(100.0, virtual_height - 100.0),
            true,
            1.0,
            100.0,    // health
            15.0,     // Damage
            "",       // Projectile Preload
            "archer", // Enemy type
        )
        .await;
        archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
        enemies.push(archer);
    }
    for _i in 0..2 {
        let mut large_slime = Enemy::new(
            "",
            100.0,
            100.0,
            rand::gen_range(100.0, virtual_width - 100.0),
            rand::gen_range(100.0, virtual_height - 100.0),
            true,
            1.0,
            200.0,         // health
            12.0,          // Damage
            "",            // Projectile Preload
            "large_slime", // Enemy type
        )
        .await;
        large_slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
        enemies.push(large_slime);
    }
    if *last_scene == "Top" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
    } else if *last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, 80.0);
    } else if *last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if *last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    }
    player.set_position(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0);
    let mut choose_open = false;
    let mut item_valid = false;
    loop {
        player.handle_keypresses(pause, musicdiscfunctions).await;
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        if *pause == false {
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            //enemy loop
            if player.get_cleared() == 12 {
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
            player.draw();
            let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
            let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
            player.set_player_activedisc(activedisc);
            if mlehit {
                enemies[index].dmg_enemy(player.get_meleedmg());
                enemies[index].knockback(player, "enemy");
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
                enemies[index].knockback(player, "enemy");
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

            player.handle_inventory();
            if enemies.is_empty() && player.get_cleared() == 12 {
                player.add_cleared();
                item_valid = true;
                choose_open = true;
                map.change_map(vec![0, 0], vec![vec![7, 0], vec![6, 0]]);
                player.add_health(30.0);
            }
            if player.get_x() > virtual_width - 10.0 {
                *last_scene = "Right".to_string();
                return "w3s2".to_string();
            }

            if player.get_y() > virtual_height - 10.0 {
                *last_scene = "Down".to_string();
                return "w3sp".to_string();
            }
        }
        let (restart, quit) = player.handle_death_screen(pause, musicdiscfunctions).await;
        if restart {
            *last_scene = "None".to_string();
            return "inn".to_string();
        }
        if quit {
            return "title_screen".to_string();
        }
        #[allow(unused)]
            let (save, exit) = player.handle_save_menu().await;
        if save {
            println!("Saving game...");
            player.update_save_data(records, client, last_scene).await;
        }
        if exit {
            return "title_screen".to_string();
        }
        (choose_open, item_valid) = player.handle_choose_item(&mut choose_open, &mut item_valid);
        next_frame().await;
    }
}

// Cleared starts at 12, goes to 13
