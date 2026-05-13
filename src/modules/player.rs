use crate::modules::{self};
use crate::modules::collision::check_collision;
use crate::modules::item::Item;
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::map::Map;
use crate::modules::still_image::StillImage;
use crate::modules::enemy::Enemy;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::f32::consts::PI;

//TO DOOOOOO
//5. World 1 screens
//6. Song uploads
//7. All Music Discs
//9. player dying

/*
//Keypresses:
Move Up - W
Move Down - S
Move Left - A
Move Right - D
Open/Close Inventory - Tab
Disc 1 - Q
Disc 2 - E
Disc 3 - X
Dash - Shift
Melee Attack - Up Arrow
Ranged Attack - Right Arrow

//IMPLEMENTATION
//in every screen write
with other crates
use crate::modules::player::Player;
use crate::modules::enemy::Enemy;

funcs
in your loop..
let mut enemies = vec![summoner.clone(), mage.clone(), large_slime.clone()]; //list of enemies for scene

player.handle_keypresses()
player.move_player();
player.handle_player_ui(&mut enemies);
player.handle_inventory();
player.draw();

ASIDES...
Angles:
With your other use commands: use std::f32::consts::PI;

and then call your functions:
img_bob.set_angle(PI/2); //rotate 90 degrees CCW
img_bob.set_angle(PI); //rotate 180 degrees CCW
img_bob.set_angle(PI*3/2); //rotate 270 degrees CCW
img_bob.set_angle(PI*2); //rotate 360 degrees CCW

img_bob.set_angle(-PI/2); //rotate 90 degrees CW
img_bob.set_angle(-PI); //rotate 180 degrees CW
img_bob.set_angle(-PI*3/2); //rotate 270 degrees CW
img_bob.set_angle(-PI*2); //rotate 360 degrees CW
*/
pub struct Player {
    view: StillImage,                                    //stillimage of player
    preloads: Vec<(Texture2D, Option<Vec<u8>>, String)>, //vec of preloads for use throughout player (especially for UI and image changing)
    move_speed: f32,                                     //movement speed in pixels per second
    movement: Vec2,                                      //movement vector for current frame
    health: i32,                                         //player health
    maxhealth: i32,                                      //player max health (for health bar purposes and hp increases from items)
    mledmg: i32,                                         //melee damage
    rngdmg: i32,                                         //ranged damage
    movespeedmult: f32,                                  //multiplier for movement speed (for items and buffs)
    cooldownmult: f32,                                   //multiplier for cooldowns (for items and buffs)
    musicoins: i32,                                      //currency
    items: Vec<Item>,                                    //vector of items in inventory
    item_titles: Vec<String>,                            //vector of item titles for listview
    equipped_items: Vec<usize>,                          //vector of indices of equipped items in the items vector
    itemstats: (Vec<String>, Vec<i32>, Vec<f32>, Vec<(Texture2D, Option<Vec<u8>>, String)>), //2d list for stats
    inventory: (Vec<ListView>, Vec<StillImage>, Vec<Label>, Vec<TextButton>), //2d list for inventory UI elements (listviews, images, labels, buttons)
    playerui: (Vec<StillImage>, Vec<Label>, Vec<StillImage>,),             //2d list for player UI elements (images, labels)
    inventoryopen: bool,                                 //is inventory open
    armor: i32,                                          //armor value for damage reduction
    attack: bool,                                        //is player currently attacking (for drawing attack labels)
    last_attack_time: f64,                               //time of last attack for timing attack labels
    attackimgfound: bool, //has the correct attack label been found for the current direction of attack (to prevent repeatedly searching for it every frame)
    attackimg: StillImage, //current attack label to be drawn when attacking 
    rangedattack: bool, //is player currently performing a ranged attack (for drawing ranged attack labels)
    last_rng_attack_time: f64, //time of last ranged attack for timing ranged attack labels and cooldowns
    rangedattackimgcreated: bool, //has the correct ranged attack label been found/created for the current direction of attack (to prevent repeatedly searching/creating for it every frame)
    ranged_movespeeds: Vec<Vec2>, //movement speed of the projectile for ranged attacks
    arrows: Vec<StillImage>, //list of self.arrow projectiles for ranged attacks
    player_direction: String, //current direction player is facing for attack purposes (up, down, left, right, etc.)


}

impl Player {
    pub async fn new(preloadlist: Vec<(Texture2D, Option<Vec<u8>>, String)>, x: f32, y: f32) -> Self {
        let mut view = StillImage::new(
            "", 40.0, // width
            60.0, // height
            x,    // x position
            y,    // y position
            true, // Enable stretching
            1.0,  // Normal zoom (100%)
        )
        .await;
        // Apply first preload to the player view if available
        view.set_preload(preloadlist[0].clone());

        let playerui = Player::create_player_ui(x, y, &preloadlist).await;
        let inventory = Player::create_inventory(&preloadlist).await;
        let attackimg = playerui.2[0].clone();

        Player {
            view,
            move_speed: 400.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
            health: 100,
            maxhealth: 100,
            mledmg: 10,
            rngdmg: 2,
            movespeedmult: 1.0,
            cooldownmult: 1.0,
            musicoins: 0,
            items: Vec::new(),
            item_titles: Vec::new(),
            equipped_items: Vec::new(),
            itemstats: (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            preloads: preloadlist.clone(),
            inventory,
            inventoryopen: false,
            armor: 0,
            playerui,
            attack: false,
            last_attack_time: get_time(),
            player_direction: "b".to_string(),
            attackimgfound: false,
            attackimg,
            rangedattack: false,
            last_rng_attack_time: get_time(),
            rangedattackimgcreated: false,
            ranged_movespeeds: Vec::new(),
            arrows: Vec::new(),
        }
    }
    //movement functions
    pub async fn handle_keypresses(&mut self, pause: &mut bool) {
        //basic movement input handling (WASD)
        let mut move_dir = vec2(0.0, 0.0);

        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            move_dir.y += 1.0;
        }
        if is_key_down(KeyCode::W) {
            move_dir.y -= 1.0;
        }

        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize(); //normnalize diagonal movement to prevent faster movement when moving diagonally
        }

        let movement = move_dir * self.move_speed * get_frame_time() * self.movespeedmult; //increasing/decreasing movement speed based on movespeedmult (for items and buffs)
        self.movement = movement;
        self.handle_image(); //handle if image changes
        if is_key_pressed(KeyCode::Tab) {
           self.inventoryopen = !self.inventoryopen; //open/close inventory on tab press (draw vs not draw)
           match pause {
                true => *pause = false, //unpause game when closing inventory
                false => *pause = true, //pause game when opening inventory
            }
        }
        if is_key_pressed(KeyCode::Up) {
            self.attack = true;
        }
        if is_key_pressed(KeyCode::Right) {
            self.rangedattack = true;
        }
    }

    pub fn handle_image(&mut self) {
        // change image based on direction of movement (8 directions)
        // Determine the desired preload index, then only set it if different from current
        let desired_index: Option<usize> = if is_key_down(KeyCode::W) && is_key_down(KeyCode::D) {
            self.player_direction = "tr".to_string();
            Some(7) //some lets us choose something that might not exist so nothing is triggered with no kepress
        } else if is_key_down(KeyCode::W) && is_key_down(KeyCode::A) {
            self.player_direction = "tl".to_string();
            Some(6)
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::D) {
            self.player_direction = "br".to_string();
            Some(9)
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::A) {
            self.player_direction = "bl".to_string();
            Some(8)
        } else if is_key_down(KeyCode::D) {
            self.player_direction = "r".to_string();
            Some(5)
        } else if is_key_down(KeyCode::A) {
            self.player_direction = "l".to_string();
            Some(4)
        } else if is_key_down(KeyCode::S) {
            self.player_direction = "b".to_string();
            Some(0)
        } else if is_key_down(KeyCode::W) {
            self.player_direction = "t".to_string();
            Some(3)
        } else {
            None
        };
        if let Some(idx) = desired_index { //create an index var from desired_index
            let desired_fname = &self.preloads[idx].2; //get filename of desired preload
            if self.view.get_filename() != *desired_fname { //if and ONLY if the filename differs
                self.view.set_preload(self.preloads[idx].clone()); //Changes filename
            } 
        }
    }

    pub fn move_x(&mut self) {
        self.view.set_x(self.view.get_x() + self.movement.x);
    }

    pub fn move_y(&mut self) {
        self.view.set_y(self.view.get_y() + self.movement.y);
    }

    pub fn move_player(&mut self, map: &Map, old_pos: Vec2, collides: &Vec<StillImage>) {
        for img in self.playerui.2.iter_mut() {
            img.set_x(img.get_x() + self.movement.x);
            img.set_y(img.get_y() + self.movement.y);
        }
        self.attackimg.set_x(self.attackimg.get_x() + self.movement.x);
        self.attackimg.set_y(self.attackimg.get_y() + self.movement.y);
        self.move_x();
        let mut collide = false;
        if !collides.is_empty() {
            collide = true;
        }
        if map.map_collision(&self.view_player()).0 {
            //collision with map
            self.set_x(old_pos.x);
            for img in self.playerui.2.iter_mut() {
                img.set_x(img.get_x() - self.movement.x); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            self.attackimg.set_x(self.attackimg.get_x() - self.movement.x);
        } else if collide {
            for img in collides {
                if self.check_x_collision(img) {
                    self.set_x(old_pos.x);
                }
            }
        }
        self.move_y();
        if map.map_collision(&self.view_player()).0 {
            //collision with map
            self.set_y(old_pos.y);
            for img in self.playerui.2.iter_mut() {
                img.set_y(img.get_y() - self.movement.y); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            self.attackimg.set_y(self.attackimg.get_y() - self.movement.y);
        } else if collide {
            for img in collides {
                if self.check_y_collision(img) {
                    self.set_y(old_pos.y);
                }
            }
        }

    }

    pub fn check_x_collision(&mut self, img2: &StillImage) -> bool {
        self.move_x();
        let mut collided = false; // Placeholder for collision check
        if check_collision(self.view_player(), img2, 1) {
            collided = true;
        }
        collided
    }

    pub fn check_y_collision(&mut self, img2: &StillImage) -> bool {
        self.move_y();
        let mut collided = false; // Placeholder for collision check
        if check_collision(self.view_player(), img2, 1) {
            collided = true;
        }
        collided
    }

    //general functions
    pub fn get_oldpos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }

    pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }

    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }

    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.view.set_x(x);
        self.view.set_y(y);
        self.playerui.2[0].set_x(x - 15.0);  //must be set manually as each label is distinctly spaced and positioned
        self.playerui.2[0].set_y(y - 30.0);
        self.playerui.2[1].set_x(x + 40.0);
        self.playerui.2[1].set_y(y - 30.0);
        self.playerui.2[2].set_x(x + 45.0);
        self.playerui.2[2].set_y(y - 10.0);
        self.playerui.2[3].set_x(x + 40.0);
        self.playerui.2[3].set_y(y + 60.0);
        self.playerui.2[4].set_x(x - 10.0);
        self.playerui.2[4].set_y(y + 65.0);
        self.playerui.2[5].set_x(x - 60.0);
        self.playerui.2[5].set_y(y + 50.0);
        self.playerui.2[6].set_x(x - 35.0);
        self.playerui.2[6].set_y(y - 10.0);
        self.playerui.2[7].set_x(x - 60.0);
        self.playerui.2[7].set_y(y - 50.0);
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }


    pub fn get_movespeed(&self) -> f32 {
        self.move_speed * self.movespeedmult
    }

    //PLAYER STATS AND MOVEMENTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT

    pub fn dash_start(&mut self) {
        self.move_speed *= 5.0;
    }

    pub fn dash_end(&mut self) {
        self.move_speed /= 5.0;
    }
    #[allow(unused)]
    pub fn get_health(&self) -> i32 {
        self.health
    }
    #[allow(unused)]
    pub fn get_stats(&self) -> (i32, i32, i32, f32) {
        (self.health, self.mledmg, self.rngdmg, self.cooldownmult)
    }
    #[allow(unused)]
    pub fn get_items(&self) -> &(Vec<String>, Vec<i32>, Vec<f32>, Vec<(Texture2D, Option<Vec<u8>>, String)>) {
        &self.itemstats
    }

    pub fn getcoins(&self) -> i32 {
        self.musicoins
    }

    pub fn addcoins(&mut self, coins: i32) {
        self.musicoins += coins;
    }

    pub fn dmgplayer(&mut self, dmg: i32) {
        let mut dmg = dmg - self.armor;
        if dmg < 0 {
            dmg = 0;
        }
        self.health -= dmg;
        let mut new_width = self.health as f32 * 4.0; // Assuming 100 health corresponds to 400 width
        let max_width = self.maxhealth as f32 * 4.0; // Maximum width based on max health
        if new_width < 0.0 {
            new_width = 0.0; // Prevent negative width
        }
        self.playerui.1[0].with_fixed_size(max_width, 25.0); //update healthbar size based on health
        self.playerui.1[1].with_fixed_size(new_width, 25.0); //update healthbar size based on health
    }

    pub fn healplayer(&mut self, heal: i32) {
        self.health += heal;
        if self.health > self.maxhealth {
            self.health = self.maxhealth;
        }
        let mut new_width = self.health as f32 * 4.0; // Assuming 100 health corresponds to 400 width
        let max_width = self.maxhealth as f32 * 4.0; // Maximum width based on max health
        if new_width > max_width {
            new_width = max_width; // Prevent negative width
        }
        self.playerui.1[0].with_fixed_size(max_width, 25.0); //update healthbar size based on health
        self.playerui.1[1].with_fixed_size(new_width, 25.0); //update healthbar size based on health
    }

    //PLAYER UIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
    pub async fn create_player_ui(x: f32, y: f32, preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>) -> (Vec<StillImage>, Vec<Label>, Vec<StillImage>) {
        let mut img_heart = StillImage::new(
            "",
            100.0, // width
            50.0,  // height
            -25.0, // x position //offset as drawn from center
            0.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_heart.set_preload(preloads[10].clone());
        let mut lbl_healthbar = Label::new("", 20.0, 40.0, 30);
        lbl_healthbar.with_fixed_size(400.0, 25.0);
        lbl_healthbar.with_colors(WHITE, Some(RED));
        lbl_healthbar.with_border(BLACK, 2.0);
        let mut lbl_healthbarbg = Label::new("", 20.0, 40.0, 30);
        lbl_healthbarbg.with_fixed_size(400.0, 25.0);
        lbl_healthbarbg.with_colors(WHITE, Some(WHITE));
        lbl_healthbarbg.with_border(BLACK, 2.0);
        let mut lbl_healthnum = Label::new("100", 6.0, 28.0, 30);
        let mut img_arrow = StillImage::new(
            "",
            40.0,  // width
            40.0,  // height
            420.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_arrow.set_preload(preloads[13].clone());
        let mut lbl_arrownum = Label::new("", 427.0, 32.0, 30);
        lbl_arrownum.with_colors(BLACK, None);
        let mut img_disc1 = StillImage::new(
            "",
            40.0,  // width
            40.0,  // height
            465.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_disc1.set_preload(preloads[1].clone());
        let mut lbl_disc1num = Label::new("99", 472.0, 32.0, 30);
        lbl_disc1num.with_colors(WHITE, None);
        let mut img_disc2 = StillImage::new(
            "",
            40.0,  // width
            40.0,  // height
            510.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_disc2.set_preload(preloads[1].clone());
        let mut lbl_disc2num = Label::new("99", 517.0, 32.0, 30);
        lbl_disc2num.with_colors(WHITE, None);
        let mut img_disc3 = StillImage::new(
            "",
            40.0,  // width
            40.0,  // height
            555.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_disc3.set_preload(preloads[1].clone());
        let mut lbl_disc3num = Label::new("99", 562.0, 32.0, 30);
        lbl_disc3num.with_colors(WHITE, None);
        //ATTACK LABELSui();
        let player_x = x;
        let player_y = y;

        let mut img_slash_t = StillImage::new(
            "",
            70.0, // width
            30.0, // height
            player_x - 15.0,
            player_y - 30.0,
            true,
            1.0,
        )
        .await;
        img_slash_t.set_preload(preloads[11].clone());
        let mut img_slash_tr = StillImage::new(
            "",
            60.0, // width
            50.0, // heightui();
            player_x + 40.0,
            player_y - 30.0,
            true,
            1.0,
        )
        .await;
        img_slash_tr.set_preload(preloads[11].clone());
        img_slash_tr.set_angle(PI / 2.0);
        let mut img_slash_r = StillImage::new(
            "",
            40.0, // width
            80.0, // height
            player_x + 45.0,
            player_y - 10.0,
            true,
            1.0,
        )
        .await;
        img_slash_r.set_preload(preloads[11].clone());
        img_slash_r.set_angle(PI);
        let mut img_slash_br = StillImage::new(
            "",
            60.0, // width
            50.0, // height
            player_x + 40.0,
            player_y + 60.0,
            true,
            1.0,
        )
        .await;
        img_slash_br.set_preload(preloads[11].clone());
        let mut img_slash_b = StillImage::new(
            "",
            70.0, // width
            30.0, // height
            player_x - 10.0,
            player_y + 65.0,
            true,
            1.0,
        )
        .await;
        img_slash_b.set_preload(preloads[11].clone());
        let mut img_slash_bl = StillImage::new(
            "",
            60.0, // width
            50.0, // height
            player_x - 60.0,
            player_y + 50.0,
            true,
            1.0,
        )
        .await;
        img_slash_bl.set_preload(preloads[11].clone());
        img_slash_bl.set_angle(PI / 2.0);
        let mut img_slash_l = StillImage::new(
            "",
            30.0, // width
            70.0, // height
            player_x - 35.0,
            player_y - 10.0,
            true,
            1.0,
        )
        .await;
        img_slash_l.set_preload(preloads[11].clone());
        let mut img_slash_tl = StillImage::new(
            "",
            60.0, // width
            50.0, // height
            player_x - 60.0,
            player_y - 50.0,
            true,
            1.0,
        )
        .await;
        img_slash_tl.set_preload(preloads[11].clone());
        (
            vec![
                img_heart,
                img_arrow,
                img_disc1,
                img_disc2,
                img_disc3,
            ],
            vec![
                lbl_healthbarbg,
                lbl_healthbar,
                lbl_healthnum,
                lbl_arrownum,
                lbl_disc1num,
                lbl_disc2num,
                lbl_disc3num,
            ],
            vec![
                img_slash_t,
                img_slash_tr,
                img_slash_r,
                img_slash_br,
                img_slash_b,
                img_slash_bl,
                img_slash_l,
                img_slash_tl,
            ]
        )
    }

    pub async fn handle_player_ui(&mut self, enemies: &mut Vec<Enemy>) {
        //update vars
        let mletimepassed = get_time() - self.last_attack_time;
        let rngtimepassed = get_time() - self.last_rng_attack_time;
        //update health number
        self.playerui.1[2].set_text(format!("{}", self.health));
        for image in self.playerui.0.iter_mut() {
            image.draw();
        }
        for label in self.playerui.1.iter_mut() {
            label.draw();
        }
        self.playerui.1[0].draw();
        self.playerui.1[1].draw(); //label must be redrawn very specifically so for loops cant be used  
        self.playerui.0[0].draw();
        self.playerui.1[2].draw();
        if self.attack {
            self.create_melee_attack(enemies);
            if mletimepassed > 0.05 && mletimepassed < 0.4 {
                self.attackimg.draw();
            }
            if mletimepassed > 0.4 { //attack label only appears for 0.6 seconds after attack
                self.attack = false;
                self.attackimgfound = false;
                self.last_attack_time = get_time();
            }
        }
        if self.rangedattack {
            self.create_range_attack().await;
            if rngtimepassed > 0.05 && rngtimepassed < 3.0 {
                let cooldown = 3.0 - rngtimepassed + 1.0; //+1 to not show 0 when cooldown is ready
                self.playerui.1[3].set_text(format!("{:.0}", cooldown));
            }
            else if rngtimepassed >= 3.0 {
                self.playerui.1[3].set_text("".to_string());
            }
            if rngtimepassed > 3.0 { 
                self.rangedattack = false;
                self.rangedattackimgcreated = false;
                self.last_rng_attack_time = get_time();
            }
        }
        let mut rac = 0; //ranged attack counter so that only one enemy is damaged per arrow
        for i in 0..self.arrows.len() {
                let y = self.arrows[i-rac].get_y();
                let x = self.arrows[i-rac].get_x();
                let movement = vec2(self.ranged_movespeeds[i-rac].x * get_frame_time(), self.ranged_movespeeds[i-rac].y * get_frame_time());
                if movement.x > 0.0 && movement.y > 0.0 { //if diagonal movement, normalize to prevent faster diagonal movement
                    self.movement.normalize(); //normalize diagonal movement to prevent faster movement when moving diagonally
                }
                self.arrows[i-rac].set_x(x + movement.x);
                self.arrows[i-rac].set_y(y + movement.y);
                self.arrows[i-rac].draw();
                if self.arrows[i-rac].get_y() < 0.0 || self.arrows[i-rac].get_y() > 800.0 || self.arrows[i-rac].get_x() < 0.0 || self.arrows[i-rac].get_x() > 1200.0 {
                    self.arrows.remove(i-rac);
                    self.ranged_movespeeds.remove(i-rac);
                    rac += 1;
                    continue; //skip collision check if arrow is removed for being out of bounds
                }
                let mut rec = 0; //remove enemy counter so that only one enemy is removed per arrow
                for j in 0..enemies.len() {
                    if check_collision(&self.arrows[i], enemies[j-rec].view_enemy(), 1) {
                        //ENEMY DAMAGE
                        self.arrows.remove(i);
                        self.ranged_movespeeds.remove(i);
                        rec += 1;
                        break; //break to prevent multiple enemies being damaged by one arrow
                    }
                }
        }
    }

    pub fn create_melee_attack(&mut self, enemies: &mut Vec<Enemy>) {
        if self.attackimgfound == false {  //attackimgbool and match must be kept in player to be used outside of if statements
                self.attackimg = match self.player_direction.as_str() {
                "t" => self.playerui.2[0].clone(),
                "tr" => self.playerui.2[1].clone(),
                "r" => self.playerui.2[2].clone(),
                "br" => self.playerui.2[3].clone(),
                "b" => self.playerui.2[4].clone(),
                "bl" => self.playerui.2[5].clone(),
                "l" => self.playerui.2[6].clone(),
                "tl" => self.playerui.2[7].clone(), 
                _ => self.playerui.2[0].clone(),
                };   
                self.attackimgfound = true;
                for enemy in enemies {
                    if check_collision(&self.attackimg, enemy.view_enemy(), 1) {
                        //ENEMY DAMAGE
                    }
                }
            }
        }

    pub async fn create_range_attack(&mut self) {
        if self.rangedattackimgcreated == false {
            let player_x = self.view.get_x();
            let player_y = self.view.get_y();
            let (coords, angle, movespeed) = match self.player_direction.as_str() {
            "t" => (vec2(player_x+20.0, player_y-15.0), -PI/2.0, vec2(0.0, -600.0)),
            "tr" => (vec2(player_x+50.0, player_y-15.0), -PI/4.0, vec2(600.0, -600.0)),
            "r" => (vec2(player_x+15.0, player_y+30.0), 0.0, vec2(600.0, 0.0)),
            "br" => (vec2(player_x+50.0, player_y+75.0), PI/4.0, vec2(600.0, 600.0)),
            "b" => (vec2(player_x+20.0, player_y+75.0), PI/2.0, vec2(0.0, 600.0)),
            "bl" => (vec2(player_x-15.0, player_y+75.0), 3.0*PI/4.0, vec2(-600.0, 600.0)),
            "l" => (vec2(player_x-15.0, player_y+30.0), PI, vec2(-600.0, 0.0)),
            "tl" => (vec2(player_x-15.0, player_y-15.0), -3.0*PI/4.0, vec2(-600.0, -600.0)), 
            _ => (vec2(0.0, 0.0), 0.0, vec2(0.0, 0.0)),
            };
            // keep per-arrow movespeeds in sync with self.arrows
            self.ranged_movespeeds.push(movespeed);
            let mut rng_attack_img = StillImage::new(
                "",
                30.0,  // width
                30.0,  // height
                coords.x, // x position
                coords.y, // y position
                true,  // Enable stretching
                1.0,   // Normal zoom (100%)
            ).await;
            rng_attack_img.set_preload(self.preloads[12].clone());
            rng_attack_img.set_angle(angle);
            self.arrows.push(rng_attack_img);
            self.rangedattackimgcreated = true;
        }
    }

    //INVENTORYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
    async fn create_inventory(preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>) -> (Vec<ListView>, Vec<StillImage>, Vec<Label>, Vec<TextButton>) {
        //creating all inventory UI elements
        let list: Vec<String> = Vec::new();
        let mut lst_inventory = ListView::new(&list, 340.0, 50.0, 60);
        lst_inventory.with_colors(WHITE, Some(BROWN), Some(LIGHTGRAY));
        lst_inventory.set_width(340.0);
        lst_inventory.with_max_visible_items(11);
        let mut item_img = StillImage::new("", 285.0, 275.0, 25.0, 25.0, true, 1.0).await;
        // Use the second preload (inv slot) for inventory placeholders if available
        let mut helmet_img = StillImage::new("", 100.0, 100.0, 825.0, 50.0, true, 1.0).await;
        let mut bodyarmor_img = StillImage::new("", 100.0, 100.0, 825.0, 280.0, true, 1.0).await;
        let mut boots_img = StillImage::new("", 100.0, 100.0, 825.0, 550.0, true, 1.0).await;
        let mut melee_img = StillImage::new("", 100.0, 100.0, 750.0, 400.0, true, 1.0).await;
        let mut ranged_img = StillImage::new("", 100.0, 100.0, 900.0, 400.0, true, 1.0).await;
        let mut shadow_img = StillImage::new("", 250.0, 650.0, 750.0, 50.0, true, 1.0).await;
        let mut disc1_img = StillImage::new("", 100.0, 100.0, 700.0, 660.0, true, 1.0).await;
        let mut disc2_img = StillImage::new("", 100.0, 100.0, 810.0, 660.0, true, 1.0).await;
        let mut disc3_img = StillImage::new("", 100.0, 100.0, 920.0, 660.0, true, 1.0).await;
        // Set inventory images to use invslot preload if available
        shadow_img.set_preload(preloads[2].clone());
        item_img.set_preload(preloads[1].clone());
        helmet_img.set_preload(preloads[1].clone());
        bodyarmor_img.set_preload(preloads[1].clone()); //set all preloads
        boots_img.set_preload(preloads[1].clone());
        melee_img.set_preload(preloads[1].clone());
        ranged_img.set_preload(preloads[1].clone());
        disc1_img.set_preload(preloads[1].clone());
        disc2_img.set_preload(preloads[1].clone());
        disc3_img.set_preload(preloads[1].clone());
        let mut lbl_title = Label::new(format!("Title"), 50.0, 375.0, 60);
        lbl_title.with_alignment(modules::label::TextAlign::Center);
        lbl_title.with_fixed_size(250.0, 75.0);
        lbl_title.with_colors(WHITE, Some(BROWN));
        let mut lbl_description = Label::new(format!("Description"), 50.0, 425.0, 20);
        lbl_description.with_fixed_size(250.0, 150.0);
        lbl_description.with_colors(WHITE, Some(BROWN));
        let mut btn_equip = TextButton::new(10.0, 580.0, 150.0, 100.0, "Equip", BLACK, GREEN, 30);
        btn_equip.with_text_color(WHITE);
        let mut btn_unequip = TextButton::new(175.0, 580.0, 150.0, 100.0, "Unequip", BLACK, GREEN, 30);
        btn_unequip.with_text_color(WHITE);
        let mut btn_trash = TextButton::new(10.0, 690.0, 315.0, 75.0, "Trash", BLACK, RED, 30);
        btn_trash.with_text_color(WHITE);
        let mut lbl_gold = Label::new(format!("Musicoins: 0"), 675.0, 40.0, 60);
        lbl_gold.with_colors(WHITE, Some(BROWN));
        //send back 2d vec of all inventory UI elements to be stored in player struct and used in inventory handling function
        (
            vec![lst_inventory],
            vec![
                shadow_img,
                item_img,
                helmet_img,
                bodyarmor_img,
                boots_img,
                melee_img,
                ranged_img,
                disc1_img,
                disc2_img,
                disc3_img,
            ],
            vec![lbl_title, lbl_description, lbl_gold],
            vec![btn_equip, btn_unequip, btn_trash],
        )
    }

    pub fn handle_inventory(&mut self) {
        if self.inventoryopen {
            //if inventory is open
            for list_view in self.inventory.0.iter_mut() {
                //for each listview
                if list_view.selected_item().is_some() && self.inventory.2[0].get_text() != *list_view.selected_item().unwrap() {
                    //if an item is selected and it is different from the one currently displayed
                    let title = list_view.selected_item().unwrap(); //get selected item title
                    for item in &self.items {
                        if item.get_itemtitle() == *title {
                            //find the item and change everything
                            self.inventory.1[1].set_preload(item.get_itemimgpath());
                            self.inventory.2[0].set_text(item.get_itemtitle());
                            self.inventory.2[1].set_text(item.get_itemdescription());
                            break;
                        }
                    }
                }
                list_view.draw();
            }
            for image in self.inventory.1.iter_mut() {
                image.draw();
            }
            for label in self.inventory.2.iter_mut() {
                label.draw();
            }
            self.inventory.2[2].set_text(format!("Musicoins:{}", self.musicoins));
            if self.inventory.3[0].click() {
                let title = self.inventory.0[0].selected_item().unwrap();
                for (i, item) in self.items.iter().enumerate() {
                    if item.get_itemtitle() == *title {
                        println!("Equipping item: {}", item.get_itemtitle());
                        println!("Item type: {}", item.get_itemtype());
                        let imageboxindex = match item.get_itemtype().as_str() {
                            "helmet" => 2,
                            "bodyarmor" => 3,
                            "boots" => 4,
                            "melee" => 5,
                            "ranged" => 6,
                            "disc" => {
                                if self.inventory.1[7].get_filename() == "assets/player_files/invslot.png" {
                                    7
                                } else if self.inventory.1[8].get_filename() == "assets/player_files/invslot.png" {
                                    8
                                } else {
                                    9
                                }
                            }
                            _ => 2,
                        };
                        if self.inventory.1[imageboxindex].get_filename() != "assets/player_files/invslot.png" {
                            for (equipped_index, equipped_item) in self.equipped_items.iter().enumerate() {
                                if self.items[*equipped_item].get_itemassetpath() == self.inventory.1[imageboxindex].get_filename() {
                                    self.equipped_items.remove(equipped_index);
                                    break;
                                }
                            }
                        }
                        self.inventory.1[imageboxindex].set_preload(item.get_itemimgpath());
                        self.equipped_items.push(i);
                        self.update_stats();
                        println!("equipped items: {:?}", self.equipped_items);
                        println!(
                            "playerstats: health: {}, mledmg: {}, rngdmg: {}, movespeedmult: {}, cooldownmult: {}, armor: {}",
                            self.health, self.mledmg, self.rngdmg, self.movespeedmult, self.cooldownmult, self.armor
                        );
                        break;
                    }
                }
            }
            if self.inventory.3[1].click() {
                let title = self.inventory.0[0].selected_item().unwrap().clone();
                println!("Unequipping item: {}", title);
                self.unequip_item(&title);
            }
            if self.inventory.3[2].click() {
                let title = self.inventory.0[0].selected_item().unwrap().clone();
                println!("{:?}", self.items.len());
                for (i, item) in self.items.iter().enumerate() {
                    if item.get_itemtitle() == *title {
                        for image in self.inventory.1.iter_mut() {
                            if image.get_filename() == self.items[0].get_itemassetpath() {
                                //self.unequip_item(&title);
                                break;
                            }

                            self.items.remove(i);
                            self.itemstats.0.remove(i);
                            self.itemstats.0.remove(i);
                            self.itemstats.1.remove(i);
                            self.itemstats.1.remove(i);
                            self.itemstats.1.remove(i);
                            self.itemstats.1.remove(i);
                            self.itemstats.2.remove(i);
                            self.itemstats.2.remove(i);
                            self.itemstats.3.remove(i);
                            self.item_titles.remove(i);
                        }
                        self.inventory.1[1].set_preload(self.preloads[1].clone());
                        self.inventory.2[0].set_text("Title");
                        self.inventory.2[1].set_text("Description");
                        self.inventory.0[0].clear();
                        self.inventory.0[0].add_items(&self.item_titles);
                        break;
                    }
                }
            }
        }
    }

    pub fn unequip_item(&mut self, title: &str) {
        for (equipped_pos, index) in self.equipped_items.iter().enumerate() {
            if *title == self.items[*index].get_itemtitle() {
                println!("SUCCESS");
                let imageboxindex = match self.items[*index].get_itemtype().as_str() {
                    "helmet" => 2,
                    "bodyarmor" => 3,
                    "boots" => 4,
                    "melee" => 5,
                    "ranged" => 6,
                    "disc" => {
                        if self.inventory.1[7].get_filename() == self.items[*index].get_itemassetpath() {
                            7
                        } else if self.inventory.1[8].get_filename() == self.items[*index].get_itemassetpath() {
                            8
                        } else {
                            9
                        }
                    }
                    _ => 2,
                };
                if self.equipped_items.len() == 1 {
                    self.equipped_items.clear();
                } else {
                    self.equipped_items.remove(equipped_pos);
                }
                self.inventory.1[imageboxindex].set_preload(self.preloads[1].clone());
                self.update_stats();
                break;
            }
        }
    }

    pub fn add_inventory_item(&mut self, item: Item) {
        //add all item stats
        self.itemstats.0.push(item.get_itemtitle());
        self.itemstats.0.push(item.get_itemdescription());
        self.itemstats.1.push(item.get_itemmledmg());
        self.itemstats.1.push(item.get_itemrngdmg());
        self.itemstats.1.push(item.get_itemhpchng());
        self.itemstats.1.push(item.get_itemarmor());
        self.itemstats.2.push(item.get_itemcooldownmult());
        self.itemstats.2.push(item.get_itemmovespeedmult());
        self.itemstats.3.push(item.get_itemimgpath());
        self.item_titles.push(item.get_itemtitle());
        self.inventory.0[0].clear();
        self.inventory.0[0].add_items(&self.item_titles);
        self.items.push(item);
        self.update_stats();
    }

    pub fn update_stats(&mut self) {
        // Reset stats to base values
        self.mledmg = 3;
        self.rngdmg = 2;
        self.movespeedmult = 1.0;
        self.cooldownmult = 1.0;
        self.maxhealth = 100;
        self.armor = 0;

        // Apply item stat changes
        for itemindex in &self.equipped_items {
            self.mledmg += self.items[*itemindex].get_itemmledmg();
            self.rngdmg += self.items[*itemindex].get_itemrngdmg();
            self.movespeedmult += self.items[*itemindex].get_itemmovespeedmult();
            self.cooldownmult += self.items[*itemindex].get_itemcooldownmult();
            self.maxhealth += self.items[*itemindex].get_itemhpchng();
            self.armor += self.items[*itemindex].get_itemarmor();
        }
    }
}
