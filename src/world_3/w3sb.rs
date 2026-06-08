/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::enemy::Enemy;
use crate::modules::label::Label;
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
    player.set_currentscreen("w3sb".to_string());
    player.set_position(virtual_width / 2.0 - 20.0, virtual_height - 100.0);

    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/magma.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    map.create_map_array(0, 1, 0, vec![3]).await;
    map.change_map(
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![
            vec![2, 2],
            vec![2, 3],
            vec![3, 6],
            vec![3, 7],
            vec![12, 2],
            vec![12, 3],
            vec![11, 6],
            vec![11, 7],
        ],
    );
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

    let mut cyric = Enemy::new(
        "",
        50.0,                //hieght
        80.0,                //width
        virtual_width / 2.0, //x
        150.0,               //y
        true,                //stretching
        1.0,                 //zoom level
        500.0,               //health
        30.0,                //damage
        "",
        "boss", //enemy type
    )
    .await;

    let mut enemies: Vec<Enemy> = vec![];
    cyric.set_preload(tm.get_preload("assets/cyric_files/cyric_f.png").unwrap());
    if player.get_cleared() >= 17 {
        cyric.set_preload(tm.get_preload("assets/cyric_files/cyric_dead").unwrap());
    } else {
        enemies.push(cyric);
    }

    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut lbl_speech = Label::new("", 50.0, 100.0, 30);
    lbl_speech.with_colors(WHITE, None);
    lbl_speech.with_scroll(true);
    let mut speech_cooldown = 0.0;
    let mut speech_num = 0;
    let speech_list: Vec<String> = vec![
        "So, it finally came to this.. ".to_string(),
        "I'm sorry it had to be this way, friend, but its what is required. ".to_string(),
        "These disks can do so much, they're wasted being in an unguarded cave. ".to_string(),
        "You can still leave if you want your life, I don't want to kill you. But if you stay, I will be forced to take your life. ".to_string(),
        ".... ".to_string(),
        "You always were stubborn. I'm sorry, my friend. ".to_string(),
    ]; 
    let speech_list: Vec<String> = vec![
        "Agh ".to_string(),
        "I'm sorry it had to be this way, friend, but its what is required. ".to_string(),
        "These disks can do so much, they're wasted being in an unguarded cave. ".to_string(),
        "You can still leave if you want your life, I don't want to kill you. But if you stay, I will be forced to take your life. ".to_string(),
        ".... ".to_string(),
        "You always were stubborn. I'm sorry, my friend. ".to_string(),
    ];

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

    lbl_speech.set_scrolling_text(speech_list[speech_num].clone());
    speech_box.set_preload(tm.get_preload("assets/map_files/textbox.png").unwrap());
    let mut name_box = Label::new("Cyric", 150.0, 575.0, 40);
    name_box.with_colors(WHITE, None);
    let mut speech_done = false;
    if player.get_cleared() < 17 {
        speech_done = true;
    }
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        background.draw();

        if player.get_cleared() < 17 {
        current_time = get_time();
        if (current_time - time_dif) > 0.1 {
            time_dif = current_time;

            if player.get_cleared() < 3 {
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
            }
        }

        if player.get_cleared() < 17 {
            if lbl_speech.get_scroll_len() == lbl_speech.get_scroll() && speech_num < speech_list.len() && speech_cooldown <= 0.0 {
                speech_cooldown = 1.0;
                speech_num += 1;
            }
        }
    } else if speech_done {
        player.handle_inventory();
        let (save, exit) = player.handle_save_menu().await;
        if save {
            println!("Saving game...");
            player.update_save_data(records, client, last_scene).await;
        }
        if exit {
            return "title_screen".to_string();
        }
        player.handle_keypresses(pause, _musicdiscfunctions).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);

        if player.get_cleared() < 17 {
            if enemies[0].get_health() <= 0.0 {
                player.addcoins(99999 - player.get_musicoins());
                player.add_cleared();
                enemies[0].set_preload(tm.get_preload("assets/cyric_files/cyric_dead").unwrap());
                map.change_map(vec![0, 0], vec![vec![7, 9], vec![6, 9]]);
                player.set_health(player.get_maxhealth());
            } else {
                enemies[0].cyric_action(player, tm, _musicdiscfunctions).await;
            }
            enemies[0].draw_bullet(player, _musicdiscfunctions);
        }
    }
        
        enemies[0].draw();
        player.draw();
        map.draw_map(&tm).await;
        if lbl_speech.get_text() != "" && player.get_cleared() < 17 {
            speech_box.draw();
            name_box.draw();
        }
        if speech_num != speech_list.len() && player.get_cleared() < 17 {
            lbl_speech.scrolling_text_draw();
        } else {
            lbl_speech.draw();
        }
        let (restart, quit) = player.handle_death_screen(pause, _musicdiscfunctions).await;
        if restart {
            *last_scene = "None".to_string();
            return "inn".to_string();
        }
        if quit {
            return "main_screen".to_string();
        }
        next_frame().await;
    }
}

// Cleared starts at 16, goes to 17