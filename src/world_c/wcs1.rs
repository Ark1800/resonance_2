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

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String) -> String {
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
    map.create_map_array(0, 2, 5, vec![1, 3]).await;
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
let mut img_back = StillImage::new(
        "",
        virtual_width, // width
        virtual_height, // height
        0.0, // x position
        0.0,  // y position
        true, // Enable stretching
        1.0,  // Normal zoom (100%)
    )
    .await;
        img_back.set_preload(tm.get_preload("assets/map_files/dungeon.png").unwrap());



    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut lbl_speech = Label::new("", 50.0, 600.0, 75);
    let mut lbl_tutorial = Label::new("", 50.0, 25.0, 75);
    lbl_speech.set_visible(false);
    let mut speech_num = 0;
    let mut next_speech = true;
    let mut speech_cooldown = 0.0;
    let mut tutorial_num = 0;
    let mut next_tutorial = true;
    let mut tutorial_cooldown = 0.0;
    let mut dash_duration = 0.0;
    let mut dash_cooldown = 0.0;
    let speech_list: Vec<String> = vec!["Come on, keep up player! Or I'll leave you behind!\nCome on man, you were the one dared to go here in the first place!".to_string(), "Finally here! Lets see what's inside!".to_string()];
    let tutorial_list: Vec<String> = vec!["Use WASD to move".to_string(), "Press SHIFT to dash".to_string()];
    let mut scrolled_speech: Vec<String> = vec![];
    let mut scrolled_tutorial: Vec<String> = vec![];
    //let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        let timer = get_time() - start_time;
        if timer > 0.1 {
        current_time = get_time();
        if (current_time - time_dif) > 0.01 {
            time_dif = current_time;
            if next_speech && speech_cooldown <= 0.0 {
                scrolled_speech = scrolling_text_create(&speech_list[speech_num]);
                next_speech = false;
            } else {
                scrolling_text_show(&scrolled_speech, &mut lbl_speech, &mut speech_num);
                if speech_num == speech_list.len() - 1 {
                    next_speech = true;
                    speech_cooldown = 1.0;
                }
            }
            if next_tutorial && tutorial_cooldown <= 0.0 {
                scrolled_tutorial = scrolling_text_create(&tutorial_list[tutorial_num]);
                next_tutorial = false;
            } else {
                scrolling_text_show(&scrolled_tutorial, &mut lbl_tutorial, &mut tutorial_num);
                if tutorial_num == tutorial_list.len() - 1 {
                    next_tutorial = true;
                    tutorial_cooldown = 1.0;
                }
            }
            if speech_cooldown > 0.0 {
                speech_cooldown -= 0.01;
            } if tutorial_cooldown > 0.0 {
                tutorial_cooldown -= 0.01;
            }
            if dash_duration > 0.0 {
                dash_duration -= 0.01;
                if dash_duration < 0.0 {
                    dash_duration = 0.0;
                    player.dash_end();
                }
            } if dash_cooldown > 0.0 {
                dash_cooldown -= 0.01;
            }
        }
        player.handle_keypresses(pause).await;
        if (is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)) && dash_cooldown <= 0.0 {
            player.dash_start();
            dash_duration = 0.1;
            dash_cooldown = 0.6;
        }

        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        
        if player.get_y() <= 10.0 {
            return "wcs2".to_string();
        }
        img_back.draw();
        player.draw();
        cyric.draw();
        lbl_speech.draw();
        next_frame().await;
    }
}
}


pub fn scrolling_text_create(sentence: &String) -> Vec<String> {
let mut scrolling_list: Vec<String> = vec![];
for i in 0..sentence.len() {
    let letter = sentence[i..i+1].trim();
    scrolling_list.push(letter.to_string());
}
scrolling_list
}

pub fn scrolling_text_show(scrolling_list: &Vec<String>, lbl_speech: &mut Label, speech_num: &mut usize) {
            let mut scrolled_text = "".to_string();
            for i in 0..scrolling_list.len() {
                scrolled_text = scrolled_text + &scrolling_list[i].clone();
            }

            lbl_speech.set_text(scrolled_text);
}