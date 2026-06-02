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
    let mut collidable_objects: Vec<StillImage> = vec![
        StillImage::new(
            "",
            350.0,                         // width
            350.0,                         // height
            (virtual_width / 2.0) - 180.0, // x position
            (virtual_width / 2.0) - 300.0, // y position
            true,                          // Enable stretching
            1.0,                           // Normal zoom (100%)
        )
        .await,
    ];
    for obj in 0..collidable_objects.len() {
        collidable_objects[obj].set_preload(tm.get_preload("assets/map_files/wall.png").unwrap());
    }
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
    let mut jeff_the_behemoth = Enemy::new(
    "",
    75.0, //height
    75.0, //width
    70.0, //x
    80.0, //y
    true, //stretching
    1.0, //zoom level
    200.0, //health
    10.0, //damage
    "",
    "jeff_the_behemoth"//enemy type
    ).await;
    
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        whirlpool.draw();
        map.draw_map(&tm).await;
        player.handle_inventory();
        player.handle_save_menu().await;
        player.handle_keypresses(pause, musicdiscfunctions).await;
        if player.get_cleared() == 1 {
                for i in 0..enemies.len() {
                    //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                    if musicdiscfunctions.get_thickofit_active() == false
                        && musicdiscfunctions.get_pandemonium_active() == false
                        && musicdiscfunctions.get_sodapop_active() == false
                    {
                                            match enemies[i].get_enemy_type() {
                        "archer" => {
                            enemies[i].archer_action(tm, player).await;
                            enemies[i].draw_bullet(player);
                        }
                        "slime" => {
                            enemies[i].slime_action(player);
                        }
                        "summoner" => {
                            let (slime1, slime2, slime3, summoned) = enemies[i].summoner_action(tm, player).await;
                            if summoned {
                                enemies.push(slime1);
                                enemies.push(slime2);
                                enemies.push(slime3);
                            }
                        }
                        "mage" => {
                            enemies[i].mage_action(tm, player).await;
                            enemies[i].draw_bullet(player);
                        }
                        "large_slime" => {
                            enemies[i].large_slime_action(tm, player).await;
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
            enemies[index].dmg_enemy(player.get_meleedmg());
            if enemies[index].get_health() <= 0.0 {
                if enemies[index].get_enemy_type() == "large_slime" {
                    let (slime1, slime2, split) = enemies[index].large_slime_action(tm, player).await;
                    if split {
                        enemies.push(slime1);
                        enemies.push(slime2);
                    }
                }
                enemies[index].add_gold(player);
                enemies.remove(index);
            }
        }
        if rnghit {
            enemies[index].dmg_enemy(player.get_rngdmg());
            if enemies[index].get_health() <= 0.0 {
                if enemies[index].get_enemy_type() == "large_slime" {
                    let (slime1, slime2, split) = enemies[index].large_slime_action(tm, player).await;
                    if split {
                        enemies.push(slime1);
                        enemies.push(slime2);
                    }
                }
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
