/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
//use crate::modules::item::Item;
use crate::modules::map::Map;
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
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
    background.set_preload(tm.get_preload("assets/map_files/magma_floor.png").unwrap());
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/magma.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
        map.create_map_array(0, 1, 0, vec![4]).await;
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
        map.create_map_array(0, 1, 0, vec![2]).await;
    } else if last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
        map.create_map_array(0, 1, 0, vec![3]).await;
    } else if last_scene == "Up" {
        player.set_position((virtual_width / 2.0) - 20.0, 80.0);
        map.create_map_array(0, 1, 0, vec![1]).await;
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
        map.create_map_array(0, 0, 0, vec![]).await;
    }
    let mut enemies: Vec<Enemy> = vec![];

    for _i in 0..2 {
        let mut mage = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 100.0, 15.0, "", "mage").await;
        mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());
        mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
        enemies.push(mage);
    }

    /*let mut archerx = 200.0;
    for _i in 0..1 {
        let mut archer = Enemy::new("", 50.0, 50.0, archerx, 200.0, true, 1.0, 10, 5, "", "archer").await;
        archerx += 100.0;
        archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
        enemies.push(archer);
    }*/

    let mut slimey = 200.0;
    for i in 0..3 {
        let mut large_slime = Enemy::new(
            "",
            75.0, //hieght
            75.0, //width
            300.0, //x
            slimey * (i as f32 * 100.0), //y
            true, //stretching
            1.0,  //zoom level
            20.0,   //health
            8.0,    //damage
            "",
            "large_slime",
        )
        .await;
        large_slime.set_preload(tm.get_preload("assets/slime_files/large_slime.png").unwrap());
        enemies.push(large_slime);
    }

    loop {
        player.handle_keypresses(pause).await;
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        if *pause == false {
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            //enemy loop
            for i in 0..enemies.len() {
                //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
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
        player.draw();
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies).await; //dont need to send enemies back because it doesnt get used again until next frame
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
                enemies.remove(index);
            }
        }
        if rnghit {
            enemies[index].dmg_enemy(player.get_rngdmg());
            if enemies[index].get_health() <= 0.0 {
                enemies.remove(index);
            }
        }

        if enemies.is_empty() {
            map.change_map(vec![0, 0], vec![vec![14, 4], vec![14, 5]]);
        }
        player.handle_inventory();
        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "w3s2".to_string();
        }
        if player.get_y() < 10.0 {
            *last_scene = "Down".to_string();
            return "w3sp".to_string();
        }
        next_frame().await;
    }
}
