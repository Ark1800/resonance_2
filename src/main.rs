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
use macroquad::prelude::*;

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
const VIRTUAL_WIDTH: f32 = 800.0;
const VIRTUAL_HEIGHT: f32 = 1200.0;

#[macroquad::main(window_conf)]
async fn main() {
    let mut current_screen = "w1s1".to_string();
    let mut last_switch = get_time() - 0.02;
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "w1s1" => world_1::w1s1::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w1s2" => world_1::w1s2::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w1s3" => world_1::w1s3::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w1s4" => world_1::w1s4::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w1s5" => world_1::w1s5::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w2s1" => world_2::w2s1::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w2s2" => world_2::w2s2::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w2s3" => world_2::w2s3::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w2s4" => world_2::w2s4::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w2s5" => world_2::w2s5::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w3s1" => world_3::w3s1::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w3s2" => world_3::w3s2::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "w3s3" => world_3::w3s3::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "wcs1" => world_c::wcs1::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "wcs2" => world_c::wcs2::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "wcs3" => world_c::wcs3::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "town" => world_hub_and_otherscreens::town::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "smith" => world_hub_and_otherscreens::smith::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "casino" => world_hub_and_otherscreens::casino::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                "shop" => world_hub_and_otherscreens::shop::run(VIRTUAL_HEIGHT, VIRTUAL_WIDTH).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}
