use crate::modules;
use crate::modules::collision::check_collision;
use crate::modules::item::Item;
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::map::Map;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;
use macroquad::texture::Texture2D;
use std::f32::consts::PI;

//TO DOOOOOO
//1. player attacks appearing
//2. player healthbar and health decreasing
//3. player ranged attacks
//5. player damage
//6. player not switching image 60 times per second

//IMPLEMENTATION
//in every screen write
/*
with other crates
use crate::modules::player::Player;

funcs
player.handle_keypresses().await;
player.move_player();
player.handle_inventory();
player.handle_player_ui();
player.draw();

ASIDES...
Angles:
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
    view: StillImage,                                                                        //stillimage of player
    preloads: Vec<(Texture2D, Option<Vec<u8>>, String)>, //vec of preloads for use throughout player (especially for UI and image changing)
    move_speed: f32,                                     //movement speed in pixels per second
    movement: Vec2,                                      //movement vector for current frame
    health: i32,                                         //player health
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
    playerui: (Vec<StillImage>, Vec<Label>),             //2d list for player UI elements (images, labels)
    inventoryopen: bool,                                 //is inventory open
    armor: i32,                                          //armor value for damage reduction
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

        Player {
            view,
            move_speed: 400.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
            health: 100,
            mledmg: 3,
            rngdmg: 2,
            movespeedmult: 1.0,
            cooldownmult: 1.0,
            musicoins: 0,
            items: Vec::new(),
            item_titles: Vec::new(),
            equipped_items: Vec::new(),
            itemstats: (Vec::new(), Vec::new(), Vec::new(), Vec::new()),
            preloads: preloadlist.clone(),
            inventory: Player::create_inventory(&preloadlist).await,
            inventoryopen: false,
            armor: 0,
            playerui: Player::create_player_ui(x, y, &preloadlist).await,
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
       
        
    }

    pub fn handle_image(&mut self) {
        //change image based on direction of movement (8 directions)
        if is_key_down(KeyCode::W) && is_key_down(KeyCode::D) {
            self.view.set_preload(self.preloads[7].clone());
        } else if is_key_down(KeyCode::W) && is_key_down(KeyCode::A) {
            self.view.set_preload(self.preloads[6].clone());
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::D) {
            self.view.set_preload(self.preloads[9].clone());
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::A) {
            self.view.set_preload(self.preloads[8].clone());
        } else if is_key_down(KeyCode::D) {
            self.view.set_preload(self.preloads[5].clone());
        } else if is_key_down(KeyCode::A) {
            self.view.set_preload(self.preloads[4].clone());
        } else if is_key_down(KeyCode::S) {
            self.view.set_preload(self.preloads[0].clone());
        } else if is_key_down(KeyCode::W) {
            self.view.set_preload(self.preloads[3].clone());
        }
    }

    pub fn move_x(&mut self) {
        self.view.set_x(self.view.get_x() + self.movement.x);
    }

    pub fn move_y(&mut self) {
        self.view.set_y(self.view.get_y() + self.movement.y);
    }

    pub fn move_player(&mut self, map: &Map, old_pos: Vec2, collides: &Vec<StillImage>) {
        self.move_x();
        let mut collide = false;
        if !collides.is_empty() {
            collide = true;
        }
        if map.map_collision(&self.view_player()).0 {
            //collision with map
            self.set_x(old_pos.x);
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
    }

    pub fn draw(&self) {
        self.view.draw();
    }

    pub fn view_player(&self) -> &StillImage {
        &self.view
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

    //PLAYER UIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII
    pub async fn create_player_ui(x: f32, y: f32, preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>) -> (Vec<StillImage>, Vec<Label>) {
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
        img_arrow.set_preload(preloads[1].clone());
        let mut lbl_arrownum = Label::new("99", 427.0, 32.0, 30);
        lbl_arrownum.with_colors(WHITE, None);
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
        //ATTACK LABELS
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
            50.0, // height
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
        let mut img_slash_dr = StillImage::new(
            "",
            60.0, // width
            50.0, // height
            player_x + 40.0,
            player_y + 60.0,
            true,
            1.0,
        )
        .await;
        img_slash_dr.set_preload(preloads[11].clone());
        let mut img_slash_d = StillImage::new(
            "",
            70.0, // width
            30.0, // height
            player_x - 10.0,
            player_y + 65.0,
            true,
            1.0,
        )
        .await;
        img_slash_d.set_preload(preloads[11].clone());
        let mut img_slash_dl = StillImage::new(
            "",
            60.0, // width
            50.0, // height
            player_x - 60.0,
            player_y + 50.0,
            true,
            1.0,
        )
        .await;
        img_slash_dl.set_preload(preloads[11].clone());
        img_slash_dl.set_angle(PI / 2.0);
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
                img_slash_t,
                img_slash_tr,
                img_slash_r,
                img_slash_dr,
                img_slash_d,
                img_slash_dl,
                img_slash_l,
                img_slash_tl,
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
        )
    }

    pub fn handle_player_ui(&mut self) {
        //update health number
        self.playerui.1[2].set_text(format!("{}", self.health));

        for image in self.playerui.0.iter_mut() {
            image.draw();
        }
        for label in self.playerui.1.iter_mut() {
            label.draw();
        }
    }

    //INVENTORYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
    async fn create_inventory(preloads: &Vec<(Texture2D, Option<Vec<u8>>, String)>) -> (Vec<ListView>, Vec<StillImage>, Vec<Label>, Vec<TextButton>) {
        //creating all inventory UI elements
        let list: Vec<String> = Vec::new();
        let mut lst_inventory = ListView::new(&list, 340.0, 50.0, 60);
        lst_inventory.with_colors(BLACK, Some(BROWN), Some(LIGHTGRAY));
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
            vec![lbl_title, lbl_description],
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
        println!("equipped items: {:?}", self.equipped_items);
        for (equipped_pos, index) in self.equipped_items.iter().enumerate() {
            println!("{}", title);
            println!("{}", self.items[*index].get_itemtitle());
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
                println!("{}", self.equipped_items.len());
                if self.equipped_items.len() == 1 {
                    self.equipped_items.clear();
                    println!("No more equipped items.");
                } else {
                    self.equipped_items.remove(equipped_pos);
                    println!("Unequipped item: {}", title);
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
        self.health = 100;
        self.armor = 0;

        // Apply item stat changes
        for itemindex in &self.equipped_items {
            self.mledmg += self.items[*itemindex].get_itemmledmg();
            self.rngdmg += self.items[*itemindex].get_itemrngdmg();
            self.movespeedmult += self.items[*itemindex].get_itemmovespeedmult();
            self.cooldownmult += self.items[*itemindex].get_itemcooldownmult();
            self.health += self.items[*itemindex].get_itemhpchng();
            self.armor += self.items[*itemindex].get_itemarmor();
        }
    }
}
