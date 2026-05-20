/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::label::Label;
use crate::modules::player::Player;
//use crate::modules::projectile::Projectile;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String, dungeon_completed: &bool) -> String {
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
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
    if *dungeon_completed {
    map.create_map_array(0, 2, 0, vec![1, 3]).await;
    } else {
    map.create_map_array(0, 1, 0, vec![3]).await;
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
    let mut cyric = StillImage::new(
        "",
        80.0, // width
        80.0, // height
        450.0, // x position
        1000.0,  // y position
        true, // Enable stretching
        1.0,  // Normal zoom (100%)
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
    let mut lbl_tutorial = Label::new("", 50.0, 25.0, 40);
    lbl_tutorial.with_colors(WHITE, None);
    lbl_tutorial.with_scroll(true);
    let mut tutorial_cooldown = 0.0;
    let mut tutorial_num = 0;
    let speech_list: Vec<String> = vec![
        "Hurry up man, or I'll lose you! ".to_string(),
        "Finally found this place! That took forever.. ".to_string(),
        "Come on, lets go further in. Try to keep up! ".to_string(),
    ];
    let tutorial_list: Vec<String> = vec![
        "WASD to move".to_string(),
        "SHIFT to dash".to_string()
    ];
    lbl_speech.set_scrolling_text(speech_list[speech_num].clone());
    lbl_tutorial.set_scrolling_text(tutorial_list[tutorial_num].clone());

    let mut speech_box = StillImage::new(
            "",
            virtual_width - 50.0,  // width
            250.0,  // height
            25.0, // x position
            500.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
    
        speech_box.set_preload(tm.get_preload("assets/map_files/textbox.png").unwrap());
        let mut name_box = Label::new("Cyric", 150.0, 575.0, 40);
        name_box.with_colors(WHITE, None);
    //let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        let timer = get_time() - start_time;
        if timer > 0.1 {
        current_time = get_time();
        if (current_time - time_dif) > 0.1 {
            time_dif = current_time;

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
                } if tutorial_cooldown > 0.0 {
                    tutorial_cooldown -= 0.1;
                    if tutorial_cooldown <= 0.0 {
                        tutorial_cooldown = 0.0;
                        lbl_tutorial.set_scrolling_text(tutorial_list[tutorial_num].to_string());
                    }
                }
                
        }
        if lbl_speech.get_scroll_len() == lbl_speech.get_scroll() && speech_num < speech_list.len() && speech_cooldown <= 0.0 {
            speech_cooldown = 1.0;
            speech_num += 1;
        } if lbl_tutorial.get_scroll_len() == lbl_tutorial.get_scroll() && tutorial_num < tutorial_list.len() - 1 && tutorial_cooldown <= 0.0 {
            tutorial_cooldown = 1.5;
            tutorial_num += 1;
        }
        player.handle_keypresses(pause).await;

        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);


        if player.get_y() > virtual_height - 10.0 {
            *last_scene = "Down".to_string();
            return "wcs2".to_string();
        }
        player.draw();
        cyric.draw();
        if lbl_speech.get_text() != "" {
            speech_box.draw();
            name_box.draw();
        }
        if speech_num != speech_list.len() {
        lbl_speech.scrolling_text_draw();
        } else {
            lbl_speech.draw();
        }
        if tutorial_num != tutorial_list.len() {
        lbl_tutorial.scrolling_text_draw();
        } else {
            lbl_tutorial.draw();
        }
        
        next_frame().await;
    }
}
}