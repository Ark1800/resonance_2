/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::animated_image::AnimatedImage;
use crate::modules::collision::check_collision;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use crate::modules::label::Label;
use crate::modules::enemy::Enemy;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
) -> String {
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
    let mut whirlpool = AnimatedImage::from_gif("", (virtual_width / 2.0) - 200.0, (virtual_height / 2.0) - 200.0, 400.0, 400.0, true).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/map_files/world1/whirlpool.gif") {
        whirlpool.set_preloaded_gif(preloaded, true);
    }
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
    let mut jeff_start_fight_cooldown = 0.0;
    let mut jeff_attacktime = 0.0;
    let mut jeff_attackcount = 0;
    let mut jeff_attackvalid = false;
    let mut jeff_drawvalid = true;
    let mut lbl_warninglabel = Label::new("", -50.0, -100.0, 30);
    let mut jeff_knifeattack_wallchoice = 0;
    let mut jeff_knife_direction = Vec2::new(0.0, 0.0);
    let mut knife_choice = 0;
    let mut jeff_cooldown = 0.0;
    let mut jeff_zzz = AnimatedImage::from_gif("", 0.0, 0.0, 0.0, 0.0, true).await;
    let mut bubblebeam_img = StillImage::new("", 0.0, 0.0, 0.0, 0.0, true, 1.0).await;
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
       // whirlpool.draw();
        if jeff_drawvalid {
            jeff.draw();
        }
        map.draw_map(&tm).await;
        player.handle_inventory();
        player.handle_save_menu().await;
        let (restart, quit) = player.handle_death_screen(pause).await;
        if restart {
            return "w1sp".to_string();
        } if quit {
            return "main_screen".to_string();
        }
        player.handle_keypresses(pause, musicdiscfunctions).await;
        if player.get_cleared() == 7 {
                for i in 0..enemies.len() {
                    //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                    if musicdiscfunctions.get_thickofit_active() == false && musicdiscfunctions.get_pandemonium_active() == false && musicdiscfunctions.get_sodapop_active() == false {
                    (jeff_valid, jeff_attackvalid) = enemies[i].jeff_checkhit(player, jeff_valid, jeff_attackvalid);
                    if jeff_valid && jeff_attackvalid {
                        jeff_start_fight_cooldown = get_time();
                        jeff.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_idle2.gif").unwrap(), true);
                        jeff_attackvalid = false;
                    }
                    if jeff_valid {
                        if get_time() - jeff_start_fight_cooldown > 3.0 { //length of gif
                            if jeff_attackcount == 0 {
                                knife_choice = jeff.jeff_choose_attack();
                                jeff_attackcount += 1;
                            }
                            match knife_choice {
                                1 => { //jeff knife dash
                                    if jeff_attackcount == 1 {
                                        jeff_attacktime = get_time();
                                        jeff.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_knife.gif").unwrap(), true);
                                        jeff_attackcount += 1;
                                    }
                                    let time = get_time() - jeff_attacktime;
                                    if time >= 1.0  && time < 3.0 {
                                        if jeff_attackcount == 2 {
                                            (jeff_knifeattack_wallchoice, lbl_warninglabel) = jeff.jeff_knifeattack1();
                                            jeff_drawvalid = false;
                                            jeff_attackcount += 1;
                                        }
                                        lbl_warninglabel.draw();
                                    }
                                    if time >= 3.0 && jeff_attackcount == 3 {
                                        jeff_drawvalid = true;
                                        jeff_knife_direction = jeff.jeff_knifeattack2(jeff_knifeattack_wallchoice, &mut lbl_warninglabel);
                                        jeff_attackcount += 1;
                                    }
                                    if time >= 3.0 && jeff_attackcount == 4 {
                                        let attackend = jeff.jeff_knifeattack3(player, jeff_knife_direction);
                                        if attackend {
                                            jeff_attackcount += 1;;
                                            jeff.jeff_normalidle(player, tm);
                                        }
                                    }
                                    if time >= 3.0 && jeff_attackcount == 5 {
                                        (jeff_cooldown, jeff_zzz) = jeff.jeff_cooldown(tm).await;
                                    }
                                    if jeff_cooldown - get_time() > 0.0 && jeff_cooldown - get_time() < 4.0 {
                                        jeff_zzz.draw();
                                    }
                                    if jeff_cooldown - get_time() >= 3.0 {
                                        jeff_attackcount = 0;
                                        jeff_valid = false;
                                        jeff_attackvalid = false;
                                        jeff_zzz = AnimatedImage::from_gif("", 0.0, 0.0, 0.0, 0.0, true).await;
                                        lbl_warninglabel = Label::new("", -50.0, -100.0, 30);
                                    }
                                    }
                                2 => { //jeff bubble beam
                                    if jeff_attackcount == 1 {
                                        jeff_attacktime = get_time();
                                        jeff.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_full.gif").unwrap(), true);
                                        jeff_attackcount += 1;
                                    }
                                    let time = get_time() - jeff_attacktime;
                                    if time >= 1.0 && time < 3.0 {
                                        if jeff_attackcount == 2 {
                                        lbl_warninglabel = jeff.jeff_bubblebeam1(tm);
                                        jeff_attackcount += 1;
                                        }
                                        lbl_warninglabel.draw();
                                    }
                                    if time >= 3.0 && time < 4.0 {
                                        if jeff_attackcount == 3 {
                                            bubblebeam_img = jeff.jeff_bubblebeam2(player, &mut lbl_warninglabel, tm).await;
                                            jeff_attackcount += 1;
                                        }
                                        bubblebeam_img.draw();
                                    }
                                    if time >= 4.0 && jeff_attackcount == 4 {
                                        (jeff_cooldown, jeff_zzz) = jeff.jeff_cooldown(tm).await;
                                    }
                                    if jeff_cooldown - get_time() > 0.0 && jeff_cooldown - get_time() < 4.0 {
                                        jeff_zzz.draw();
                                    }
                                    if jeff_cooldown - get_time() >= 3.0 {
                                        jeff_attackcount = 0;
                                        jeff_valid = false;
                                        jeff_attackvalid = false;
                                        jeff_zzz = AnimatedImage::from_gif("", 0.0, 0.0, 0.0, 0.0, true).await;
                                        lbl_warninglabel = Label::new("", -50.0, -100.0, 30);
                                    }
                                }
                                3 => {} //jeff whirlpool bounce
                                _ => {}
                            }
                        };
                    }
                }
                    enemies[i].draw();
                }
            }
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &collidable_objects);
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        if enemies.is_empty() && player.get_cleared() == 7 {
            player.add_cleared();
            map.change_map(vec![0, 0], vec![vec![7, 0], vec![6, 0]]);
        }
        if mlehit {
            jeff_valid = true;
            jeff_attackvalid = true;
            enemies[index].dmg_enemy(player.get_meleedmg());
            if enemies[index].get_health() <= 0.0 {
                enemies[index].add_gold(player);
                enemies.remove(index);
            }
        }
        if rnghit {
            jeff_valid = true;
            jeff_attackvalid = true;
            enemies[index].dmg_enemy(player.get_rngdmg());
            if enemies[index].get_health() <= 0.0 {
                enemies[index].add_gold(player);
                enemies.remove(index);
            }
        }
        player.draw();
        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "w1s4".to_string();
        }
        next_frame().await;
    }
}
