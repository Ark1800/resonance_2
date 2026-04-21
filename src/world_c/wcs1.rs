/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::label::Label;
use crate::modules::player::Player;
use crate::modules::progressbar::ProgressBar;
//use crate::modules::projectile::Projectile;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(virtual_height: f32, virtual_width: f32, player: &mut Player) -> String {
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    let cyric = StillImage::new(
        "assets/player_files/player_t.png",
        80.0, // width
        80.0, // height
        450.0, // x position
        1000.0,  // y position
        true, // Enable stretching
        1.0,  // Normal zoom (100%)
    )
    .await;

    let mut hp_bar = ProgressBar::new(
        10.0, 10.0, // Position (x, y)
        200.0, 30.0, // Size (width, height)
        0.0, 100.0, // Range (min, max)
        100.0, // Initial value
    );

    let background = StillImage::new(
        "assets/slime.png",
        virtual_width,  // width
        virtual_height, // height
        0.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
    )
    .await;


    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut lbl_speech = Label::new("", 600.0, 50.0, 75);
    lbl_speech.set_visible(false);
    let mut speech_duration = 0.0;
    let mut speech_num = 0;
    let mut dash_duration = 0.0;
    let mut dash_cooldown = 0.0;
    let speech_list: Vec<String> = vec!["Speech test 1".to_string(), "Speech test 2".to_string()];
    //let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        let timer = get_time() - start_time;
        if timer > 0.1 {
        current_time = get_time();
        if (current_time - time_dif) > 0.01 {
            time_dif = current_time;
            if speech_duration > 0.0 {
                speech_duration -= 0.01;
                if speech_duration < 0.0 {
                    speech_duration = 0.0;
                    speech_bubble_hide(&mut lbl_speech, &mut speech_num);
                }
            } if dash_duration > 0.0 {
                dash_duration -= 0.01;
                if dash_duration < 0.0 {
                    dash_duration = 0.0;
                    player.dash_end();
                }
            } if dash_cooldown > 0.0 {
                dash_cooldown -= 0.01;
            }
        }
        clear_background(WHITE);
        background.draw();
        player.handle_keypresses().await;
        if is_key_down(KeyCode::LeftShift) && dash_cooldown <= 0.0 {
            player.dash_start();
            dash_duration = 0.1;
            dash_cooldown = 0.6;
        }

        let old_pos = player.get_oldpos();
        if player.check_x_collision(&cyric) || player.get_x() > virtual_width - 40.0 || player.get_x() < 40.0 {
            player.set_x(old_pos.x);
        } if player.check_y_collision(&cyric) || player.get_y() > virtual_height - 40.0 || player.get_y() < 10.0 {
            player.set_y(old_pos.y);
        }
        

        if speech_num == 0 {
            speech_bubble_show(&speech_list[0], &mut lbl_speech, &mut speech_duration);
        } else if speech_num == 1 {
            speech_bubble_show(&speech_list[1], &mut lbl_speech, &mut speech_duration);
        


        }
        player.draw();
        cyric.draw();
        hp_bar.draw();
        next_frame().await;
    }
}
}


pub fn speech_bubble_show(speech: &String, lbl_speech: &mut Label, speech_duration: &mut f64) {
            *speech_duration = 3.0;
            lbl_speech.set_text(format!("{}", speech));
            lbl_speech.set_visible(true);
}

pub fn speech_bubble_hide(lbl_speech: &mut Label, speech_num: &mut i32) {
            lbl_speech.set_visible(false);
            *speech_num += 1;
}