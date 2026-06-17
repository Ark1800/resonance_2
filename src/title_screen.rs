/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/
use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::label::Label;
use crate::modules::messagebox::MessageBox;
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::prelude::*;

#[allow(unused)]
pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    tm: &TextureManager,
    _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
    records: &mut Vec<DatabaseTable>,
    player: &mut Player,
    client: &DatabaseClient,
    last_scene: &mut String,
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
    background.set_preload(tm.get_preload("assets/map_files/background_start.png").unwrap());
    let btn_new = TextButton::new(virtual_width / 2.0 - 100.0, 300.0, 200.0, 60.0, "New Game", BLUE, GREEN, 30);
    let btn_load = TextButton::new(virtual_width / 2.0 - 100.0, 400.0, 200.0, 60.0, "Load Game", BLUE, GREEN, 30);
    let btn_help = TextButton::new(virtual_width / 2.0 - 100.0, 500.0, 200.0, 60.0, "Controls", BLUE, GREEN, 30);
    let btn_back = TextButton::new(virtual_width / 2.0 - 100.0, 500.0, 200.0, 60.0, "Back to menu", BLUE, GREEN, 30);
    let btn_start = TextButton::new(virtual_width / 2.0 - 100.0, 400.0, 200.0, 60.0, "Start Game", BLUE, GREEN, 30);
    let btn_exit = TextButton::new(virtual_width / 2.0 - 100.0, 600.0, 200.0, 60.0, "Leave game", BLUE, GREEN, 30);
    let mut lbl_title = Label::new("Resonance 2", (virtual_width / 2.0) - 200.0, 200.0, 80);
    let mut lbl_team = Label::new("Made by Team Berry", (virtual_width / 2.0) - 250.0, 250.0, 50);
    let mut txt_username = TextInput::new(virtual_width / 2.0 - 150.0, 200.0, 300.0, 40.0, 25.0);
    txt_username.set_prompt("Username");
    let mut txt_password = TextInput::new(virtual_width / 2.0 - 150.0, 300.0, 300.0, 40.0, 25.0);
    txt_password.set_prompt("Password");
    let mut info_box = MessageBox::info("Controls", "WASD to move.\n\nUp arrow for melee attack.\nRight arrow for ranged attack.");
    let mut load_box = MessageBox::info("Error", "Invalid username or password.");
    let mut new_box = MessageBox::info("Error", "Password already exists.");
    let mut empty_box = MessageBox::info("Error", "Username or password field is empty.");
    let mut speed = 0.0;
    // Main Screen
    let mut start_btns_show = true;
    // New/Load Screen
    let mut txt_inputs_show = false;
    let mut new_load = "None".to_string();
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        background.draw();

        if start_btns_show {
            if btn_new.click() {
                txt_inputs_show = true;
                start_btns_show = false;
                txt_username.set_text("".to_string());
                txt_password.set_text("".to_string());
                new_load = "New".to_string();
            }
            if btn_load.click() {
                txt_inputs_show = true;
                start_btns_show = false;
                txt_username.set_text("".to_string());
                txt_password.set_text("".to_string());
                new_load = "Load".to_string();
            }
            if btn_help.click() {
                info_box.centered();
                info_box.show();
            }
            if btn_exit.click() {
                return "Exit".to_string();
            }
            lbl_title.draw();
            lbl_team.draw();
        } else if txt_inputs_show {
            txt_username.draw();
            txt_password.draw();
            if btn_back.click() {
                txt_username.set_text("".to_string());
                txt_password.set_text("".to_string());
                txt_inputs_show = false;
                start_btns_show = true;
            }
            if btn_start.click() && txt_username.get_text().trim() != "" && txt_password.get_text().trim() != "" {
                if new_load == "New" {
                    let mut proceed = true;
                    for i in 0..records.len() {
                        if records[i].user_name == txt_username.get_text()
                            || txt_username.get_text().trim() == ""
                            || txt_password.get_text().trim() == ""
                        {
                            proceed = false;
                            break;
                        }
                    }
                    if proceed {
                        let new_record = DatabaseTable {
                            id: 0,
                            user_name: txt_username.get_text(),
                            user_password: txt_password.get_text(),
                            player_clearedvar: 0,
                            player_currentscreenvar: "wcs1".to_string(),
                            player_x: (virtual_width / 2.0) as f64,
                            player_y: 100.0,
                            musicoins: 0,
                            inv_1: 0,
                            inv_2: 0,
                            inv_3: 0,
                            inv_4: 0,
                            inv_5: 0,
                            inv_6: 0,
                            inv_7: 0,
                            inv_8: 0,
                            inv_9: 0,
                            inv_10: 0,
                            inv_11: 0,
                            inv_12: 0,
                            inv_13: 0,
                            inv_14: 0,
                            inv_15: 0,
                            inv_16: 0,
                            inv_17: 0,
                            inv_18: 0,
                            inv_19: 0,
                            inv_20: 0,
                            currenthealth: 100.0,
                        };
                        let insert_results = client.insert_record("save_table", &new_record).await;
                        if let Ok(id) = insert_results {
                            // Inserted, id contains the new record's id
                            player.set_save_data(&new_record);
                            player.set_usernamepassword(txt_username.get_text(), txt_password.get_text());
                            return "wcs1".to_string();
                        } else {
                        }
                    } else {
                        if txt_username.get_text().trim() == "" || txt_password.get_text().trim() == "" {
                            empty_box.centered();
                            empty_box.show();
                        } else {
                            new_box.centered();
                            new_box.show();
                        }
                    }
                } else if new_load == "Load"  {
                    let mut proceed = false;
                    let mut active_scene = "None".to_string();
                    for i in 0..records.len() {
                        if records[i].user_name == txt_username.get_text().trim() && records[i].user_password == txt_password.get_text().trim() && txt_username.get_text().trim() != "" && txt_password.get_text().trim() != "" {
                            proceed = true;
                            active_scene = records[i].player_currentscreenvar.clone();
                            player.set_save_data(&records[i].clone());
                            player.set_usernamepassword(txt_username.get_text(), txt_password.get_text());
                            break;
                        }
                    }
                    if !proceed {
                        load_box.centered();
                        load_box.show();
                    } else {
                        *last_scene = "null".to_string();
                        return active_scene;
                    }
                }
            }
        }

        if info_box.is_visible() {
            info_box.draw();
        }
        if new_box.is_visible() {
            new_box.draw();
        }
        if load_box.is_visible() {
            load_box.draw();
        }
        if empty_box.is_visible() {
            empty_box.draw();
        }

        next_frame().await
    }
}
