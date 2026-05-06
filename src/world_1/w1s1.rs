/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::grid::draw_grid;
use crate::modules::item::Item;
use crate::modules::listview::ListView;
use crate::modules::map::Map;
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::progressbar::ProgressBar;
use crate::modules::projectile::Projectile;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut Player, tm: &TextureManager) -> String {
    //
    let testitem1 = Item::new(
        tm.get_preload("assets/slime.png").unwrap(), // Preload
        "assets/slime.png".to_string(), // Image path
        "Slime Essence".to_string(), // Name
        "A viscous substance that can be used to craft various items.".to_string(), // Description
        "melee".to_string(), // Type
        1, // Melee
        0, // Ranged
        1.0, // Move speed mult
        -0.1, // Cooldown mult
        0, // Health
        0, // Armour
    )
    .await;
    let testitem2 = Item::new(
        tm.get_preload("assets/fireball.png").unwrap(),
        "assets/fireball.png".to_string(),
        "Fireball".to_string(),
        "A powerful explosive spell that can be cast to deal damage to enemies.".to_string(),
        "ranged".to_string(),
        0,
        2,
        1.5,
        -0.2,
        0,
        0,
    )
    .await;
    player.add_inventory_item(testitem1);
    player.add_inventory_item(testitem2);
    //
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    let mut summoner = Enemy::new("", 50.0, 50.0, 70.0, 80.0, true, 1.0, 20, 10, "").await;
    summoner.set_preload(tm.get_preload("assets/summoner_files/summoner_standR.png").unwrap());

    let mut mage = Enemy::new("", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10, "").await;

    mage.set_preload(tm.get_preload("assets/mage_files/mage_standR.png").unwrap());

    mage.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
 
    let mut slime_list: Vec<Enemy> = vec![];
    let mut archerx=200.0;
    let mut archer_list: Vec<Enemy> = vec![];
    for _i in 0..3 {
        let mut archer = Enemy::new(
            "",
            50.0,
            50.0,
            archerx,
            200.0,
            true,
            1.0,
            10,
            5,
            ""
            
        )
        .await;
         archerx += 100.0;
        archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap());
        archer_list.push(archer);
    }
        
    
    
   
    let mut map = Map::new(virtual_width, virtual_height).await;
    map.create_map_array(0, 0, 4, 0, vec![1, 2, 3, 4]).await;
    map.change_map(
        vec![1, 1, 1, 1, 2, 3],
        vec![vec![2, 2], vec![2, 3], vec![12, 2], vec![12, 3], vec![1, 1], vec![13, 1]],
    );
    loop {
        player.handle_keypresses().await;
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        map.draw_map(&tm).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        draw_grid(50.0, BLACK);



for archer in archer_list.iter_mut() {
    archer.archer_action(tm, player).await;
    archer.draw();
    archer.draw_bullet(player);
}

mage.mage_action(tm, player).await;
summoner.summoner_action(tm, player, &mut slime_list).await;
       

       

        
           
        
        player.draw();
  for slime in slime_list.iter_mut() {
       slime.moveing(player.get_x(), player.get_y());
       slime.draw();
    }
        summoner.draw();
        mage.draw();
       
        mage.draw_bullet(player);
        player.handle_inventory();

        next_frame().await;
    }
}
