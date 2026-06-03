/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::animated_image::AnimatedImage;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
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
    200.0, //height
200.0, //width
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
    let mut jeff_basic_cooldown = get_time();
    let mut jeff_count = 0;
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
       // whirlpool.draw();
        jeff.draw();
        map.draw_map(&tm).await;
        player.handle_inventory();
        player.handle_save_menu().await;
        player.handle_keypresses(pause, musicdiscfunctions).await;
        if player.get_cleared() == 4 {
                for i in 0..enemies.len() {
                    //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                    if musicdiscfunctions.get_thickofit_active() == false
                        && musicdiscfunctions.get_pandemonium_active() == false
                        && musicdiscfunctions.get_sodapop_active() == false
                    {
                        match enemies[i].get_enemy_type() {
                        "jeff" => {
                            (jeff_valid, jeff_count) = enemies[i].jeff_checkhit(player, jeff_valid, jeff_count);
                            if jeff_valid && jeff_count == 0 {
                                jeff_basic_cooldown = get_time();
                                jeff.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_idle2.gif").unwrap(), true);
                            }
                            if jeff_valid {
                                if get_time() - jeff_basic_cooldown > 2.0 {
                                    println!("jeff attack");
                                    jeff_basic_cooldown = get_time();
                                }
                            }
                        }
                        _ => {}
                    }
                    enemies[i].draw();
                }
            }
         }
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &collidable_objects);
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        if mlehit {
            jeff_valid = true;
            jeff_count += 1;
            enemies[index].dmg_enemy(player.get_meleedmg());
            if enemies[index].get_health() <= 0.0 {
                enemies[index].add_gold(player);
                enemies.remove(index);
            }
        }
        if rnghit {
            jeff_valid = true;
            jeff_count += 1;
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
