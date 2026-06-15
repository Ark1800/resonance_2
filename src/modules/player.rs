use crate::modules::animated_image::AnimatedImage;
use crate::modules::collision::check_collision;
use crate::modules::database::DatabaseClient;
use crate::modules::database::DatabaseTable;
use crate::modules::enemy::Enemy;
use crate::modules::item::Item;
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::map::Map;
use crate::modules::musicdisc::Musicdisc;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::{VIRTUAL_HEIGHT, VIRTUAL_WIDTH, modules};
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use miniquad::date;
use std::f32::consts::PI;
use macroquad::audio::{PlaySoundParams, play_sound, stop_sound};

// STUFF TO TELL DUSOME
//1. slime, fireball, arrow are outside of folders for ease of testing but also used in the program
//2. deleting stuff from text input sometimes causes the program to crash, it gives error for saying Is_char_boundary is off
//3. ranged attack cooldown randomly doesnt activate 

//other notes
//1. guide
//2. buffed tutorial

//STILL TO DOOOOOO
//1. note player needs to equip items when loading in
//2. readding stuff back from commit 
//3. removing printlns

/*
BUGS TO FIX
1. not being able to get same item twice
2. normalize arrow directional movement
3. inventory and pause menus moved to being drawn above bg
4. going back into w1s1 after beating w1sb breaks everything
5. death from music discs

//Keypresses:
Move Up - W
Move Down - S
Move Left - A
Move Right - D
Open/Close Inventory - Tab
Save Menu - Escape
Disc 1 - Q
Disc 2 - E
Disc 3 - X
Melee Attack - Up Arrow
Ranged Attack - Right Arrow

//IMPLEMENTATION
//in every screen write
with other crates
use crate::modules::player::Player;
use crate::modules::enemy::Enemy;

funcs
outisde loop...

in your loop..
let mut enemies = vec![summoner, mage, large_slime]; //list of enemies for scene

player.handle_keypresses(pause, musicdiscs).await;
player.move_player();
player.handle_player_ui(&mut enemies, musicdiscfunctions).await;
player.handle_inventory();
player.handle_save_menu();
player.draw();
let activedisc = musicdiscfunctions.handle_musicdisccooldowns(player.get_player_activedisc(););
musicdiscfunctions.handle_musicdisccooldowns(player.get_player_activedisc());

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
    view: StillImage,                                                                              //stillimage of player
    preloads: Vec<(Texture2D, Option<Vec<u8>>, String)>, //vec of preloads for use throughout player (especially for UI and image changing)
    move_speed: f32,                                     //movement speed in pixels per second
    movement: Vec2,                                      //movement vector for current frame
    health: f32,                                         //player health
    maxhealth: f32,                                      //player max health (for health bar purposes and hp increases from items)
    mledmg: f32,                                         //melee damage
    rngdmg: f32,                                         //ranged damage
    movespeedmult: f32,                                  //multiplier for movement speed (for items and buffs)
    cooldownmult: f32,                                   //multiplier for cooldowns (for items and buffs)
    musicoins: i32,                                      //currency
    items: Vec<Item>,                                    //vector of items in inventory
    item_titles: Vec<String>,                            //vector of item titles for listview
    equipped_items: Vec<usize>,                          //vector of indices of equipped items in the items vector
    itemstats: (Vec<String>, Vec<i32>, Vec<f32>, Vec<(Texture2D, Option<Vec<u8>>, String)>), //2d list for stats
    inventory: (Vec<ListView>, Vec<StillImage>, Vec<Label>, Vec<TextButton>), //2d list for inventory UI elements (listviews, images, labels, buttons)
    playerui: (Vec<StillImage>, Vec<Label>, Vec<AnimatedImage>, Vec<StillImage>, Vec<StillImage>), //2d list for player UI elements (images, labels)
    inventoryopen: bool,
    savemenu: (Vec<StillImage>, Vec<Label>, Vec<TextButton>), //2d list for save menu UI elements (images, labels, buttons)
    save_menu_open: bool,                              //is inventory open
    armor: i32,                                               //armor value for damage reduction
    attack: bool,                                             //is player currently attacking (for drawing attack labels)
    last_attack_time: f64,                                    //time of last attack for timing attack labels
    attackimgfound: bool, //has the correct attack label been found for the current direction of attack (to prevent repeatedly searching for it every frame)
    attackimg: AnimatedImage, //current attack label to be drawn when attacking
    hitboximg: StillImage, //invisible image used for collision detection for attacks to prevent the player from having to be pixel perfect with their attacks
    mlevalid: bool,        //is player currently performing a melee attack (for preventing multiple melee hits from one attack input)
    rangedattack: bool,    //is player currently performing a ranged attack (for drawing ranged attack labels)
    last_rng_attack_time: f64, //time of last ranged attack for timing ranged attack labels and cooldowns
    rangedattackimgcreated: bool, //has the correct ranged attack label been found/created for the current direction of attack (to prevent repeatedly searching/creating for it every frame)
    ranged_movespeeds: Vec<Vec2>, //movement speed of the projectile for ranged attacks
    arrows: Vec<StillImage>,      //list of self.arrow projectiles for ranged attacks
    player_direction: String,     //current direction player is facing for attack purposes (up, down, left, right, etc.)
    activedisc: String,
    cleared: i32,
    death_screen: (Vec<StillImage>, Vec<Label>, Vec<TextButton>),
    death_screen_open: bool,
    possible_items: Vec<Item>,
    name: String,
    password: String,
    itemui: (Vec<StillImage>, Vec<Label>, Vec<TextButton>), //2d list for item description UI elements (images, labels, buttons)
    itemindex1: usize,
    itemindex2: usize,
    itemindex3: usize,
    currentscreen: String,
    mleattackdrawing: bool, //is the melee attack currently being drawn (for timing when to stop drawing melee attack labels)
    lbl_boss_mocking: Label, //label used to mock the player for trying to use musicdiscs on bosses that are immune to them
    last_mocking_time: f64, //time when the boss mocking label was last updated for timing how long to draw the mocking label
}

impl Player {
    pub async fn new(preloadlist: Vec<(Texture2D, Option<Vec<u8>>, String)>, x: f32, y: f32, tm: &TextureManager) -> Self {
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

        let playerui = Player::create_player_ui(x, y, &preloadlist, tm).await;
        let inventory = Player::create_inventory(&preloadlist).await;
        let savemenu = Player::create_save_menu(&preloadlist).await;
        let itemui = Player::create_item_ui(tm).await;
        let death_screen = Player::create_death_screen(&preloadlist).await;
        let attackimg = playerui.2[0].clone();
        let hitboximg = playerui.4[0].clone();
        let possible_items = Player::create_all_items(tm).await;

        Player {
            view,
            move_speed: 400.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
            health: 100.0,
            maxhealth: 100.0,
            mledmg: 3.0,
            rngdmg: 5.0,
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
            savemenu,
            save_menu_open: false,
            armor: 0,
            itemui,
            playerui,
            attack: false,
            last_attack_time: get_time(),
            player_direction: "b".to_string(),
            attackimgfound: false,
            attackimg,
            hitboximg,
            mlevalid: true,
            rangedattack: false,
            last_rng_attack_time: 0.0,
            rangedattackimgcreated: false,
            ranged_movespeeds: Vec::new(),
            arrows: Vec::new(),
            activedisc: "none".to_string(),
            cleared: 3,
            death_screen,
            death_screen_open: false,
            name: String::new(),
            possible_items,
            password: String::new(),
            itemindex1: 0,
            itemindex2: 0,
            itemindex3: 0,
            currentscreen: "".to_string(),
            mleattackdrawing: false,
            lbl_boss_mocking: Label::new("", 300.0, 100.0, 30),
            last_mocking_time: 0.0,
        }
    }
    //movement functions
    pub async fn handle_keypresses(&mut self, pause: &mut bool, musicdiscs: &mut Musicdisc) {
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
            if self.save_menu_open {
                //if save menu is open, close save menu instead of inventory on tab press
                self.save_menu_open = false;
                *pause = false; //unpause game when closing save menu
            }
            match pause {
                true => *pause = false, //unpause game when closing inventory
                false => *pause = true, //pause game when opening inventory
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            if self.inventoryopen {
                //if inventory is open, close inventory instead of save menu on escape press
                self.inventoryopen = false;
                *pause = false; //unpause game when closing inventory
            }
            self.save_menu_open = !self.save_menu_open; //open/close save menu on escape press (draw vs not draw)
            match pause {
                true => *pause = false, //unpause game when closing save menu
                false => *pause = true, //pause game when opening save menu
            }
        }
        if is_key_pressed(KeyCode::Up) {
            self.attack = true;
        }
        if is_key_pressed(KeyCode::Right) {
            let rngtimepassed = get_time() - self.last_rng_attack_time;
            self.rangedattack = true;
            //  if self.rangedattack {
            self.create_range_attack().await;
            
          
        }
       
        if self.get_player_activedisc() == "none" {
            if self.get_currentscreen() != "w1sb" && self.get_currentscreen() != "w2sb" && self.get_currentscreen() != "w3sb" {
                if is_key_pressed(KeyCode::Q) {
                    self.handle_musicdiscs(musicdiscs, 7);
                }
                if is_key_pressed(KeyCode::E) {
                    self.handle_musicdiscs(musicdiscs, 8);
                }
                if is_key_pressed(KeyCode::X) {
                    self.handle_musicdiscs(musicdiscs, 9);
                }
            }
            if is_key_pressed(KeyCode::Q) || is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::X) {
                if self.get_currentscreen() == "w1sb" {
                    self.lbl_boss_mocking.set_text("Rawr Rargh...I have earbuds in!");
                    self.lbl_boss_mocking.with_colors(WHITE, Some(BLUE));
                    self.last_mocking_time = get_time();
                }
                else if self.get_currentscreen() == "w2sb" {
                    self.lbl_boss_mocking.set_text("You call that music? My plantlist slaps harder than that!");
                    self.lbl_boss_mocking.with_colors(GREEN, Some(BROWN));
                    self.last_mocking_time = get_time();
                }
                else if self.get_currentscreen() == "w3sb" {
                    self.lbl_boss_mocking.set_text("Is that a...Spotify playlist? How quaint.");
                    self.lbl_boss_mocking.with_colors(RED, Some(BLUE));
                    self.last_mocking_time = get_time();
                }
            }
        }
    }
    #[allow(unused)]
    pub fn interact(&mut self, interactable: &StillImage) -> bool {
        let mut interact = false;
        //handle interaction with interactable objects
        if is_key_pressed(KeyCode::F) {
            if check_collision(self.view_player(), interactable, 1) {
                interact = true;
            }
        }
        interact
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
        if let Some(idx) = desired_index {
            //create an index var from desired_index
            let desired_fname = &self.preloads[idx].2; //get filename of desired preload
            if self.view.get_filename() != *desired_fname {
                //if and ONLY if the filename differs
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

    pub fn move_x_rev(&mut self) {
        self.view.set_x(self.view.get_x() - self.movement.x);
    }

    pub fn move_y_rev(&mut self) {
        self.view.set_y(self.view.get_y() - self.movement.y);
    }

    pub fn move_player(&mut self, map: &Map, old_pos: Vec2, collides: &Vec<StillImage>) {
        for img in self.playerui.2.iter_mut() {
            img.set_x(img.get_x() + self.movement.x);
            img.set_y(img.get_y() + self.movement.y);
        }
        for img in self.playerui.3.iter_mut() {
            img.set_x(img.get_x() + self.movement.x);
            img.set_y(img.get_y() + self.movement.y);
        }
        for img in self.playerui.4.iter_mut() {
            img.set_x(img.get_x() + self.movement.x);
            img.set_y(img.get_y() + self.movement.y);
        }
        self.attackimg.set_x(self.attackimg.get_x() + self.movement.x);
        self.attackimg.set_y(self.attackimg.get_y() + self.movement.y);
        self.move_x();
        let new_x = self.get_x();
        self.playerui.3[0].set_x(new_x); //move hitbox with player for accurate collision detection
        let mut collide = false;
        if !collides.is_empty() {
            collide = true;
        }
        if map.map_collision(&self.get_playerhitbox()).0 {
            println!("collided with map on x axis");
            //collision with map
            self.set_x(old_pos.x);
            self.playerui.3[0].set_x(old_pos.x);
            for img in self.playerui.2.iter_mut() {
                img.set_x(img.get_x() - self.movement.x); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            for img in self.playerui.3.iter_mut() {
                img.set_x(img.get_x() - self.movement.x); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            for img in self.playerui.4.iter_mut() {
                img.set_x(img.get_x() - self.movement.x); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            self.attackimg.set_x(self.attackimg.get_x() - self.movement.x);
        } else if collide {
            for img in collides {
                if self.check_x_collision(img) {
                    self.set_position(old_pos.x, self.get_y()); //move player back to old x position but keep new y position for smoother movement when colliding with objects
                    break;
                }
            }
        }
        self.move_y();
        let new_y = self.get_y();
        self.playerui.3[0].set_y(new_y); //move hitbox with player for accurate collision detection
        if map.map_collision(&self.get_playerhitbox()).0 {
            //collision with map
            self.set_y(old_pos.y);
            self.playerui.3[0].set_y(old_pos.y);
            for img in self.playerui.2.iter_mut() {
                img.set_y(img.get_y() - self.movement.y); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            for img in self.playerui.3.iter_mut() {
                img.set_y(img.get_y() - self.movement.y); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            for img in self.playerui.4.iter_mut() {
                img.set_y(img.get_y() - self.movement.y); //move player UI elements back if collision to prevent them getting stuck in walls
            }
            self.attackimg.set_y(self.attackimg.get_y() - self.movement.y);
        } else if collide {
            for img in collides {
                if self.check_y_collision(img) {
                    self.set_position(self.get_x(), old_pos.y);
                    break;
                }
            }
        }
    }

    pub fn get_playerhitbox(&self) -> StillImage {
        self.playerui.3[0].clone() //the invisible hitbox image in playerui.3 is used for collision detection for attacks to prevent the player from having to be pixel perfect with their attacks
    }

    pub fn check_x_collision(&mut self, img2: &StillImage) -> bool {
        self.move_x();
        let mut collided = false; // Placeholder for collision check
        if check_collision(self.view_player(), img2, 1) {
            collided = true;
        } else {
            self.move_x_rev();
        }
        collided
    }

    pub fn check_y_collision(&mut self, img2: &StillImage) -> bool {
        self.move_y();
        let mut collided = false; // Placeholder for collision check
        if check_collision(self.view_player(), img2, 1) {
            collided = true;
        } else {
            self.move_y_rev();
        }
        collided
    }

    //general functions
    pub fn get_oldpos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
        self.playerui.2[0].set_x(x - 15.0);
        self.playerui.2[1].set_x(x + 40.0);
        self.playerui.2[2].set_x(x + 45.0);
        self.playerui.2[3].set_x(x + 40.0);
        self.playerui.2[4].set_x(x - 10.0);
        self.playerui.2[5].set_x(x - 60.0);
        self.playerui.2[6].set_x(x - 65.0);
        self.playerui.2[7].set_x(x - 60.0);
        self.playerui.3[0].set_x(x);
    }

    pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }

    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
        self.playerui.2[0].set_y(y - 30.0);
        self.playerui.2[1].set_y(y - 30.0);
        self.playerui.2[2].set_y(y + 10.0);
        self.playerui.2[3].set_y(y + 60.0);
        self.playerui.2[4].set_y(y + 65.0);
        self.playerui.2[5].set_y(y + 60.0);
        self.playerui.2[6].set_y(y + 10.0);
        self.playerui.2[7].set_y(y - 30.0);
        self.playerui.3[0].set_y(y);
    }

    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.view.set_x(x);
        self.view.set_y(y);
        self.playerui.2[0].set_x(x - 15.0); //must be set manually as each label is distinctly spaced and positioned
        self.playerui.2[0].set_y(y - 30.0);
        self.playerui.2[1].set_x(x + 40.0);
        self.playerui.2[1].set_y(y - 30.0);
        self.playerui.2[2].set_x(x + 45.0);
        self.playerui.2[2].set_y(y + 10.0);
        self.playerui.2[3].set_x(x + 40.0);
        self.playerui.2[3].set_y(y + 60.0);
        self.playerui.2[4].set_x(x - 10.0);
        self.playerui.2[4].set_y(y + 65.0);
        self.playerui.2[5].set_x(x - 60.0);
        self.playerui.2[5].set_y(y + 50.0);
        self.playerui.2[6].set_x(x - 65.0);
        self.playerui.2[6].set_y(y + 10.0);
        self.playerui.2[7].set_x(x - 60.0);
        self.playerui.2[7].set_y(y - 50.0);
        self.playerui.3[0].set_x(x);
        self.playerui.3[0].set_y(y);
        self.playerui.4[0].set_x(x - 15.0);
        self.playerui.4[0].set_y(y - 30.0);
        self.playerui.4[1].set_x(x + 40.0);
        self.playerui.4[1].set_y(y - 30.0);
        self.playerui.4[2].set_x(x + 45.0);
        self.playerui.4[2].set_y(y + 10.0);
        self.playerui.4[3].set_x(x + 40.0);
        self.playerui.4[3].set_y(y + 60.0);
        self.playerui.4[4].set_x(x - 10.0);
        self.playerui.4[4].set_y(y + 65.0);
        self.playerui.4[5].set_x(x - 60.0);
        self.playerui.4[5].set_y(y + 50.0);
        self.playerui.4[6].set_x(x - 65.0);
        self.playerui.4[6].set_y(y + 10.0);
        self.playerui.4[7].set_x(x - 60.0);
        self.playerui.4[7].set_y(y - 50.0);
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }

    #[allow(unused)]
    pub fn get_movespeed(&self) -> f32 {
        self.move_speed * self.movespeedmult
    }

    pub fn get_meleedmg(&self) -> f32 {
        self.mledmg
    }

    pub fn get_rngdmg(&self) -> f32 {
        self.rngdmg
    }

    //PLAYER STATS AND MOVEMENTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT
    #[allow(unused)]
    pub fn dash_start(&mut self) {
        self.move_speed *= 5.0;
    }
    #[allow(unused)]
    pub fn dash_end(&mut self) {
        self.move_speed /= 5.0;
    }
    #[allow(unused)]
    pub fn get_health(&self) -> f32 {
        self.health
    }
    #[allow(unused)]
    pub fn get_stats(&self) -> (f32, f32, f32, f32) {
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
        self.inventory.2[2].set_text(format!("{}", self.musicoins));
    }

    pub fn dmgplayer(&mut self, dmg: f32, issactive: bool, enemy: &mut Enemy) {
        if issactive == false {
            let mut dmg = dmg - self.armor as f32;
            if dmg < 0.0 {
                dmg = 0.0;
            }
            let randomnum = rand::gen_range(1, 5);
            if self.inventory.1[4].get_filename() == "assets/item_files/armour/shadow_boots.png" {
                if randomnum == 1 || randomnum == 3 {
                    dmg = 0.0;
                }
            }
            self.health -= dmg;
            let mut new_width = self.health as f32 * 4.0; // Assuming 100 health corresponds to 400 width
            let mut max_width = self.maxhealth as f32 * 4.0; // Maximum width based on max health
            if self.inventory.1[3].get_filename() == "assets/item_files/armour/lifeforce_armor.png" {
                new_width = self.health * 2.0; // Assuming 100 health corresponds to 400 width
                max_width = self.maxhealth * 2.0; // Double the maximum health
            }
            if new_width < 0.0 {
                new_width = 0.0; // Prevent negative width
            }
            self.playerui.1[0].with_fixed_size(max_width, 25.0); //update healthbar size based on health
            self.playerui.1[1].with_fixed_size(new_width, 25.0); //update healthbar size based on health
            if self.inventory.1[2].get_filename() == "assets/item_files/armour/helmet_of_thorns.png" {
                enemy.dmg_enemy(dmg);
            }
        }
    }

    #[allow(unused)]
    pub fn get_hitboximg(&self) -> StillImage {
        self.hitboximg.clone()
    }

    #[allow(unused)]
    pub fn healplayer(&mut self, heal: f32) {
        self.health += heal;
        if self.health > self.maxhealth {
            self.health = self.maxhealth;
        }
        let mut new_width = self.health as f32 * 4.0; // Assuming 100 health corresponds to 400 width
        let mut max_width = self.maxhealth as f32 * 4.0; // Maximum width based on max health
        if self.inventory.1[3].get_filename() == "assets/item_files/armor/lifeforce_armor.png" {
            new_width = self.health * 2.0; // Assuming 100 health corresponds to 400 width
            max_width = self.maxhealth * 2.0; // Double the maximum health
        }
        if new_width > max_width {
            new_width = max_width; // Prevent negative width
        }
        self.playerui.1[0].with_fixed_size(max_width, 25.0); //update healthbar size based on health
        self.playerui.1[1].with_fixed_size(new_width, 25.0); //update healthbar size based on health
    }

    pub fn get_player_activedisc(&self) -> String {
        self.activedisc.clone()
    }

    pub fn set_player_activedisc(&mut self, disc: String) {
        if disc.is_empty() {
            self.activedisc = "none".to_string();
        } else {
            self.activedisc = disc;
        }
    }

    pub fn add_cleared(&mut self) {
        self.cleared += 1;
    }

    pub fn get_cleared(&self) -> i32 {
        self.cleared
    }

    pub fn get_movespeedmult(&self) -> f32 {
        self.movespeedmult
    }

    pub fn get_cooldownmult(&self) -> f32 {
        self.cooldownmult
    }

    #[allow(unused)]
    pub fn get_musicoins(&self) -> i32 {
        self.musicoins
    }

    pub fn get_maxhealth(&self) -> f32 {
        self.maxhealth
    }

    pub fn get_armor(&self) -> i32 {
        self.armor
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    #[allow(unused)]
    pub fn get_password(&self) -> String {
        self.password.clone()
    }
    #[allow(unused)]
    pub fn get_mleattackdrawing(&self) -> bool {
        self.mleattackdrawing
    }

    //PLAYER UIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
    #[allow(unused)]
    pub async fn create_player_ui(
        x: f32,
        y: f32,
        preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>,
        tm: &TextureManager,
    ) -> (Vec<StillImage>, Vec<Label>, Vec<AnimatedImage>, Vec<StillImage>, Vec<StillImage>) {
        let mut img_heart = StillImage::new(
            "", 100.0, // width
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
            "", 40.0,  // width
            40.0,  // height
            420.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        let mut img_musicoin = StillImage::new(
            "", 80.0, // width
            80.0, // height
            0.0,  // x position
            40.0, // y position
            true, // Enable stretching
            1.0,  // Normal zoom (100%)
        )
        .await;
        img_musicoin.set_preload(tm.get_preload("assets/item_files/musicoin.png").unwrap().clone());
        img_arrow.set_preload(preloads[12].clone());
        let mut lbl_arrownum = Label::new("", 427.0, 32.0, 30);
        lbl_arrownum.with_colors(BLACK, None);
        let mut img_disc1 = StillImage::new(
            "", 40.0,  // width
            40.0,  // height
            465.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_disc1.set_preload(preloads[1].clone());
        let mut lbl_disc1num = Label::new("", 472.0, 32.0, 30);
        lbl_disc1num.with_colors(WHITE, None);
        let mut img_disc2 = StillImage::new(
            "", 40.0,  // width
            40.0,  // height
            510.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_disc2.set_preload(preloads[1].clone());
        let mut lbl_disc2num = Label::new("", 517.0, 32.0, 30);
        lbl_disc2num.with_colors(WHITE, None);
        let mut img_disc3 = StillImage::new(
            "", 40.0,  // width
            40.0,  // height
            555.0, // x position
            5.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_disc3.set_preload(preloads[1].clone());
        let mut lbl_disc3num = Label::new("", 562.0, 32.0, 30);
        lbl_disc3num.with_colors(WHITE, None);
        //ATTACK LABELSui();
        let player_x = x;
        let player_y = y;

        let mut img_slash_t = AnimatedImage::from_gif("", player_x - 15.0, player_y - 30.0, 70.0, 30.0, true).await;

        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_t.set_preloaded_gif(preloaded, false);
        }

        let mut img_slash_tr = AnimatedImage::from_gif("", player_x + 40.0, player_y - 30.0, 70.0, 30.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_tr.set_preloaded_gif(preloaded, false);
        }
        img_slash_tr.set_angle(PI / 2.0);

        let mut img_slash_r = AnimatedImage::from_gif("", player_x + 45.0, player_y + 10.0, 70.0, 30.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_r.set_preloaded_gif(preloaded, false);
        }
        img_slash_r.set_angle(PI);

        let mut img_slash_br = AnimatedImage::from_gif("", player_x + 40.0, player_y + 60.0, 70.0, 30.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_br.set_preloaded_gif(preloaded, false);
        }

        let mut img_slash_b = AnimatedImage::from_gif("", player_x - 10.0, player_y + 65.0, 70.0, 30.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_b.set_preloaded_gif(preloaded, false);
        }

        let mut img_slash_bl = AnimatedImage::from_gif("", player_x - 60.0, player_y + 50.0, 70.0, 30.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_bl.set_preloaded_gif(preloaded, false);
        }
        img_slash_bl.set_angle(PI / 2.0);
        let mut img_slash_l = AnimatedImage::from_gif("", player_x - 65.0, player_y + 10.0, 70.0, 30.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_l.set_preloaded_gif(preloaded, false);
        }
        let mut img_slash_tl = AnimatedImage::from_gif("", player_x - 60.0, player_y - 50.0, 70.0, 30.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
            img_slash_tl.set_preloaded_gif(preloaded, false);
        }
        let mut img_slash_t_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x - 15.0, // x position
            player_y - 30.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_t_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut img_slash_tr_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x + 40.0, // x position
            player_y - 30.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_tr_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut img_slash_r_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x + 45.0, // x position
            player_y + 10.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_r_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut img_slash_br_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x + 40.0, // x position
            player_y + 60.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_br_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut img_slash_b_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x - 10.0, // x position
            player_y + 65.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_b_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut img_slash_bl_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x - 60.0, // x position
            player_y + 50.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_bl_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut img_slash_l_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x - 65.0, // x position
            player_y + 10.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_l_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut img_slash_tl_hitbox = StillImage::new(
            "",
            70.0,            // width
            30.0,            // height
            player_x - 60.0, // x position
            player_y - 50.0, // y position
            true,            // Enable stretching
            1.0,             // Normal zoom (100%)
        )
        .await;
        img_slash_tl_hitbox.set_preload(tm.get_preload("assets/map_files/wall.png").unwrap().clone());
        let mut player_hitbox = StillImage::new(
            "", 40.0, // width
            60.0, // height
            player_x, player_y, true, 1.0,
        )
        .await;
        player_hitbox.set_preload(preloads[11].clone());
        (
            vec![img_heart, img_arrow, img_disc1, img_disc2, img_disc3, img_musicoin],
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
            ],
            vec![player_hitbox],
            vec![
                img_slash_t_hitbox,
                img_slash_tr_hitbox,
                img_slash_r_hitbox,
                img_slash_br_hitbox,
                img_slash_b_hitbox,
                img_slash_bl_hitbox,
                img_slash_l_hitbox,
                img_slash_tl_hitbox,
            ],
        )
    }

    pub async fn handle_player_ui(&mut self, enemies: &mut Vec<Enemy>, musicdiscs: &mut Musicdisc) -> (bool, bool, usize) {
        //update vars
        let mletimepassed = get_time() - self.last_attack_time;
        let rngtimepassed = get_time() - self.last_rng_attack_time;
        let mockingtimepassed = get_time() - self.last_mocking_time;
        let mut mlehit = false;
        let mut rnghit = false;
        let mut index = 0;
        if mockingtimepassed < 2.0 {
            self.lbl_boss_mocking.draw();
        }
        //update health number
        self.playerui.1[2].set_text(format!("{}", self.health));
        for image in self.playerui.0.iter_mut() {
            image.draw();
        }
        for label in self.playerui.1.iter_mut() {
            label.draw();
        }
        self.inventory.2[2].draw(); //draw gold label
        self.playerui.1[0].draw();
        self.playerui.1[1].draw(); //label must be redrawn very specifically so for loops cant be used  
        self.playerui.0[0].draw();
        self.playerui.1[2].draw();
        for i in 0..self.playerui.0.len() {
            let img = &self.playerui.0[i];
            match img.get_filename() {
                "assets/musicdisc_files/covers/backinblack.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[0]));
                    if times[0] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                "assets/musicdisc_files/covers/thickofit.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[1]));
                    if times[1] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                "assets/musicdisc_files/covers/howitsdone.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[2]));
                    if times[2] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                "assets/musicdisc_files/covers/imstillstanding.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[3]));
                    if times[3] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                "assets/musicdisc_files/covers/pandemonium.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[4]));
                    if times[4] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                "assets/musicdisc_files/covers/sixhundredstrike.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[5]));
                    if times[5] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                "assets/musicdisc_files/covers/sodapop.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[6]));
                    if times[6] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                "assets/musicdisc_files/covers/greatestshow.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i + 2].set_text(format!("{:.0}", times[7]));
                    if times[7] <= 0.0 {
                        self.playerui.1[i + 2].set_text("".to_string());
                    }
                }
                _ => {}
            }
        }
        if self.attack {
            if self.mlevalid == true {
                (index, mlehit) = self.create_melee_attack(enemies, index, mlehit);
                self.mlevalid = false; //prevents multiple melee hits from one attack input
            }
            if mletimepassed > 0.1 && mletimepassed < 1.0 {
                self.mleattackdrawing = true;
                self.attackimg.draw();
            }
            if mletimepassed > 1.0 {
                //attack label only appears for 0.6 seconds after attack
                self.attack = false;
                self.mlevalid = true;
                self.attackimgfound = false;
                self.mleattackdrawing = false;
                self.last_attack_time = get_time();
            }
        }
        if self.rangedattack {
        
            if rngtimepassed < 3.0 {
                let cooldown = (3.0 - rngtimepassed + 1.0) as f32 * self.cooldownmult; //+1 to not show 0 when cooldown is ready
                self.playerui.1[3].set_text(format!("{:.0}", cooldown));
            } else if rngtimepassed >= 3.0 {
                self.playerui.1[3].set_text("".to_string());
            }if rngtimepassed > 3.0 {
                self.rangedattack = false;
                self.rangedattackimgcreated = false;
            }
        }
        let mut rac = 0; //ranged attack counter so that only one enemy is damaged per arrow
        for i in 0..self.arrows.len() {
            let idx = i - rac;
            let y = self.arrows[idx].get_y();
            let x = self.arrows[idx].get_x();
            let movement = vec2(
                self.ranged_movespeeds[idx].x * get_frame_time(),
                self.ranged_movespeeds[idx].y * get_frame_time(),
            );
            if movement.x > 0.0 && movement.y > 0.0 {
                //if diagonal movement, normalize to prevent faster diagonal movement
                self.movement = self.movement.normalize(); //normalize diagonal movement to prevent faster movement when moving diagonally
            }
            self.arrows[idx].set_x(x + movement.x);
            self.arrows[idx].set_y(y + movement.y);
            self.arrows[idx].draw();
            if self.arrows[idx].get_y() < 0.0
                || self.arrows[idx].get_y() > 800.0
                || self.arrows[idx].get_x() < 0.0
                || self.arrows[idx].get_x() > 1200.0
            {
                self.arrows.remove(idx);
                self.ranged_movespeeds.remove(idx);
                rac += 1;
                continue; //skip collision check if arrow is removed for being out of bounds
            }
            for j in 0..enemies.len() {
                let enemy_view_type = enemies[i].get_enemy_view_type();
                if enemy_view_type == "still" {
                    if check_collision(&self.arrows[idx], enemies[j].view_enemy(), 1) {
                        rnghit = true;
                        self.arrows.remove(idx);
                        self.ranged_movespeeds.remove(idx);
                        break; //break to prevent multiple enemies being damaged by one arrow
                    }
                } else if enemy_view_type == "animated" {
                    if check_collision(&self.arrows[idx], enemies[j].view_enemy_animated(), 1) {
                        rnghit = true;
                        self.arrows.remove(idx);
                        self.ranged_movespeeds.remove(idx);
                        break; //break to prevent multiple enemies being damaged by one arrow
                    }
                }
            }
        }
        (mlehit, rnghit, index)
    }

    pub fn create_melee_attack(&mut self, enemies: &mut Vec<Enemy>, mut index: usize, mut mlehit: bool) -> (usize, bool) {
        if self.attackimgfound == false {
            //attackimgbool and match must be kept in player to be used outside of if statements
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
            self.attackimg.reset();
            self.attackimgfound = true;
            self.hitboximg = match self.player_direction.as_str() {
                "t" => self.playerui.4[0].clone(),
                "tr" => self.playerui.4[1].clone(),
                "r" => self.playerui.4[2].clone(),
                "br" => self.playerui.4[3].clone(),
                "b" => self.playerui.4[4].clone(),
                "bl" => self.playerui.4[5].clone(),
                "l" => self.playerui.4[6].clone(),
                "tl" => self.playerui.4[7].clone(),
                _ => self.playerui.4[0].clone(),
            };
            for i in 0..enemies.len() {
                let enemy_view_type = enemies[i].get_enemy_view_type();
                if enemy_view_type == "still" {
                    if check_collision(&self.hitboximg, enemies[i].view_enemy(), 1) {
                        mlehit = true;
                        index = i;
                    }
                } else if enemy_view_type == "animated" {
                    if check_collision(&self.hitboximg, enemies[i].view_enemy_animated(), 1) {
                        mlehit = true;
                        index = i;
                    }
                }
                if self.inventory.1[5].get_filename() == "assets/item_files/weapons/mosquito_rapier.png" {
                    if mlehit == true {
                        self.healplayer(3.0);
                    }
                }
            }
        }
        (index, mlehit)
    }

    pub async fn create_range_attack(&mut self) {
        // enforce cooldown (3.0s) to prevent firing multiple arrows
        let cooldown = 3.0;
        let now = get_time();
        if now - self.last_rng_attack_time < cooldown {
            return;
        }
        if self.rangedattackimgcreated == false {
            // mark as created and record time before awaiting to avoid re-entrancy
            self.rangedattackimgcreated = true;
            self.last_rng_attack_time = now;
            let player_x = self.view.get_x();
            let player_y = self.view.get_y();
            let (coords, angle, movespeed) = match self.player_direction.as_str() {
                "t" => (vec2(player_x + 20.0, player_y - 15.0), -PI / 2.0, vec2(0.0, -600.0)),
                "tr" => (vec2(player_x + 50.0, player_y - 15.0), -PI / 4.0, vec2(600.0, -600.0)),
                "r" => (vec2(player_x + 15.0, player_y + 30.0), 0.0, vec2(600.0, 0.0)),
                "br" => (vec2(player_x + 50.0, player_y + 75.0), PI / 4.0, vec2(600.0, 600.0)),
                "b" => (vec2(player_x + 20.0, player_y + 75.0), PI / 2.0, vec2(0.0, 600.0)),
                "bl" => (vec2(player_x - 15.0, player_y + 75.0), 3.0 * PI / 4.0, vec2(-600.0, 600.0)),
                "l" => (vec2(player_x - 15.0, player_y + 30.0), PI, vec2(-600.0, 0.0)),
                "tl" => (vec2(player_x - 15.0, player_y - 15.0), -3.0 * PI / 4.0, vec2(-600.0, -600.0)),
                _ => (vec2(0.0, 0.0), 0.0, vec2(0.0, 0.0)),
            };
            // keep per-arrow movespeeds in sync with self.arrows
            self.ranged_movespeeds.push(movespeed);
            let mut rng_attack_img = StillImage::new(
                "", 30.0,     // width
                30.0,     // height
                coords.x, // x position
                coords.y, // y position
                true,     // Enable stretching
                1.0,      // Normal zoom (100%)
            )
            .await;
            rng_attack_img.set_preload(self.preloads[11].clone());
            rng_attack_img.set_angle(angle);
            self.arrows.push(rng_attack_img);
            self.rangedattackimgcreated = true;
            self.last_rng_attack_time = get_time();
        }
    }
    //SAVE MENUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUUU:)
    async fn create_save_menu(preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>) -> (Vec<StillImage>, Vec<Label>, Vec<TextButton>) {
        let mut shadow = StillImage::new("", VIRTUAL_WIDTH, VIRTUAL_HEIGHT, 0.0, 0.0, true, 1.0).await;
        shadow.set_preload(preloads[13].clone());
        shadow.set_opacity(0.7);
        let mut lbl_paused = Label::new("Paused", VIRTUAL_WIDTH / 2.0 - 40.0, VIRTUAL_HEIGHT / 2.0 - 50.0, 60);
        lbl_paused.with_colors(WHITE, Some(BLACK));
        let mut btn_save = TextButton::new(VIRTUAL_WIDTH / 2.0 - 50.0, VIRTUAL_HEIGHT / 2.0, 200.0, 75.0, "Save", BLACK, GREEN, 30);
        btn_save.with_text_color(WHITE);
        let mut btn_exit = TextButton::new(VIRTUAL_WIDTH / 2.0 - 50.0, VIRTUAL_HEIGHT / 2.0 + 200.0, 200.0, 75.0, "Exit to Menu", BLACK, RED, 30,);
        btn_exit.with_text_color(WHITE);
        let mut lbl_controls = Label::new("Controls:\nWASD to Move\nUp Arrow to melee Attack\nRight arrow to Ranged Attack\nE or Q or X to Use Musicdiscs\nTab to open/close Inventory\nEsc to open/close Pause Menu\nWARNING: WAIT A FEW SECONDS AFTER SAVING!!!!", VIRTUAL_WIDTH / 2.0 - 430.0, VIRTUAL_HEIGHT / 2.0 - 100.0, 30);
        lbl_controls.with_colors(WHITE, None);
        (vec![shadow], vec![lbl_paused, lbl_controls], vec![btn_save, btn_exit])
    }

    pub async fn handle_save_menu(&mut self) -> (bool, bool) {
        let (mut save, mut exit) = (false, false);
        if self.save_menu_open == true {
            //draw menu
            for image in self.savemenu.0.iter_mut() {
                image.draw();
            }
            for label in self.savemenu.1.iter_mut() {
                label.draw();
            }
            //handle button interactions

            if self.savemenu.2[0].click() {
                //save button
                save = true;
                self.save_menu_open = false;
            }  
            if self.savemenu.2[1].click() {
                //exit to menu button
                exit = true;
                self.save_menu_open = false;
            }
                }
        (save, exit)
    }

    async fn create_death_screen(preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>) -> (Vec<StillImage>, Vec<Label>, Vec<TextButton>) {
        let mut shadow = StillImage::new("", VIRTUAL_WIDTH, VIRTUAL_HEIGHT, 0.0, 0.0, true, 1.0).await;
        shadow.set_preload(preloads[13].clone());
        shadow.set_opacity(0.7);
        let mut lbl_death = Label::new("You Died", VIRTUAL_WIDTH / 2.0 - 100.0, VIRTUAL_HEIGHT / 2.0 - 50.0, 60);
        lbl_death.with_colors(WHITE, Some(BLACK));
        let mut btn_retry = TextButton::new(VIRTUAL_WIDTH / 2.0 - 100.0, VIRTUAL_HEIGHT / 2.0, 200.0, 75.0, "Retry", BLACK, GREEN, 30);
        btn_retry.with_text_color(WHITE);
        let mut btn_exit = TextButton::new(
            VIRTUAL_WIDTH / 2.0 - 100.0,
            VIRTUAL_HEIGHT / 2.0 + 200.0,
            200.0,
            75.0,
            "Exit to Menu",
            BLACK,
            RED,
            30,
        );
        btn_exit.with_text_color(WHITE);
        (vec![shadow], vec![lbl_death], vec![btn_retry, btn_exit])
    }

    pub async fn handle_death_screen(&mut self, pause: &mut bool, musicdiscfunctions: &mut Musicdisc) -> (bool, bool) {
        let mut btn_clicks = (false, false);
        if self.death_screen_open == true {
            //draw menu
            for image in self.death_screen.0.iter_mut() {
                image.draw();
            }
            for label in self.death_screen.1.iter_mut() {
                label.draw();
            }
            //handle button interactions

            if self.death_screen.2[0].click() {
                //retry button
                btn_clicks.0 = true;
                self.health = self.maxhealth;
                self.playerui.1[1].with_fixed_size(400.0, 25.0);
                self.death_screen_open = false;
                *pause = false;
            } else if self.death_screen.2[1].click() {
                //exit to menu button
                btn_clicks.1 = true;
                self.health = self.maxhealth;
                self.death_screen_open = false;
                *pause = false;
            }
        } else if self.health <= 0.0 && self.death_screen_open == false && musicdiscfunctions.get_imstillstanding_active() == false {
            self.death_screen_open = true;
            stop_sound(musicdiscfunctions.get_currently_playing());
            play_sound(musicdiscfunctions.get_bgmusic(), PlaySoundParams { looped: false, volume: 1.0 });
            *pause = true; //pause game when death screen opens
        }
        btn_clicks
    }

    #[allow(unused)]
    pub async fn death_screen_open(&self) -> bool {
        self.death_screen_open
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
        let mut lbl_gold = Label::new(format!("0"), 80.0, 100.0, 60);
        lbl_gold.with_colors(WHITE, None);
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
            self.inventory.2[0].draw();
            self.inventory.2[1].draw();
            self.inventory.2[2].set_text(format!("{}", self.musicoins));
            if self.inventory.3[0].click() {
                let title = self.inventory.0[0].selected_item().unwrap();
                for (i, item) in self.items.iter().enumerate() {
                    if item.get_itemtitle() == *title {
                        println!("Equipping item: {}", item.get_itemtitle());
                        println!("Item type: {}", item.get_itemtype());
                        if item.get_itemtype() == "disc" {
                            let already_equipped = self.equipped_items.iter().any(|equipped_item| {
                                self.items[*equipped_item].get_itemtype() == "disc"
                                    && self.items[*equipped_item].get_itemtitle() == item.get_itemtitle()
                            });

                            if already_equipped {
                                break;
                            }
                        }
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
                        if item.get_itemtype() == "disc" {
                            self.refresh_disc_display();
                        }
                        self.update_stats();
                        println!("equipped items: {:?}", self.equipped_items);
                        println!(
                            "playerstats: health: {}, mledmg: {}, rngdmg: {}, movespeedmult: {}, cooldownmult: {}, armor: {}",
                            self.health, self.mledmg, self.rngdmg, self.movespeedmult, self.cooldownmult, self.armor
                        );
                        break;
                    }
                }
                let mut new_width = self.health as f32 * 4.0; // Assuming 100 health corresponds to 400 width
                let mut max_width = self.maxhealth as f32 * 4.0; // Maximum width based on max health
                if self.inventory.1[3].get_filename() == "assets/item_files/armour/lifeforce_armor.png" {
                    new_width = self.health * 2.0; // Assuming 100 health corresponds to 400 width
                    max_width = self.maxhealth * 2.0; // Double the maximum health
                }
                if new_width < 0.0 {
                    new_width = 0.0; // Prevent negative width
                }
                self.playerui.1[0].with_fixed_size(max_width, 25.0); //update healthbar size based on health
                self.playerui.1[1].with_fixed_size(new_width, 25.0); //update healthbar size based on health
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
        let equipped_pos = self.equipped_items.iter().position(|index| *title == self.items[*index].get_itemtitle());

        if let Some(equipped_pos) = equipped_pos {
            let index = self.equipped_items[equipped_pos];
            let imageboxindex = match self.items[index].get_itemtype().as_str() {
                "helmet" => 2,
                "bodyarmor" => 3,
                "boots" => 4,
                "melee" => 5,
                "ranged" => 6,
                "disc" => {
                    if self.inventory.1[7].get_filename() == self.items[index].get_itemassetpath() {
                        7
                    } else if self.inventory.1[8].get_filename() == self.items[index].get_itemassetpath() {
                        8
                    } else {
                        9
                    }
                }
                _ => 2,
            };

            self.equipped_items.remove(equipped_pos);
            self.inventory.1[imageboxindex].set_preload(self.preloads[1].clone());
            if self.items[index].get_itemtype() == "disc" {
                self.refresh_disc_display();
            }
            self.update_stats();
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
        self.mledmg = 3.0;
        self.rngdmg = 5.0;
        self.movespeedmult = 1.0;
        self.cooldownmult = 1.0;
        self.maxhealth = 100.0;
        self.armor = 0;

        // Apply item stat changes
        for itemindex in &self.equipped_items {
            self.mledmg += self.items[*itemindex].get_itemmledmg() as f32;
            self.rngdmg += self.items[*itemindex].get_itemrngdmg() as f32;
            self.movespeedmult += self.items[*itemindex].get_itemmovespeedmult() as f32;
            self.cooldownmult += self.items[*itemindex].get_itemcooldownmult() as f32;
            self.maxhealth += self.items[*itemindex].get_itemhpchng() as f32;
            self.armor += self.items[*itemindex].get_itemarmor();
        }
    }

    pub fn stat_override(&mut self, mledmg: f32, rngdmg: f32, movespeedmult: f32, cooldownmult: f32, maxhealth: f32, armor: i32) {
        self.mledmg = mledmg;
        self.rngdmg = rngdmg;
        self.movespeedmult = movespeedmult;
        self.cooldownmult = cooldownmult;
        self.maxhealth = maxhealth;
        self.armor = armor;
    }

    fn refresh_disc_display(&mut self) {
        //just an index, like a vec2 or a 2d list, check slot7 hud2, slot8 hud3, slot9 hud4, if disc in inventory slot, set preload to disc img, else set to invslot preload
        let disc_slots = [(7usize, 2usize), (8usize, 3usize), (9usize, 4usize)];

        for (inventory_slot, hud_slot) in disc_slots {
            let slot_filename = self.inventory.1[inventory_slot].get_filename().to_string();
            let mut found_disc = false;

            for item in self.items.iter() {
                if item.get_itemtype() == "disc" && item.get_itemassetpath() == slot_filename {
                    self.playerui.0[hud_slot].set_preload(item.get_itemimgpath());
                    found_disc = true;
                    break;
                }
            }

            if found_disc == false {
                self.playerui.0[hud_slot].set_preload(self.preloads[1].clone());
            }
        }
    }

    fn activate_disc_by_title(&mut self, musicdiscs: &mut Musicdisc, disc_title: &str) {
        let validity = musicdiscs.get_musicdisc_validity();

        match disc_title {
            "Back In Black" => {
                if validity[0] == true {
                    self.activedisc = "Back In Black".to_string();
                    musicdiscs.start_musicdisc_time("Back In Black");
                }
            }
            "Thick Of It" => {
                if validity[1] == true {
                    self.activedisc = "Thick Of It".to_string();
                    musicdiscs.start_musicdisc_time("Thick Of It");
                }
            }
            "How It's Done" => {
                if validity[2] == true {
                    self.activedisc = "How It's Done".to_string();
                    musicdiscs.start_musicdisc_time("How It's Done");
                }
            }
            "I'm Still Standing" => {
                if validity[3] == true {
                    self.activedisc = "I'm Still Standing".to_string();
                    musicdiscs.start_musicdisc_time("I'm Still Standing");
                }
            }
            "Pandemonium" => {
                if validity[4] == true {
                    self.activedisc = "Pandemonium".to_string();
                    musicdiscs.start_musicdisc_time("Pandemonium");
                }
            }
            "Six Hundred Strike" => {
                if validity[5] == true {
                    self.activedisc = "Six Hundred Strike".to_string();
                    musicdiscs.start_musicdisc_time("Six Hundred Strike");
                }
            }
            "Soda Pop" => {
                if validity[6] == true {
                    self.activedisc = "Soda Pop".to_string();
                    musicdiscs.start_musicdisc_time("Soda Pop");
                }
            }
            "Greatest Show" => {
                if validity[7] == true {
                    self.activedisc = "The Greatest Show".to_string();
                    musicdiscs.start_musicdisc_time("The Greatest Show");
                }
            }
            _ => {}
        }
    }

    pub fn handle_musicdiscs(&mut self, musicdiscs: &mut Musicdisc, inventory_slot_index: usize) {
        if inventory_slot_index >= self.inventory.1.len() {
            return;
        }

        let slot_filename = self.inventory.1[inventory_slot_index].get_filename().to_string();
        if slot_filename == "assets/player_files/invslot.png" {
            return;
        }

        for item in self.items.iter() {
            if item.get_itemtype() == "disc" && item.get_itemassetpath() == slot_filename {
                self.activate_disc_by_title(musicdiscs, &item.get_itemtitle());
                break;
            }
        }
    }

    pub fn set_usernamepassword(&mut self, name: String, password: String) {
        self.name = name;
        self.password = password;
    }

    pub fn set_health(&mut self, health: f32) {
        self.health = health;
        let mut new_width = self.health as f32 * 4.0; // Assuming 100 health corresponds to 400 width
        let mut max_width = self.maxhealth as f32 * 4.0; // Maximum width based on max health
        if self.inventory.1[3].get_filename() == "assets/item_files/armour/lifeforce_armor.png" {
            new_width = self.health * 2.0; // Assuming 100 health corresponds to 400 width
            max_width = self.maxhealth * 2.0; // Double the maximum health
        }
        if new_width < 0.0 {
            new_width = 0.0; // Prevent negative width
        }
        self.playerui.1[0].with_fixed_size(max_width, 25.0); //update healthbar size based on health
        self.playerui.1[1].with_fixed_size(new_width, 25.0); //update healthbar size based on health
    }

    pub fn add_health(&mut self, health: f32) {
        self.health += health;
        if self.health > self.maxhealth {
            self.health = self.maxhealth;
        }
        let mut new_width = self.health as f32 * 4.0; // Assuming 100 health corresponds to 400 width
        let mut max_width = self.maxhealth as f32 * 4.0; // Maximum width based on max health
        if self.inventory.1[3].get_filename() == "assets/item_files/armour/lifeforce_armor.png" {
            new_width = self.health * 2.0; // Assuming 100 health corresponds to 400 width
            max_width = self.maxhealth * 2.0; // Double the maximum health
        }
        if new_width < 0.0 {
            new_width = 0.0; // Prevent negative width
        }
        self.playerui.1[0].with_fixed_size(max_width, 25.0); //update healthbar size based on health
        self.playerui.1[1].with_fixed_size(new_width, 25.0); //update healthbar size based on health
    }

    pub fn set_save_data(&mut self, record: &DatabaseTable) {
        self.cleared = record.player_clearedvar;
        self.set_x(record.player_x as f32);
        self.set_y(record.player_y as f32);
        self.addcoins(record.musicoins as i32);
        self.set_health(record.currenthealth as f32);
        let mut tempinventory = vec![];
        tempinventory.push(record.inv_1);
        tempinventory.push(record.inv_2);
        tempinventory.push(record.inv_3);
        tempinventory.push(record.inv_4);
        tempinventory.push(record.inv_5);
        tempinventory.push(record.inv_6);
        tempinventory.push(record.inv_7);
        tempinventory.push(record.inv_8);
        tempinventory.push(record.inv_9);
        tempinventory.push(record.inv_10);
        tempinventory.push(record.inv_11);
        tempinventory.push(record.inv_12);
        tempinventory.push(record.inv_13);
        tempinventory.push(record.inv_14);
        tempinventory.push(record.inv_15);
        tempinventory.push(record.inv_16);
        tempinventory.push(record.inv_17);
        tempinventory.push(record.inv_18);
        tempinventory.push(record.inv_19);
        tempinventory.push(record.inv_20);
        self.inventory.0[0].clear();
        for iteminteger in tempinventory {
            if iteminteger != 0 {
                self.add_inventory_item(self.possible_items[iteminteger as usize - 1].clone());
            }
        }
    }

    pub fn get_currentscreen(&self) -> String {
        self.currentscreen.clone()
    }

    pub fn set_currentscreen(&mut self, screen: String) {
        self.currentscreen = screen;
    }

    pub async fn update_save_data(&self, records: &Vec<DatabaseTable>, client: &DatabaseClient, _last_scene: &String) {
        let mut save_id = 1;
        for i in 0..records.len() {
            if records[i].user_name == self.get_name() {
                save_id = records[i].id;
                println!("Found matching save record with id: {}", save_id);
            }
        }
        if let Ok(_updated_count) = client
            .update_record_by_id("save_table", save_id as i64, "player_clearedvar", self.get_cleared().to_string().as_str())
            .await
        {}
        if let Ok(_updated_count) = client
            .update_record_by_id("save_table", save_id as i64, "player_currentscreenvar", self.get_currentscreen().as_str())
            .await
        {}
        if let Ok(_updated_count) = client
            .update_record_by_id("save_table", save_id as i64, "player_x", self.get_x().to_string().as_str())
            .await
        {}
        if let Ok(_updated_count) = client
            .update_record_by_id("save_table", save_id as i64, "player_y", self.get_y().to_string().as_str())
            .await
        {}
        if let Ok(_updated_count) = client
            .update_record_by_id("save_table", save_id as i64, "musicoins", self.get_musicoins().to_string().as_str())
            .await
        {}
        if let Ok(_updated_count) = client
            .update_record_by_id("save_table", save_id as i64, "current_health", self.get_health().to_string().as_str())
            .await
        {}
        for (slot_index, inventory_item) in self.items.iter().enumerate() {
            let column_name = format!("inv_{}", slot_index + 1);
            for (item_index, possible_item) in self.possible_items.iter().enumerate() {
                if inventory_item.get_itemtitle() == possible_item.get_itemtitle() {
                    if let Ok(_updated_count) = client
                        .update_record_by_id("save_table", save_id as i64, column_name.as_str(), (item_index + 1).to_string().as_str())
                        .await
                    {}
                    break;
                }
            }
        }
    }

    pub async fn create_item_ui(tm: &TextureManager) -> (Vec<StillImage>, Vec<Label>, Vec<TextButton>) {
        let mut lbl_bg_item1 = Label::new("", 50.0, 75.0, 30);
        lbl_bg_item1.with_colors(WHITE, Some(BLUE));
        lbl_bg_item1.with_fixed_size(250.0, 700.0);
        let mut lbl_bg_item2 = Label::new("", 400.0, 75.0, 30);
        lbl_bg_item2.with_colors(WHITE, Some(RED));
        lbl_bg_item2.with_fixed_size(250.0, 700.0);
        let mut lbl_bg_item3 = Label::new("", 750.0, 75.0, 30);
        lbl_bg_item3.with_colors(WHITE, Some(GREEN));
        lbl_bg_item3.with_fixed_size(250.0, 700.0);
        let mut img_item1 = StillImage::new(
            "", 220.0, // width
            200.0, // height
            60.0,  // x position
            60.0,  // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_item1.set_preload(tm.get_preload("assets/arrow.png").unwrap());
        let mut lbl_item1_title = Label::new(format!("Item 1"), 50.0, 300.0, 40);
        lbl_item1_title.with_colors(BLACK, None);
        lbl_item1_title.with_fixed_size(250.0, 75.0);
        lbl_item1_title.with_alignment(modules::label::TextAlign::Center);
        let mut lbl_item1_desc = Label::new(format!("This is a description of item 1."), 50.0, 350.0, 20);
        lbl_item1_desc.with_colors(BLACK, None);
        lbl_item1_desc.with_fixed_size(250.0, 700.0);
        lbl_item1_desc.with_alignment(modules::label::TextAlign::Left);
        let btn_item1 = TextButton::new(50.0, 640.0, 240.0, 100.0, "Choose!", PINK, GREEN, 30);
        let mut img_item2 = StillImage::new(
            "", 220.0, // width
            200.0, // height
            410.0, // x position
            60.0,  // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_item2.set_preload(tm.get_preload("assets/arrow.png").unwrap());
        let mut lbl_item2_title = Label::new(format!("Item 2"), 400.0, 300.0, 40);
        lbl_item2_title.with_colors(BLACK, None);
        lbl_item2_title.with_fixed_size(250.0, 75.0);
        lbl_item2_title.with_alignment(modules::label::TextAlign::Center);
        let mut lbl_item2_desc = Label::new(format!("This is a description of item 2."), 400.0, 350.0, 20);
        lbl_item2_desc.with_colors(BLACK, None);
        lbl_item2_desc.with_fixed_size(250.0, 700.0);
        lbl_item2_desc.with_alignment(modules::label::TextAlign::Left);
        let btn_item2 = TextButton::new(400.0, 640.0, 240.0, 100.0, "Choose!", ORANGE, GREEN, 30);
        let mut img_item3 = StillImage::new(
            "", 220.0, // width
            200.0, // height
            760.0, // x position
            60.0,  // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_item3.set_preload(tm.get_preload("assets/arrow.png").unwrap());
        let mut lbl_item3_title = Label::new(format!("Item 3"), 750.0, 300.0, 40);
        lbl_item3_title.with_colors(BLACK, None);
        lbl_item3_title.with_fixed_size(250.0, 75.0);
        lbl_item3_title.with_alignment(modules::label::TextAlign::Center);
        let mut lbl_item3_desc = Label::new(format!("This is a description of item 3."), 750.0, 350.0, 20);
        lbl_item3_desc.with_colors(BLACK, None);
        lbl_item3_desc.with_fixed_size(250.0, 700.0);
        lbl_item3_desc.with_alignment(modules::label::TextAlign::Left);
        let btn_item3 = TextButton::new(750.0, 640.0, 240.0, 100.0, "Choose!", BROWN, GREEN, 30);

        (
            vec![img_item1, img_item2, img_item3],
            vec![
                lbl_bg_item1,
                lbl_bg_item2,
                lbl_bg_item3,
                lbl_item1_title,
                lbl_item1_desc,
                lbl_item2_title,
                lbl_item2_desc,
                lbl_item3_title,
                lbl_item3_desc,
            ],
            vec![btn_item1, btn_item2, btn_item3],
        )
    }

    pub fn handle_choose_item(&mut self, choose_open: &mut bool, item_valid: &mut bool) -> (bool, bool) {
        rand::srand(date::now() as u64);
        if choose_open == &true {
            for label in self.itemui.1.iter_mut() {
                label.draw();
            }
            for item in self.itemui.0.iter_mut() {
                item.draw();
            }
            if item_valid == &true {
                let item_count = self.possible_items.len();
                if item_count < 3 {
                    return (*choose_open, *item_valid);
                }
                let pick_unique_index = |used_indices: &[usize]| -> usize {
                    loop {
                        let candidate = rand::gen_range(0, item_count as i32) as usize;
                        if !used_indices.contains(&candidate) && !self.equipped_items.contains(&candidate) {
                            return candidate;
                        }
                    }
                };
                loop {
                    self.itemindex1 = pick_unique_index(&[]);
                    if self.possible_items[self.itemindex1].get_itemtitle() != self.inventory.2[0].get_text() {
                        break;
                    }
                }
                self.itemindex2 = pick_unique_index(&[self.itemindex1]);
                self.itemindex3 = pick_unique_index(&[self.itemindex1, self.itemindex2]);
                self.itemui.0[0].set_preload(self.possible_items[self.itemindex1].get_itemimgpath());
                self.itemui.0[1].set_preload(self.possible_items[self.itemindex2].get_itemimgpath());
                self.itemui.0[2].set_preload(self.possible_items[self.itemindex3].get_itemimgpath());
                self.itemui.1[3].set_text(self.possible_items[self.itemindex1].get_itemtitle());
                self.itemui.1[4].set_text(self.possible_items[self.itemindex1].get_itemdescription());
                self.itemui.1[5].set_text(self.possible_items[self.itemindex2].get_itemtitle());
                self.itemui.1[6].set_text(self.possible_items[self.itemindex2].get_itemdescription());
                self.itemui.1[7].set_text(self.possible_items[self.itemindex3].get_itemtitle());
                self.itemui.1[8].set_text(self.possible_items[self.itemindex3].get_itemdescription());
                *item_valid = false;
            }
            if self.itemui.2[0].click() {
                if self.possible_items[self.itemindex1].get_itemtype() == "health" {
                    self.add_health(50.0);
                } else {
                    self.add_inventory_item(self.possible_items[self.itemindex1].clone());
                }
                *choose_open = false;
            }
            if self.itemui.2[1].click() {
                if self.possible_items[self.itemindex2].get_itemtype() == "health" {
                    self.add_health(50.0);
                } else {
                    self.add_inventory_item(self.possible_items[self.itemindex2].clone());
                }
                *choose_open = false;
            }
            if self.itemui.2[2].click() {
                if self.possible_items[self.itemindex3].get_itemtype() == "health" {
                    self.add_health(50.0);
                } else {
                    self.add_inventory_item(self.possible_items[self.itemindex3].clone());
                }
                *choose_open = false;
            }
        }
        (*choose_open, *item_valid)
    }

    pub async fn create_all_items(tm: &TextureManager) -> Vec<Item> {
        let mut possible_items = vec![];
        let backinblackitem = Item::new(
            tm.get_preload("assets/musicdisc_files/covers/backinblack.png").unwrap(),
            "assets/musicdisc_files/covers/backinblack.png".to_string(),
            "Back In Black".to_string(),
            "A Disc that allows the user to summon periodic pillars of fire".to_string(),
            "disc".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(backinblackitem);
        let thickofititem = Item::new(
            tm.get_preload("assets/musicdisc_files/covers/thickofit.png").unwrap(),
            "assets/musicdisc_files/covers/thickofit.png".to_string(),
            "Thick Of It".to_string(),
            "A Disc that sounds so bad all enemies stop attacking and move away, enemies hate it so much they will teleport away if need be"
                .to_string(),
            "disc".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(thickofititem);
        let howitsdoneitem = Item::new(
            tm.get_preload("assets/musicdisc_files/covers/howitsdone.png").unwrap(),
            "assets/musicdisc_files/covers/howitsdone.png".to_string(),
            "How It's Done".to_string(),
            "A Disc that puts the user into a flow state multiplying all stats largely making the user near invincible".to_string(),
            "disc".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(howitsdoneitem);
        let pandemoniumitem = Item::new(
            tm.get_preload("assets/musicdisc_files/covers/pandemonium.png").unwrap(),
            "assets/musicdisc_files/covers/pandemonium.png".to_string(),
            "Pandemonium".to_string(),
            "A Disc that causes extreme confusion, making all enemies attack the highest health enemy on screen".to_string(),
            "disc".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(pandemoniumitem);
        let sixhundredstrikeitem = Item::new(
            tm.get_preload("assets/musicdisc_files/covers/sixhundredstrike.png").unwrap(),
            "assets/musicdisc_files/covers/sixhundredstrike.png".to_string(),
            "Six Hundred Strike".to_string(),
            "A Disc that calls upon the wrath of odysseus to strike down the highest opponent for massive damage periodically".to_string(),
            "disc".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(sixhundredstrikeitem);
        let sodapopitem = Item::new(
            tm.get_preload("assets/musicdisc_files/covers/sodapop.png").unwrap(),
            "assets/musicdisc_files/covers/sodapop.png".to_string(),
            "Soda Pop".to_string(),
            "A Disc that forces all enemies to stop and dance for 10 seconds".to_string(),
            "disc".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(sodapopitem);
        let greatestshowitem = Item::new(
            tm.get_preload("assets/musicdisc_files/covers/greatestshowman.png").unwrap(),
            "assets/musicdisc_files/covers/greatestshowman.png".to_string(),
            "Greatest Show".to_string(),
            "A Disc that calls upon the power of the greatest showman, summoning a meteor that gets bigger the longer you arent hit".to_string(),
            "disc".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(greatestshowitem);
        //sword 1
        let time_sword = Item::new(
            tm.get_preload("assets/item_files/weapons/time_sword.png").unwrap(),
            "assets/item_files/weapons/time_sword.png".to_string(),
            "Time Sword".to_string(),
            "A Sword wielded across timelines, increasing damage and movespeed".to_string(),
            "melee".to_string(),
            10,
            0,
            0.0,
            1.5,
            0,
            0,
        )
        .await;
        possible_items.push(time_sword);
        //bow 1
        let future_bow = Item::new(
            tm.get_preload("assets/item_files/weapons/future_bow.png").unwrap(),
            "assets/item_files/weapons/future_bow.png".to_string(),
            "Future Bow".to_string(),
            "A Bow that can shoot into the future increasing damage and decreasing cooldowns".to_string(),
            "ranged".to_string(),
            0,
            10,
            0.5,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(future_bow);
        //bodyarmor 1
        let diamond_armor = Item::new(
            tm.get_preload("assets/item_files/armour/diamond_armor.png").unwrap(),
            "assets/item_files/armour/diamond_armor.png".to_string(),
            "Diamond BA".to_string(),
            "A chesplate rumoured to be unbreakable, increasing armor signifigantly".to_string(),
            "bodyarmor".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            8,
        )
        .await;
        possible_items.push(diamond_armor);
        //boots 1
        let hermes_boots = Item::new(
            tm.get_preload("assets/item_files/armour/hermes_boots.png").unwrap(),
            "assets/item_files/armour/hermes_boots.png".to_string(),
            "Hermes Boots".to_string(),
            "A pair of boots a god once used to take flight, increases movespeed to an astonishing degree, also increaes armor slightly".to_string(),
            "boots".to_string(),
            0,
            0,
            0.0,
            3.0,
            0,
            1,
        )
        .await;
        possible_items.push(hermes_boots);
        //helmet 1
        let helmet_of_thorns: Item = Item::new(
            tm.get_preload("assets/item_files/armour/helmet_of_thorns.png").unwrap(),
            "assets/item_files/armour/helmet_of_thorns.png".to_string(),
            "Helmet Of Thorns".to_string(),
            "A helmet that punishes any who dare hit the wearer with ravenous thorns, also increaes armor slightly".to_string(),
            "helmet".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            2,
        )
        .await;
        possible_items.push(helmet_of_thorns);
        //sword 2
        let tmos_rapier = Item::new(
            tm.get_preload("assets/item_files/weapons/mosquito_rapier.png").unwrap(),
            "assets/item_files/weapons/mosquito_rapier.png".to_string(),
            "TMos Rapier".to_string(),
            "A legendary rapier forged from the beak of a moquisto once wielded by a group of teenagers to fight ominent, decreases attack but adds lifesteal".to_string(),
            "melee".to_string(),
            -2,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(tmos_rapier);
        //bow 2
        let axl_greatbow = Item::new(
            tm.get_preload("assets/item_files/weapons/axl_greatbow.png").unwrap(),
            "assets/item_files/weapons/axl_greatbow.png".to_string(),
            "Axl Greatbow".to_string(),
            "A powerful slow bow that can shoot arrows with incredible force, increasing damage and cooldowns".to_string(),
            "ranged".to_string(),
            0,
            18,
            1.5,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(axl_greatbow);
        //bodyarmor 2
        let lifeforce_armor = Item::new(
            tm.get_preload("assets/item_files/armour/lifeforce_armor.png").unwrap(),
            "assets/item_files/armour/lifeforce_armor.png".to_string(),
            "Scorned Heart".to_string(),
            "An armor that increases the wearer's very life force, doubling max health".to_string(),
            "bodyarmor".to_string(),
            0,
            0,
            0.0,
            0.0,
            100,
            0,
        )
        .await;
        possible_items.push(lifeforce_armor);
        //boots 2
        let shadow_boots = Item::new(
            tm.get_preload("assets/item_files/armour/shadow_boots.png").unwrap(),
            "assets/item_files/armour/shadow_boots.png".to_string(),
            "Shadow Boots".to_string(),
            "A pair of boots once worn by the sneakiest of rogues, gives the wearer a chance to dodge if an enemy hits them".to_string(),
            "boots".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(shadow_boots);
        //helmet 2
        let l_cap = Item::new(
            tm.get_preload("assets/item_files/armour/l_cap.png").unwrap(),
            "assets/item_files/armour/l_cap.png".to_string(),
            "L's Cap".to_string(),
            "A ballcap that infuses the wearer with a sense of plumbing confidence, and all around efficiency, lightly buffs every stat.".to_string(),
            "helmet".to_string(),
            2,
            2,
            0.8,
            1.2,
            0,
            2,
        )
        .await;
        possible_items.push(l_cap);
        let healthpot = Item::new(
            tm.get_preload("assets/item_files/healthpot.png").unwrap(),
            "assets/item_files/healthpot.png".to_string(),
            "Health Potion".to_string(),
            "Restores 50 health instantly".to_string(),
            "health".to_string(),
            0,
            0,
            0.0,
            0.0,
            0,
            0,
        )
        .await;
        possible_items.push(healthpot);
        possible_items
    }
}
