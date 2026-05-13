/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
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
    background.set_preload(tm.get_preload("assets/map_files/grass.png").unwrap());
    if last_scene == "Left" {
        player.set_position(virtual_width - 80.0, virtual_height / 2.0);
    } else if last_scene == "Right" {
        player.set_position(80.0, virtual_height / 2.0);
    } else if last_scene == "Top" {
        player.set_position(virtual_width / 2.0, virtual_height - 80.0);
    } else if last_scene == "down" {
        player.set_position(virtual_width / 2.0, 80.0);
    } else {
        player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    }
    println!("Last scene: {}", last_scene);
    let mut map = Map::new(virtual_width, virtual_height, vec!["assets/map_files/world1/beach.png".to_string(), "assets/map_files/chest.png".to_string()]).await;
    map.create_map_array(0, 4, 0, vec![1, 2, 3, 4]).await;
    map.change_map(
        vec![1, 1, 1, 1, 2, 3],
        vec![vec![2, 2], vec![2, 3], vec![12, 2], vec![12, 3], vec![1, 1], vec![13, 1]],
    );
    let mut summoner = Enemy::new("", 50.0, 50.0, 70.0, 80.0, true, 1.0, 20, 10, "").await;
    let mut large_slime = Enemy::new("", 75.0, 75.0, 150.0, 200.0, true, 1.0, 20, 10, "").await;
    large_slime.set_preload(tm.get_preload("assets/slime.png").unwrap());
    summoner.set_preload(tm.get_preload("assets/summoner_files/summoner_standR.png").unwrap());

    let mut mage = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10, "").await;

    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());

    mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());

    let mut slime_list: Vec<Enemy> = vec![];
    let mut archerx = 200.0;
    let mut archer_list: Vec<Enemy> = vec![];
    for _i in 0..3 {
        let mut archer = Enemy::new("", 50.0, 50.0, archerx, 200.0, true, 1.0, 10, 5, "").await;
        archerx += 100.0;
        archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
        archer_list.push(archer);
    }

    loop {
        let mut enemies = vec![summoner.clone(), large_slime.clone(), mage.clone()]; //views have to be cloned to be sent
        player.handle_keypresses(pause).await;                                               //for clones to stay consistent they must be set every loop
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        background.draw();
        map.draw_map(&tm).await;
        if *pause == false {
            let old_pos = player.get_oldpos();
            player.move_player(&map, old_pos, &vec![]);
            for archer in 0..archer_list.len() {
                archer_list[archer].archer_action(tm, player).await;
                archer_list[archer].draw();
                archer_list[archer].draw_bullet(player);
                if archer_list[archer].get_health() <= 0 {
                    archer_list.remove(archer);
                    break;
                }
            }

            mage.mage_action(tm, player).await;
            summoner.summoner_action(tm, player, &mut slime_list).await;
            large_slime.large_slime_action(tm, player, &mut slime_list);

            for slime in slime_list.iter_mut() {
                slime.moveing(player.get_x(), player.get_y());
                slime.draw();
            }
            summoner.draw();
            large_slime.draw();
            mage.draw();
            mage.draw_bullet(player);
for archer in 0..archer_list.len() {

    let arrow_list = archer_list[archer].get_projectiles();
                for arrow in 0..arrow_list.len() {
                     let collision = check_collision(arrow_list[arrow].view_player(), player.view_player(), 1); // 1 = pixel skip (for performance)
                    if collision {
                        player.dmgplayer(archer_list[archer].get_dmg());
                        archer_list[archer].remove_projectile(arrow);
                
                        break;
                }
            }
            
        }
        player.draw();

        player.handle_player_ui(&mut enemies).await;
        player.handle_inventory();

        if player.get_x() > virtual_width - 10.0 {
            *last_scene = "Right".to_string();
            return "town".to_string();
        }
        next_frame().await;
    }
}
}