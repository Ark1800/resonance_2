/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::{self, Enemy};
use crate::modules::item::Item;
use crate::modules::map::Map;
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use crate::modules::collision::check_collision;
pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut Player,
    tm: &TextureManager,
    pause: &mut bool,
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
    background.set_preload(tm.get_preload("assets/map_files/world1/beach2.png").unwrap());
    if last_scene == "Right" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Left" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Down" {
        player.set_position((virtual_width / 2.0)-20.0, virtual_height - 80.0);
    } else if last_scene == "Top" {
        player.set_position((virtual_width / 2.0)-20.0, 80.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
    println!("Last scene: {}", last_scene);
    let mut enemies: Vec<Enemy> = vec![];
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/world1/watertile.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
    map.create_map_array(0, 2, 0, vec![1, 4]).await;
    let mut summoner = Enemy::new("", 50.0, 50.0, 70.0, 80.0, true, 1.0, 20, 10, "").await;
    let mut large_slime = Enemy::new("", 75.0, 75.0, 150.0, 200.0, true, 1.0, 20, 10, "").await;
    large_slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
    large_slime.set_enemy_type("large_slime");
    summoner.set_preload(tm.get_preload("assets/summoner_files/summoner_standR.png").unwrap());
    summoner.set_enemy_type("summoner");

    let mut mage = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10, "").await;

    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());
    mage.set_enemy_type("mage");
    mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
    enemies.push(summoner);
    enemies.push(mage);
    enemies.push(large_slime);
    let mut archerx = 200.0;
    for _i in 0..3 {
        let mut archer = Enemy::new("", 50.0, 50.0, archerx, 200.0, true, 1.0, 10, 5, "").await;
        archerx += 100.0;
        archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
        archer.set_enemy_type("archer");
        enemies.push(archer);
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
            for i in 0..enemies.len() { //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                match enemies[i].get_enemy_type() {
                    "archer" => {
                        enemies[i].archer_action(tm, player).await;
                        enemies[i].draw_bullet(player);
                        let arrow_list = enemies[i].get_projectiles();
                        for arrow in 0..arrow_list.len() {                                            
                            let collision = check_collision(arrow_list[arrow].view_player(), player.view_player(), 1);
                            if collision {
                                player.dmgplayer(enemies[i].get_dmg());
                                enemies[i].remove_projectile(arrow);
                                break;
                            }
                        }
                    }
                    "slime" => {
                        enemies[i].moveing(player.get_x(), player.get_y());
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
                        let fireball_list = enemies[i].get_projectiles();
                        for fireball in 0..fireball_list.len() {
                            let collision = check_collision(fireball_list[fireball].view_player(), player.view_player(), 1);
                            if collision {
                                player.dmgplayer(enemies[i].get_dmg());
                                enemies[i].remove_projectile(fireball);
                                break;
                            }
                        }
                    }
                    "large_slime" => {
                        let (slime1, slime2, split) = enemies[i].large_slime_action(tm, player).await;
                        if split {
                            enemies.push(slime1);
                            enemies.push(slime2);
                        }
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
           if enemies[index].get_health() <= 0 {
                enemies.remove(index);
            }
        }
        if rnghit {
           enemies[index].dmg_enemy(player.get_rngdmg());
           if enemies[index].get_health() <= 0 {
              enemies.remove(index);
           }
        }
        player.handle_inventory();
        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Left".to_string();
            return "w1sp".to_string();
        }
        if player.get_y() < 10.0 {
            *last_scene = "Down".to_string();
            return "w1s2".to_string();
        }
        next_frame().await;
    
    }
}