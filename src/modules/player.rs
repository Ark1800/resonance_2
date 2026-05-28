use crate::modules::{self, musicdisc};
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
use crate::modules::animated_image::AnimatedImage;
use crate::modules::preload_image::TextureManager;
use crate::modules::musicdisc::Musicdisc;

//TO DOOOOOO
/*
//Bug fixes/extras
//1. add hitboxes to swords
//2. collision for thickofit

Work
//1. All Music Discs
//2. W1S1 Enemies
//3. W1S2 Enemies
//4. W1S3 Enemies
//5. W1S4 Enemies
//6. W1SB Boss
//8. Item after each scene
//9. All items
//10. Player Dying
//11. Inventory Db
//12. Player Db
//13. User Db
//14. Start Screen
 
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
outisde loop...

in your loop..
let mut enemies = vec![summoner, mage, large_slime]; //list of enemies for scene

player.handle_keypresses(pause, musicdiscs).await;
player.move_player();
player.handle_player_ui(&mut enemies, musicdiscfunctions).await;
player.handle_inventory();
player.handle_playerdamaging(&enemies);
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
    view: StillImage,                                    //stillimage of player
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
    playerui: (Vec<StillImage>, Vec<Label>, Vec<AnimatedImage>, Vec<StillImage>),             //2d list for player UI elements (images, labels)
    inventoryopen: bool,                                 //is inventory open
    armor: i32,                                          //armor value for damage reduction
    attack: bool,                                        //is player currently attacking (for drawing attack labels)
    last_attack_time: f64,                               //time of last attack for timing attack labels
    attackimgfound: bool, //has the correct attack label been found for the current direction of attack (to prevent repeatedly searching for it every frame)
    attackimg: AnimatedImage, //current attack label to be drawn when attacking 
    mlevalid: bool, //is player currently performing a melee attack (for preventing multiple melee hits from one attack input)
    rangedattack: bool, //is player currently performing a ranged attack (for drawing ranged attack labels)
    last_rng_attack_time: f64, //time of last ranged attack for timing ranged attack labels and cooldowns
    rangedattackimgcreated: bool, //has the correct ranged attack label been found/created for the current direction of attack (to prevent repeatedly searching/creating for it every frame)
    ranged_movespeeds: Vec<Vec2>, //movement speed of the projectile for ranged attacks
    arrows: Vec<StillImage>, //list of self.arrow projectiles for ranged attacks
    player_direction: String, //current direction player is facing for attack purposes (up, down, left, right, etc.)
    activedisc: String,
    cleared: i32,
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
        let attackimg = playerui.2[0].clone();

        Player {
            view,
            move_speed: 400.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
            health: 100.0,
            maxhealth: 100.0,
            mledmg: 1000.0,
            rngdmg: 100.0,
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
            mlevalid: true,
            rangedattack: false,
            last_rng_attack_time: 0.0,
            rangedattackimgcreated: false,
            ranged_movespeeds: Vec::new(),
            arrows: Vec::new(),
            activedisc: "none".to_string(),
            cleared: 0,
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

        if is_key_pressed(KeyCode::Q) {
            self.handle_musicdiscs(musicdiscs);
        }
    }

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
            self.attackimg.set_x(self.attackimg.get_x() - self.movement.x);
        } else if collide {
            for img in collides {
                if self.check_x_collision(img) {
                    self.set_position(old_pos.x, self.get_y()); //move player back to old x position but keep new y position for smoother movement when colliding with objects
                    break
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
            self.attackimg.set_y(self.attackimg.get_y() - self.movement.y);
        } else if collide {
            for img in collides {
                if self.check_y_collision(img) {
                    self.set_position(self.get_x(), old_pos.y);
                    break
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
        self.playerui.3[0].set_x(x);
        self.playerui.3[0].set_y(y);
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

    pub fn get_meleedmg(&self) -> f32 {
        self.mledmg
    }

    pub fn get_rngdmg(&self) -> f32 {
        self.rngdmg
    }

    //PLAYER STATS AND MOVEMENTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT

    pub fn dash_start(&mut self) {
        self.move_speed *= 5.0;
    }

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
    }

    pub fn dmgplayer(&mut self, dmg: f32) {
        let mut dmg = dmg - self.armor as f32;
        if dmg < 0.0 {
            dmg = 0.0;
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

    pub fn healplayer(&mut self, heal: f32) {
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

    pub fn handle_playerdamaging(&mut self, enemies: &Vec<Enemy>) {
        for enemy in enemies.iter() {
            if check_collision(self.view_player(), enemy.view_enemy(), 1) {
                self.dmgplayer(enemy.get_dmg());
            }
        }
    }

    pub fn get_player_activedisc(&self) -> String {
        self.activedisc.clone()
    }

    pub fn set_player_activedisc(&mut self, disc: String) {
        self.activedisc = disc;
    }

    pub fn add_cleared(&mut self) {
        self.cleared += 1;
    }

    pub fn get_cleared(&self) -> i32 {
        self.cleared
    }

    //PLAYER UIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
    pub async fn create_player_ui(x: f32, y: f32, preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>, tm: &TextureManager) -> (Vec<StillImage>, Vec<Label>, Vec<AnimatedImage>, Vec<StillImage>) {
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
        let mut img_musicoin = StillImage::new(
            "",
            80.0,  // width
            80.0,  // height
            0.0, // x position
            40.0,   // y position
            true,  // Enable stretching
            1.0,   // Normal zoom (100%)
        )
        .await;
        img_musicoin.set_preload(tm.get_preload("assets/item_files/musicoin.png").unwrap().clone());
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
        let mut lbl_disc1num = Label::new("", 472.0, 32.0, 30);
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
        let mut lbl_disc2num = Label::new("", 517.0, 32.0, 30);
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
        let mut lbl_disc3num = Label::new("", 562.0, 32.0, 30);
        lbl_disc3num.with_colors(WHITE, None);
        //ATTACK LABELSui();
        let player_x = x;
        let player_y = y;

        // let mut img_slash_t = StillImage::new(
        //     "",
        //     70.0, // width
        //     30.0, // height
        //     player_x - 15.0,
        //     player_y - 30.0,
        //     true,
        //     1.0,
        // )
        // .await;
    let mut img_slash_t = AnimatedImage::from_gif(
        "", 
        player_x -15.0, player_y - 30.0,           
        70.0, 30.0,          
        true                   
    ).await;
       // img_slash_t.set_preload(preloads[11].clone());

          if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_t.set_preloaded_gif(preloaded, false);
    }
        // let mut img_slash_tr = StillImage::new(
        //     "",
        //     60.0, // width
        //     50.0, // heightui();
        //     player_x + 40.0,
        //     player_y - 30.0,
        //     true,
        //     1.0,
        // )
        // .await;
        // img_slash_tr.set_preload(preloads[11].clone());

let mut img_slash_tr = AnimatedImage::from_gif(
        "", 
        player_x +40.0, player_y - 30.0,           
        70.0, 30.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_tr.set_preloaded_gif(preloaded, false);
    }
        img_slash_tr.set_angle(PI / 2.0);


        // let mut img_slash_r = StillImage::new(
        //     "",
        //     40.0, // width
        //     80.0, // height
        //     player_x + 45.0,
        //     player_y - 10.0,
        //     true,
        //     1.0,
        // )
        // .await;
        // img_slash_r.set_preload(preloads[11].clone());

        let mut img_slash_r = AnimatedImage::from_gif(
        "", 
        player_x +45.0, player_y - 10.0,           
        70.0, 30.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_r.set_preloaded_gif(preloaded, false);
    }
        img_slash_r.set_angle(PI);


        // let mut img_slash_br = StillImage::new(
        //     "",
        //     60.0, // width
        //     50.0, // height
        //     player_x + 40.0,
        //     player_y + 60.0,
        //     true,
        //     1.0,
        // )
        // .await;
        // img_slash_br.set_preload(preloads[11].clone());

         let mut img_slash_br = AnimatedImage::from_gif(
        "", 
        player_x +40.0, player_y + 60.0,           
        70.0, 30.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_br.set_preloaded_gif(preloaded, false);
    }
        // let mut img_slash_b = StillImage::new(
        //     "",
        //     70.0, // width
        //     30.0, // height
        //     player_x - 10.0,
        //     player_y + 65.0,
        //     true,
        //     1.0,
        // )
        // .await;
        // img_slash_b.set_preload(preloads[11].clone());

         let mut img_slash_b = AnimatedImage::from_gif(
        "", 
        player_x -10.0, player_y + 65.0,           
        70.0, 30.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_b.set_preloaded_gif(preloaded, false);
    }
        // let mut img_slash_bl = StillImage::new(
        //     "",
        //     60.0, // width
        //     50.0, // height
        //     player_x - 60.0,
        //     player_y + 50.0,
        //     true,
        //     1.0,
        // )
        // .await;
        // img_slash_bl.set_preload(preloads[11].clone());

         let mut img_slash_bl = AnimatedImage::from_gif(
        "", 
        player_x -60.0, player_y + 50.0,           
        70.0, 30.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_bl.set_preloaded_gif(preloaded, false);
    }
        img_slash_bl.set_angle(PI / 2.0);
        // let mut img_slash_l = StillImage::new(
        //     "",
        //     30.0, // width
        //     70.0, // height
        //     player_x - 35.0,
        //     player_y - 10.0,
        //     true,
        //     1.0,
        // )
        // .await;
        // img_slash_l.set_preload(preloads[11].clone());
         let mut img_slash_l = AnimatedImage::from_gif(
        "", 
        player_x -35.0, player_y - 10.0,           
        70.0, 30.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_l.set_preloaded_gif(preloaded, false);
    }
        // let mut img_slash_tl = StillImage::new(
        //     "",
        //     60.0, // width
        //     50.0, // height
        //     player_x - 60.0,
        //     player_y - 50.0,
        //     true,
        //     1.0,
        // )
        // .await;
        // img_slash_tl.set_preload(preloads[11].clone());

         let mut img_slash_tl = AnimatedImage::from_gif(
        "", 
        player_x -60.0, player_y - 50.0,           
        70.0, 30.0,          
        true                   
    ).await;
    if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/player_files/sword_slash.gif") {
        img_slash_tl.set_preloaded_gif(preloaded, false);
    }
        let mut player_hitbox = StillImage::new(
            "",
            40.0, // width
            60.0, // height
            player_x,
            player_y,
            true,
            1.0,
        )
        .await;
        player_hitbox.set_preload(preloads[11].clone());
        (
            vec![
                img_heart,
                img_arrow,
                img_disc1,
                img_disc2,
                img_disc3,
                img_musicoin,
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
            ],
            vec![
                player_hitbox,
            ]
        )
    }

    pub async fn handle_player_ui(&mut self, enemies: &mut Vec<Enemy>, musicdiscs: &mut Musicdisc) -> (bool, bool, usize) {
        //update vars
        let mletimepassed = get_time() - self.last_attack_time;
        let rngtimepassed = get_time() - self.last_rng_attack_time;
        let mut mlehit = false;
        let mut rnghit = false;
        let mut index = 0;
        //update health number
        self.playerui.1[2].set_text(format!("{}", self.health));
        for image in self.playerui.0.iter_mut() {
            image.draw();
        }
        for label in self.playerui.1.iter_mut() {
            label.draw();
        }
        self.inventory.2[2].draw();//draw gold label
        self.playerui.1[0].draw();
        self.playerui.1[1].draw(); //label must be redrawn very specifically so for loops cant be used  
        self.playerui.0[0].draw();
        self.playerui.1[2].draw();
        for i  in 0..self.playerui.0.len() {
            let img = &self.playerui.0[i];
            match img.get_filename() {
                "assets/fireball.png" => {
                    let times = musicdiscs.get_musicdisc_cooldowns();
                    self.playerui.1[i+2].set_text(format!("{:.0}", 30.0-times[0]));
                        if times[0] <= 0.0 {
                            self.playerui.1[i+2].set_text("".to_string());
                        }
                },
                _ => {}
            }
        }
            /* 
                if self.items[*item].get_itemtype() == "disc".to_string() {
                    let title = self.items[*item].get_itemtitle();
                    let times = self.musicdiscfunctions.get_musicdisc_cooldowns();
                    if title == "Back In Black" {
                        self.playerui.1[4].set_text(format!("{}", times.0));
                        println!("{} cooldown: {}", title, times.0);
                        if times.0 <= 0.0 {
                            self.playerui.1[4].set_text("".to_string());
                        }
                    }
                }
            }
            */
        if self.attack {
            if self.mlevalid == true {
                (index, mlehit) = self.create_melee_attack(enemies, index, mlehit);
                self.mlevalid = false; //prevents multiple melee hits from one attack input
            }
            if mletimepassed > 0.1 && mletimepassed < 1.0 {
                self.attackimg.draw();
            }
            if mletimepassed > 1.0 { //attack label only appears for 0.6 seconds after attack
                self.attack = false;
                self.mlevalid = true;
                self.attackimgfound = false;
                self.last_attack_time = get_time();
            }
        }
        if self.rangedattack {
            self.create_range_attack().await;
            if rngtimepassed > 0.5 && rngtimepassed < 3.0 {
                let cooldown = 3.0 - rngtimepassed + 1.0; //+1 to not show 0 when cooldown is ready
                self.playerui.1[3].set_text(format!("{:.0}", cooldown));
            }
            else if rngtimepassed >= 3.0 {
                self.playerui.1[3].set_text("".to_string());
            }
            if rngtimepassed > 3.0 { 
                self.rangedattack = false;
                self.rangedattackimgcreated = false;
            }
        }
        let mut rac = 0; //ranged attack counter so that only one enemy is damaged per arrow
        for i in 0..self.arrows.len() {
                let idx = i - rac;
                let y = self.arrows[idx].get_y();
                let x = self.arrows[idx].get_x();
                let movement = vec2(self.ranged_movespeeds[idx].x * get_frame_time(), self.ranged_movespeeds[idx].y * get_frame_time());
                if movement.x > 0.0 && movement.y > 0.0 { //if diagonal movement, normalize to prevent faster diagonal movement
                    self.movement.normalize(); //normalize diagonal movement to prevent faster movement when moving diagonally
                }
                self.arrows[idx].set_x(x + movement.x);
                self.arrows[idx].set_y(y + movement.y);
                self.arrows[idx].draw();
                if self.arrows[idx].get_y() < 0.0 || self.arrows[idx].get_y() > 800.0 || self.arrows[idx].get_x() < 0.0 || self.arrows[idx].get_x() > 1200.0 {
                    self.arrows.remove(idx);
                    self.ranged_movespeeds.remove(idx);
                    rac += 1;
                    continue; //skip collision check if arrow is removed for being out of bounds
                }
                let mut rec = 0; //remove enemy counter so that only one enemy is removed per arrow
                for j in 0..enemies.len() {
                    if check_collision(&self.arrows[idx], enemies[j].view_enemy(), 1) {
                        //ENEMY DAMAGE: mark hit and remove arrow
                        rnghit = true;
                        self.arrows.remove(idx);
                        self.ranged_movespeeds.remove(idx);
                        rec += 1;
                        break; //break to prevent multiple enemies being damaged by one arrow
                    }
                }
        }
        (mlehit, rnghit, index)
    }

    pub fn create_melee_attack(&mut self, enemies: &mut Vec<Enemy>, mut index: usize, mut mlehit: bool) -> (usize, bool) {
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

                self.attackimg.reset();                
                self.attackimgfound = true;
                for i in 0..enemies.len() {
                    if check_collision(&self.attackimg, enemies[i].view_enemy(), 1) {
                        mlehit = true;
                        index = i;
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
            self.last_rng_attack_time = get_time();
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
            //self.inventory.2[2].set_text(format!("Musicoins:{}", self.musicoins));
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
                        self.playerui.0[2].set_preload(item.get_itemimgpath());
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
        self.mledmg = 3.0;
        self.rngdmg = 2.0;
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


    pub fn handle_musicdiscs(&mut self, musicdiscs: &mut Musicdisc) {
        if self.equipped_items.len() > 0 {
            for i in 0..self.equipped_items.len() {
                let item = &self.items[self.equipped_items[i]];
                if item.get_itemtype() == "disc".to_string() {
                    match item.get_itemtitle().as_str() {
                        "Back In Black" => {
                            let validity = musicdiscs.get_musicdisc_validity();
                            if validity[0] == true {
                            self.activedisc = "Back In Black".to_string();
                            musicdiscs.get_musicdisc_times();
                            }
                        }
                        "Thick Of It" => {
                            let validity = musicdiscs.get_musicdisc_validity();
                            if validity[1] == true {
                            self.activedisc = "Thick Of It".to_string();
                            musicdiscs.get_musicdisc_times();
                            }
                        }
                        _ => {

                        }
                    }
                }
            }           
        }
    }
}
