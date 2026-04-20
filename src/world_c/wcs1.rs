/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
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

    let mut cyric = StillImage::new(
        "assets/person.png",
        50.0, // width
        50.0, // height
        70.0, // x position
        0.0,  // y position
        true, // Enable stretching
        1.0,  // Normal zoom (100%)
    )
    .await;

    let mut hp_bar = ProgressBar::new(
        10.0, 10.0, // Position (x, y)
        200.0, 30.0, // Size (width, height)
        0.0, 100.0, // Range (min, max)
        100.0, // Initial value
    );

    let background = StillImage::new(
        "assets/slime.png",
        virtual_width,  // width
        virtual_height, // height
        0.0,            // x position
        0.0,            // y position
        true,           // Enable stretching
        1.0,            // Normal zoom (100%)
    )
    .await;

    let start_time = get_time();
    let mut current_time: f64;
    let mut time_dif = start_time;
    let mut time_left = 30.0;
    let mut cutscene = false;
    let mut projectile_list: Vec<Projectile> = vec![];
    loop {
        clear_background(RED);
        background.draw();
        draw_grid(50.0, BLACK);
        if !cutscene {
        player.moveing();
        /*
        player.move_check_collision_x()
        player.move_check_collision_y();*/
            
        } else {
            
        }

        player.draw();
        cyric.draw();
        hp_bar.draw();
        next_frame().await;
    }
}
