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
    musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
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
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec![
            "assets/map_files/world1/watertile.png".to_string(),
            "assets/map_files/chest.png".to_string(),
        ],
    )
    .await;
// If cleared
    if player.get_cleared() == 0 {
    map.create_map_array(0, 1, 0, vec![4]).await;
    } else {
    map.create_map_array(0, 2, 0, vec![1, 4]).await;
    }
    println!("Last scene: {}", last_scene);
    let mut enemies: Vec<Enemy> = vec![];
    let mut summoner = Enemy::new("", 50.0, 50.0, 70.0, 80.0, true, 1.0, 20.0, 1000.0, "", "summoner").await;
    let mut large_slime = Enemy::new("", 75.0, 75.0, 150.0, 200.0, true, 1.0, 20.0, 10.0, "", "large_slime").await;
    large_slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
    summoner.set_preload(tm.get_preload("assets/summoner_files/summoner_standR.png").unwrap());

    let mut mage = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20.0, 10.0, "", "mage").await;

    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());
    mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
    enemies.push(summoner);
    enemies.push(mage);
    enemies.push(large_slime);
    let mut archerx = 200.0;
    for _i in 0..3 {
        let mut archer = Enemy::new("", 50.0, 50.0, archerx, 200.0, true, 1.0, 10.0, 5.0, "", "archer").await;
        archerx += 100.0;
        archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
        enemies.push(archer);
    }
    if *last_scene == "Top" {
        player.set_position((virtual_width / 2.0)-20.0, virtual_height - 80.0);
    } else if *last_scene == "Down" {
        player.set_position((virtual_width / 2.0)-20.0, 80.0);
    } else if *last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if *last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    }
    loop {
        player.handle_keypresses(pause, musicdiscfunctions).await;
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        if *pause == false {
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            //enemy loop
            if player.get_cleared() == 0 {
                let mut rec = 0;
                for i in 0..enemies.len() {
                    //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                    if musicdiscfunctions.get_thickofit_active() == false && musicdiscfunctions.get_pandemonium_active() == false {
                        match enemies[i+rec].get_enemy_type() {
                            "archer" => {
                                enemies[i+rec].archer_action(tm, player).await;
                                enemies[i+rec].draw_bullet(player);
                            }
                            "slime" => {
                                enemies[i+rec].slime_action( player);
                            }
                            "summoner" => {
                                let (slime1, slime2, slime3, summoned) = enemies[i+rec].summoner_action(tm, player).await;
                                if summoned {
                                    enemies.push(slime1);
                                    enemies.push(slime2);
                                    enemies.push(slime3);
                                }
                            }
                            "mage" => {
                                enemies[i+rec].mage_action(tm, player).await;
                                enemies[i+rec].draw_bullet(player);
                            }
                            "large_slime" => {
                                enemies[i+rec].large_slime_action(tm, player).await;
                            }
                            _ => {}
                        }
                        enemies[i+rec].draw();
                        if enemies[i+rec].get_health() <= 0.0 {
                            if enemies[i+rec].get_enemy_type() == "large_slime" {
                                let (slime1, slime2, split) = enemies[i+rec].large_slime_action(tm, player).await;
                                if split {
                                    enemies.push(slime1);
                                    enemies.push(slime2);
                                }
                            }
                            enemies.remove(i+rec);
                            break;
                        }
                    }
                    else if musicdiscfunctions.get_thickofit_active() == true {
                        enemies[i].draw();
                        let enemy_old_pos = enemies[i].get_pos();
                        enemies[i].reversereverse(player.get_x(), player.get_y(), &map, enemy_old_pos);
                    }
                    else if musicdiscfunctions.get_pandemonium_active() == true {
                        enemies[i].draw();
                        let mut enemy_healthlist: Vec<i32> = vec![];
                        for j in 0..enemies.len() {
                            let health = enemies[j].get_health();
                            enemy_healthlist.push(health as i32);
                        }
                        let highesthealthenemy = enemy_healthlist.iter().max().unwrap();
                        let highesthealthenemyindex = enemy_healthlist.iter().position(|&x| x == *highesthealthenemy).unwrap(); // find index with same value
                        let highesthealthenemypos = enemies[highesthealthenemyindex].get_pos();
                        if i == highesthealthenemyindex {
                        }
                        else {
                            let enemy_old_pos = enemies[i].get_pos();
                            enemies[i].pandemonium(highesthealthenemypos, enemy_old_pos);
                            if enemies[i].check_collision(enemies[highesthealthenemyindex].view_enemy()) {
                                enemies[highesthealthenemyindex].dmg_enemy(1.0);
                                enemies[i].pushback(enemy_old_pos, highesthealthenemypos);
                            }
                        }
                    }
                    let mut healthbar = enemies[i].set_healthbar();
                        healthbar.draw();
                        if enemies[i].get_health() <= 0.0 {
                            if enemies[i].get_enemy_type() == "large_slime" {
                                let (slime1, slime2, split) = enemies[i].large_slime_action(tm, player).await;
                                if split {
                                    enemies.push(slime1);
                                    enemies.push(slime2);
                                }
                            }
                            enemies.remove(i);
                            rec += 1;
                            break;
                    }
                }
            }
        }
        player.draw();
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player);
        player.set_player_activedisc(activedisc);
        if mlehit {
            enemies[index].dmg_enemy(player.get_meleedmg());
        }
        if rnghit {
            enemies[index].dmg_enemy(player.get_rngdmg());
        }
      
        player.handle_inventory();
        player.handle_save_menu().await;
        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "w1sp".to_string();
        }
        if enemies.is_empty() && player.get_cleared() == 0 {
            player.add_cleared();
            map.change_map(vec![0, 0], vec![vec![7, 0], vec![6, 0]]);
        }
        if player.get_y() < 10.0 && player.get_cleared() >= 1{
            *last_scene = "Top".to_string();
            println!("Returning w1s2");
            return "w1s2".to_string();
        }
        next_frame().await;
    }
}
