/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
//use crate::modules::item::Item;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::label::Label;
use crate::modules::map::Map;
use crate::modules::musicdisc::Musicdisc;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::messagebox::MessageBox;
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    musicdiscfunctions: &mut Musicdisc,
    records: &Vec<DatabaseTable>,
    client: &DatabaseClient,
) -> String {
    player.set_currentscreen("town".to_string());
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/wall.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Top" {
        player.set_position(virtual_width / 2.0, virtual_height - 80.0);
    } else if last_scene == "down" {
        player.set_position(virtual_width / 2.0, 80.0);
    } else if last_scene == "Inn" {
        player.set_position(750.0, 300.0);
    } else if last_scene == "Shop" {
        player.set_position(200.0, 240.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
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
    background.set_preload(tm.get_preload("assets/map_files/town.png").unwrap());

    let mut collidable_objects: Vec<StillImage> = vec![
        StillImage::new("", 120.0, 50.0, 0.0, 250.0, true, 1.0).await,
        StillImage::new("", 125.0, 50.0, 250.0, 250.0, true, 1.0).await,
        StillImage::new("", 10.0, 250.0, 250.0, 0.0, true, 1.0).await,
        StillImage::new("", 150.0, 275.0, 675.0, 0.0, true, 1.0).await,
        StillImage::new("", 225.0, 50.0, 800.0, 250.0, true, 1.0).await,
        StillImage::new("", 290.0, 100.0, 0.0, 510.0, true, 1.0).await,
        StillImage::new("", 150.0, 100.0, 260.0, 560.0, true, 1.0).await,
        StillImage::new("", 200.0, 100.0, 600.0, 560.0, true, 1.0).await,
        StillImage::new("", 230.0, 100.0, 790.0, 510.0, true, 1.0).await,
    ];
    for obj in 0..collidable_objects.len() {
        collidable_objects[obj].set_preload(tm.get_preload("assets/map_files/wall.png").unwrap());
    }

    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;

    let mut lbl_speech = Label::new("", 150.0, 610.0, 30);
    lbl_speech.with_colors(WHITE, None);
    if player.get_cleared() == 3 {
        lbl_speech.with_scroll(true);
    }
    let mut speech_cooldown = 0.0;
    let first_line = "I think I hear something to the left of town!".to_string();
    lbl_speech.set_scrolling_text(first_line);

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
    let mut enemies: Vec<Enemy> = vec![];
    player.add_health(30.0);
    let mut info_box = MessageBox::info("Controls", "WASD to move.\n\nUp arrow for melee attack.\nRight arrow for ranged attack.");
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        map.draw_map(&tm).await;
        player.handle_keypresses(pause, musicdiscfunctions).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &collidable_objects);
        if player.get_y() > virtual_height - 10.0 {
            *last_scene = "Down".to_string();
            return "wcs1".to_string();
        } else if player.get_x() < 10.0 {
            *last_scene = "Left".to_string();
            return "w1sp".to_string();
        } else if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "w2sp".to_string();
        } else if player.get_y() < 10.0 {
            *last_scene = "Top".to_string();
            return "w3sp".to_string();
        } else if (player.get_x() > 130.0 && player.get_x() < 200.00) && (player.get_y() > 200.0 && player.get_y() < 270.00) {
            return "shop".to_string();
        } else if (player.get_x() > 700.0 && player.get_x() < 800.0) && (player.get_y() > 250.0 && player.get_y() < 300.00) {
            *last_scene = "Top".to_string();
            return "inn".to_string();
        }
        if player.get_cleared() <= 3 {
            if lbl_speech.get_scroll_len() == lbl_speech.get_scroll() && speech_cooldown <= 0.0 {
                speech_cooldown = 1.5;
            }

            current_time = get_time();
            if (current_time - time_dif) > 0.1 {
                time_dif = current_time;
                if speech_cooldown > 0.0 {
                    speech_cooldown -= 0.1;
                    if speech_cooldown <= 0.0 {
                        lbl_speech.with_scroll(false);
                        lbl_speech.set_text("");
                    }
                }
            }
            if lbl_speech.scroll() {
                lbl_speech.scrolling_text_draw();
                speech_box.draw();
                name_box.draw();
            }
        }
        background.draw();
        player.draw();
        player.handle_player_ui(&mut enemies, musicdiscfunctions).await;
        player.handle_inventory();
        #[allow(unused)]
            let (save, exit) = player.handle_save_menu().await;
        if save {
            player.update_save_data(records, client, last_scene).await;
        }
        if exit {
            return "title_screen".to_string();
        }
        if is_key_pressed(KeyCode::O) {
            info_box.centered();
            info_box.show();
        }
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm).await;
        player.set_player_activedisc(activedisc);
        if last_scene == "title_screen" {
            player.show_player_messagebox();
        }
        next_frame().await;
    }
}
