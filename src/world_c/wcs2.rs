/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::label::Label;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
//use crate::modules::projectile::Projectile;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String) -> String {
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
    map.create_map_array(0, 2, 0, vec![1, 3]).await;
    if last_scene == "Top" {
        player.set_position(virtual_width / 2.0, virtual_height - 80.0);
    } else if last_scene == "down" {
        player.set_position(virtual_width / 2.0, 80.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
    let cyric = StillImage::new(
        "assets/player_files/player_t.png",
        80.0,   // width
        80.0,   // height
        450.0,  // x position
        1000.0, // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
    )
    .await;

    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut lbl_speech = Label::new("", 50.0, 600.0, 75);
    lbl_speech.set_visible(false);
    let mut speech_duration = 0.0;
    let mut speech_num = 0;
    let mut speech_letter = 0;
    let speech_list: Vec<String> = vec![
        "We're at the end!".to_string(),
        "....What are those?".to_string(),
        "They look like music disks..".to_string(),
        "You should touch one.".to_string(),
        "It'd be funny.".to_string(),
    ];
    let mut scrolling_list: Vec<String> = vec![];
    let tutorial_list = "Press UP ARROW to use your melee attack\nPress DOWN ARROW to use your ranged attack".to_string();
    //let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        let timer = get_time() - start_time;
        if timer > 0.1 {
            current_time = get_time();
            if (current_time - time_dif) > 0.1 {
                time_dif = current_time;
                if speech_duration > 0.0 {
                    speech_duration -= 0.05;
                    if speech_num <= (speech_list.len() as i32 -1) && speech_letter <= (scrolling_list.len() as i32) {
                    scrolling_text_show(&scrolling_list, &mut lbl_speech, &speech_letter);
                    speech_letter += 1;

                    }
                    if speech_duration < 0.0 {
                        speech_duration = 0.0;
                        speech_num += 1;
                    }
                }
            }
        }
        player.handle_keypresses(pause).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);

        if speech_duration == 0.0 && speech_num <= (speech_list.len() as i32 - 1) {
                scrolling_list = scrolling_text_create(&speech_list[speech_num as usize]);
                speech_letter = 1;
                if speech_num < 2 {
                    speech_duration = 3.0;
                } else{
                    speech_duration = 5.0;
                }
            }
        if player.get_y() <= 10.0 {
            return "wcs3".to_string();
        } else if player.get_y() >= virtual_height - 10.0 {
            return "wcs1".to_string();
        }
        lbl_speech.draw();
        player.draw();
        next_frame().await;
    }
}

pub fn scrolling_text_create(sentence: &String) -> Vec<String> {
    let mut scrolling_list: Vec<String> = vec![];
    for i in 0..sentence.len() {
        let letter = sentence[i..i + 1].to_string();
        scrolling_list.push(letter.to_string());
    }
    scrolling_list
}

pub fn scrolling_text_show(scrolling_list: &Vec<String>, lbl_speech: &mut Label, speech_letter: &i32) {
    let mut scrolled_text = "".to_string();
    for i in 0..*speech_letter {
        scrolled_text = scrolled_text + &scrolling_list[i as usize].clone();
    }
    lbl_speech.set_text(scrolled_text);
}
