/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use crate::modules::musicdisc::Musicdisc;
use crate::modules::item::Item;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    musicdiscfunctions: &mut Musicdisc,
    town_completed: &mut bool,
) -> String {
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
    if !*town_completed {
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
    let backinblackitem = Item::new(tm.get_preload("assets/musicdisc_files/covers/backinblack.png").unwrap(), "assets/musicdisc_files/covers/backinblack.png".to_string(), "Back In Black".to_string(), "A Disc that allows the user to summon periodic pillars of fire".to_string(), "disc".to_string(), 0, 0, 0.0, 0.0, 0, 0).await;
    player.add_inventory_item(backinblackitem.clone());
    let thickofititem = Item::new(tm.get_preload("assets/musicdisc_files/covers/thickofit.png").unwrap(), "assets/musicdisc_files/covers/thickofit.png".to_string(), "Thick Of It".to_string(), "A Disc that sounds so bad all enemies stop attacking and move away, enemies hate it so much they will teleport away if need be".to_string(), "disc".to_string(), 0, 0, 0.0, 0.0, 0, 0).await;
    player.add_inventory_item(thickofititem.clone());
    let howitsdoneitem = Item::new(tm.get_preload("assets/musicdisc_files/covers/howitsdone.png").unwrap(), "assets/musicdisc_files/covers/howitsdone.png".to_string(), "How It's Done".to_string(), "A Disc that puts the user into a flow state multiplying all stats largely making the user near invincible".to_string(), "disc".to_string(), 0, 0, 0.0, 0.0, 0, 0).await;
    player.add_inventory_item(howitsdoneitem.clone());
    let mut pandemoniumitem = Item::new(tm.get_preload("assets/musicdisc_files/covers/pandemonium.png").unwrap(), "assets/musicdisc_files/covers/pandemonium.png".to_string(), "Pandemonium".to_string(), "A Disc that causes extreme confusion, making all enemies attack the highest health enemy on screen".to_string(), "disc".to_string(), 0, 0, 0.0, 0.0, 0, 0).await;
    player.add_inventory_item(pandemoniumitem.clone());
    let mut sixhundredstrikeitem = Item::new(tm.get_preload("assets/musicdisc_files/covers/sixhundredstrike.png").unwrap(), "assets/musicdisc_files/covers/sixhundredstrike.png".to_string(), "Six Hundred Strike".to_string(), "A Disc that calls upon the wrath of odysseus to strike down the highest opponent for massive damage periodically".to_string(), "disc".to_string(), 0, 0, 0.0, 0.0, 0, 0).await;
    player.add_inventory_item(sixhundredstrikeitem.clone());
    let mut sodapopitem = Item::new(tm.get_preload("assets/musicdisc_files/covers/sodapop.png").unwrap(), "assets/musicdisc_files/covers/sodapop.png".to_string(), "Soda Pop".to_string(), "A Disc that forces all enemies to stop and dance for 10 seconds".to_string(), "disc".to_string(), 0, 0, 0.0, 0.0, 0, 0).await;
    player.add_inventory_item(sodapopitem.clone());
    let mut greatestshowitem = Item::new(tm.get_preload("assets/musicdisc_files/covers/greatestshowman.png").unwrap(), "assets/musicdisc_files/covers/greatestshowman.png".to_string(), "The Greatest Show".to_string(), "A Disc that calls upon the power of the greatest showman, summoning a meteor that gets bigger the longer you arent hit".to_string(), "disc".to_string(), 0, 0, 0.0, 0.0, 0, 0).await;
    player.add_inventory_item(greatestshowitem.clone());
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
            return "w3s1".to_string();
        } else if (player.get_x() > 130.0 && player.get_x() < 200.00) && (player.get_y() > 200.0 && player.get_y() < 270.00) {
            return "shop".to_string();
        }

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
        background.draw();
        draw_grid(50.0, BLACK);
        player.handle_player_ui(&mut enemies, musicdiscfunctions).await;
        player.handle_inventory();
        player.handle_save_menu().await;
        player.handle_playerdamaging(&enemies);
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        player.draw();
        next_frame().await;
    }
}
