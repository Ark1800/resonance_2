/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
        let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies);
        player.set_player_activedisc(activedisc);
Program Details:
*/

use crate::modules::animated_image::AnimatedImage;
use crate::modules::enemy::Enemy;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::text_button::TextButton;
use crate::modules::still_image::StillImage;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use crate::modules::progressbar::ProgressBar;

use macroquad::prelude::*;
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
    player.set_currentscreen("w2sb".to_string());
    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/tree.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    let mut enemies: Vec<Enemy> = vec![];
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Down" {
        player.set_position((virtual_width / 2.0) - 20.0, virtual_height - 80.0);
    } else if last_scene == "Up" {
        player.set_position(virtual_width / 2.0, 30.0);
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
    let mut boss = Enemy::new("", 100.0, 100.0, 100.0, 120.0, true, 1.0, 400.0, 50.0, "", "").await;
    boss.set_preload_gif(tm.get_preloaded_animated_gif("assets/world2_boss/boss_idleR.gif").unwrap(), true);
    boss.set_projectile_preload(tm.get_preload("assets/world2_boss/slime_ball.png").unwrap());
    let mut attack =true;
    let mut shoot = false;
    let mut chomp = false;
    let mut dig = false;
    let mut attack_choice = 0;
    let mut timer=0.0;
    map.create_map_array(0, 1, 0, vec![2]).await;
    if player.get_cleared() == 8 {
        map.create_map_array(0, 2, 0, vec![2, 1]).await;
    }
    let mut choose_open = false;
    let mut item_valid = false;
   
     let mut healthbar = ProgressBar::new(
        120.0, virtual_height - 50.0,      // Position (x, y)
        800.0, 90.0,       // Size (width, height)
        0.0, 400.0,        // Range (min, max)
        400.0                // Initial value
    );
    
     healthbar.with_animation(true, 2.0);
     healthbar.with_colors(RED, PURPLE, WHITE);
     healthbar.with_border(true, BLACK, 2.0);
        loop {
        player.handle_keypresses(pause, musicdiscfunctions).await;
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(BLACK);
        background.draw();
        boss.plant_boss_action(player, &tm, &mut attack,&mut timer,&mut shoot,&mut chomp, &mut attack_choice, &mut dig, musicdiscfunctions).await;
        boss.draw_bullet(player, musicdiscfunctions);
        healthbar.set_value(boss.get_health());
       








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
        }
        if quit {
            return "main_screen".to_string();
        }
        map.draw_map(&tm).await;
        if *pause == false {
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            //enemy loop
            boss.draw();
        }
        player.draw();
        let (mlehit, rnghit, _index) = player.handle_player_ui(&mut enemies, musicdiscfunctions).await; //dont need to send enemies back because it doesnt get used again until next frame
        let activedisc = musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);
        if mlehit {
            boss.dmg_enemy(player.get_meleedmg());
            
        }
        if rnghit {
            boss.dmg_enemy(player.get_rngdmg());
        }

        if boss.get_health() <= 0.0 && player.get_cleared() == 11 {
            player.add_cleared();
            item_valid = true;
            choose_open = true;
            map.change_map(vec![0, 0], vec![vec![14, 4], vec![14, 5]]); // opens right side of map when all enemies are dead
        }
       
        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Left".to_string();
            return "w2s3".to_string();
        }

        if player.get_x() < 10.0 {
            *last_scene = "Up".to_string();
            return "w2s1".to_string();
        }
        player.draw();
        (choose_open, item_valid) = player.handle_choose_item(&mut choose_open, &mut item_valid);
         healthbar.draw();
        next_frame().await;
    }
}
