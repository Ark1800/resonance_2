/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::label::Label;
use crate::modules::player::Player;
//use crate::modules::projectile::Projectile;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
) -> String {
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    map.create_map_array(0, 1, 0, vec![3]).await;
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
    let cyric = StillImage::new(
        "assets/player_files/player_t.png",
        80.0,  // width
        80.0,  // height
        450.0, // x position
        550.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;

    let podium_list: Vec<StillImage> = vec![
        StillImage::new(
            "assets/map_files/pedestal.png",
            30.0,  // width
            30.0,  // height
            450.0, // x position
            550.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
        StillImage::new(
            "assets/map_files/pedestal.png",
            30.0,  // width
            30.0,  // height
            450.0, // x position
            600.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
        StillImage::new(
            "assets/map_files/pedestal.png",
            30.0,  // width
            30.0,  // height
            450.0, // x position
            650.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
    ];

    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut lbl_speech = Label::new("", 50.0, 600.0, 75);
    lbl_speech.set_visible(false);
    let mut speech_duration = 0.0;
    let mut speech_num = 0;
    let mut dash_duration = 0.0;
    let mut dash_cooldown = 0.0;
    let speech_list: Vec<String> = vec![
        "We're at the end!".to_string(),
        "....What are those?".to_string(),
        "They look like music disks..".to_string(),
        "You should touch one".to_string(),
        "It'd be funny".to_string(),
    ];
    let mut scrolling_list: Vec<String> = vec![];
    //let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        let timer = get_time() - start_time;
        if timer > 0.1 {
            current_time = get_time();
            if (current_time - time_dif) > 0.01 {
                time_dif = current_time;
                if speech_duration > 0.0 {
                    speech_duration -= 0.01;
                    if speech_num <= (speech_list.len() as i32) - 1 {
                    scrolling_text_show(&scrolling_list, &mut lbl_speech, &speech_num);
                    }
                    if speech_duration < 0.0 {
                        speech_duration = 0.0;
                        speech_num += 1;
                    }
                }
                if dash_duration > 0.0 {
                    dash_duration -= 0.01;
                    if dash_duration < 0.0 {
                        dash_duration = 0.0;
                        player.dash_end();
                    }
                }
                if dash_cooldown > 0.0 {
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
            player.move_player(&map, old_pos, &podium_list);

            if speech_duration == 0.0 {
                if speech_num <= (speech_list.len() as i32) - 1 {
                scrolling_list = scrolling_text_create(&speech_list[speech_num as usize]);
                }
                if speech_num < 2 {
                    speech_duration = 3.0;
                } else {
                    speech_duration = 5.0;
                }
            }
            
            for podium in 0..podium_list.len() {
                podium_list[podium].draw();
            }
            player.draw();
            cyric.draw();
            next_frame().await;
        }
    }
}

pub fn scrolling_text_create(sentence: &String) -> Vec<String> {
    let mut scrolling_list: Vec<String> = vec![];
    for i in 0..sentence.len() {
        let letter = sentence[i..i + 1].trim();
        scrolling_list.push(letter.to_string());
    }
    scrolling_list
}

pub fn scrolling_text_show(scrolling_list: &Vec<String>, lbl_speech: &mut Label, speech_num: &i32) {
    let mut scrolled_text = "".to_string();
    for i in 0..*speech_num {
        scrolled_text = scrolled_text + &scrolling_list[i as usize].clone();
    }

    lbl_speech.set_text(scrolled_text);
}
