use macroquad::prelude::*;
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use crate::modules::collision::check_collision;
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::enemy::Enemy;

/* 
run through player 

outside the loop...
musicdisc::get_musicdisc_time();

in loop...
use musicdisc::handle_musicdisccooldowns; in each loop

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
    disc_elements: Vec<StillImage>,
}

impl Musicdisc {
    pub async fn new(tm: &TextureManager) -> Self {
        let musicpaths = vec!["assets/musicdisc_files/music/backinblack.ogg".to_string(), "assets/musicdisc_files/music/thickofit.ogg".to_string()];
        tm.preload_sound(musicpaths[0].as_str()).await;
        let backinblack_sound = tm.get_preloaded_sound(musicpaths[0].as_str()).unwrap();
        tm.preload_sound(musicpaths[1].as_str()).await;
        let thickofit_sound = tm.get_preloaded_sound(musicpaths[1].as_str()).unwrap();
        Musicdisc {
            musicpaths,
            sounds: vec![backinblack_sound, thickofit_sound],
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
        }

    }


    pub async fn test_musicdisc(&self) {
        println!("Playing music disc: {}", self.musicpaths[0]);
        play_sound(&self.sounds[0], PlaySoundParams {looped: false, volume: 1.0 });
    }


    pub fn get_musicdisc_times(&mut self) {
        self.backinblack_starttime = get_time(); 
        self.thickofit_starttime = get_time();
    }

    pub fn get_musicdisc_cooldowns(&mut self) -> Vec<f64> {
        vec![self.backinblack_cct, self.thickofit_cct]
    }

    pub fn get_musicdisc_validity(&self) -> Vec<bool> {
        vec![self.backinblack_valid, self.thickofitvalid]
    }

    pub fn get_thickofit_active(&self) -> bool {
        self.thickofit
    }

    pub fn handle_musicdiscs(&mut self, activedisc: String, enemies: &mut Vec<Enemy>) -> String {
        let mut discmatch = activedisc.as_str();
        match discmatch {
            "Back In Black" => { //fireball hit 8 times in 15 seconds
                if self.backinblack_valid == true {
                    let time = get_time() - self.backinblack_starttime;
                    if time >= 0.0 && time <= 1.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                play_sound(&self.sounds[0], PlaySoundParams {looped: false, volume: 1.0 });
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time > 1.0 && time <= 2.0 {
                        self.backinblack_hit = false;
                    }
                    else if time > 2.0 && time <= 3.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time > 3.0 && time <= 4.0 {
                        //no draw
                    }
                    else if time > 4.0 && time <= 5.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time > 5.0 && time <= 6.0 {
                        //no draw
                    }
                    else if time > 6.0 && time <= 7.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time > 7.0 && time <= 8.0 {
                        //no draw
                    }
                    else if time > 8.0 && time <= 9.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time > 9.0 && time <= 10.0 {
                        //no draw
                    }
                    else if time > 10.0 && time <= 11.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time > 11.0 && time <= 12.0 {
                        //no draw
                    }
                    else if time > 12.0 && time <= 13.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time > 13.0 && time <= 14.0 {
                        //no draw
                    }
                    else if time > 14.0 && time <= 15.0 {
                        for image in self.disc_elements.iter() {
                            image.draw();
                            if self.backinblack_hit == false {
                                for enemy in enemies.iter_mut() {
                                    if check_collision(image, enemy.view_enemy(), 1) {
                                        enemy.dmg_enemy(20.0);
                                    }
                                }
                                self.backinblack_hit = true;
                            }
                        }
                    }
                    else if time >= 15.0 {
                        //no draw
                        self.backinblack_hit = false;
                        self.backinblack_valid = false;
                        self.backinblack_cooldown = get_time();
                        discmatch = "";
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
            }
        }
        //println!("Discmatch: {}", discmatch);
        discmatch.to_string()
    }

    pub async fn create_disc_elements(tm: &TextureManager) -> Vec<StillImage> {
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
            vec![bib_img1, bib_img2, bib_img3, bib_img4, bib_img5, bib_img6]
    }
}

