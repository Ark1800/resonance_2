use macroquad::prelude::*;
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use crate::modules::collision::check_collision;
use crate::modules::player::Player;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::enemy::Enemy;

/* 
run through player 

outside the loop...
musicdisc::get_musicdisc_time();

in loop...
use musicdisc::handle_musicdisccooldowns; in each loop

//1.1 backinblack
//1.2 howitsdone
//1.3 imstillstanding
//1.4 pandemonium
//1.5 sixhundredstrike
//1.6 sodapop
//1.7 greatestshow
//1.8 thickofit
*/

#[derive(Clone)]
pub struct Musicdisc {
    //musicpath: &str,
    musicpaths: Vec<String>,
    sounds: Vec<Sound>,
    backinblack_starttime: f64,
    backinblack_valid: bool,
    backinblack_cooldown: f64,
    backinblack_cct: f64, //current cooldown time to be displayed in player
    backinblack_hit: bool,
    thickofit: bool,
    thickofit_hit: bool,
    thickofit_starttime: f64,
    thickofitvalid: bool,
    thickofitcooldown: f64,
    thickofit_cct: f64,
    howitsdone_starttime: f64,
    howitsdone_hit: bool,
    howitsdone_valid: bool,
    howitsdone_cooldown: f64,
    howitsdone_cct: f64,
    imstillstanding_starttime: f64,
    imstillstanding_valid: bool,
    imstillstanding_hit: bool,
    imstillstanding_cooldown: f64,
    imstillstanding_cct: f64,
    pandemonium_starttime: f64,
    pandemonium_valid: bool,
    pandemonium_hit: bool,
    pandemonium_cooldown: f64,
    pandemonium_cct: f64,
    pandemonium: bool,
    sixhundredstrike_starttime: f64,
    sixhundredstrike_valid: bool,
    sixhundredstrike_hit: bool,
    sixhundredstrike_last_cycle: i32,
    sixhundredstrike_cooldown: f64,
    sixhundredstrike_cct: f64,
    sodapop_starttime: f64,
    sodapop_valid: bool,
    sodapop_hit: bool,
    sodapop_cooldown: f64,
    sodapop_cct: f64,
    greatestshow_starttime: f64,
    greatestshow_valid: bool,
    greatestshow_hit: bool,
    greatestshow_cooldown: f64,
    greatestshow_cct: f64,
    disc_elements: (Vec<StillImage>, Vec<StillImage>),
}

impl Musicdisc {
    pub async fn new(tm: &TextureManager) -> Self {
        let musicpaths = vec!["assets/musicdisc_files/music/backinblack.ogg".to_string(), "assets/musicdisc_files/music/thickofit.ogg".to_string(), "assets/musicdisc_files/music/howitsdone.ogg".to_string(), "assets/musicdisc_files/music/imstillstanding.ogg".to_string(), "assets/musicdisc_files/music/pandemonium.ogg".to_string(), "assets/musicdisc_files/music/sixhundredstrike.ogg".to_string(), "assets/musicdisc_files/music/sodapop.ogg".to_string(), "assets/musicdisc_files/music/thegreatestshow.ogg".to_string()];
        tm.preload_sound(musicpaths[0].as_str()).await;
        let backinblack_sound = tm.get_preloaded_sound(musicpaths[0].as_str()).unwrap();
        tm.preload_sound(musicpaths[1].as_str()).await;
        let thickofit_sound = tm.get_preloaded_sound(musicpaths[1].as_str()).unwrap();
        tm.preload_sound(musicpaths[2].as_str()).await;
        let howitsdone_sound = tm.get_preloaded_sound(musicpaths[2].as_str()).unwrap();
        tm.preload_sound(musicpaths[3].as_str()).await;
        let imstillstanding_sound = tm.get_preloaded_sound(musicpaths[3].as_str()).unwrap();
        tm.preload_sound(musicpaths[4].as_str()).await;
        let pandemonium_sound = tm.get_preloaded_sound(musicpaths[4].as_str()).unwrap();
        tm.preload_sound(musicpaths[5].as_str()).await;
        let sixhundredstrike_sound = tm.get_preloaded_sound(musicpaths[5].as_str()).unwrap();
        tm.preload_sound(musicpaths[6].as_str()).await;
        let sodapop_sound = tm.get_preloaded_sound(musicpaths[6].as_str()).unwrap();
        tm.preload_sound(musicpaths[7].as_str()).await;
        let greatestshow_sound = tm.get_preloaded_sound(musicpaths[7].as_str()).unwrap();
        Musicdisc {
            musicpaths,
            sounds: vec![backinblack_sound, thickofit_sound, howitsdone_sound, imstillstanding_sound, pandemonium_sound, sixhundredstrike_sound, sodapop_sound, greatestshow_sound],
            disc_elements: Musicdisc::create_disc_elements(&tm).await,
            backinblack_starttime: 0.0,
            backinblack_valid: true,
            backinblack_hit: false,
            backinblack_cooldown: 0.0,
            backinblack_cct: 0.0,
            thickofit: false,
            thickofit_starttime: 0.0,
            thickofitvalid: true,
            thickofitcooldown: 0.0,
            thickofit_cct: 0.0,
            thickofit_hit: false,
            howitsdone_starttime: 0.0,
            howitsdone_valid: true,
            howitsdone_cooldown: 0.0,
            howitsdone_cct: 0.0,
            howitsdone_hit: false,
            imstillstanding_starttime: 0.0,
            imstillstanding_valid: true,
            imstillstanding_hit: false,
            imstillstanding_cooldown: 0.0,
            imstillstanding_cct: 0.0,
            pandemonium_starttime: 0.0,
            pandemonium_valid: true,
            pandemonium_hit: false,
            pandemonium_cooldown: 0.0,
            pandemonium_cct: 0.0,
            pandemonium: false,
            sixhundredstrike_starttime: 0.0,
            sixhundredstrike_valid: true,
            sixhundredstrike_hit: false,
            sixhundredstrike_last_cycle: -1,
            sixhundredstrike_cooldown: 0.0, 
            sixhundredstrike_cct: 0.0,
            sodapop_starttime: 0.0,
            sodapop_valid: true,
            sodapop_hit: false, 
            sodapop_cooldown: 0.0,
            sodapop_cct: 0.0,
            greatestshow_starttime: 0.0,
            greatestshow_valid: true,
            greatestshow_hit: false,
            greatestshow_cooldown: 0.0,
            greatestshow_cct: 0.0
        }

    }


    pub async fn test_musicdisc(&self) {
        println!("Playing music disc: {}", self.musicpaths[0]);
        play_sound(&self.sounds[0], PlaySoundParams {looped: false, volume: 1.0 });
    }


    pub fn get_musicdisc_times(&mut self) {
        self.backinblack_starttime = get_time(); 
        self.thickofit_starttime = get_time();
        self.howitsdone_starttime = get_time();
        self.imstillstanding_starttime = get_time();
        self.pandemonium_starttime = get_time();
        self.sixhundredstrike_starttime = get_time();
        self.sodapop_starttime = get_time();
        self.greatestshow_starttime = get_time();
    }

    pub fn get_musicdisc_cooldowns(&mut self) -> Vec<f64> {
        vec![self.backinblack_cct, self.thickofit_cct, self.howitsdone_cct, self.imstillstanding_cct, self.pandemonium_cct, self.sixhundredstrike_cct, self.sodapop_cct, self.greatestshow_cct]
    }

    pub fn get_musicdisc_validity(&self) -> Vec<bool> {
        vec![self.backinblack_valid, self.thickofitvalid, self.howitsdone_valid, self.imstillstanding_valid, self.pandemonium_valid, self.sixhundredstrike_valid, self.sodapop_valid, self.greatestshow_valid]
    }

    pub fn get_thickofit_active(&self) -> bool {
        self.thickofit
    }

    pub fn get_pandemonium_active(&self) -> bool {
        self.pandemonium
    }

    pub fn handle_musicdiscs(&mut self, activedisc: String, enemies: &mut Vec<Enemy>, player: &mut Player) -> String {
        let mut discmatch = activedisc.as_str();
        match discmatch {
            "Back In Black" => { //fireball hit 8 times in 15 seconds
                if self.backinblack_valid == true {
                    let time = get_time() - self.backinblack_starttime;
                    if time >= 15.0 {
                        //no draw
                        self.backinblack_hit = false;
                        self.backinblack_valid = false;
                        self.backinblack_cooldown = get_time();
                        discmatch = "";
                    } else {
                        let is_attack_window = (time as i32) % 2 == 0; //time * 2% gets all even numbers so basically if an even number run the shid
                        if is_attack_window {
                            for image in self.disc_elements.0.iter() {
                                image.draw();
                            }

                            if self.backinblack_hit == false {
                                for image in self.disc_elements.0.iter() {
                                    for enemy in enemies.iter_mut() {
                                        if check_collision(image, enemy.view_enemy(), 1) {
                                            println!("Hit enemy with Back In Black for 20 damage!");
                                            enemy.dmg_enemy(20.0);
                                        }
                                    }
                                }
                                if time <= 1.0 {
                                    play_sound(&self.sounds[0], PlaySoundParams {looped: false, volume: 1.0 });
                                }

                                self.backinblack_hit = true;
                            }
                        } else {
                            self.backinblack_hit = false;
                        }
                    }
                     
                }
            }
            "Thick Of It" => {
                if self.thickofitvalid == true {
                    if self.thickofit_hit == false {
                        play_sound(&self.sounds[1], PlaySoundParams {looped: false, volume: 1.0 });
                        self.thickofit_hit = true;
                    }
                    let time = get_time() - self.thickofit_starttime;
                    self.thickofit=true;
                    if time >= 25.0 {
                        self.thickofit = false;
                        self.thickofit_hit = false;
                        self.thickofitvalid = false;
                        self.thickofitcooldown = get_time();
                        discmatch = "";
                    }
                }
            }
            "How It's Done" => {
                if self.howitsdone_valid == true {
                    let time = get_time() - self.howitsdone_starttime;
                    if self.howitsdone_hit == false {
                        play_sound(&self.sounds[2], PlaySoundParams {looped: false, volume: 1.0 });
                        let mledmg = player.get_meleedmg() * 3.0;
                        let rngdmg = player.get_rngdmg() * 3.0;
                        let movespeedmult = player.get_movespeedmult() * 1.5;
                        let cooldownmult = player.get_cooldownmult() * 0.5;
                        let maxhealth = player.get_maxhealth() * 1.0;
                        let armor = player.get_armor() + 10;
                        player.stat_override(mledmg, rngdmg, movespeedmult, cooldownmult, maxhealth, armor);
                        self.howitsdone_hit = true;
                    }
                    if time >= 20.0 {
                        player.update_stats();
                        self.howitsdone_hit = false;
                        self.howitsdone_valid = false;
                        self.howitsdone_cooldown = get_time();
                        discmatch = "";
                    }
                }
            }
            "I'm Still Standing" => {

            }
            "Pandemonium" => {
                if self.pandemonium_valid == true {
                    if self.pandemonium_hit == false {
                        play_sound(&self.sounds[4], PlaySoundParams {looped: false, volume: 1.0 });
                        self.pandemonium_hit = true;
                    }
                    let time = get_time() - self.pandemonium_starttime;
                    self.pandemonium=true;
                    if time >= 15.0 {
                        self.pandemonium = false;
                        self.pandemonium_hit = false;
                        self.pandemonium_valid = false;
                        self.pandemonium_cooldown = get_time();
                        discmatch = "";
                    }
                }
            }
            "Six Hundred Strike" => {
                if self.sixhundredstrike_valid == true {
                    let time = get_time() - self.sixhundredstrike_starttime;
                    if time >= 22.0 {
                        //no draw
                        self.sixhundredstrike_hit = false;
                        self.sixhundredstrike_last_cycle = -1;
                        self.sixhundredstrike_valid = false;
                        self.sixhundredstrike_cooldown = get_time();
                        discmatch = "";
                    } else {
                        if !enemies.is_empty() && !self.disc_elements.1.is_empty() {
                            let phase = time % 3.0; // every 3 seconds run cycle
                            let cycle = (time / 3.0).floor() as i32; //draws for only 0.5 seconds each cycle (floor returns 0.33->0.5)

                            let mut highesthealthenemyindex = 0;
                            let mut enemy_healthlist: Vec<i32> = vec![];
                            for j in 0..enemies.len() {
                                let health = enemies[j].get_health();
                                enemy_healthlist.push(health as i32);
                            }
                            let highesthealthenemy = enemy_healthlist.iter().max().unwrap();
                            highesthealthenemyindex = enemy_healthlist.iter().position(|&x| x == *highesthealthenemy).unwrap(); // find index with same value
                            if phase < 0.5 {
                                let highesthealthenemypos = enemies[highesthealthenemyindex].get_pos();
                                self.disc_elements.1[0].set_position(highesthealthenemypos);
                                self.disc_elements.1[0].draw();

                                // Damage once at the start of each 3-second cycle.
                                if self.sixhundredstrike_last_cycle != cycle {
                                    enemies[highesthealthenemyindex].dmg_enemy(100.0);
                                    self.sixhundredstrike_last_cycle = cycle;
                                }
                            }
                                
                            if self.sixhundredstrike_hit == false {
                                play_sound(&self.sounds[5], PlaySoundParams {looped: false, volume: 1.0 });
                                self.sixhundredstrike_hit = true;
                            }
                        }
                    }
                }
            }
            "Soda Pop" => {

            }
            "The Greatest Show" => {
            }
            _ => {
                if self.backinblack_valid == false {
                    self.backinblack_cct = get_time() - self.backinblack_cooldown;
                    if self.backinblack_cct >= 45.0 {
                        self.backinblack_valid = true;
                        self.backinblack_cct = 0.0;
                    }
                }
                if self.thickofitvalid == false {
                    self.thickofit_cct = get_time() - self.thickofitcooldown;
                    if self.thickofit_cct >= 60.0 {
                        self.thickofitvalid = true;
                        self.thickofit_cct = 0.0;
                    }
                }
                if self.howitsdone_valid == false {
                    self.howitsdone_cct = get_time() - self.howitsdone_cooldown;
                    if self.howitsdone_cct >= 70.0 {
                        self.howitsdone_valid = true;
                        self.howitsdone_hit = false;
                        self.howitsdone_cct = 0.0;
                    }
                }
                if self.imstillstanding_valid == false {
                    self.imstillstanding_cct = get_time() - self.imstillstanding_cooldown;
                    if self.imstillstanding_cct >= 60.0 {
                        self.imstillstanding_valid = true;
                        self.imstillstanding_hit = false;
                        self.imstillstanding_cct = 0.0;
                    }
                }
                if self.pandemonium_valid == false {
                    self.pandemonium_cct = get_time() - self.pandemonium_cooldown;
                    if self.pandemonium_cct >= 60.0 {
                        self.pandemonium_valid = true;
                        self.pandemonium_hit = false;
                        self.pandemonium_cct = 0.0;
                    }
                }
                if self.sixhundredstrike_valid == false {
                    self.sixhundredstrike_cct = get_time() - self.sixhundredstrike_cooldown;
                    if self.sixhundredstrike_cct >= 60.0 {
                        self.sixhundredstrike_valid = true;
                        self.sixhundredstrike_hit = false;
                        self.sixhundredstrike_cct = 0.0;
                    }
                }
                if self.sodapop_valid == false {
                    self.sodapop_cct = get_time() - self.sodapop_cooldown;
                    if self.sodapop_cct >= 60.0 {
                        self.sodapop_valid = true;
                        self.sodapop_hit = false;
                        self.sodapop_cct = 0.0;
                    }
                }
                if self.greatestshow_valid == false {
                    self.greatestshow_cct = get_time() - self.greatestshow_cooldown;
                    if self.greatestshow_cct >= 60.0 {
                        self.greatestshow_valid = true;
                        self.greatestshow_hit = false;
                        self.greatestshow_cct = 0.0;
                    }
                }
            }
        }
        //println!("Discmatch: {}", discmatch);
        discmatch.to_string()
    }

    pub async fn create_disc_elements(tm: &TextureManager) -> (Vec<StillImage>, Vec<StillImage>) {
        let mut bib_img1 = StillImage::new(
        "",
        200.0,  // width
        200.0,  // height
        100.0,  // x position
        100.0,   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
        ).await;
        bib_img1.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img2 = StillImage::new(
            "",
            200.0,  // width
            200.0,  // height
            400.0,  // x position
            100.0,   // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        bib_img2.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img3 = StillImage::new(
            "",
            200.0,  // width
            200.0,  // height
            700.0,  // x position
            100.0,   // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        bib_img3.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img4 = StillImage::new(
            "",
            200.0,  // width
            200.0,  // height
            100.0,  // x position
            500.0,   // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        bib_img4.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img5
            = StillImage::new(
                "",
                200.0,  // width
                200.0,  // height
                400.0,  // x position
                500.0,   // y position
                true,   // Enable stretching
                1.0,    // Normal zoom (100%)
            )
            .await;
        bib_img5.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img6 = StillImage::new(
            "",
            200.0,  // width
            200.0,  // height
            700.0,  // x position
            500.0,   // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        bib_img6.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut trident_img = StillImage::new(
            "",
            40.0,  // width
            40.0,  // height
            700.0,  // x position
            500.0,   // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)
        ).await;
        trident_img.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/trident.png").unwrap());
        let bibs = vec![bib_img1, bib_img2, bib_img3, bib_img4, bib_img5, bib_img6];
        let tridents = vec![trident_img];
        (bibs, tridents)
        
    }
}

