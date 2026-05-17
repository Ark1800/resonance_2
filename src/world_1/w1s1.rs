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
    background.set_preload(tm.get_preload("assets/map_files/world1/beachtile.png").unwrap());
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
    let mut archer_list: Vec<Enemy> = vec![];
    let mut slime_list: Vec<Enemy> = vec![];
    let mut summoner_list: Vec<Enemy> = vec![];
    let mut mage_list: Vec<Enemy> = vec![];
    let mut large_slime_list: Vec<Enemy> = vec![];
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/world1/watertile.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
    map.create_map_array(0, 2, 0, vec![1, 4]).await;
    let mut summoner = Enemy::new("", 50.0, 50.0, 70.0, 80.0, true, 1.0, 20, 10, "").await;
    let mut large_slime = Enemy::new("", 75.0, 75.0, 150.0, 200.0, true, 1.0, 20, 10, "").await;
    large_slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
    large_slime.set_enemy_type("large_slime");
    large_slime.set_enemy_count(1);
    summoner.set_preload(tm.get_preload("assets/summoner_files/summoner_standR.png").unwrap());
    summoner.set_enemy_type("summoner");
    summoner.set_enemy_count(1);

    let mut mage = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10, "").await;

    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());
    mage.set_enemy_type("mage");
    mage.set_enemy_count(1);
    mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
    summoner_list.push(summoner);
    mage_list.push(mage);
    large_slime_list.push(large_slime);
    let mut archerx = 200.0;
    for i in 0..3 {
        let mut archer = Enemy::new("", 50.0, 50.0, archerx, 200.0, true, 1.0, 10, 5, "").await;
        archerx += 100.0;
        archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
        archer.set_enemy_type("archer");
        archer.set_enemy_count(1+i);
        archer_list.push(archer);
    }
    let mut enemy_lists: Vec<Vec<Enemy>> = vec![];
    enemy_lists.push(archer_list);
    enemy_lists.push(slime_list);
    enemy_lists.push(summoner_list);
    enemy_lists.push(mage_list);
    enemy_lists.push(large_slime_list);
    loop {
        let mut enemies: Vec<Enemy> = vec![];
        player.handle_keypresses(pause).await;                                          
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        if *pause == false {
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            for enemy_list in 0..enemy_lists.len() {                   
                let (left_lists, right_lists) = enemy_lists.split_at_mut(enemy_list);
                let current_list = &mut right_lists[0]; //right list needs to be split off so baby slimes can be added 
                for enemy in (0..current_list.len()).rev() {
                    let enemy_type = current_list[enemy].get_enemy_type().to_string();
                    match enemy_type.as_str() {
                        "archer" => {
                            current_list[enemy].archer_action(tm, player).await;
                            current_list[enemy].draw();
                            current_list[enemy].draw_bullet(player);
                            let arrow_list = current_list[enemy].get_projectiles();
                            for arrow in 0..arrow_list.len() {                                            
                                let collision = check_collision(arrow_list[arrow].view_player(), player.view_player(), 1);
                                if collision {
                                    player.dmgplayer(current_list[enemy].get_dmg());
                                    current_list[enemy].remove_projectile(arrow);
                                    break;
                                }
                            }
                        }
                        "slime" => {
                            current_list[enemy].moveing(player.get_x(), player.get_y());
                            current_list[enemy].draw();
                        }
                        "summoner" => {
                            current_list[enemy].summoner_action(tm, player, &mut left_lists[1]).await;
                            current_list[enemy].draw();
                        }
                        "mage" => {
                            current_list[enemy].mage_action(tm, player).await;
                            current_list[enemy].draw();
                            current_list[enemy].draw_bullet(player);
                            let fireball_list = current_list[enemy].get_projectiles();
                            for fireball in 0..fireball_list.len() {
                                let collision = check_collision(fireball_list[fireball].view_player(), player.view_player(), 1);
                                if collision {
                                    player.dmgplayer(current_list[enemy].get_dmg());
                                    current_list[enemy].remove_projectile(fireball);
                                    break;
                                }
                            }
                        }
                        "large_slime" => {
                            current_list[enemy].large_slime_action(tm, player, &mut left_lists[1]);
                            current_list[enemy].draw();
                        }
                        _ => {}
                    }
                    
                }
            }
        }
        player.draw();
        for enemy_list in 0..enemy_lists.len() {
            for enemy in 0..enemy_lists[enemy_list].len() {
                enemies.push(enemy_lists[enemy_list][enemy].clone());
            }
        }
        let (mlehit, rnghit, index, mut enemies) = player.handle_player_ui(&mut enemies).await;
        if mlehit {
            damage_enemy(&mut enemy_lists, &mut enemies[index], player.get_meleedmg());
        }
        if rnghit {
            damage_enemy(&mut enemy_lists, &mut enemies[index], player.get_rngdmg());
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

pub fn damage_enemy(enemy_lists: &mut Vec<Vec<Enemy>>, enemy: &mut Enemy, dmg: i32) {
        match enemy.get_enemy_type() {
            "archer" => {
                for i in 0..enemy_lists[0].len() {
                    if enemy_lists[0][i].get_enemy_count() == enemy.get_enemy_count() {
                        let dead = enemy_lists[0][i].dmg_enemy(dmg);
                        if dead {
                            enemy_lists[0].remove(i);
                        }
                        break;
                    }
                }
            }
            "slime" => {
                for i in 0..enemy_lists[1].len() {
                    if enemy_lists[1][i].get_enemy_count() == enemy.get_enemy_count() {
                        let dead = enemy_lists[1][i].dmg_enemy(dmg);
                        if dead {
                            enemy_lists[1].remove(i);
                        }
                        break;
                    }
                }
            }
            "summoner" => {
                for i in 0..enemy_lists[2].len() {
                    if enemy_lists[2][i].get_enemy_count() == enemy.get_enemy_count() {
                        let dead = enemy_lists[2][i].dmg_enemy(dmg);
                        if dead {
                            enemy_lists[2].remove(i);
                        }
                        break;
                    }
                }
            }
            "mage" => {
                for i in 0..enemy_lists[3].len() {
                    if enemy_lists[3][i].get_enemy_count() == enemy.get_enemy_count() {
                        let dead = enemy_lists[3][i].dmg_enemy(dmg);
                        if dead {
                            enemy_lists[3].remove(i);
                        }
                        break;
                    }
                }
            }
            "large_slime" => {
                for i in 0..enemy_lists[4].len() {
                    if enemy_lists[4][i].get_enemy_count() == enemy.get_enemy_count() {
                        let dead = enemy_lists[4][i].dmg_enemy(dmg);
                        if dead {
                            enemy_lists[4].remove(i);
                        }
                        break;
                    }
                }
            }
            _ => {}
        }

    }
