/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::enemy::Enemy;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    pause: &mut bool,
    last_scene: &mut String,
    _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
    game_completed: &mut bool,
) -> String {
    player.set_position(virtual_width / 2.0 - 20.0, virtual_height - 100.0);

    let mut map = Map::new(
        virtual_width,
        virtual_height,
        vec!["assets/map_files/magma.png".to_string(), "assets/map_files/chest.png".to_string()],
    )
    .await;
    map.create_map_array(0, 1, 0, vec![3]).await;
    map.change_map(
        vec![1, 1, 1, 1, 1, 1, 1, 1],
        vec![
            vec![2, 2],
            vec![2, 3],
            vec![3, 6],
            vec![3, 7],
            vec![12, 2],
            vec![12, 3],
            vec![11, 6],
            vec![11, 7],
        ],
    );
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
    background.set_preload(tm.get_preload("assets/map_files/magma_floor.png").unwrap());

    let mut cyric = Enemy::new(
        "",
        50.0,                //hieght
        80.0,                //width
        virtual_width / 2.0, //x
        150.0,               //y
        true,                //stretching
        1.0,                 //zoom level
        500.0,               //health
        30.0,                //damage
        "",
        "boss", //enemy type
    )
    .await;

    let mut enemies: Vec<Enemy> = vec![];
    cyric.set_preload(tm.get_preload("assets/cyric_files/cyric_f.png").unwrap());
    if *game_completed {
        cyric.set_preload(tm.get_preload("assets/cyric_files/cyric_dead").unwrap());
    } else {
        enemies.push(cyric);
    }
    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);
        background.draw();
        player.handle_inventory();
        player.handle_save_menu().await;
        player.handle_keypresses(pause, _musicdiscfunctions).await;
        let old_pos = player.get_oldpos();
        player.move_player(&map, old_pos, &vec![]);
        let activedisc = _musicdiscfunctions.handle_musicdiscs(player.get_player_activedisc(), &mut enemies, player, &mut map, tm);
        player.set_player_activedisc(activedisc);

        if enemies[0].get_health() <= 0.0 {
            *game_completed = true;
            enemies[0].set_preload(tm.get_preload("assets/cyric_files/cyric_dead").unwrap());
            map.change_map(vec![0, 0], vec![vec![7, 9], vec![6, 9]]);
        } else {
            enemies[0].cyric_action(player, tm).await;
        }
        enemies[0].draw_bullet(player);
        enemies[0].draw();
        player.draw();
        map.draw_map(&tm).await;
        next_frame().await;
    }
}
