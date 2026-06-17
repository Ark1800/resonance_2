/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details: final cyric bossssssss
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
    musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
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
    let mut lbl_speech = Label::new("", 50.0, 600.0, 30);
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
    if player.get_cleared() <= 17 {
        speech_done = true;
    }

    let mut cyric_img_heart = StillImage::new(
        "",
        100.0,                 // width
        50.0,                  // height
        60.0,                  // x position //offset as drawn from center
        virtual_height - 50.0, // y position
        true,                  // Enable stretching
        1.0,                   // Normal zoom (100%)
    )
    .await;
    cyric_img_heart.set_preload(tm.get_preload("assets/player_files/heart.png").unwrap());
    let mut lbl_cyric_healthbar = Label::new("", 120.0, virtual_height - 10.0, 30);
    lbl_cyric_healthbar.with_fixed_size(800.0, 25.0);
    lbl_cyric_healthbar.with_colors(WHITE, Some(PURPLE));
    lbl_cyric_healthbar.with_border(BLACK, 2.0);
    let mut lbl_cyric_healthbarbg = Label::new("", 120.0, virtual_height - 10.0, 30);
    lbl_cyric_healthbarbg.with_fixed_size(800.0, 25.0);
    lbl_cyric_healthbarbg.with_colors(WHITE, Some(WHITE));
    lbl_cyric_healthbarbg.with_border(BLACK, 2.0);
    let mut lbl_cyric_healthnum = Label::new("1000", 90.0, virtual_height - 20.0, 30);
    let mut lbl_cyric_name = Label::new("cyric The Betrayer", 135.0, virtual_height - 45.0, 30);
    lbl_cyric_name.with_colors(BLACK, Some(RED));
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        background.draw();
        map.draw_map(&tm).await;


        if player.get_cleared() < 17 {
            current_time = get_time();
            if (current_time - time_dif) > 0.1 {
                time_dif = current_time;

                if player.get_cleared() == 16 {
                    if speech_cooldown > 0.0 {
                        speech_cooldown -= 0.1;
                        if speech_cooldown <= 0.0 {
                            speech_cooldown = 0.0;
                            if speech_num == speech_list.len() {
                                lbl_speech.set_text("");
                                speech_done = true;
                            } else {
                                lbl_speech.set_scrolling_text(speech_list[speech_num].to_string());
                            }
                        }
                    }
                }
            }

            if player.get_cleared() == 16 {
                if lbl_speech.get_scroll_len() == lbl_speech.get_scroll() && speech_num < speech_list.len() && speech_cooldown <= 0.0 {
                    speech_cooldown = 1.0;
                    speech_num += 1;
                }
            }
        } else if speech_done && *pause == false {
            player.handle_keypresses(pause, musicdiscfunctions).await;
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
            if mlehit {
                enemies[index].dmg_enemy(player.get_meleedmg());
                enemies[index].knockback(player, "enemy");
            }
            if rnghit {
                enemies[index].dmg_enemy(player.get_rngdmg());
                enemies[index].knockback(player, "enemy");
            }
            if player.get_cleared() < 17 {
                if enemies[0].get_health() <= 0.0 {
                    player.addcoins(99999 - player.get_musicoins());
                    player.add_cleared();
                    enemies[0].set_preload(tm.get_preload("assets/cyric_files/cyric_dead").unwrap());
                    map.change_map(vec![0, 0], vec![vec![7, 9], vec![6, 9]]);
                    player.set_health(player.get_maxhealth());
                } else {
                    enemies[0].cyric_action(player, tm, musicdiscfunctions).await;
                }
                enemies[0].draw_bullet(player, musicdiscfunctions);
            }
        }

        enemies[0].draw();
        let mut new_width = enemies[0].get_health() as f32 * 4.0; // Assuming 100 health corresponds to 400 width
        let max_width = 200 as f32 * 4.0; // Maximum width based on max health
        if new_width < 0.0 {
            new_width = 0.0; // Prevent negative width
        }
        lbl_cyric_healthbarbg.with_fixed_size(max_width, 25.0); //update healthbar size based on health
        lbl_cyric_healthbar.with_fixed_size(new_width, 25.0); //update healthbar size based on health
        lbl_cyric_healthbarbg.draw();
        lbl_cyric_healthbar.draw();
        cyric_img_heart.draw();
        lbl_cyric_healthnum.set_text(enemies[0].get_health().to_string());
        lbl_cyric_healthnum.draw();
        player.draw();
        if lbl_speech.get_text() != "" && player.get_cleared() < 17 {
            speech_box.draw();
            name_box.draw();
        }
        if speech_num != speech_list.len() && player.get_cleared() < 17 {
            lbl_speech.scrolling_text_draw();
        } else {
            lbl_speech.draw();
        }
        player.handle_inventory();
        let (save, exit) = player.handle_save_menu().await;
        if save {
            player.update_save_data(records, client, last_scene).await;
            *pause = false;
        }
        if exit {
            return "title_screen".to_string();
        }
        if last_scene == "null" {
            player.show_player_messagebox();
            *last_scene = "".to_string();
        }
        player.draw_player_messagebox();
        let (restart, quit) = player.handle_death_screen(pause, musicdiscfunctions).await;
        if restart {
            *last_scene = "None".to_string();
            return "inn".to_string();
        }
        if quit {
            return "title_screen".to_string();
        }
        next_frame().await;
    }
}

// Cleared starts at 16, goes to 17
