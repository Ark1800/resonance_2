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
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(virtual_height: f32, virtual_width: f32, player: &mut Player) -> String {
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    let mut enemy = Enemy::new(
        "assets/archer_files/archer_readyR.png",
        50.0,  // width
        50.0,  // height
        200.0, // x position
        200.0,   // y position
        true,  // Enable stretching
        1.0,   // Normal zoom (100%)
        20,
        10,
    )
    .await;
    let mut hp_bar = ProgressBar::new(
        10.0, 10.0, // Position (x, y)
        200.0, 30.0, // Size (width, height)
        0.0, 100.0, // Range (min, max)
        100.0, // Initial value
    );

    let mut timer = get_time();
    let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        player.handle_keypresses().await;
        player.move_player();
        draw_grid(50.0, BLACK);
        // player.move_check_collision_x();
        //  player.move_check_collision_y();
        //enemy.moveing(player.get_x(), player.get_y());

        if get_time() - timer > 1.0 {
            timer = get_time();
            let mut projectile = Projectile::new(
                "assets/arrow.png",
                50.0,          // width
                50.0,          // height
                enemy.get_x(), // x position (centered on player)
                enemy.get_y(), // y position (same as player)
                true,          // Enable stretching
                1.0,           // Normal zoom (100%)
            )
            .await;
        /*
        let place1 = (player.get_x() - enemy.get_x()).powf(2.0);
        let place2 = (player.get_y() - enemy.get_y()).powf(2.0);
        let distance = (place1 + place2).sqrt();
        let angle = (distance.tan());
        projectile.set_angle(angle);
        projectile.set_direction(player.get_oldpos());
         */
        let angle = projectile.set_rotation(projectile, player.get_x(), player.get_y(), enemy.get_x(), enemy.get_y());
        projectile.set_angle(angle);
        projectile.set_direction(player.get_oldpos());
        println!("angle: {}", angle);
        projectile_list.push(projectile);

        
             
              
            
        }

        player.draw();
        for projectile in 0..projectile_list.len() {
                projectile_list[projectile].move_projectiles(player.get_oldpos());
            projectile_list[projectile].draw();
        }

        enemy.draw();
        next_frame().await;
    }
}
