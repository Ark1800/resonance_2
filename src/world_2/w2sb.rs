/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
        let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies);
        player.set_player_activedisc(activedisc);
Program Details:
*/

use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;
use crate::modules::preload_image::TextureManager;
use crate::modules::map::Map;
use crate::modules::still_image::StillImage;
use crate::modules::enemy::Enemy;
pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager, pause: &mut bool, last_scene: &mut String, _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc) -> String {
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/tree.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
     let mut enemies: Vec<Enemy> = vec![];
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    
    } else if last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
    
    } else if last_scene == "Up" {
        player.set_position((virtual_width / 2.0), 30.0);
    
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

    background.set_preload(tm.get_preload("assets/map_files/grass.png").unwrap());
        map.create_map_array(0, 1, 0, vec![2]).await;
    if player.get_cleared() == 8 {
        map.create_map_array(0, 2, 0, vec![2, 1]).await;
    }
    loop {

 player.handle_keypresses(pause, _musicdiscfunctions).await;
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        map.draw_map(&tm).await;
        if *pause == false {
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            //enemy loop
             if player.get_cleared() == 9 {
            for i in 0..enemies.len() {
                //matches each enemy with its type and performs the appropriate action (movement, attacking, etc.)
                match enemies[i].get_enemy_type() {
                    "archer" => {
                        enemies[i].archer_action(tm, player).await;
                        enemies[i].draw_bullet(player);
                    }
                    "slime" => {
                        enemies[i].slime_action( player);
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
        }}
        player.draw();
        let (mlehit, rnghit, index) = player.handle_player_ui(&mut enemies, _musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
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

        if enemies.is_empty() && player.get_cleared() == 9 {
            player.add_cleared();
            map.change_map(vec![0, 0], vec![vec![14, 4], vec![14, 5]]);// opens right side of map when all enemies are dead
        }








        if player.get_x() >  virtual_width - 10.0  {
            *last_scene = "Left".to_string();
            return "w2s3".to_string();
        }

        

        if player.get_x() < 10.0 {
            *last_scene = "Up".to_string();
            return "w2s1".to_string();
        }
       
        
        player.draw();
        next_frame().await;
    }
}
