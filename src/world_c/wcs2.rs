/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::label::Label;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
//use crate::modules::projectile::Projectile;
use crate::modules::database::{DatabaseClient, DatabaseTable};
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
    player.set_currentscreen("wcs2".to_string());
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
    background.set_preload(tm.get_preload("assets/map_files/dungeon.png").unwrap());
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    if player.get_cleared() == 1 {
        map.create_map_array(0, 1, 0, vec![1]).await;
    } else {
        map.create_map_array(0, 2, 0, vec![1, 3]).await;
    }
    if last_scene == "Top" {
        player.set_position(virtual_width / 2.0 - 50.0, virtual_height - 80.0);
    } else if last_scene == "Down" {
        player.set_position(virtual_width / 2.0 - 50.0, 80.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
    let mut cyric = StillImage::new(
        "", 80.0,   // width
        80.0,   // height
        450.0,  // x position
        1000.0, // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;
    cyric.set_preload(tm.get_preload("assets/player_files/player_t.png").unwrap());

    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut lbl_speech = Label::new("", 150.0, 610.0, 30);
    lbl_speech.with_colors(WHITE, None);
    lbl_speech.with_scroll(true);
    let mut speech_cooldown = 0.0;
    let mut speech_num = 0;
    let mut lbl_tutorial = Label::new("", 50.0, 40.0, 40);
    lbl_tutorial.with_colors(WHITE, None);
    lbl_tutorial.with_scroll(true);
    let mut tutorial_cooldown = 0.0;
    let mut tutorial_num = 0;
    let speech_list: Vec<String> = vec!["Woah, bogie alert!".to_string(), "You take them, you have the sword!".to_string()];
    let tutorial_list: Vec<String> = vec![
        "Press UP ARROW to use your melee attack\nPress DOWN ARROW to use your ranged attack".to_string(),
        "If you get music disks, you can use them using Q, E, and X".to_string(),
    ];

    lbl_speech.set_scrolling_text(speech_list[speech_num].clone());
    lbl_tutorial.set_scrolling_text(tutorial_list[tutorial_num].clone());

    let mut enemies: Vec<Enemy> = vec![];
    if player.get_cleared() < 2 {
        let mut large_slime = Enemy::new("", 75.0, 75.0, 150.0, 200.0, true, 1.0, 15.0, 10.0, "", "large_slime").await;
        large_slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
        enemies.push(large_slime);
    }
    let mut speech_box = StillImage::new(
        "",
        virtual_width - 50.0, // width
        250.0,                // height
        25.0,                 // x position
        500.0,                // y position
        true,                 // Enable stretching
        1.0,                  // Normal zoom (100%)
    )
    .await;
    speech_box.set_preload(tm.get_preload("assets/map_files/textbox.png").unwrap());
    let mut name_box = Label::new("Cyric", 150.0, 575.0, 40);
    name_box.with_colors(WHITE, None);
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;

        if player.get_cleared() == 1 && enemies.len() == 0 {
            map.create_map_array(0, 2, 0, vec![1, 3]).await;
        }

        if *pause == false {
            let timer = get_time() - start_time;
            if timer > 0.1 {
                current_time = get_time();
                if (current_time - time_dif) > 0.1 {
                    time_dif = current_time;
                    if player.get_cleared() < 2 {
                        if speech_cooldown > 0.0 {
                            speech_cooldown -= 0.1;
                            if speech_cooldown <= 0.0 {
                                speech_cooldown = 0.0;
                                if speech_num == speech_list.len() {
                                    lbl_speech.set_text("");
                                } else {
                                    lbl_speech.set_scrolling_text(speech_list[speech_num].to_string());
                                }
                            }
                        }
                        if tutorial_cooldown > 0.0 {
                            tutorial_cooldown -= 0.1;
                            if tutorial_cooldown <= 0.0 {
                                tutorial_cooldown = 0.0;
                                if tutorial_num == tutorial_list.len() {
                                    lbl_tutorial.set_text("");
                                } else {
                                    lbl_tutorial.set_scrolling_text(tutorial_list[tutorial_num].to_string());
                                }
                            }
                        }
                    }
                }
            }
            if player.get_cleared() < 2 {
                if lbl_speech.get_scroll_len() == lbl_speech.get_scroll() && speech_num < speech_list.len() {
                    speech_cooldown = 0.5;
                    speech_num += 1;
                }
                if lbl_tutorial.get_scroll_len() == lbl_tutorial.get_scroll() && tutorial_num < tutorial_list.len() {
                    tutorial_cooldown = 0.5;
                    tutorial_num += 1;
                }
            }

            player.handle_keypresses(pause, _musicdiscfunctions).await;
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            //enemy loop
            for i in 0..enemies.len() {
                //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                match enemies[i].get_enemy_type() {
                    "archer" => {
                        enemies[i].archer_action(tm, player, _musicdiscfunctions).await;
                        enemies[i].draw_bullet(player, _musicdiscfunctions);
                    }
                    "slime" => {
                        enemies[i].slime_action(player, _musicdiscfunctions);
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
                        enemies[i].mage_action(tm, player, _musicdiscfunctions).await;
                        enemies[i].draw_bullet(player, _musicdiscfunctions);
                    }
                    "large_slime" => {
                        enemies[i].large_slime_action(tm, player, _musicdiscfunctions).await;
                    }
                    _ => {}
                }
                enemies[i].draw();
            }
        }
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, _musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
        let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        if mlehit {
            enemies[index].dmg_enemy(player.get_meleedmg());
            if enemies[index].get_health() <= 0.0 {
                if enemies[index].get_enemy_type() == "large_slime" {
                    let (slime1, slime2, split) = enemies[index].large_slime_action(tm, player, _musicdiscfunctions).await;
                    if split {
                        enemies.push(slime1);
                        enemies.push(slime2);
                    }
                }
                enemies.remove(index);
            }
        }
        if rnghit {
            enemies[index].dmg_enemy(player.get_rngdmg());
            if enemies[index].get_health() <= 0.0 {
                if enemies[index].get_enemy_type() == "large_slime" {
                    let (slime1, slime2, split) = enemies[index].large_slime_action(tm, player, _musicdiscfunctions).await;
                    if split {
                        enemies.push(slime1);
                        enemies.push(slime2);
                    }
                }
                enemies.remove(index);
            }
        }
        let (save, exit) = player.handle_save_menu().await;
        if save {
            println!("Saving game...");
            player.update_save_data(records, client, last_scene).await;
        }
        if exit {
            return "title_screen".to_string();
        }
        player.handle_inventory();
        if player.get_y() > virtual_height - 10.0 {
            if player.get_cleared() == 1 {
                player.add_cleared();
            }
            *last_scene = "Down".to_string();
            return "wcs3".to_string();
        }

        if enemies.is_empty() {
            map.change_map(vec![0, 0], vec![vec![7, 9], vec![6, 9]]);
        }
        if player.get_y() < 10.0 {
            *last_scene = "Top".to_string();
            return "wcs1".to_string();
        }
        player.draw();
        if player.get_cleared() == 1 {
            if lbl_speech.get_text() != "" && player.get_cleared() == 1 {
                speech_box.draw();
                name_box.draw();
            }
            lbl_speech.scrolling_text_draw();
            lbl_tutorial.scrolling_text_draw();
        }

        let (restart, quit) = player.handle_death_screen(pause, _musicdiscfunctions).await;
        if restart {
            *last_scene = "None".to_string();
            return "wcs1".to_string();
        }
        if quit {
            return "title_screen".to_string();
        }
        next_frame().await;
    }
}
