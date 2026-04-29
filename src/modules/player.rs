use crate::modules;
use crate::modules::collision::check_collision;
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use macroquad::prelude::*;

//IMPLEMENTATION
//in every screen write
/*
with other crates
use crate::modules::player::Player;

funcs
player.handle_keypresses().await;
player.move_player();
player.handle_inventory();
player.draw();
*/
pub struct Player {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
    health: f32,
    mledmg: i32,
    rngdmg: i32,
    cooldownmult: f32,
    musicoins: i32,
    items: Vec<Vec<String>>,
    inventory: (Vec<ListView>, Vec<StillImage>, Vec<Label>, Vec<TextButton>),
    inventoryopen: bool,
}

impl Player {
    pub async fn new(vec<macroquad::texture::Texture2D, Option<Vec<u8>>, String>, x: f32, y: f32) -> Self {
        let mut view = StillImage::new(
            "", 
            40.0, // width
            80.0, // height
            x,    // x position
            y,    // y position
            true, // Enable stretching
            1.0,  // Normal zoom (100%)
        )
        .await;
        view.set_preload(preloadlist[0]);

        Player {
            view,
            move_speed: 400.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
            health: 100.0,
            mledmg: 3,
            rngdmg: 2,
            cooldownmult: 1.0,
            musicoins: 0,
            items: Vec::new(),
            inventory: Player::create_inventory().await,
            inventoryopen: false,
        }
    }
    //movement functions
    pub async fn handle_keypresses(&mut self) {
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
            move_dir = move_dir.normalize();
        }

        let movement = move_dir * self.move_speed * get_frame_time();
        self.movement = movement;
        self.handle_image().await;
        if is_key_pressed(KeyCode::Tab) {
            self.inventoryopen = !self.inventoryopen;
        }
    }

    pub async fn handle_image(&mut self) {
        if is_key_down(KeyCode::W) && is_key_down(KeyCode::D) && self.view.get_filename() != "assets/player_files/player_tr.png" {
            self.view.set_image("assets/player_files/player_tr.png").await;
        } else if is_key_down(KeyCode::W) && is_key_down(KeyCode::A) && self.view.get_filename() != "assets/player_files/player_tl.png" {
            self.view.set_image("assets/player_files/player_tl.png").await;
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::D) && self.view.get_filename() != "assets/player_files/player_br.png" {
            self.view.set_image("assets/player_files/player_br.png").await;
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::A) && self.view.get_filename() != "assets/player_files/player_bl.png" {
            self.view.set_image("assets/player_files/player_bl.png").await;
        } else if is_key_down(KeyCode::D) && self.view.get_filename() != "assets/player_files/player_r.png" {
            self.view.set_image("assets/player_files/player_r.png").await;
        } else if is_key_down(KeyCode::A) && self.view.get_filename() != "assets/player_files/player_l.png" {
            self.view.set_image("assets/player_files/player_l.png").await;
        } else if is_key_down(KeyCode::S) && self.view.get_filename() != "assets/player_files/player_b.png" {
            self.view.set_image("assets/player_files/player_b.png").await;
        } else if is_key_down(KeyCode::W) && self.view.get_filename() != "assets/player_files/player_t.png" {
            self.view.set_image("assets/player_files/player_t.png").await;
        }
    }

    pub fn move_x(&mut self) {
        self.view.set_x(self.view.get_x() + self.movement.x);
    }

    pub fn move_y(&mut self) {
        self.view.set_y(self.view.get_y() + self.movement.y);
    }

    pub fn move_player(&mut self, map: &Map, old_pos: Vec2) {
        self.move_x();
        if map.map_collision(&self.view_player()).0 {
            &self.set_x(old_pos.x);
        }
        self.move_y();
        if map.map_collision(&self.view_player()).0 {
            &self.set_x(old_pos.x);
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

    pub fn dash_start(&mut self) {
        self.move_speed *= 5.0;
    }

    pub fn dash_end(&mut self) {
        self.move_speed /= 5.0;
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    pub fn get_stats(&self) -> (f32, i32, i32, f32) {
        (self.health, self.mledmg, self.rngdmg, self.cooldownmult)
    }

    pub fn get_items(&self) -> &Vec<Vec<String>> {
        &self.items
    }

    pub fn getcoins(&self) -> i32 {
        self.musicoins
    }

    pub fn addcoins(&mut self, coins: i32) {
        self.musicoins += coins;
    }

    //INVENTORYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY
    async fn create_inventory() -> (Vec<ListView>, Vec<StillImage>, Vec<Label>, Vec<TextButton>) {
        let list = vec!["Helmet", "Body Armor", "Boots"];
        let mut lst_inventory = ListView::new(&list, 340.0, 50.0, 60);
        lst_inventory.with_colors(BLACK, Some(BROWN), Some(LIGHTGRAY));
        lst_inventory.set_width(340.0);
        lst_inventory.with_max_visible_items(11);
        let mut item_img = StillImage::new("", 285.0, 275.0, 25.0, 25.0, true, 1.0).await;
        //item_img.set_preload(tm.get_preload("assets/SpaceInvadersLogo.png").unwrap());
        let mut helmet_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 825.0, 50.0, true, 1.0).await;
        let mut bodyarmor_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 825.0, 280.0, true, 1.0).await;
        let mut boots_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 825.0, 550.0, true, 1.0).await;
        let mut sword_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 750.0, 400.0, true, 1.0).await;
        let mut bow_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 900.0, 400.0, true, 1.0).await;
        let mut shadow_img = StillImage::new("assets/player_files/player_shadow.png", 250.0, 650.0, 750.0, 50.0, true, 1.0).await;
        let mut disc1_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 700.0, 660.0, true, 1.0).await;
        let mut disc2_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 810.0, 660.0, true, 1.0).await;
        let mut disc3_img = StillImage::new("assets/fireball.png", 100.0, 100.0, 920.0, 660.0, true, 1.0).await;
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
        (
            vec![lst_inventory],
            vec![
                shadow_img,
                item_img,
                helmet_img,
                bodyarmor_img,
                boots_img,
                sword_img,
                bow_img,
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
            for list_view in self.inventory.0.iter_mut() {
                list_view.draw();
            }
            for image in self.inventory.1.iter_mut() {
                image.draw();
            }
            for label in self.inventory.2.iter_mut() {
                label.draw();
            }
            if self.inventory.3[0].click() {
                // Handle equip button click
            }
            if self.inventory.3[1].click() {
                // Handle unequip button click
            }
            if self.inventory.3[2].click() {
                // Handle trash button click
            }
        }
    }
}
