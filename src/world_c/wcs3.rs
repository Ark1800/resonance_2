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
use crate::modules::grid::draw_grid;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
    dungeon_completed: &mut bool,
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
    background.set_preload(tm.get_preload("assets/map_files/dungeon.png").unwrap());

    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    map.create_map_array(0, 1, 0, vec![1]).await;
    let mut cyric = StillImage::new(
        "", 45.0,  // width
        70.0,  // height
        250.0, // x position
        100.0, // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    cyric.set_preload(tm.get_preload("assets/cyric_files/cyric_f.png").unwrap());
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Top" {
        player.set_position(virtual_width / 2.0, virtual_height - 80.0);
    } else if last_scene == "Down" {
        player.set_position(virtual_width / 2.0, 80.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }

    let mut podium_list: Vec<StillImage> = vec![
        StillImage::new(
            "", 80.0,  // width
            80.0,  // height
            425.0, // x position
            250.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
        StillImage::new(
            "", 80.0,  // width
            80.0,  // height
            300.0, // x position
            450.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
        StillImage::new(
            "", 80.0,  // width
            80.0,  // height
            550.0, // x position
            450.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await,
    ];
    for i in 0..podium_list.len() {
        podium_list[i].set_preload(tm.get_preload("assets/map_files/pedestal.png").unwrap());
    }

    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut lbl_speech = Label::new("", 50.0, 600.0, 30);
    lbl_speech.with_scroll(true);
    let mut speech_cooldown = 0.0;
    let mut speech_num = 0;
    let mut script_num = 0;
    let mut script_list: Vec<Vec<String>> = vec![];
    let speech_list: Vec<String> = vec![
        "I think we're at the end!".to_string(),
        "....What are those?".to_string(),
        "They look like music disks..".to_string(),
        "You should touch one.".to_string(),
        "It'd be funny.".to_string(),
    ];
    let speech_list2: Vec<String> = vec![
        "Woah, what's happening??".to_string(),
        "What was that flash of light?".to_string(),
        "I think I heard something outside, lets go!".to_string(),
    ];
    let speech_list3: Vec<String> = vec!["......".to_string()];
    script_list.push(speech_list);
    script_list.push(speech_list2);
    script_list.push(speech_list3);
    let mut cutscene_going = false;
    let mut cutscene_num = 0;
    let mut first_time = true;
    if *dungeon_completed {
        first_time = false;
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
    let mut enemies: Vec<crate::modules::enemy::Enemy> = vec![];
    let mut lbl_speech = Label::new("", 150.0, 610.0, 30);
    lbl_speech.with_colors(WHITE, None);
    lbl_speech.with_scroll(true);

    let mut lbl_interact = Label::new("", 0.0, 0.0, 30);
    lbl_interact.with_scroll_speed(0.22);
    lbl_interact.with_colors(WHITE, None);
    let mut can_interact = true;


    lbl_speech.set_scrolling_text(script_list[script_num][speech_num].clone());

    

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
                if first_time {
                    if speech_cooldown > 0.0 {
                        speech_cooldown -= 0.1;
                        if speech_cooldown <= 0.0 {
                            speech_cooldown = 0.0;
                            if speech_num == script_list[script_num].len() {
                                lbl_speech.set_text("");
                                if script_num == 1 {
                                cutscene_going = false;
                            }
                            if script_num == 2 {
                                *dungeon_completed = true;
                                return "wcs2".to_string();
                            }
                            } else {
                                lbl_speech.set_scrolling_text(script_list[script_num][speech_num].to_string());
                                println!("Script num: {}, Speech num: {}", script_num, speech_num);
                            }
                        }
                    }
                }
            }
            if first_time {
                if lbl_speech.get_scroll_len() == lbl_speech.get_scroll() && speech_num < script_list[script_num].len() && speech_cooldown <= 0.0 {
                    speech_cooldown = 1.0;
                    speech_num += 1;
                    if script_num == 2 {
                        cyric.set_preload(tm.get_preload("assets/cyric_files/cyric_b.png").unwrap());
                        speech_cooldown = 2.0;
                    }
                }
            }

            if !cutscene_going {
                player.handle_keypresses(pause, _musicdiscfunctions).await;
                let old_pos = player.get_oldpos();
                player.move_player(&map, old_pos, &podium_list);
                let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies);
                player.set_player_activedisc(activedisc);
            }

            if first_time {
                for _podium in 0..podium_list.len() {
                    if (player.get_oldpos().x - podium_list[0].get_x()).abs() < 100.0
                        && (player.get_oldpos().y - podium_list[0].get_y()).abs() < 100.0
                        && !lbl_interact.scroll()
                        && can_interact
                    {
                        lbl_interact.with_scroll(true);
                        lbl_interact.set_position(player.get_x() - 75.0, player.get_y() - 50.0);
                        lbl_interact.set_scrolling_text("Press E to interact.".to_string());
                        break;
                    } else if ((player.get_oldpos().x - podium_list[0].get_x()).abs() >= 100.0
                        || (player.get_oldpos().y - podium_list[0].get_y()).abs() >= 100.0)
                        && lbl_interact.scroll()
                    {
                        lbl_interact.with_scroll(false);
                    }
                }
            }

            if player.get_y() < 10.0 {
                if first_time && cutscene_num == 1 {
                    *last_scene = "Top".to_string();
                    cutscene_going = true;
                    player.set_position(10000.0, 10000.0);
                    script_num = 2;
                    speech_num = 0;
                    cutscene_num += 1;
                    speech_cooldown = 0.0;
                    lbl_speech.with_scroll_speed(0.25);
                    lbl_speech.set_scrolling_text(script_list[script_num][speech_num].clone());
                } else {
                    return "wcs2".to_string();
                }
            }

            if first_time {
                if lbl_interact.scroll() && is_key_pressed(KeyCode::E) {
                    cutscene_going = true;
                    can_interact = false;
                    lbl_interact.with_scroll(false);
                    script_num = 1;
                    speech_num = 0;
                    cutscene_num += 1;
                    speech_cooldown = 0.0;
                    lbl_speech.set_scrolling_text(script_list[script_num][speech_num].clone());
                }
            }

            for podium in 0..podium_list.len() {
                podium_list[podium].draw();
            }
            player.draw();
            if lbl_interact.scroll() && can_interact && first_time {
                lbl_interact.set_position(player.get_x() - 75.0, player.get_y() - 50.0);
                lbl_interact.scrolling_text_draw();
            }
            if first_time {
                if lbl_speech.get_text() != "" {
                speech_box.draw();
                name_box.draw();
                }
                lbl_speech.scrolling_text_draw();
                cyric.draw();
            }

            draw_grid(50.0, BROWN);
            next_frame().await;
        }
    }
}
