/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details: RPG program
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
use crate::modules::preload_image::GifLoadingScreenInfo;
use crate::modules::musicdisc::Musicdisc;
use crate::modules::database::{create_database_client, DatabaseTable};
use macroquad::audio::{PlaySoundParams, play_sound};

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

#[macroquad::main(window_conf())]
async fn main() {
    //PRELOADEEDDDDDDDDD           
    let all_assets = vec![
    "assets/player_files/heart.png", "assets/player_files/sword_slash.gif", "assets/player_files/player_shadow.png", "assets/player_files/invslot.png", "assets/player_files/player_b.png", "assets/player_files/player_t.png", "assets/player_files/player_l.png", "assets/player_files/player_r.png", "assets/player_files/player_tl.png", "assets/player_files/player_tr.png", "assets/player_files/player_bl.png", "assets/player_files/player_br.png", "assets/player_files/arrow.png", "assets/player_files/bow_arrow_image.png",
    "assets/mage_files/mage_shootL.png", "assets/mage_files/mage_shootR.png", "assets/mage_files/mage_standL.png", "assets/mage_files/mage_standR.png", "assets/map_files/circle.png",
    "assets/slime.png", "assets/fireball.png", "assets/arrow.png", "assets/map_files/magma.png", "assets/map_files/magma_floor.png", "assets/map_files/tavern.png",
    "assets/archer_files/archer_deadL.png", "assets/archer_files/archer_deadR.png", "assets/archer_files/archer_knockbackL.png", "assets/archer_files/archer_knockbackR.png", "assets/archer_files/archer_readyL.png", "assets/archer_files/archer_readyR.png", "assets/archer_files/archer_runL.png", "assets/archer_files/archer_runR.png", "assets/archer_files/archer_shootL.png", "assets/archer_files/archer_shootR.png", "assets/archer_files/archer_standL.png", "assets/archer_files/archer_standR.png", 
    "assets/map_files/wall.png", "assets/map_files/chest.png", "assets/map_files/world1/beach.png", "assets/map_files/red_portal.gif", "assets/map_files/background_start.png",
    "assets/summoner_files/summoner_standL.png", "assets/summoner_files/summoner_standR.png", "assets/summoner_files/summoner_summonL.png", "assets/summoner_files/summoner_summonR.png","assets/summoner_files/portalL.png", "assets/summoner_files/portalR.png", 
    "assets/map_files/grass.png", "assets/map_files/dungeon.png", "assets/map_files/world1/watertile.png", "assets/map_files/world1/beachtile.png", "assets/map_files/tree.png", "assets/map_files/world2_start.png", "assets/map_files/world1/beach2.png","assets/map_files/black_square.png","assets/map_files/white.png",
    "assets/item_files/armour/diamond_armor.png", "assets/item_files/armour/hermes_boots.png","assets/item_files/armour/helmet_of_thorns.png", "assets/item_files/weapons/time_sword.png", "assets/item_files/weapons/future_bow.png", "assets/item_files/musicoin.png", "assets/item_files/weapons/mosquito_rapier.png", "assets/item_files/armour/lifeforce_armor.png", "assets/item_files/weapons/axl_greatbow.png", "assets/item_files/armour/l_cap.png", "assets/item_files/armour/shadow_boots.png", "assets/item_files/healthpot.png",
    "assets/map_files/1_rn.png", "assets/map_files/2_rn.png", "assets/map_files/3_rn.png",
    "assets/map_files/pedestal.png", "assets/map_files/town.png", "assets/map_files/shop.png","assets/map_files/world1/blueportal.gif", "assets/map_files/green_portal.gif","assets/map_files/red_portal.gif" ,"assets/map_files/world1/whirlpool.gif", "assets/map_files/textbox.png", 
    "assets/cyric_files/cyric_f.png", "assets/cyric_files/cyric_b.png", "assets/cyric_files/cyric_dead.png", "assets/cyric_files/meteor.png",
    "assets/musicdisc_files/effectimages/bibimg.png", "assets/musicdisc_files/effectimages/trident.png", "assets/musicdisc_files/effectimages/black.png", "assets/musicdisc_files/effectimages/meteor.png", 
    "assets/musicdisc_files/covers/backinblack.png", "assets/musicdisc_files/covers/thickofit.png", "assets/musicdisc_files/covers/howitsdone.png", "assets/musicdisc_files/covers/imstillstanding.png", "assets/musicdisc_files/covers/pandemonium.png", "assets/musicdisc_files/covers/sixhundredstrike.png", "assets/musicdisc_files/covers/sodapop.png", "assets/musicdisc_files/covers/greatestshowman.png",
    "assets/world1_boss/jeff_angry.gif", "assets/world1_boss/jeff_annoyed.gif", "assets/world1_boss/jeff_asleep.gif", "assets/world1_boss/jeff_happy.gif","assets/world1_boss/jeff_idleL.gif", "assets/world1_boss/jeff_idleR.gif", "assets/world1_boss/jeff_idle1.gif", "assets/world1_boss/jeff_idle2.gif", "assets/world1_boss/tentaclemonster.gif","assets/world1_boss/jeff_knife.gif", "assets/world1_boss/jeff_openmouth1.gif", "assets/world1_boss/jeff_heart.png", "assets/world1_boss/jeff_zzz2.gif", "assets/world1_boss/jeff_zzz3.gif", "assets/world1_boss/jeff_openmouth2R.gif", "assets/world1_boss/jeff_preattack.gif", "assets/world1_boss/jeff_sad.gif", "assets/world1_boss/jeff_serious.gif", "assets/world1_boss/jeff_smile.gif", "assets/world1_boss/jeff_tired.gif", "assets/world1_boss/jeff_full.gif", "assets/world1_boss/jeff_openmouth2L.gif", "assets/world1_boss/jeff_bubblebeam.png",
    "assets/world2_boss/boss_biteL.gif", "assets/world2_boss/boss_biteR.gif", "assets/world2_boss/boss_idleL.gif", "assets/world2_boss/boss_idleR.gif", "assets/world2_boss/boss_digL.gif", "assets/world2_boss/boss_digR.gif","assets/world2_boss/boss_shootL.gif", "assets/world2_boss/boss_shootR.gif",  "assets/world2_boss/boss_dig_upL.gif","assets/world2_boss/boss_dig_upR.gif", "assets/world2_boss/slime_ball.png",];
    let tm = TextureManager::new();
    let all_sounds = vec!["assets/musicdisc_files/music/backinblack.ogg", "assets/musicdisc_files/music/thickofit.ogg", "assets/musicdisc_files/music/howitsdone.ogg", "assets/musicdisc_files/music/imstillstanding.ogg", "assets/musicdisc_files/music/pandemonium.ogg", "assets/musicdisc_files/music/sixhundredstrike.ogg", "assets/musicdisc_files/music/sodapop.ogg", "assets/musicdisc_files/music/thegreatestshow.ogg",  "assets/music_and_sfx/bgmusic.ogg", "assets/music_and_sfx/arrowrelease.ogg", "assets/music_and_sfx/swordslash.ogg"];
    // Using custom loading screen appearance
    let loading_options = LoadingScreenOptions {
        title: Some("Resonance 2".to_string()),
        background_color: BLUE,
        bar_fill_color: GREEN,
        text_color: BLACK,
        // Show blueportal spinning while loading other assets
        loading_screen_gifs: vec![GifLoadingScreenInfo::new(
            "assets/player_files/sword_slash.gif".to_string(),
            screen_width() / 2.0 - 64.0, // Centered
            screen_height() / 3.0,       // Upper third
            128.0,                       // width
            128.0,                       // height
        )],
        ..Default::default()
    };
    tm.preload_with_loading_screen(&all_assets, Some(&all_sounds), Some(loading_options)).await;
    //VARSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS
    let preloadlist: Vec<(Texture2D, Option<Vec<u8>>, String)> = vec![tm.get_preload("assets/player_files/player_b.png").unwrap(), tm.get_preload("assets/player_files/invslot.png").unwrap(), tm.get_preload("assets/player_files/player_shadow.png").unwrap(), tm.get_preload("assets/player_files/player_t.png").unwrap(), tm.get_preload("assets/player_files/player_l.png").unwrap(), tm.get_preload("assets/player_files/player_r.png").unwrap(), tm.get_preload("assets/player_files/player_tl.png").unwrap(), tm.get_preload("assets/player_files/player_tr.png").unwrap(), tm.get_preload("assets/player_files/player_bl.png").unwrap(), tm.get_preload("assets/player_files/player_br.png").unwrap(), tm.get_preload("assets/player_files/heart.png").unwrap(), tm.get_preload("assets/player_files/arrow.png").unwrap(), tm.get_preload("assets/player_files/bow_arrow_image.png").unwrap(), tm.get_preload("assets/map_files/black_square.png").unwrap()];
    let mut current_screen = "title_screen".to_string();
    let mut pause = false;
    let mut last_switch = get_time() - 0.02;
    println!("all sounds {:?}", all_sounds);
    let mut musicdiscfunctions = Musicdisc::new(&tm, all_sounds).await;
    let mut last_scene = "None".to_string();
    let mut player = Player::new(preloadlist, 30.0, 30.0, &tm).await;
    //Save Data
    let client = create_database_client();
    let mut records: Vec<DatabaseTable> = Vec::new();
    let fetched_results = client.fetch_table("save_table").await;
    if let Ok(result) = fetched_results {
        records = result;
    } else {
       println!("Error fetching records from database: {} ",fetched_results.err().unwrap());
    }
    play_sound(musicdiscfunctions.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "w1s1" => world_1::w1s1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w1s2" => world_1::w1s2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w1s3" => world_1::w1s3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w1s4" => world_1::w1s4::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w1sb" => world_1::w1sb::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w1sp" => world_1::w1sp::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w2s1" => world_2::w2s1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w2s2" => world_2::w2s2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w2s3" => world_2::w2s3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w2sb" => world_2::w2sb::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w2sp" => world_2::w2sp::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w3s1" => world_3::w3s1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w3s2" => world_3::w3s2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w3s3" => world_3::w3s3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w3s4" => world_3::w3s4::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w3sb" => world_3::w3sb::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "w3sp" => world_3::w3sp::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "wcs1" => world_c::wcs1::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "wcs2" => world_c::wcs2::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "wcs3" => world_c::wcs3::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions,&records, &client).await,
                "town" => world_hub_and_otherscreens::town::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "shop" => world_hub_and_otherscreens::shop::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions).await,
                "inn" => world_hub_and_otherscreens::inn::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &mut player, &tm, &mut pause, &mut last_scene, &mut musicdiscfunctions, &records, &client).await,
                "title_screen" => title_screen::run(VIRTUAL_WIDTH, VIRTUAL_HEIGHT, &tm, &mut musicdiscfunctions, &mut records, &mut player, &client).await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
}


