/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::animated_image::AnimatedImage;
use crate::modules::collision::check_collision;
use crate::modules::item;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use crate::modules::label::Label;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::enemy::Enemy;
use crate::modules::text_button::TextButton;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
    records: &Vec<DatabaseTable>,
    client: &DatabaseClient

) -> String {
    player.set_currentscreen("w1sb".to_string());
    let mut jeff_valid = false;
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Down" {
        player.set_position(virtual_width / 2.0, virtual_height - 80.0);
    } else if last_scene == "Up" {
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
    background.set_preload(tm.get_preload("assets/map_files/world1/beach2.png").unwrap());
    let mut whirlpool = AnimatedImage::from_gif("", (virtual_width / 2.0) - 200.0, (virtual_height / 2.0) - 200.0, 250.0, 250.0, true).await;
    let mut whirlpool_hitbox = StillImage::new("", 250.0, 250.0, (virtual_width / 2.0) - 200.0, (virtual_height / 2.0) - 200.0, true, 1.0).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/map_files/world1/whirlpool.gif") {
        whirlpool.set_preloaded_gif(preloaded, true);
    }
    whirlpool_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap());
    let collidable_objects = vec![];
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec![
            "assets/map_files/world1/watertile.png".to_string(),
            "assets/map_files/chest.png".to_string(),
        ],
    )
    .await;
    map.create_map_array(0, 1, 0, vec![4]).await;
    let mut enemies: Vec<crate::modules::enemy::Enemy> = vec![];
    let mut jeff = Enemy::new(
    "",
    150.0, //height
    150.0, //width
    300.0, //x
    300.0, //y
    true, //stretching
    1.0, //zoom level
    200.0, //health
    10.0, //damage
    "",
    "jeff_the_behemoth"//enemy type
    ).await;
    jeff.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_idleR.gif").unwrap(), true);
    enemies.push(jeff);
    let mut jeff_start_fight_cooldown = 0.0;
    let mut jeff_attacktime = 0.0;
    let mut jeff_attackcount = 0;
    let mut jeff_attackvalid = false;
    let mut jeff_drawvalid = true;
    let mut lbl_warninglabel = Label::new("", -50.0, -100.0, 30);
    let mut jeff_knifeattack_wallchoice = 0;
    let mut jeff_knife_direction = Vec2::new(0.0, 0.0);
    let mut attack_choice = 0;
    let mut jeff_cooldown = 0.0;
    let mut jeff_zzz = AnimatedImage::from_gif("", 0.0, 0.0, 0.0, 0.0, true).await;
    let mut bubblebeam_img = StillImage::new("", 0.0, 0.0, 0.0, 0.0, true, 1.0).await;
    let mut whirlpool_direction = Vec2::new(1.0, 1.0);
    let mut choose_open = false;
    let mut item_valid = false;
    let mut run_once = true;
    let mut jeff_on_cooldown = false;
    let mut jeff_img_heart = StillImage::new(
        "", 
        100.0, // width
        50.0,  // height
        60.0, // x position //offset as drawn from center
        virtual_height-50.0,   // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    jeff_img_heart.set_preload(tm.get_preload("assets/world1_boss/jeff_heart.png").unwrap());
    let mut lbl_jeff_healthbar = Label::new("", 120.0, virtual_height-10.0, 30);
    lbl_jeff_healthbar.with_fixed_size(800.0, 25.0);
    lbl_jeff_healthbar.with_colors(WHITE, Some(BLUE));
    lbl_jeff_healthbar.with_border(BLACK, 2.0);
    let mut lbl_jeff_healthbarbg = Label::new("", 120.0, virtual_height-10.0, 30);
    lbl_jeff_healthbarbg.with_fixed_size(800.0, 25.0);
    lbl_jeff_healthbarbg.with_colors(WHITE, Some(WHITE));
    lbl_jeff_healthbarbg.with_border(BLACK, 2.0);
    let mut lbl_jeff_healthnum = Label::new("200", 90.0, virtual_height-20.0, 30);
    let mut lbl_jeff_name = Label::new("Jeff The Landshark", 135.0, virtual_height-45.0, 30);
    lbl_jeff_name.with_colors(BLACK, Some(BLUE));
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
       // whirlpool.draw();
        map.draw_map(&tm).await;
        player.handle_inventory();
        let (save, exit) = player.handle_save_menu().await;
        if save {
            println!("Saving game...");
            player.update_save_data(records, client, last_scene).await;
        } if exit {
            return "title_screen".to_string();
        }
        let (restart, quit) = player.handle_death_screen(pause, musicdiscfunctions).await;
        if restart {
            *last_scene = "None".to_string();
            return "inn".to_string();
        } if quit {
            return "main_screen".to_string();
        }
        player.handle_keypresses(pause, musicdiscfunctions).await;
        if player.get_cleared() <= 7 {
            for i in 0..enemies.len() {
                //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                if musicdiscfunctions.get_thickofit_active() == false && musicdiscfunctions.get_pandemonium_active() == false && musicdiscfunctions.get_sodapop_active() == false {
                    (jeff_valid, jeff_attackvalid) = enemies[i].jeff_checkhit(player, jeff_valid, jeff_attackvalid);
                    if jeff_valid && jeff_attackvalid && run_once {
                        jeff_start_fight_cooldown = get_time();
                        enemies[0].set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_serious.gif").unwrap(), true);
                        jeff_attackvalid = false;
                        run_once = false;
                    }
                    if jeff_valid && player.get_health() > 0.0 {
                        if get_time() - jeff_start_fight_cooldown > 3.0 { //length of gif
                            if jeff_attackcount == 0 {
                                attack_choice = enemies[0].jeff_choose_attack();
                                jeff_attackcount += 1;
                            }
                        }
                        match attack_choice {
                            1 => { //jeff knife dash
                                if jeff_attackcount == 1 {
                                    jeff_attacktime = get_time();
                                    enemies[0].set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_knife.gif").unwrap(), true);
                                    jeff_attackcount += 1;
                                }
                                let time = get_time() - jeff_attacktime;
                                if time >= 1.0  && time < 3.0 {
                                    jeff_drawvalid = false;
                                    if jeff_attackcount == 2 {
                                        (jeff_knifeattack_wallchoice, lbl_warninglabel) = enemies[0].jeff_knifeattack1();
                                        jeff_attackcount += 1;
                                    }
                                    lbl_warninglabel.draw();
                                }
                                if time >= 3.0 && jeff_attackcount == 3 {
                                    jeff_drawvalid = true;
                                    jeff_knife_direction = enemies[0].jeff_knifeattack2(jeff_knifeattack_wallchoice, &mut lbl_warninglabel);
                                    jeff_attackcount += 1;
                                }
                                if time >= 3.0 && jeff_attackcount == 4 {
                                    enemies[0].set_move_speed(1200.0);
                                    let attackend = enemies[0].jeff_knifeattack3(player, jeff_knife_direction, musicdiscfunctions);
                                    if attackend {
                                        jeff_attackcount += 1;;
                                        enemies[0].jeff_normalidle(player, tm);
                                        enemies[0].set_move_speed(200.0);
                                    }
                                }
                                if jeff_attackcount == 5 {
                                    (jeff_cooldown, jeff_zzz) = enemies[0].jeff_cooldown(tm).await;
                                    jeff_on_cooldown = true;
                                    jeff_attackcount += 1;
                                }
                                if jeff_on_cooldown {
                                    if get_time() - jeff_cooldown > 0.0 && get_time() - jeff_cooldown < 4.0 {
                                        jeff_zzz.draw();
                                    }
                                    if get_time() - jeff_cooldown >= 2.0 {
                                        jeff_attackcount = 0;
                                        jeff_zzz = AnimatedImage::from_gif("", 0.0, 0.0, 0.0, 0.0, true).await;
                                        lbl_warninglabel = Label::new("", -50.0, -100.0, 30);
                                        jeff_on_cooldown = false;
                                    }
                                }
                            }
                            2 => { //jeff bubble beam
                                if jeff_attackcount == 1 {
                                    jeff_attacktime = get_time();
                                    enemies[0].set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_full.gif").unwrap(), true);
                                    jeff_attackcount += 1;
                                }
                                let time = get_time() - jeff_attacktime;
                                if time >= 1.0 && time < 3.0 {
                                    if jeff_attackcount == 2 {
                                    lbl_warninglabel = enemies[0].jeff_bubblebeam1(tm);
                                    jeff_attackcount += 1;
                                    }
                                    lbl_warninglabel.draw();
                                }
                                if time >= 3.0 && time < 4.0 {
                                    if jeff_attackcount == 3 {
                                        bubblebeam_img = enemies[0].jeff_bubblebeam2(player, &mut lbl_warninglabel, tm, musicdiscfunctions).await;
                                        jeff_attackcount += 1;
                                    }
                                    bubblebeam_img.draw();
                                }
                                if time > 4.0 && jeff_attackcount == 4 {
                                    jeff_attackcount += 1;
                                }
                                if jeff_attackcount == 5 {
                                    (jeff_cooldown, jeff_zzz) = enemies[0].jeff_cooldown(tm).await;
                                    jeff_on_cooldown = true;
                                    jeff_attackcount += 1;
                                }
                                if jeff_on_cooldown {
                                    if get_time() - jeff_cooldown > 0.0 && get_time() - jeff_cooldown < 4.0 {
                                        jeff_zzz.draw();
                                    }
                                    if get_time() - jeff_cooldown >= 2.0 {
                                        jeff_attackcount = 0;
                                        jeff_zzz = AnimatedImage::from_gif("", 0.0, 0.0, 0.0, 0.0, true).await;
                                        lbl_warninglabel = Label::new("", -50.0, -100.0, 30);
                                        jeff_on_cooldown = false;
                                    }
                                }
                            }
                            3 => { //jeff whirlpool bounce
                                if jeff_attackcount == 1 {
                                    jeff_attacktime = get_time();
                                    enemies[0].set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_idle1.gif").unwrap(), true);
                                    jeff_attackcount += 1;
                                }
                                let time = get_time() - jeff_attacktime;
                                if time >= 1.0 && time < 10.0 {
                                    if jeff_attackcount == 2 {
                                    jeff_drawvalid = false;
                                    jeff_attackcount += 1;
                                    }
                                    whirlpool_direction = enemies[0].jeff_whirlpoolbounce(player, &mut whirlpool, &mut whirlpool_hitbox, &mut map, whirlpool_direction, musicdiscfunctions)
                                }
                                if time >= 10.0 && jeff_attackcount == 3 {
                                    jeff_attackcount += 1;
                                }
                                if jeff_attackcount == 4 {
                                    jeff_drawvalid = true;
                                    (jeff_cooldown, jeff_zzz) = enemies[0].jeff_cooldown(tm).await;
                                    jeff_on_cooldown = true;
                                    jeff_attackcount += 1;
                                }
                                if jeff_on_cooldown {
                                    if get_time() - jeff_cooldown > 0.0 && get_time() - jeff_cooldown < 4.0 {
                                        jeff_zzz.draw();
                                    }
                                    if get_time() - jeff_cooldown >= 2.0 {
                                        jeff_attackcount = 0;
                                        jeff_zzz = AnimatedImage::from_gif("", 0.0, 0.0, 0.0, 0.0, true).await;
                                        jeff_on_cooldown = false;
                                        let x = (virtual_width / 2.0) - 200.0;
                                        let y = (virtual_height / 2.0) - 200.0;
                                        let position = Vec2::new(x, y);
                                        whirlpool.set_position(x, y);
                                        whirlpool_direction = Vec2::new(1.0, 1.0);  
                                        whirlpool_hitbox.set_position(position);
                                    }
                                }
                            } 
                            _ => {}
                        }
                        let mut new_width = enemies[0].get_health() as f32 * 4.0; // Assuming 100 health corresponds to 400 width
                        let max_width = 200 as f32 * 4.0; // Maximum width based on max health
                        if new_width < 0.0 {
                            new_width = 0.0; // Prevent negative width
                        }
                        lbl_jeff_healthbarbg.with_fixed_size(max_width, 25.0); //update healthbar size based on health
                        lbl_jeff_healthbar.with_fixed_size(new_width, 25.0); //update healthbar size based on health
                        lbl_jeff_healthbarbg.draw();
                        lbl_jeff_healthbar.draw();
                        jeff_img_heart.draw();
                        lbl_jeff_healthnum.set_text(enemies[0].get_health().to_string());
                        lbl_jeff_healthnum.draw();
                        lbl_jeff_name.draw();
                    };
                }
            }
        }
        if enemies[0].get_health() <= 0.0 && player.get_cleared() <= 7 {
            jeff_valid = false;
            player.add_cleared();
            item_valid = true;
            choose_open = true;
            map.change_map(vec![0, 0], vec![vec![7, 0], vec![6, 0]]);
            player.addcoins(100);
            enemies[0].set_position(60.0, 40.0);
            enemies[0].set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_sad.gif").unwrap(), true);
        }
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &collidable_objects);
        if jeff_drawvalid {
            enemies[0].draw();
        }
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        if mlehit {
            jeff_valid = true;
            jeff_attackvalid = true;
            enemies[index].dmg_enemy(player.get_meleedmg());
        }
        if rnghit {
            jeff_valid = true;
            jeff_attackvalid = true;
            enemies[index].dmg_enemy(player.get_rngdmg());
        }
        player.draw();
        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "w1s4".to_string();
        }
        if player.get_y() < 10.0 {
            *last_scene = "Up".to_string();
            return "w1sp".to_string();
        }
        (choose_open, item_valid) = player.handle_choose_item(&mut choose_open, &mut item_valid);
        next_frame().await;
    }
}
