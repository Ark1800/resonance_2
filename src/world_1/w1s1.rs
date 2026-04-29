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
use macroquad::prelude::*;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut Player, tm: &TextureManager) -> String {
    player.set_position(virtual_width / 2.0, virtual_height / 2.0);
    let mut archer = Enemy::new("assets/archer_files/archer_standR.png", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10).await;
    let mut mage = Enemy::new("assets/mage_files/mage_standR.png", 50.0, 50.0, 200.0, 200.0, true, 1.0, 20, 10).await;
    let mut hp_bar = ProgressBar::new(
        10.0, 10.0, // Position (x, y)
        200.0, 30.0, // Size (width, height)
        0.0, 100.0, // Range (min, max)
        100.0, // Initial value
    );

    let mut archer_time = get_time();
    let mut mage_time = get_time();
    let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        player.handle_keypresses().await;
        player.move_player();
        draw_grid(50.0, BLACK);

        if ((archer.get_x() - player.get_x()).abs() < 450.0) && ((archer.get_y() - player.get_y()).abs() < 450.0) {
            if get_time() - archer_time > 0.5 {
                archer = archer_img_change(player.get_x(), archer.get_x(), "ready", archer).await;
            }

            if get_time() - archer_time > 1.0 {
                archer_time = get_time();
                let mut projectile = Projectile::new("assets/arrow.png", 50.0, 50.0, archer.get_x(), archer.get_y(), true, 1.0).await;
                archer = archer_img_change(player.get_x(), archer.get_x(), "attack", archer).await;
                let angle = projectile.set_rotation(player.get_x(), player.get_y(), archer.get_x(), archer.get_y());
                projectile.set_angle(angle);
                projectile.set_direction(player.get_oldpos());
                projectile_list.push(projectile);
            }
        } else {
            archer.moveing(player.get_x(), player.get_y());
            archer = archer_img_change(player.get_x(), archer.get_x(), "move", archer).await;
        }


         if ((mage.get_x() - player.get_x()).abs() < 300.0) && ((mage.get_y() - player.get_y()).abs() < 300.0) {
            if get_time() - mage_time > 0.5 {
                mage = mage_img_change(player.get_x(), mage.get_x(), "ready", mage).await;
            }

            if get_time() - mage_time > 2.0 {
                mage_time = get_time();
                let mut projectile = Projectile::new("assets/fireball.png", 80.0, 80.0, mage.get_x(), mage.get_y(), true, 1.0).await;
                mage = mage_img_change(player.get_x(), mage.get_x(), "attack", mage).await;
                let angle = projectile.set_rotation(player.get_x(), player.get_y(), mage.get_x(), mage.get_y());
                projectile.set_angle(angle);
                projectile.set_direction(player.get_oldpos());
                projectile_list.push(projectile);
            }
        } else {
            mage.moveing(player.get_x(), player.get_y());
            mage = mage_img_change(player.get_x(), mage.get_x(), "ready", mage).await;
        }

        player.draw();
        for projectile in 0..projectile_list.len() {
            projectile_list[projectile].move_projectiles(player.get_oldpos());
            projectile_list[projectile].draw();
        }

        player.handle_inventory();
        mage.draw();
        archer.draw();
        //INVENTORYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
        //let mut list_view = ListView::new(&items, x, y, font_size);
        next_frame().await;
    }
}

pub async fn archer_img_change(playerx: f32, archerx: f32, action: &str, mut archer: Enemy) -> Enemy {
    let mut way = "";
    if archerx < playerx {
        way = "R";
    } else {
        way = "L";
    }
    match action {
        "move" => {
            archer.set_image(format!("assets/archer_files/archer_run{}.png", way).as_str()).await;
        }
        "ready" => {
            archer.set_image(format!("assets/archer_files/archer_ready{}.png", way).as_str()).await;
        }
        "attack" => {
            archer.set_image(format!("assets/archer_files/archer_shoot{}.png", way).as_str()).await;
        }
        _ => {}
    }
    archer
}

pub async fn mage_img_change(playerx: f32, magex: f32, action: &str, mut mage: Enemy) -> Enemy {
    let mut way = "";
    if magex < playerx {
        way = "R";
    } else {
        way = "L";
    }
    match action {
        "ready" => {
            mage.set_image(format!("assets/mage_files/mage_stand{}.png", way).as_str()).await;
        }
        "attack" => {
            mage.set_image(format!("assets/mage_files/mage_shoot{}.png", way).as_str()).await;
        }
        _ => {}
    }
    mage
}