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
use crate::modules::preload_image::TextureManager;
use crate::modules::map::Map;
use macroquad::prelude::*;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut Player, tm: &TextureManager) -> String {
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    let mut archer = Enemy::new("assets/archer_files/archer_standR.png", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10).await;
    let mut mage = Enemy::new("assets/mage_files/mage_standR.png", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10).await;
    let mut hp_bar = ProgressBar::new(
        10.0, 10.0, // Position (x, y)
        200.0, 30.0, // Size (width, height)
        0.0, 100.0, // Range (min, max)
        player.get_health(), // Initial value
    );

    let mut archer_time = get_time();
    let mut mage_time = get_time();
    let mut projectile_list: Vec<Projectile> = vec![];
    let mut map = Map::new().await;
    map.create_map_array(0, 0, 4, 0, vec![1, 2, 3, 4]).await;
    map.change_map(vec![1, 1, 1, 1, 2], vec![vec![2,2], vec![2,3], vec![7, 2], vec![7, 3], vec![1, 1]]);
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        map.draw_map().await;
        player.handle_keypresses().await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos);
        draw_grid(50.0, BLACK);

        if ((archer.get_x() - player.get_x()).abs() < 450.0) && ((archer.get_y() - player.get_y()).abs() < 450.0) {
            if get_time() - archer_time > 0.5 {
                 archer.archer_img_change(player.get_x(), archer.get_x(), "move").await;
            }

             if get_time() - archer_time > 1.0 {
                archer_time = get_time();
                let mut projectile = Projectile::new("assets/arrow.png", 50.0, 50.0, archer.get_x(), archer.get_y(), true, 1.0).await;
                archer.archer_img_change(player.get_x(), archer.get_x(), "move").await;
                let angle = projectile.set_rotation(player.get_x(), player.get_y(), archer.get_x(), archer.get_y());
                projectile.set_angle(angle);
                projectile.set_direction(player.get_oldpos());
                projectile_list.push(projectile);
            }
        } else {
            archer.moveing(player.get_x(), player.get_y());
            archer.archer_img_change(player.get_x(), archer.get_x(), "move").await;
        }


         if ((mage.get_x() - player.get_x()).abs() < 300.0) && ((mage.get_y() - player.get_y()).abs() < 300.0) {
            if get_time() - mage_time > 0.5 {
               mage.mage_img_change(player.get_x(), mage.get_x(), "ready").await;
            }

            if get_time() - mage_time > 2.0 {
                mage_time = get_time();
                let mut projectile = Projectile::new("assets/fireball.png", 80.0, 80.0, mage.get_x(), mage.get_y(), true, 1.0).await;
                mage.mage_img_change(player.get_x(), mage.get_x(), "ready").await;
                let angle = projectile.set_rotation(player.get_x(), player.get_y(), mage.get_x(), mage.get_y());
                projectile.set_angle(angle);
                projectile.set_direction(player.get_oldpos());
                projectile_list.push(projectile);
            }
        } else {
            mage.moveing(player.get_x(), player.get_y());
             mage.mage_img_change(player.get_x(), mage.get_x(), "ready").await;
        }

        player.draw();
        for projectile in 0..projectile_list.len() {
            projectile_list[projectile].move_projectiles(player.get_oldpos());
            projectile_list[projectile].draw();
        }

        let tm = player.handle_inventory();
        mage.draw();
        archer.draw();
        archer.draw_bullet();
        //INVENTORYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
        //let mut list_view = ListView::new(&items, x, y, font_size);
        next_frame().await;
    }
}










