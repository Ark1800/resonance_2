/*pub mod enemy;

use crate::modules::enemy::Enemy;

ENEMY SETUPS
let mut summoner = Enemy::new(
"summoner.png",
50.0, //hieght
50.0, //width
70.0, //x
80.0, //y
true, //stretching
1.0, //zoom level
20, //health
10, //damage
"").await;

let mut archer = Enemy::new(
"archer.png",
50.0, //hieght
50.0, //width
70.0, //x
80.0, //y
true, //stretching
1.0, //zoom level
15, //health
5, //damage
"arrow.png"//projectile
).await;

let mut mage = Enemy::new(
"mage.png",
50.0, //hieght
50.0, //width
70.0, //x
80.0, //y
true, //stretching
1.0, //zoom level
20, //health
10, //damage
"fireball.png" //projectile
).await;

let mut slime = Enemy::new(
"slime.png",
25.0, //hieght
25.0, //width
70.0, //x
80.0, //y
true, //stretching
1.0, //zoom level
10, //health
2, //damage
"").await;

let mut large_slime = Enemy::new(
"large_slime.png",
75.0, //hieght
75.0, //width
70.0, //x
80.0, //y
true, //stretching
1.0, //zoom level
20, //health
8, //damage
"").await;

MULTIPLE ENEMIES
 let mut archer_list: Vec<Enemy> = vec![];
    for _i in 0..3 {
        let mut archer = Enemy::new(
            "archer.png",
            50.0,
            50.0,
            archerx,
            200.0,
            true,
            1.0,
            10,
            5,
            "arrow.png",
        )
        .await;
         archerx += 100.0; // Adjust the x position for the next archer
        archer_list.push(archer);
    }

ENEMY ACTIONS
mage.mage_action(tm, player).await;
summoner.summoner_action(tm, player, &mut slime_list).await;


MULTIPLE ENEMY ACTIONS
for archer in archer_list.iter_mut() {
    archer.archer_action(tm, player).await;
    archer.draw();
    archer.draw_bullet(player);
}

*/
use crate::modules::player::Player;
use crate::modules::preload_image::TextureManager;
use crate::modules::projectile::Projectile;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use miniquad::date;

#[derive(Clone)]
pub struct Enemy {
    view: StillImage,
    projectile_image: StillImage,
    move_speed: f32,
    movement: Vec2,
    health: i32,
    dmg: i32,
    projectiles: Vec<Projectile>,
    cooldown: f64,
    cooldown2: f64,
}

impl Enemy {
    pub async fn new(
        asset_path: &str,
        width: f32,
        height: f32,
        x: f32,
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32,
        health: i32,
        dmg: i32,
        projectile_path: &str,
    ) -> Enemy {
        Enemy {
            view: StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await,

            move_speed: 200.0, // Default speed
            movement: Vec2::ZERO,
            health,
            dmg,
            projectile_image: StillImage::new(projectile_path, width, height, 0.0, 0.0, false, 1.0).await,
            projectiles: Vec::new(),
            cooldown: 0.0,
            cooldown2: 0.0,
        }
    }
    #[allow(unused)]
    pub async fn archer_img_change(&mut self, playerx: f32, archerx: f32, action: &str, tm: &TextureManager) -> &Enemy {
        let mut way = ""; // Determine direction based on player and archer positions

        if archerx < playerx {
            way = "R";
        } else {
            way = "L";
        }
        // Update preload based on action
        match action {
            "move" => {
                self.set_preload(tm.get_preload(format!("assets/archer_files/archer_run{}.png", way).as_str()).unwrap());
            }
            "ready" => {
                self.set_preload(tm.get_preload(format!("assets/archer_files/archer_ready{}.png", way).as_str()).unwrap());
            }
            "attack" => {
                self.set_preload(tm.get_preload(format!("assets/archer_files/archer_shoot{}.png", way).as_str()).unwrap());
            }
            _ => {}
        }
        self
    }
    #[allow(unused)]
    pub async fn mage_img_change(&mut self, playerx: f32, magex: f32, action: &str, tm: &TextureManager) -> &Enemy {
        let mut way = ""; // Determine direction based on player and archer positions
        if magex < playerx {
            way = "R";
        } else {
            way = "L";
        }
        // Update preload based on action
        match action {
            "ready" => {
                self.set_preload(tm.get_preload(format!("assets/mage_files/mage_stand{}.png", way).as_str()).unwrap());
            }
            "attack" => {
                self.set_preload(tm.get_preload(format!("assets/mage_files/mage_shoot{}.png", way).as_str()).unwrap());
            }
            _ => {}
        }
        self
    }
    #[allow(unused)]
    pub async fn summoner_img_change(&mut self, playerx: f32, summonerx: f32, action: &str, tm: &TextureManager) -> &Enemy {
        let mut way = ""; // Determine direction based on player and archer positions
        if summonerx < playerx {
            way = "R";
        } else {
            way = "L";
        }
        // Update preload based on action
        match action {
            "ready" => {
                self.set_preload(
                    tm.get_preload(format!("assets/summoner_files/summoner_stand{}.png", way).as_str())
                        .unwrap(),
                );
            }
            "attack" => {
                self.set_preload(
                    tm.get_preload(format!("assets/summoner_files/summoner_summon{}.png", way).as_str())
                        .unwrap(),
                );
            }
            "move" => {
                self.set_preload(tm.get_preload(format!("assets/summoner_files/portal{}.png", way).as_str()).unwrap());
            }
            _ => {}
        }
        self
    }
    // Setter for projectile preload
    #[allow(unused)]
    pub fn set_projectile_preload(&mut self, preloaded: (Texture2D, Option<Vec<u8>>, String)) {
        let (texture, mask, filename) = preloaded;
        self.projectile_image.texture = texture;
        self.projectile_image.transparency_mask = mask;
        self.projectile_image.filename = filename;
    }
    // Setter for enemy preload
    #[allow(unused)]
    pub fn set_preload(&mut self, preloaded: (Texture2D, Option<Vec<u8>>, String)) {
        let (texture, mask, filename) = preloaded;
        self.view.texture = texture;
        self.view.transparency_mask = mask;
        self.view.filename = filename;
    }
    #[allow(unused)]
    pub fn moveing(&mut self, player_x: f32, player_y: f32) {
        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        self.movement = move_dir * self.move_speed * get_frame_time();

        if self.view.get_x() < player_x {
            move_dir.x += 1.0; // Move right
            self.set_x(self.get_x() + 1.0);
        } else if self.view.get_x() > player_x {
            move_dir.x -= 1.0; // Move left
            self.set_x(self.get_x() - 1.0);
        }

        if self.view.get_y() < player_y {
            move_dir.y += 1.0; // Move down
            self.set_y(self.get_y() + 1.0);
        } else if self.view.get_y() > player_y {
            move_dir.y -= 1.0; // Move up
            self.set_y(self.get_y() - 1.0);
        }
        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // Apply movement based on frame time
    }
    //change dmg
    #[allow(unused)]
    pub fn set_dmg(&mut self, dmg: i32) -> &mut Self {
        self.dmg = dmg;
        self
    }
    //changes image
    #[allow(unused)]
    pub async fn set_image(&mut self, image_path: &str) {
        self.view.set_texture(image_path).await;
    }
    //changes health
    #[allow(unused)]
    pub fn set_health(&mut self, health: i32) -> &mut Self {
        self.health = health;
        self
    }

    #[allow(unused)]
    pub fn draw(&self) {
        // Only draw if the label is visible
        self.view.draw();
    }

    #[allow(unused)]
    //change speed
    pub fn set_speed(&mut self, move_speed: f32) -> &mut Self {
        self.move_speed = move_speed;
        self
    }

    // Getter for position as Vec2
    #[allow(unused)]
    pub fn get_position(&self) -> Vec2 {
        Vec2::new(self.view.get_x(), self.view.get_y())
    }

    // Getter for visibility
    #[allow(unused)]

    pub fn view_player(&self) -> &StillImage {
        &self.view
    }
    // Setter for position
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        self.view.set_x(x);
        self.view.set_y(y);
        self
    }
    pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }

    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }
    pub fn get_pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }
    // Get and set y position
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }

    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }
    #[allow(unused)]
    pub fn pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

    // Calculate direction towards player
    #[allow(unused)]
    pub fn set_direction(&self, player_pos: Vec2) -> Vec2 {
        let direction = (player_pos - self.get_pos()).normalize();
        let movement = direction * self.move_speed * get_frame_time();

        movement
    }
    // Enemy shooting method
    pub async fn shoot(&mut self, player: &mut Player, width: f32, height: f32) {
        let mut projectile = Projectile::new(self.projectile_image.clone(), width, height, self.get_x(), self.get_y(), true, 1.0).await; // Create a projectile at the enemy's position
        // Calculate the angle towards the player and set it for the projectile
        let angle = projectile.set_rotation(player.get_x(), player.get_y(), self.get_x(), self.get_y());
        projectile.set_angle(angle);
        projectile.set_direction(player.get_oldpos());
        self.projectiles.push(projectile);
    }
    // Draw all owned enemy bullets
    pub fn draw_bullet(&mut self, player: &mut Player) {
        for projectile in &mut self.projectiles {
            projectile.move_projectiles(player.get_oldpos());
            projectile.draw();
        }
    }
    // Summoning method for the summoner enemy
    pub async fn summon(&mut self, tm: &TextureManager, slime_list: &mut Vec<Enemy>) -> Vec<Enemy> {
        let mut summonx = self.get_x();
        let mut summony = self.get_y() + 50.0;
        // Summon 3 slimes around the summoner
        for i in 0..3 {
            if i == 1 {
                summonx = self.get_x() + 50.0;
                summony = self.get_y() - 50.0;
            } else if i == 2 {
                summonx = self.get_x() - 50.0;
                summony = self.get_y() - 50.0;
            }
            let mut summoned_enemy = Enemy::new("", 25.0, 25.0, summonx, summony, true, 1.0, 100, 10, "").await;
            summoned_enemy.set_preload(tm.get_preload("assets/slime.png").unwrap());
            slime_list.push(summoned_enemy.clone());
        }
        slime_list.to_vec()

        // Add the summoned enemy to your game world (e.g., a list of enemies)
    }
    // Teleport method for the summoner
    pub fn teleport(&mut self) {
        rand::srand(date::now() as u64);
        let rand_x = rand::gen_range(70.0, 900.0);
        let rand_y = rand::gen_range(80.0, 630.0);
        self.set_x(rand_x);
        self.set_y(rand_y);
    }
    // Archer action method that handles movement, image changes, and shooting based on player proximity and cooldowns
    pub async fn archer_action(&mut self, tm: &TextureManager, player: &mut Player) {
        if ((self.get_x() - player.get_x()).abs() < 225.0) && ((self.get_y() - player.get_y()).abs() < 225.0) {
            if get_time() - self.cooldown > 0.5 {
                self.archer_img_change(player.get_x(), self.get_x(), "ready", &tm).await;
            }

            if get_time() - self.cooldown > 1.0 {
                self.archer_img_change(player.get_x(), self.get_x(), "attack", &tm).await;
                self.cooldown = get_time();
                self.shoot(player, 40.0, 40.0).await;
            }
        } else {
            self.moveing(player.get_x(), player.get_y());
            self.archer_img_change(player.get_x(), self.get_x(), "move", &tm).await;
        }
    }
    // Mage action method that handles movement, image changes, and shooting based on player proximity and cooldowns
    pub async fn mage_action(&mut self, tm: &TextureManager, player: &mut Player) {
        if ((self.get_x() - player.get_x()).abs() < 150.0) && ((self.get_y() - player.get_y()).abs() < 150.0) {
            if get_time() - self.cooldown > 0.5 {
                self.mage_img_change(player.get_x(), self.get_x(), "ready", &tm).await;
            }

            if get_time() - self.cooldown > 2.0 {
                self.cooldown = get_time();
                self.shoot(player, 80.0, 80.0).await;
                self.mage_img_change(player.get_x(), self.get_x(), "attack", &tm).await;
            }
        } else {
            self.moveing(player.get_x(), player.get_y());

            self.mage_img_change(player.get_x(), self.get_x(), "ready", &tm).await;
        }
    }
    // Summoner action method that handles movement, image changes, and summoning slimes based on player proximity and cooldowns
    pub async fn summoner_action(&mut self, tm: &TextureManager, player: &mut Player, slime_list: &mut Vec<Enemy>) -> Vec<Enemy> {
        if get_time() - self.cooldown2 > 10.1 {
            self.summoner_img_change(player.get_x(), self.get_x(), "move", &tm).await;
            if get_time() - self.cooldown2 > 10.6 {
                self.teleport();
                self.cooldown2 = get_time();
            }
        } else if get_time() - self.cooldown > 1.0 {
            self.summoner_img_change(player.get_x(), self.get_x(), "ready", &tm).await;
        }

        if get_time() - self.cooldown > 5.0 {
            self.cooldown = get_time();
            self.summon(&tm, slime_list).await;
            self.summoner_img_change(player.get_x(), self.get_x(), "attack", &tm).await;
        }

        slime_list.to_vec()
    }
    // Slime splitting method that creates two smaller slimes upon the death of a large slime
    #[allow(unused)]
    pub async fn split(&mut self, tm: &TextureManager, slime_list: &mut Vec<Enemy>) -> Vec<Enemy> {
        let mut summonx = self.get_x() - 10.0;

        for i in 0..2 {
            if i == 1 {
                summonx = self.get_x() + 10.0;
            }
            let mut summoned_enemy = Enemy::new("", 25.0, 25.0, summonx, self.get_y(), true, 1.0, 100, 10, "").await;
            summoned_enemy.set_preload(tm.get_preload("assets/slime.png").unwrap());
            slime_list.push(summoned_enemy.clone());
        }
        slime_list.to_vec()
    }
    // Slime action method that handles movement and splitting into smaller slimes upon death
    #[allow(unused)]
    pub fn large_slime_action(&mut self, tm: &TextureManager, player: &mut Player, slime_list: &mut Vec<Enemy>) -> Vec<Enemy> {
        self.moveing(player.get_x(), player.get_y());
        if self.health <= 0 {
            self.split(tm, slime_list);
        }
        slime_list.to_vec()
    }
}
//     pub fn move_check_collision_y(&mut self, img_other: &StillImage) -> bool {
//         let mut answer = false;
//         if self.movement.y != 0.0 {
//             self.set_y(self.get_y() + self.movement.y);
//             if check_collision(self.view_player(), img_other, 1) {
//                 answer = true;
//             }
//         }

//         answer
//     }
//     pub fn move_check_collision_x(&mut self, img_other: &StillImage) -> bool {
//         let mut answer = false;
//         if self.movement.x != 0.0 {
//             self.set_x(self.get_x() + self.movement.x);
//             if check_collision(self.view_player(), img_other, 1) {
//                 answer = true;
//             }
//         }

//         answer
//     }
// }
