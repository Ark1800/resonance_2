/*
By: <Your Name Here>
Date: 2026-04-14
Program Details: <Program Description Here>
*/

mod modules;
mod world_1;
mod world_2;
mod world_3;
mod world_c;
mod world_hub_and_otherscreens;
mod title_screen;
use macroquad::prelude::*;
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "resonance_2".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

// Virtual resolution constants
const VIRTUAL_WIDTH: f32 = 1024.0;
const VIRTUAL_HEIGHT: f32 = 768.0;

#[macroquad::main(window_conf)]
async fn main() {
    //PRELOADEEDDDDDDDDD           
    let all_assets = vec!["assets/player_files/player_b.png", "assets/player_files/player_t.png", "assets/player_files/player_l.png", "assets/player_files/player_r.png", "assets/player_files/player_tl.png", "assets/player_files/player_tr.png", "assets/player_files/player_bl.png", "assets/player_files/player_br.png", "assets/mage_files/mage_shootL.png", "assets/mage_files/mage_shootR.png", "assets/mage_files/mage_standL.png", "assets/mage_files/mage_standR.png", "assets/slime.png", "assets/fireball.png", "assets/arrow.png", "assets/archer_files/archer_deadL.png", "assets/archer_files/archer_deadR.png", "assets/archer_files/archer_knockbackL.png", "assets/archer_files/archer_knockbackR.png", "assets/archer_files/archer_readyL.png", "assets/archer_files/archer_readyR.png", "assets/archer_files/archer_runL.png", "assets/archer_files/archer_runR.png", "assets/archer_files/archer_shootL.png", "assets/archer_files/archer_shootR.png", "assets/archer_files/archer_standL.png", "assets/archer_files/archer_standR.png"];
    let tm = TextureManager::new();
    // Using custom loading screen appearance
    let loading_options = LoadingScreenOptions {
       title: Some("Resonance 2".to_string()),
       background_color: BLUE,
       bar_fill_color: GREEN,
       // Use default values for other options
       ..Default::default()
    };
    tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;
    //VARSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let preloadlist = vec![tm.get_preload("assets/player_files/player_b.png").unwrap(), tm.get_preload("assets/player_files/invslot.png").unwrap()];
    let mut current_screen = "w1s1".to_string();
    let mut last_switch = get_time() - 0.02;
    let mut player = Player::new(preloadlist, 30.0, 30.0).await;
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "w1s1" => world_1::w1s1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w1s2" => world_1::w1s2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w1s3" => world_1::w1s3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w1s4" => world_1::w1s4::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w1s5" => world_1::w1s5::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w2s1" => world_2::w2s1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w2s2" => world_2::w2s2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w2s3" => world_2::w2s3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w2s4" => world_2::w2s4::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w2s5" => world_2::w2s5::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w3s1" => world_3::w3s1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w3s2" => world_3::w3s2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "w3s3" => world_3::w3s3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "wcs1" => world_c::wcs1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "wcs2" => world_c::wcs2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "wcs3" => world_c::wcs3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "town" => world_hub_and_otherscreens::town::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "smith" => world_hub_and_otherscreens::smith::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "casino" => world_hub_and_otherscreens::casino::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "shop" => world_hub_and_otherscreens::shop::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm).await,
                "title_screen" => title_screen::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &tm).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}


