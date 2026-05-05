/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::grid::draw_grid;
use crate::modules::listview::ListView;
use crate::modules::player::Player;
use crate::modules::progressbar::ProgressBar;
use crate::modules::projectile::Projectile;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::item::Item;
use crate::modules::preload_image::TextureManager;
use crate::modules::map::Map;
use macroquad::prelude::*;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut Player, tm: &TextureManager) -> String {
    //
    let testitem1 = Item::new(tm.get_preload("assets/slime.png").unwrap(), "assets/slime.png".to_string(), "Slime Essence".to_string(), "A viscous substance that can be used to craft various items.".to_string(), "melee".to_string(), 1, 0, 1.0, -0.1, 0, 0).await;
    let testitem2 = Item::new(tm.get_preload("assets/fireball.png").unwrap(), "assets/fireball.png".to_string(), "Fireball".to_string(), "A powerful explosive spell that can be cast to deal damage to enemies.".to_string(), "ranged".to_string(), 0, 2, 1.5, -0.2, 0, 0).await;
    player.add_inventory_item(testitem1);
    player.add_inventory_item(testitem2);
    //
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    

   
    let mut archer = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10, "").await;
    let mut mage = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10, "").await;
   archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());
    archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
    mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
    let mut archer_time = get_time();
    let mut mage_time = get_time();
    let mut projectile_list: Vec<Projectile> = vec![];
    let mut map = Map::new(virtual_width, virtual_height).await;
    map.create_map_array(0, 0, 4, 0, vec![1, 2, 3, 4]).await;
    map.change_map(vec![1, 1, 1, 1, 2, 3], vec![vec![2,2], vec![2,3], vec![12, 2], vec![12, 3], vec![1, 1], vec![13, 1]]);
    loop {
        player.handle_keypresses().await;
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        map.draw_map(&tm).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos);
        draw_grid(50.0, BLACK);

        
        if ((archer.get_x() - player.get_x()).abs() < 450.0) && ((archer.get_y() - player.get_y()).abs() < 450.0) {
           
            if get_time() - archer_time > 0.5 {
                 archer.archer_img_change(player.get_x(), archer.get_x(), "ready", &tm).await;
            }

             if get_time() - archer_time > 1.0 {
                archer.archer_img_change(player.get_x(), archer.get_x(), "attack", &tm).await;
                archer_time = get_time();
                archer.shoot(player, 40.0, 40.0).await;
            }
        } else {
            archer.moveing(player.get_x(), player.get_y());
            archer.archer_img_change(player.get_x(), archer.get_x(), "move", &tm).await;
        }


         if ((mage.get_x() - player.get_x()).abs() < 300.0) && ((mage.get_y() - player.get_y()).abs() < 300.0) {
            if get_time() - mage_time > 0.5 {
               mage.mage_img_change(player.get_x(), mage.get_x(), "ready", &tm).await;
            }

            if get_time() - mage_time > 2.0 {
                mage_time = get_time();
             mage.shoot(player, 80.0, 80.0).await;
               mage.mage_img_change(player.get_x(), mage.get_x(), "attack", &tm).await;
           
            
            }

        } else {
            mage.moveing(player.get_x(), player.get_y()); 
            
             mage.mage_img_change(player.get_x(), mage.get_x(), "ready", &tm).await;
        }

        player.draw();
        

        mage.draw();
        archer.draw();
        archer.draw_bullet(player);
        mage.draw_bullet(player);
        player.handle_inventory();

        next_frame().await;
    }
}










