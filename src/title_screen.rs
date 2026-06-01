/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::label::Label;
use crate::modules::text_input::TextInput;
use crate::modules::messagebox::MessageBox;
use macroquad::prelude::*;
#[allow(unused)]
pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    tm: &TextureManager,
    _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,

) -> String {
    let mut background = StillImage::new(
        "", 0.0, // width
        virtual_height, // height
        virtual_width, // x position
        60.0,  // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    background.set_preload(tm.get_preload("assets/map_files/background_start.png").unwrap());

    let btn_new = TextButton::new(100.0, 200.0, 200.0, 60.0, "New Game", BLUE, GREEN, 30);
    let btn_load = TextButton::new(100.0, 200.0, 200.0, 60.0, "Load Game", BLUE, GREEN, 30);
    let btn_start = TextButton::new(100.0, 200.0, 200.0, 60.0, "Start Game", BLUE, GREEN, 30);
    let btn_exit = TextButton::new(100.0, 200.0, 200.0, 60.0, "Leave game", BLUE, GREEN, 30);
    let mut btn_play = TextButton::new(100.0, 200.0, 200.0, 60.0, "", BLUE, GREEN, 30);
    let btn_help = TextButton::new(100.0, 200.0, 200.0, 60.0, "Controls", BLUE, GREEN, 30);
    let btn_back = TextButton::new(100.0, 200.0, 200.0, 60.0, "Back to menu", BLUE, GREEN, 30);
    let mut lbl_start = Label::new("Click to start", virtual_width / 2.0, 980.0, 30);
    let mut txt_username = TextInput::new(100.0, 100.0, 300.0, 40.0, 25.0);
    txt_username.set_prompt("Username");
    let mut txt_password = TextInput::new(100.0, 100.0, 300.0, 40.0, 25.0);
    txt_password.set_prompt("Password");
    let mut info_box = MessageBox::info("Controls", "WASD to move.\n\nUp arrow for melee attack.\nRight arrow for ranged attack.");
    let mut speed = 0.0;
    let mut start_btns_show = false;
    let mut txt_inputs_show = false;
    let mut new_load = "None".to_string();


    loop {
        if lbl_start.get_y() >= 1000.0 {
            speed -= 0.1;
            lbl_start.set_position(lbl_start.get_x(), lbl_start.get_y() + speed);
        } else {
            speed += 0.1;
            lbl_start.set_position(lbl_start.get_x(), lbl_start.get_y() + speed);
        }
        btn_play.visible = true;
        if btn_play.click() {
            start_btns_show = true;
        }
        btn_play.visible = false;
        if start_btns_show {
            if btn_new.click() {
                txt_inputs_show = true;
                new_load = "New".to_string();
            }
            if btn_load.click() {
                txt_inputs_show = true;
                new_load = "Load".to_string();
            }
            if btn_exit.click() {
                return "Exit".to_string();
            }
            if btn_help.click() {
                info_box.show();
            }
        } else if txt_inputs_show {
            txt_username.draw();
            txt_password.draw();
            if btn_back.click() {
                txt_username.set_text("".to_string());
                txt_password.set_text("".to_string());
                txt_inputs_show = false;
                start_btns_show = true;
            }
            if btn_start.click() {
                if new_load == "New" {

                } else if new_load == "Load" {
                    
                }
            }
        }

        if info_box.is_visible() {
            info_box.draw();
        }
 
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLUE);
        background.draw();
        next_frame().await
    }

}
