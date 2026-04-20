/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::grid::draw_grid;
use crate::modules::player::Player;
use crate::modules::progressbar::ProgressBar;
use crate::modules::projectile::Projectile;
use crate::modules::still_image::StillImage;
use crate::{VIRTUAL_HEIGHT, modules::scale::use_virtual_resolution};
use macroquad::{color, prelude::*};

pub async fn run(virtual_height: f32, virtual_width: f32) -> String {
    use_virtual_resolution(virtual_height, virtual_width);
    let mut player = Player::new(
        "assets/person.png",
        50.0, // width
        50.0, // height
        70.0, // x position
        0.0,  // y position
        true, // Enable stretching
        1.0,  // Normal zoom (100%)
    )
    .await;
    let mut enemy = Enemy::new(
        "assets/slime.png",
        50.0,  // width
        50.0,  // height
        200.0, // x position
        0.0,   // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
        20,
        10,
    ).await;
    let mut hp_bar = ProgressBar::new(
        10.0, 10.0, // Position (x, y)
        200.0, 30.0, // Size (width, height)
        0.0, 100.0, // Range (min, max)
        100.0, // Initial value
    );

    let mut timer = get_time();
    let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        clear_background(RED);
        draw_grid(50.0, BLACK);
        player.moveing();
        player.move_check_collision_x();
        player.move_check_collision_y();
        enemy.moveing(player.get_x(), player.get_y());

        if get_time() - timer > 1.0 {
         timer = get_time();
        let mut projectile = Projectile::new(
            "assets/slime.png",
            20.0,  // width
            20.0,  // height
            enemy.get_x(), // x position (centered on player)
            enemy.get_y(), // y position (same as player)
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        ).await;
        projectile.set_direction(player.get_position());
        projectile_list.push(projectile);


        }

         for projectile in 0..projectile_list.len() {
                projectile_list[projectile].move_projectiles(player.get_position());
                projectile_list[projectile].draw();
         //projectile_list[projectile].move_projectiles(projectile_list);
         
           // let movement = self.direction * self.move_speed * get_frame_time();
           // projectile.set_x(projectile.get_x() + movement.x);
            //projectile.set_y(projectile.get_y() + movement.y);
        
         }
        
        

        player.draw();
        enemy.draw();
        next_frame().await;
    }
}
