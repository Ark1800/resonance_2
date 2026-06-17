use crate::modules::collision::check_collision;
use crate::modules::enemy::Enemy;
use crate::modules::map::Map;
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH};
use macroquad::audio::{PlaySoundParams, Sound, play_sound, stop_sound};
use macroquad::prelude::*;

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
    #[allow(unused)]
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
    imstillstanding: bool,
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
    sixhundredstrike_imagetime: f64,
    sodapop_starttime: f64,
    sodapop_valid: bool,
    sodapop_hit: bool,
    sodapop_cooldown: f64,
    sodapop_cct: f64,
    sodapop: bool,
    sodapopposlist: Vec<Vec2>,
    greatestshow_starttime: f64,
    greatestshow_valid: bool,
    greatestshow_hit: bool,
    greatestshow_cooldown: f64,
    greatestshow_cct: f64,
    greatestshow_complete: bool,
    greatestshow_playerhealth: f64,
    greatestshow_timevalid: bool,
    greatestshow_currentime: f64,
    disc_elements: (Vec<StillImage>, Vec<StillImage>, Vec<StillImage>), //0 is backinblack, 1 is sixhundredstrike, 2 is greatestshow
    musicdisc_hit: bool,
}

impl Musicdisc {
    pub async fn new(tm: &TextureManager, musicpaths: Vec<&str>) -> Self {
        let backinblack_sound = tm.get_preloaded_sound(musicpaths[0]).unwrap();
        let thickofit_sound = tm.get_preloaded_sound(musicpaths[1]).unwrap();
        let howitsdone_sound = tm.get_preloaded_sound(musicpaths[2]).unwrap();
        let imstillstanding_sound = tm.get_preloaded_sound(musicpaths[3]).unwrap();
        let pandemonium_sound = tm.get_preloaded_sound(musicpaths[4]).unwrap();
        let sixhundredstrike_sound = tm.get_preloaded_sound(musicpaths[5]).unwrap();
        let sodapop_sound = tm.get_preloaded_sound(musicpaths[6]).unwrap();
        let greatestshow_sound = tm.get_preloaded_sound(musicpaths[7]).unwrap();
        let bg_music = tm.get_preloaded_sound(musicpaths[8]).unwrap();
        Musicdisc {
            sounds: vec![
                backinblack_sound,
                thickofit_sound,
                howitsdone_sound,
                imstillstanding_sound,
                pandemonium_sound,
                sixhundredstrike_sound,
                sodapop_sound,
                greatestshow_sound,
                bg_music,
            ],
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
            imstillstanding: false,
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
            sixhundredstrike_imagetime: 0.0,
            sodapop_starttime: 0.0,
            sodapop_valid: true,
            sodapop_hit: false,
            sodapop_cooldown: 0.0,
            sodapop_cct: 0.0,
            sodapop: false,
            sodapopposlist: vec![],
            greatestshow_complete: false,
            greatestshow_starttime: 0.0,
            greatestshow_valid: true,
            greatestshow_hit: false,
            greatestshow_cooldown: 0.0,
            greatestshow_cct: 0.0,
            greatestshow_playerhealth: 0.0,
            greatestshow_timevalid: true,
            greatestshow_currentime: 0.0,
            musicdisc_hit: false,
        }
    }

    pub fn start_musicdisc_time(&mut self, disc_title: &str) {
        match disc_title {
            "Back In Black" => {
                self.backinblack_starttime = get_time();
            }
            "Thick Of It" => {
                self.thickofit_starttime = get_time();
            }
            "How It's Done" => {
                self.howitsdone_starttime = get_time();
            }
            "I'm Still Standing" => {
                self.imstillstanding_starttime = get_time();
            }
            "Pandemonium" => {
                self.pandemonium_starttime = get_time();
            }
            "600 Strike" => {
                self.sixhundredstrike_starttime = get_time();
                self.sixhundredstrike_imagetime = get_time();
            }
            "Soda Pop" => {
                self.sodapop_starttime = get_time();
            }
            "Greatest Show" => {
                self.greatestshow_starttime = get_time();
            }
            _ => {}
        }
    }

    pub fn master_reset_every_variable(&mut self) {
        self.backinblack_cct = 0.0;
        self.backinblack_cooldown = 0.0;
        self.backinblack_hit = false;
        self.backinblack_valid = true;
        self.thickofit_cct = 0.0;
        self.thickofitcooldown = 0.0;
        self.thickofit_hit = false;
        self.thickofitvalid = true;
        self.thickofit = false;
        self.howitsdone_cct = 0.0;
        self.howitsdone_cooldown = 0.0;
        self.howitsdone_hit = false;
        self.howitsdone_valid = true;
        self.imstillstanding_cct = 0.0;
        self.imstillstanding_cooldown = 0.0;
        self.imstillstanding_hit = false;
        self.imstillstanding_valid = true;
        self.imstillstanding = false;
        self.pandemonium_cct = 0.0;
        self.pandemonium_cooldown = 0.0;
        self.pandemonium_hit = false;
        self.pandemonium_valid = true;
        self.pandemonium = false;
        self.sixhundredstrike_cct = 0.0;
        self.sixhundredstrike_cooldown = 0.0;
        self.sixhundredstrike_hit = false;
        self.sixhundredstrike_valid = true;
        self.sixhundredstrike_last_cycle = -1;
        self.sodapop_cct = 0.0;
        self.sodapop_cooldown = 0.0;
        self.sodapop_hit = false;
        self.sodapop_valid = true;
        self.sodapop = false;
        self.greatestshow_cct = 0.0;
        self.greatestshow_cooldown = 0.0;
        self.greatestshow_hit = false;
        self.greatestshow_valid = true;
        self.greatestshow_complete = false;
        self.greatestshow_playerhealth = 0.0;
        self.greatestshow_timevalid = true;
        self.greatestshow_currentime = 0.0;
        self.thickofit = false;
        self.musicdisc_hit = false;
    }

    pub fn get_musicdisc_cooldowns(&mut self) -> Vec<f64> {
        let backinblack_remaining = (45.0 - (get_time() - self.backinblack_cooldown)).max(0.0);
        let thickofit_remaining = (60.0 - (get_time() - self.thickofitcooldown)).max(0.0);
        let howitsdone_remaining = (70.0 - (get_time() - self.howitsdone_cooldown)).max(0.0);
        let imstillstanding_remaining = (120.0 - (get_time() - self.imstillstanding_cooldown)).max(0.0);
        let pandemonium_remaining = (1.0 - (get_time() - self.pandemonium_cooldown)).max(0.0);
        let sixhundredstrike_remaining = (70.0 - (get_time() - self.sixhundredstrike_cooldown)).max(0.0);
        let sodapop_remaining = (70.0 - (get_time() - self.sodapop_cooldown)).max(0.0);
        let greatestshow_remaining = (80.0 - (get_time() - self.greatestshow_cooldown)).max(0.0);

        vec![
            backinblack_remaining,
            thickofit_remaining,
            howitsdone_remaining,
            imstillstanding_remaining,
            pandemonium_remaining,
            sixhundredstrike_remaining,
            sodapop_remaining,
            greatestshow_remaining,
        ]
    }

    pub fn get_musicdisc_validity(&self) -> Vec<bool> {
        vec![
            self.backinblack_valid,
            self.thickofitvalid,
            self.howitsdone_valid,
            self.imstillstanding_valid,
            self.pandemonium_valid,
            self.sixhundredstrike_valid,
            self.sodapop_valid,
            self.greatestshow_valid,
        ]
    }

    pub fn get_thickofit_active(&self) -> bool {
        self.thickofit
    }

    pub fn get_pandemonium_active(&self) -> bool {
        self.pandemonium
    }

    pub fn get_sodapop_active(&self) -> bool {
        self.sodapop
    }

    pub fn get_imstillstanding_active(&self) -> bool {
        self.imstillstanding
    }

    pub fn get_currently_playing(&self) -> &Sound {
        if self.backinblack_valid == false {
            &self.sounds[0]
        } else if self.thickofitvalid == false {
            &self.sounds[1]
        } else if self.howitsdone_valid == false {
            &self.sounds[2]
        } else if self.imstillstanding_valid == false {
            &self.sounds[3]
        } else if self.pandemonium_valid == false {
            &self.sounds[4]
        } else if self.sixhundredstrike_valid == false {
            &self.sounds[5]
        } else if self.sodapop_valid == false {
            &self.sounds[6]
        } else if self.greatestshow_valid == false {
            &self.sounds[7]
        } else {
            &self.sounds[8]
        }
    }

    pub fn get_bgmusic(&self) -> &Sound {
        &self.sounds[8]
    }

    fn update_musicdisc_cooldowns(&mut self, player: &mut Player) {
        if self.backinblack_valid == false {
            self.backinblack_cct = get_time() - self.backinblack_cooldown;
            if self.backinblack_cct >= 45.0 * player.get_cooldownmult() as f64 {
                self.backinblack_valid = true;
                self.backinblack_cct = 0.0;
            }
        }
        if self.thickofitvalid == false {
            self.thickofit_cct = get_time() - self.thickofitcooldown;
            if self.thickofit_cct >= 60.0 * player.get_cooldownmult() as f64 {
                self.thickofitvalid = true;
                self.thickofit_cct = 0.0;
            }
        }
        if self.howitsdone_valid == false {
            self.howitsdone_cct = get_time() - self.howitsdone_cooldown;
            if self.howitsdone_cct >= 70.0 * player.get_cooldownmult() as f64 {
                self.howitsdone_valid = true;
                self.howitsdone_hit = false;
                self.howitsdone_cct = 0.0;
            }
        }
        if self.imstillstanding_valid == false {
            self.imstillstanding_cct = get_time() - self.imstillstanding_cooldown;
            if self.imstillstanding_cct >= 120.0 * player.get_cooldownmult() as f64 {
                self.imstillstanding_valid = true;
                self.imstillstanding_hit = false;
                self.imstillstanding_cct = 0.0;
            }
        }
        if self.pandemonium_valid == false {
            self.pandemonium_cct = get_time() - self.pandemonium_cooldown;
            if self.pandemonium_cct >= 100.0 * player.get_cooldownmult() as f64 {
                self.pandemonium_valid = true;
                self.pandemonium_hit = false;
                self.pandemonium_cct = 0.0;
            }
        }
        if self.sixhundredstrike_valid == false {
            self.sixhundredstrike_cct = get_time() - self.sixhundredstrike_cooldown;
            if self.sixhundredstrike_cct >= 70.0 * player.get_cooldownmult() as f64 {
                self.sixhundredstrike_valid = true;
                self.sixhundredstrike_hit = false;
                self.sixhundredstrike_cct = 0.0;
            }
        }
        if self.sodapop_valid == false {
            self.sodapop_cct = get_time() - self.sodapop_cooldown;
            if self.sodapop_cct >= 70.0 * player.get_cooldownmult() as f64 {
                self.sodapop_valid = true;
                self.sodapop_hit = false;
                self.sodapop_cct = 0.0;
            }
        }
        if self.greatestshow_valid == false {
            self.greatestshow_cct = get_time() - self.greatestshow_cooldown;
            if self.greatestshow_cct >= 80.0 * player.get_cooldownmult() as f64 {
                self.greatestshow_valid = true;
                self.greatestshow_hit = false;
                self.greatestshow_cct = 0.0;
            }
        }
    }

    pub async fn handle_musicdiscs(
        &mut self,
        activedisc: String,
        enemies: &mut Vec<Enemy>,
        player: &mut Player,
        map: &mut Map,
        tm: &TextureManager,
    ) -> String {
        self.update_musicdisc_cooldowns(player);
        let mut discmatch = activedisc.as_str();
        match discmatch {
            "Back In Black" => {
                //fireball hit 8 times in 15 seconds
                if self.backinblack_valid == true {
                    let time = get_time() - self.backinblack_starttime;
                    if time >= 15.0 {
                        //no draw
                        self.backinblack_hit = false;
                        self.backinblack_valid = false;
                        self.backinblack_cooldown = get_time();
                        play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
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
                                        if enemy.get_enemy_view_type() == "still" {
                                            if check_collision(image, enemy.view_enemy(), 1) {
                                                self.musicdisc_hit = true;
                                                enemy.dmg_enemy(20.0);
                                            }
                                        }
                                        if enemy.get_enemy_view_type() == "animated" {
                                            if check_collision(image, enemy.view_enemy_animated(), 1) {
                                                self.musicdisc_hit = true;
                                                enemy.dmg_enemy(20.0);
                                            }
                                        }
                                    }
                                }
                                if time <= 1.0 {
                                    stop_sound(self.get_bgmusic());
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
                        stop_sound(self.get_bgmusic());
                        play_sound(&self.sounds[1], PlaySoundParams { looped: false, volume: 1.0 });
                        self.thickofit_hit = true;
                    }
                    let time = get_time() - self.thickofit_starttime;
                    self.thickofit = true;
                    if time < 30.0 {
                        for i in 0..enemies.len() {
                            let mut healthbar = enemies[i].set_healthbar();
                            healthbar.draw();
                            enemies[i].draw();
                            let enemy_old_pos = enemies[i].get_pos();
                            enemies[i].reversereverse(player.get_x(), player.get_y(), &map, enemy_old_pos);
                        }
                    }
                    if time >= 30.0 {
                        self.thickofit = false;
                        self.thickofit_hit = false;
                        self.thickofitvalid = false;
                        self.thickofitcooldown = get_time();
                        play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
                        discmatch = "";
                    }
                }
            }
            "How It's Done" => {
                if self.howitsdone_valid == true {
                    let time = get_time() - self.howitsdone_starttime;
                    if self.howitsdone_hit == false {
                        stop_sound(self.get_bgmusic());
                        play_sound(&self.sounds[2], PlaySoundParams { looped: false, volume: 1.0 });
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
                        play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
                        discmatch = "";
                    }
                }
            }
            "I'm Still Standing" => {
                //dont do nothing, player cant activate
            }
            "Pandemonium" => {
                if self.pandemonium_valid == true {
                    if self.pandemonium_hit == false {
                        stop_sound(self.get_bgmusic());
                        play_sound(&self.sounds[4], PlaySoundParams { looped: false, volume: 1.0 });
                        self.pandemonium_hit = true;
                    }
                    let time = get_time() - self.pandemonium_starttime;
                    self.pandemonium = true;
                    if time < 15.0 {
                        for i in 0..enemies.len() {
                            let mut healthbar = enemies[i].set_healthbar();
                            healthbar.draw();
                            enemies[i].draw();
                            let mut enemy_healthlist: Vec<i32> = vec![];
                            for j in 0..enemies.len() {
                                let health = enemies[j].get_health();
                                enemy_healthlist.push(health as i32);
                            }
                            let highesthealthenemy = enemy_healthlist.iter().max().unwrap();
                            let highesthealthenemyindex = enemy_healthlist.iter().position(|&x| x == *highesthealthenemy).unwrap(); // find index with same value
                            let highesthealthenemypos = enemies[highesthealthenemyindex].get_pos();
                            if i == highesthealthenemyindex {
                            } else {
                                let enemy_old_pos = enemies[i].get_pos();
                                enemies[i].pandemonium(highesthealthenemypos, enemy_old_pos);
                                if enemies[i].get_enemy_view_type() == "still" {
                                    if check_collision(enemies[i].view_enemy(), enemies[highesthealthenemyindex].view_enemy(), 1) {
                                        self.musicdisc_hit = true;
                                        enemies[highesthealthenemyindex].dmg_enemy(3.0);
                                        enemies[i].pushback(enemy_old_pos, highesthealthenemypos);
                                    }
                                }
                                if enemies[i].get_enemy_view_type() == "animated" {
                                    if check_collision(
                                        enemies[i].view_enemy_animated(),
                                        enemies[highesthealthenemyindex].view_enemy_animated(),
                                        1,
                                    ) {
                                        self.musicdisc_hit = true;
                                        enemies[highesthealthenemyindex].dmg_enemy(3.0);
                                        enemies[i].pushback(enemy_old_pos, highesthealthenemypos);
                                    }
                                }
                            }
                        }
                    }
                    if time >= 15.0 {
                        self.pandemonium = false;
                        self.pandemonium_hit = false;
                        self.pandemonium_valid = false;
                        self.pandemonium_cooldown = get_time();
                        play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
                        discmatch = "";
                    }
                }
            }
            "600 Strike" => {
                if self.sixhundredstrike_valid == true {
                    let time = get_time() - self.sixhundredstrike_starttime;
                    if time >= 22.0 {
                        //no draw
                        self.sixhundredstrike_hit = false;
                        self.sixhundredstrike_last_cycle = -1;
                        self.sixhundredstrike_valid = false;
                        self.sixhundredstrike_cooldown = get_time();
                        play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
                        discmatch = "";
                    } else {
                        let attack_cycle = (time / 3.0).floor() as i32; //sets 0.3->0.5 etc. so every 0.5 seconds image dissapears and every 3 seconds runs again
                        if attack_cycle != self.sixhundredstrike_last_cycle {
                            self.sixhundredstrike_last_cycle = attack_cycle;
                            self.sixhundredstrike_imagetime = get_time();
                            self.sixhundredstrike_hit = false;
                        }
                        let is_attack_window = get_time() - self.sixhundredstrike_imagetime <= 0.5;
                        if is_attack_window && !enemies.is_empty() {
                            let mut enemy_healthlist: Vec<i32> = vec![];
                            for j in 0..enemies.len() {
                                let health = enemies[j].get_health();
                                enemy_healthlist.push(health as i32);
                            }
                            let highesthealthenemy = enemy_healthlist.iter().max().unwrap();
                            let highesthealthenemyindex = enemy_healthlist.iter().position(|&x| x == *highesthealthenemy).unwrap(); // find index with same value
                            let highesthealthenemypos = enemies[highesthealthenemyindex].get_pos();
                            for image in self.disc_elements.1.iter_mut() {
                                image.set_position(highesthealthenemypos);
                                image.draw();
                            }
                            if self.sixhundredstrike_hit == false {
                                if time <= 1.0 {
                                    stop_sound(self.get_bgmusic());
                                    play_sound(&self.sounds[5], PlaySoundParams { looped: false, volume: 1.0 });
                                }
                                self.musicdisc_hit = true;
                                enemies[highesthealthenemyindex].dmg_enemy(60.0);
                                self.sixhundredstrike_hit = true;
                            }
                        }
                    }
                }
            }
            "Soda Pop" => {
                if self.sodapop_valid == true {
                    if self.sodapop_hit == false {
                        stop_sound(self.get_bgmusic());
                        play_sound(&self.sounds[6], PlaySoundParams { looped: false, volume: 1.0 });
                        for i in 0..enemies.len() {
                            self.sodapopposlist.push(enemies[i].get_pos());
                        }
                        self.sodapop_hit = true;
                    }
                    let time = get_time() - self.sodapop_starttime;
                    self.sodapop = true;
                    if time < 20.0 {
                        for i in 0..enemies.len() {
                            let mut healthbar = enemies[i].set_healthbar();
                            healthbar.draw();
                            enemies[i].draw();
                            enemies[i].sodapop(self.sodapopposlist[i], map);
                        }
                    }
                    if time >= 20.0 {
                        self.sodapop = false;
                        self.sodapop_hit = false;
                        self.sodapop_valid = false;
                        self.sodapop_cooldown = get_time();
                        play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
                        discmatch = "";
                    }
                }
            }
            "Greatest Show" => {
                if self.greatestshow_valid == true {
                    if self.greatestshow_hit == false {
                        stop_sound(self.get_bgmusic());
                        play_sound(&self.sounds[7], PlaySoundParams { looped: false, volume: 1.0 });
                        self.greatestshow_hit = true;
                        self.greatestshow_playerhealth = player.get_health() as f64;
                    }
                    let time = get_time() - self.greatestshow_starttime;
                    if time < 60.0 {
                        for i in 0..self.disc_elements.2.len() {
                            let width = self.disc_elements.2[i].get_width() + (time * 0.01) as f32; 
                            let height = self.disc_elements.2[i].get_height() + (time * 0.01) as f32;
                            let new_position = vec2(
                                self.disc_elements.2[i].get_x() - (time * 0.0005) as f32,
                                self.disc_elements.2[i].get_y() - (time * 0.0005) as f32,
                            ); //multiply by half for even growing
                            self.disc_elements.2[i].set_size(width, height);
                            self.disc_elements.2[i].set_position(new_position);
                            self.disc_elements.2[i].draw();
                        }
                    }
                    if time >= 60.0 || self.greatestshow_playerhealth > player.get_health() as f64 {
                        if self.greatestshow_timevalid == true {
                            self.greatestshow_timevalid = false;
                            self.greatestshow_currentime = time;
                        }
                        if self.greatestshow_complete == true {
                            self.greatestshow_hit = false;
                            self.greatestshow_valid = false;
                            self.greatestshow_cooldown = get_time();
                            self.greatestshow_playerhealth = 0.0;
                            play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
                            discmatch = "";
                        } else if time >= self.greatestshow_currentime && time <= self.greatestshow_currentime + 1.0 {
                            for i in 0..self.disc_elements.2.len() {
                                self.disc_elements.2[i].set_preload(tm.get_preload("assets/musicdisc_files/effectimages/meteor.png").unwrap());
                                self.disc_elements.2[i].draw();
                            }
                            for i in 0..enemies.len() {
                                if enemies[i].get_enemy_view_type() == "still" {
                                    if enemies[i].check_collision(&self.disc_elements.2[0]) {
                                        self.musicdisc_hit = true;
                                        enemies[i].dmg_enemy(200.0);
                                    }
                                }
                                if enemies[i].get_enemy_view_type() == "animated" {
                                    if enemies[i].check_collision(&self.disc_elements.2[0]) {
                                        self.musicdisc_hit = true;
                                        enemies[i].dmg_enemy(200.0);
                                    }
                                }
                            }
                        } else if time > 61.0 {
                            self.greatestshow_complete = true;
                        }
                    }
                }
            }
            _ => {}
        }
        if self.backinblack_valid == false
            && self.thickofitvalid == false
            && self.pandemonium_valid == false
            && self.sixhundredstrike_valid == false
            && self.sodapop_valid == false
            && self.howitsdone_valid == false
            && self.greatestshow_valid == false
        {
            if player.get_health() < 0.0 {
                if self.imstillstanding_hit == false {
                    discmatch = "I'm Still Standing";
                    self.imstillstanding = true;
                    stop_sound(self.get_bgmusic());
                    play_sound(&self.sounds[3], PlaySoundParams { looped: false, volume: 1.0 });
                    self.imstillstanding_hit = true;
                    self.imstillstanding_starttime = get_time();
                    player.set_health(30.0);
                }
                let time = get_time() - self.imstillstanding_starttime;
                if time >= 10.0 {
                    self.imstillstanding = false;
                    self.imstillstanding_hit = false;
                    self.imstillstanding_valid = false;
                    self.imstillstanding_cooldown = get_time();
                    play_sound(self.get_bgmusic(), PlaySoundParams { looped: true, volume: 1.0 });
                    discmatch = "";
                }
            }
        }
        if self.musicdisc_hit == true {
            for index in 0..enemies.len() {
                enemies[index].knockback(player, "enemy");
                    if enemies[index].get_health() <= 0.0 {
                    if enemies[index].get_enemy_type() == "large_slime" {
                        let (slime1, slime2, split) = enemies[index].large_slime_action(tm, player, self).await;
                        if split {
                            enemies.push(slime1);
                            enemies.push(slime2);
                        }
                    }
                    enemies[index].add_gold(player);
                    enemies.remove(index);
                    break;
                }
            }
            self.musicdisc_hit = false;
        }
        discmatch.to_string()
    }

    pub async fn create_disc_elements(tm: &TextureManager) -> (Vec<StillImage>, Vec<StillImage>, Vec<StillImage>) {
        let mut bib_img1 = StillImage::new(
            "", 200.0, // width
            200.0, // height
            100.0, // x position
            100.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        bib_img1.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img2 = StillImage::new(
            "", 200.0, // width
            200.0, // height
            400.0, // x position
            100.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        bib_img2.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img3 = StillImage::new(
            "", 200.0, // width
            200.0, // height
            700.0, // x position
            100.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        bib_img3.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img4 = StillImage::new(
            "", 200.0, // width
            200.0, // height
            100.0, // x position
            500.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        bib_img4.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img5 = StillImage::new(
            "", 200.0, // width
            200.0, // height
            400.0, // x position
            500.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        bib_img5.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut bib_img6 = StillImage::new(
            "", 200.0, // width
            200.0, // height
            700.0, // x position
            500.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        bib_img6.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());
        let mut trident_img = StillImage::new(
            "", 40.0,  // width
            40.0,  // height
            700.0, // x position
            500.0, // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        trident_img.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/trident.png").unwrap());
        let mut greatestshow_preimg = StillImage::new(
            "",
            20.0,                 // width
            20.0,                 // height
            VIRTUAL_WIDTH / 2.0,  // x position
            VIRTUAL_HEIGHT / 2.0, // y position
            true,                 // Enable stretching
            1.0,                  // Normal zoom (100%)
        )
        .await;
        greatestshow_preimg.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/black.png").unwrap());

        let bibs = vec![bib_img1, bib_img2, bib_img3, bib_img4, bib_img5, bib_img6];
        let tridents = vec![trident_img];
        let greatestshow = vec![greatestshow_preimg];
        (bibs, tridents, greatestshow)
    }
}
