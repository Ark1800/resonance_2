use core::panic;

use crate::VIRTUAL_HEIGHT;
use crate::VIRTUAL_WIDTH;
/*pub mod enemy;

//enemy knockback on hit

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
"",
"summoner"//enemy type
).await;

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
"archer"//enemy type
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
"mage"//enemy type
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
"",
"slime"//enemy type
).await;

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
"",
"large_slime"//enemy type
).await;

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
    archer.set_preload(tm.get_preload("assets/archer_files/archer_standR.png").unwrap());//must preload enemy img
        archer.set_projectile_preload(tm.get_preload("assets/arrow.png").unwrap()); //onl;y required for anyone with projectiles
         archerx += 100.0; // Adjust the x position for the next archer
        archer_list.push(archer);
    }

ENEMY ACTIONS
mage.mage_action(tm, player).await;
summoner.summoner_action(tm, player, &mut slime_list).await;


MULTIPLE ENEMY ACTIONS
for archer in 0..archer_list.len() {
                archer_list[archer].archer_action(tm, player).await;
                archer_list[archer].draw();
                archer_list[archer].draw_bullet(player);
                if archer_list[archer].get_health() <= 0 {
                    archer_list.remove(archer);
                    break;
                }
            }

for mage in 0..mage_list.len() {
                mage_list[mage].mage_action(tm, player).await;
                mage_list[mage].draw();
                mage_list[mage].draw_bullet(player);
                if mage_list[mage].get_health() <= 0 {
                    mage_list.remove(mage);
                    break;
                }
            }

for summoner in 0..summoner_list.len() {
                summoner_list[summoner].summoner_action(tm, player).await;
                summoner_list[summoner].draw();
                summoner_list[summoner].draw_bullet(player);
                if summoner_list[summoner].get_health() <= 0 {
                    summoner_list.remove(summoner);
                    break;
                }
            }

for slime in 0..slime_list.len() {
                slime_list[slime].slime_action(tm, player).await;
                slime_list[slime].draw();
                slime_list[slime].draw_bullet(player);
                if slime_list[slime].get_health() <= 0 {
                    slime_list.remove(slime);
                    break;
                }
            }

for large_slime in 0..large_slime_list.len() {
                large_slime_list[large_slime].large_slime_action(tm, player).await;
                large_slime_list[large_slime].draw();
                large_slime_list[large_slime].draw_bullet(player);
                if large_slime_list[large_slime].get_health() <= 0 {
                    large_slime_list.remove(large_slime);
                    break;
                }
            }

ENEMY PROJECTILE REMOVAL
for archer in 0..archer_list.len() {
    let arrow_list = archer_list[archer].get_projectiles();
                for arrow in 0..arrow_list.len() {
                     let collision = check_collision(arrow_list[arrow].view_player(), player.view_player(), 1); // 1 = pixel skip (for performance)
                    if collision {
                        player.dmgplayer(archer_list[archer].get_dmg());
                        archer_list[archer].remove_projectile(arrow);

                        break;
                }
            }

        }


for mage in 0..mage_list.len() {
    let fireball_list = mage_list[mage].get_projectiles();
                for fireball in 0..fireball_list.len() {
                     let collision = check_collision(fireball_list[fireball].view_player(), player.view_player(), 1); // 1 = pixel skip (for performance)
                    if collision {
                        player.dmgplayer(mage_list[mage].get_dmg());
                        mage_list[mage].remove_projectile(fireball);

                        break;
                }
            }

        }


PNG EXAMPLE
let mut slime = Enemy::new(
    "assets/slime.png",
    25.0,
    25.0,
    70.0,
    80.0,
    true,
    1.0,
    10.0,
    2.0,
    "",
    "slime",
)
.await;
GIF EXAMPLE
let mut flying_blob = Enemy::new(
    "assets/flying_blob.gif",
    64.0,
    64.0,
    100.0,
    120.0,
    true,
    1.0,
    15.0,
    3.0,
    "",
    "flying_blob",
)
.await;

GIF PRELOAD EXAMPLE
flying_blob.set_preload_gif(tm.get_preloaded_animated_gif("assets/flying_blob.gif").unwrap(), true);

*/
use crate::modules::animated_image::AnimatedImage;
use crate::modules::collision::check_collision;
use crate::modules::label::Label;
use crate::modules::map;
use crate::modules::musicdisc::Musicdisc;
use crate::modules::player::Player;
use crate::modules::preload_image::{PreloadedAnimatedGif, TextureManager};
use crate::modules::progressbar::ProgressBar;
use crate::modules::projectile::Projectile;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use miniquad::date;

#[derive(Clone)]
// Internal representation for an enemy's visual
// This allows the enemy to be backed by either a plain still image
// or by an animated image (like a GIF).
enum EnemyView {
    Still(StillImage),
    Animated(AnimatedImage),
}

#[derive(Clone)]
pub struct Enemy {
    view: EnemyView,
    projectile_image: StillImage,
    move_speed: f32,
    movement: Vec2,
    health: f32,
    max_health: f32,
    dmg: f32,
    projectiles: Vec<Projectile>,
    cooldown: f64,
    cooldown2: f64,
    enemy_type: String,
}

impl Enemy {
    // Detects whether the provided path points to a GIF file.
    fn is_gif(path: &str) -> bool {
        path.to_lowercase().ends_with(".gif")
    }

    // Create the right kind of internal view depending on asset_path.
    // - For GIF files, this creates an AnimatedImage directly from the path.
    // - For all other image paths, it creates a normal StillImage.
    async fn make_view(asset_path: &str, width: f32, height: f32, x: f32, y: f32, stretch_enabled: bool, zoom_level: f32) -> EnemyView {
        if !asset_path.is_empty() && Enemy::is_gif(asset_path) {
            EnemyView::Animated(AnimatedImage::from_gif(asset_path, x, y, width, height, true).await)
        } else {
            EnemyView::Still(StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await)
        }
    }

    pub async fn new(
        asset_path: &str,
        width: f32,
        height: f32,
        x: f32,
        y: f32,
        stretch_enabled: bool,
        zoom_level: f32,
        health: f32,
        dmg: f32,
        projectile_path: &str,
        enemy_type: &str,
    ) -> Enemy {
        Enemy {
            view: Enemy::make_view(asset_path, width, height, x, y, stretch_enabled, zoom_level).await,
            move_speed: 150.0, // Default speed
            movement: Vec2::ZERO,
            health,
            max_health: health,
            dmg,
            enemy_type: enemy_type.to_string(),
            projectile_image: StillImage::new(projectile_path, width, height, 0.0, 0.0, false, 1.0).await,
            projectiles: Vec::new(),
            cooldown: 0.0,
            cooldown2: 0.0,
        }
    }

    pub fn get_enemy_type(&self) -> &str {
        &self.enemy_type
    }

    pub fn set_move_speed(&mut self, speed: f32) {
        self.move_speed = speed;
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
    // Setter for enemy preload.
    // This only works for still-image enemies.
    // Animated enemies do not use this path because they are backed by AnimatedImage.
    #[allow(unused)]
    pub fn set_preload(&mut self, preloaded: (Texture2D, Option<Vec<u8>>, String)) {
        if let EnemyView::Still(still) = &mut self.view {
            still.set_preload(preloaded);
        }
    }

    // Setter for a preloaded animated GIF.
    // This is the GIF equivalent of set_preload for still images.
    #[allow(unused)]
    pub fn set_preload_gif(&mut self, preloaded: PreloadedAnimatedGif, loop_animation: bool) {
        let x = self.get_x();
        let y = self.get_y();
        let width = self.get_width();
        let height = self.get_height();

        self.view = EnemyView::Animated(AnimatedImage::from_preloaded_gif(preloaded, x, y, width, height, loop_animation));
    }

    // Internal getter for the enemy's X position.
    // This abstracts over the two view types so callers can just use get_x().
    fn get_view_x(&self) -> f32 {
        match &self.view {
            EnemyView::Still(still) => still.get_x(),
            EnemyView::Animated(animated) => animated.get_x(),
        }
    }

    // Internal getter for the enemy's Y position.
    // This abstracts over the two view types so callers can just use get_y().
    fn get_view_y(&self) -> f32 {
        match &self.view {
            EnemyView::Still(still) => still.get_y(),
            EnemyView::Animated(animated) => animated.get_y(),
        }
    }

    // Internal setter for the enemy's X position.
    // Updates the correct underlying view type.
    fn set_view_x(&mut self, x: f32) {
        match &mut self.view {
            EnemyView::Still(still) => still.set_x(x),
            EnemyView::Animated(animated) => animated.set_x(x),
        }
    }

    // Internal setter for the enemy's Y position.
    // Updates the correct underlying view type.
    fn set_view_y(&mut self, y: f32) {
        match &mut self.view {
            EnemyView::Still(still) => still.set_y(y),
            EnemyView::Animated(animated) => animated.set_y(y),
        }
    }

    #[allow(unused)]
    pub fn moveing(&mut self, player: &mut Player, musicdiscfunctions: &mut Musicdisc) {
        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        if self.get_view_x() < player.get_x() {
            move_dir.x += 1.0; // Move right
        } else if self.get_view_x() > player.get_x() {
            move_dir.x -= 1.0; // Move left
        }
        if self.get_view_y() < player.get_y() {
            move_dir.y += 1.0; // Move down
        } else if self.get_view_y() > player.get_y() {
            move_dir.y -= 1.0; // Move up
        }
        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        self.movement = move_dir * self.move_speed * get_frame_time();

        self.set_view_x(self.get_view_x() + self.movement.x);
        self.set_view_y(self.get_view_y() + self.movement.y);
        let collision = check_collision(self.view_enemy(), player.view_player(), 1);

        if collision {
            let issactive = musicdiscfunctions.get_imstillstanding_active();
            player.dmgplayer(self.get_dmg(), issactive, self);
            self.knockback(player, "player");
            self.knockback(player, "enemy");
        }
        // Apply movement based on frame time
    }
    //change dmg
    #[allow(unused)]
    pub fn set_dmg(&mut self, dmg: f32) -> &mut Self {
        self.dmg = dmg;
        self
    }
    //changes image
    // Rebuilds the internal view from the new asset path.
    // If the new path is a GIF, this will switch the enemy to AnimatedImage.
    // If the new path is a still image, it remains or becomes StillImage.
    #[allow(unused)]
    pub async fn set_image(&mut self, image_path: &str) {
        let x = self.get_x();
        let y = self.get_y();
        let width = self.get_width();
        let height = self.get_height();
        self.view = Enemy::make_view(image_path, width, height, x, y, true, 1.0).await;
    }
    //changes health
    #[allow(unused)]
    pub fn set_health(&mut self, health: f32) -> &mut Self {
        self.health = health;
        self
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }

    #[allow(unused)]
    pub fn draw(&mut self) {
        // Draw the active view type, whether it's still or animated.
        match &mut self.view {
            EnemyView::Still(still) => still.draw(),
            EnemyView::Animated(animated) => animated.draw(),
        }
    }

    #[allow(unused)]
    //change speed
    pub fn set_speed(&mut self, move_speed: f32) -> &mut Self {
        self.move_speed = move_speed;
        self
    }

    #[allow(unused)]
    pub fn get_speed(&self) -> f32 {
        self.move_speed
    }

    // Getter for position as Vec2
    #[allow(unused)]
    pub fn get_position(&self) -> Vec2 {
        Vec2::new(self.get_x(), self.get_y())
    }

    // Getter for visibility
    #[allow(unused)]

    pub fn view_enemy(&self) -> &StillImage {
        match &self.view {
            EnemyView::Still(still) => still,
            EnemyView::Animated(animated) => panic!("Cannot get still image from an animated enemy"),
        }
    }

    pub fn view_enemy_animated(&self) -> &AnimatedImage {
        match &self.view {
            EnemyView::Still(_still) => panic!("Cannot get animated image from a still enemy"),
            EnemyView::Animated(animated) => animated,
        }
    }

    pub fn get_enemy_view_type(&self) -> String {
        match &self.view {
            EnemyView::Still(_still) => "still".into(),
            EnemyView::Animated(_animated) => "animated".into(),
        }
    }

    // Setter for position
    #[allow(unused)]
    pub fn set_position(&mut self, x: f32, y: f32) -> &mut Self {
        match &mut self.view {
            EnemyView::Still(still) => {
                still.set_x(x);
                still.set_y(y);
            }
            EnemyView::Animated(animated) => animated.set_position(x, y),
        }
        self
    }

    // Public getter for the enemy's X coordinate.
    // Delegates to the current view implementation (still or animated).
    pub fn get_x(&self) -> f32 {
        self.get_view_x()
    }

    #[allow(unused)]
    // Public setter for the enemy's X coordinate.
    // Updates the underlying view's X position.
    pub fn set_x(&mut self, x: f32) {
        self.set_view_x(x);
    }

    pub fn get_width(&self) -> f32 {
        match &self.view {
            EnemyView::Still(still) => still.get_width(),
            EnemyView::Animated(animated) => animated.size().x,
        }
    }

    pub fn get_height(&self) -> f32 {
        match &self.view {
            EnemyView::Still(still) => still.get_height(),
            EnemyView::Animated(animated) => animated.size().y,
        }
    }

    // Public getter for the enemy's position as a Vec2.
    // This uses the same X/Y dispatch helpers as the individual accessors.
    pub fn get_pos(&self) -> Vec2 {
        vec2(self.get_view_x(), self.get_view_y())
    }

    // Public getter for the enemy's Y coordinate.
    // Delegates to the current view implementation (still or animated).
    #[allow(unused)]
    pub fn get_y(&self) -> f32 {
        self.get_view_y()
    }

    #[allow(unused)]
    // Public setter for the enemy's Y coordinate.
    // Updates the underlying view's Y position.
    pub fn set_y(&mut self, y: f32) {
        self.set_view_y(y);
    }

    #[allow(unused)]
    // Alias for getting the enemy position as Vec2.
    // Equivalent to get_pos and uses the same view-based coordinates.
    pub fn pos(&self) -> Vec2 {
        vec2(self.get_view_x(), self.get_view_y())
    }

    // Calculate direction towards player
    #[allow(unused)]
    pub fn set_direction(&self, player_pos: Vec2) -> Vec2 {
        let direction = (player_pos - self.get_pos()).normalize();
        let movement = direction * self.move_speed * get_frame_time();

        movement
    }
    pub fn get_dmg(&self) -> f32 {
        self.dmg
    }

    pub fn dmg_enemy(&mut self, dmg: f32) -> bool {
        let mut dead = false;
        self.health -= dmg;
        if self.health <= 0.0 {
            dead = true;
        }
        dead
    }
    #[allow(unused)]
    pub fn get_projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
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
    pub fn draw_bullet(&mut self, player: &mut Player, musicdiscfunctions: &mut Musicdisc) {
        let dmg = self.get_dmg();
        for projectile in 0..self.projectiles.len() {
            let collision = check_collision(self.projectiles[projectile].view_player(), player.view_player(), 1);
            self.projectiles[projectile].move_projectiles(player.get_oldpos());
            if collision {
                let issactive = musicdiscfunctions.get_imstillstanding_active();
                player.dmgplayer(dmg, issactive, self);
                self.knockback(player, "player");

                self.projectiles.remove(projectile);

                break;
            }

            if self.projectiles[projectile].get_x() > 3000.0 || self.projectiles[projectile].get_y() > 2000.0 {
                self.projectiles.remove(projectile);
                break;
            }

            self.projectiles[projectile].draw();
        }
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
    pub async fn archer_action(&mut self, tm: &TextureManager, player: &mut Player, musicdiscfunctions: &mut Musicdisc) {
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
            self.moveing(player, musicdiscfunctions);
            self.archer_img_change(player.get_x(), self.get_x(), "move", &tm).await;
        }
        let mut healthbar = self.set_healthbar();
        healthbar.draw();
    }
    // Mage action method that handles movement, image changes, and shooting based on player proximity and cooldowns
    pub async fn mage_action(&mut self, tm: &TextureManager, player: &mut Player, musicdiscfunctions: &mut Musicdisc) {
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
            self.moveing(player, musicdiscfunctions);

            self.mage_img_change(player.get_x(), self.get_x(), "ready", &tm).await;
        }
        let mut healthbar = self.set_healthbar();
        healthbar.draw();
    }

    // Summoner action method that handles movement, image changes, and summoning slimes based on player proximity and cooldowns
    pub async fn summoner_action(&mut self, tm: &TextureManager, player: &mut Player) -> (Enemy, Enemy, Enemy, bool) {
        let mut slime1 = Enemy::new(
            "",
            25.0,
            25.0,
            self.get_x(),
            self.get_y() + 50.0,
            true,
            1.0,
            self.max_health * 1.3,
            self.dmg,
            "",
            "slime",
        )
        .await;
        let mut slime2 = Enemy::new(
            "",
            25.0,
            25.0,
            self.get_x() + 50.0,
            self.get_y() - 50.0,
            true,
            1.0,
            self.max_health * 1.3,
            self.dmg,
            "",
            "slime",
        )
        .await;
        let mut slime3 = Enemy::new(
            "",
            25.0,
            25.0,
            self.get_x() - 50.0,
            self.get_y() - 50.0,
            true,
            1.0,
            self.max_health * 1.3,
            self.dmg,
            "",
            "slime",
        )
        .await;

        let mut summoned = false;
        if get_time() - self.cooldown2 > 10.1 {
            self.summoner_img_change(player.get_x(), self.get_x(), "move", &tm).await;
            if get_time() - self.cooldown2 > 10.6 {
                self.teleport();
                self.cooldown2 = get_time();
            }
        } else if get_time() - self.cooldown > 1.0 {
            self.summoner_img_change(player.get_x(), self.get_x(), "ready", &tm).await;
        }

        if get_time() - self.cooldown > 15.0 {
            self.cooldown = get_time();

            slime1.set_preload(tm.get_preload("assets/slime.png").unwrap());
            slime2.set_preload(tm.get_preload("assets/slime.png").unwrap());
            slime3.set_preload(tm.get_preload("assets/slime.png").unwrap());
            summoned = true;
            self.summoner_img_change(player.get_x(), self.get_x(), "attack", &tm).await;
        }
        let mut healthbar = self.set_healthbar();
        healthbar.draw();
        (slime1, slime2, slime3, summoned)
    }
    // Slime splitting method that creates two smaller slimes upon the death of a large slime
    #[allow(unused)]
    pub async fn split(&mut self, tm: &TextureManager) -> (Enemy, Enemy) {
        let mut slime1 = Enemy::new(
            "",
            25.0,
            25.0,
            self.get_x() + 10.0,
            self.get_y(),
            true,
            1.0,
            self.max_health / 2.0,
            self.dmg / 2.0,
            "",
            "slime",
        )
        .await;
        let mut slime2 = Enemy::new(
            "",
            25.0,
            25.0,
            self.get_x() - 10.0,
            self.get_y(),
            true,
            1.0,
            self.max_health / 2.0,
            self.dmg / 2.0,
            "",
            "slime",
        )
        .await;
        slime1.set_preload(tm.get_preload("assets/slime.png").unwrap());
        slime2.set_preload(tm.get_preload("assets/slime.png").unwrap());
        (slime1, slime2)
        /*
            let mut summonx = self.get_x() - 10.0;

            for i in 0..2.0 {
                if i == 1 {
                    summonx = self.get_x() + 10.0;
                }
                let mut summoned_enemy = Enemy::new("", 25.0, 25.0, summonx, self.get_y(), true, 1.0, 100, 10, "").await;
                summoned_enemy.set_preload(tm.get_preload("assets/slime.png").unwrap());
            }
        */
    }
    // Slime action method that handles movement and splitting into smaller slimes upon death
    #[allow(unused)]
    pub async fn large_slime_action(&mut self, tm: &TextureManager, player: &mut Player, musicdiscfunctions: &mut Musicdisc) -> (Enemy, Enemy, bool) {
        let mut slime1 = Enemy::new(
            "",
            25.0,
            25.0,
            self.get_x() + 10.0,
            self.get_y(),
            true,
            1.0,
            self.max_health / 2.0,
            self.dmg / 2.0,
            "",
            "slime",
        )
        .await;
        let mut slime2 = Enemy::new(
            "",
            25.0,
            25.0,
            self.get_x() - 10.0,
            self.get_y(),
            true,
            1.0,
            self.max_health / 2.0,
            self.dmg / 2.0,
            "",
            "slime",
        )
        .await;

        let mut split = false;
        self.moveing(player, musicdiscfunctions);
        if self.health <= 0.0 {
            split = true;
            (slime1, slime2) = self.split(tm).await;
        }
        let mut healthbar = self.set_healthbar();
        healthbar.draw();
        (slime1, slime2, split)
    }
    pub fn slime_action(&mut self, player: &mut Player, musicdiscfunctions: &mut Musicdisc) {
        self.moveing(player, musicdiscfunctions);
        let mut healthbar = self.set_healthbar();
        healthbar.draw();
    }

    pub async fn cyric_action(&mut self, player: &mut Player, tm: &TextureManager, musicdiscfunctions: &mut Musicdisc) {
        rand::srand(date::now() as u64);
        if !((self.get_x() - player.get_x()).abs() < 150.0) || !((self.get_y() - player.get_y()).abs() < 150.0) {
            self.moveing(player, musicdiscfunctions);
        }
        if (get_time() - self.cooldown).abs() > self.cooldown2 {
            self.cooldown = get_time();
            let attack_choice = rand::gen_range(0, 2);
            if attack_choice >= 0 {
                self.meteors(&tm).await;
                self.cooldown2 = get_time() + 3.0;
            } else {
                self.shoot(player, 80.0, 80.0).await;
                self.cooldown2 = get_time() + 2.0;
            }
        }
        for i in 0..self.projectiles.len() {
            self.projectiles[i].move_projectiles(player.get_oldpos());
            if self.projectiles[i].check_collide(player) {
                player.dmgplayer(35.0, true, self);
            }
            self.projectiles[i].draw();
        }
        let mut healthbar = self.set_healthbar();
        healthbar.draw();
    }

    pub async fn meteors(&mut self, tm: &TextureManager) {
        self.set_projectile_preload(tm.get_preload("assets/cyric_files/meteor.png").unwrap());

        for i in 0..20 {
            let mut meteor = Projectile::new(
                self.projectile_image.clone(),
                50.0,
                50.0,
                rand::gen_range(200.0, 1800.0),
                100.0,
                true,
                1.0,
            )
            .await; // Create a projectile at the enemy's position
            meteor.set_speed(450.0);
            let meteor_pos = vec2(meteor.get_x(), meteor.get_y());
            let rand_pos = rand::gen_range(300.0, 1000.0);
            self.projectiles.push(meteor.clone());
            self.projectiles[i].set_pos(meteor_pos.x + rand_pos, meteor_pos.y - rand_pos);
            self.projectiles[i].set_direction(meteor_pos);
        }
        self.set_projectile_preload(tm.get_preload("assets/fireball.png").unwrap());
    }

    pub fn get_maxhealth(&self) -> f32 {
        self.max_health
    }

    pub fn set_healthbar(&self) -> ProgressBar {
        let maxhealth = self.get_maxhealth();

        let healthbar = ProgressBar::new(
            self.get_x(),
            self.get_y() - 20.0, // Position (x, y)
            30.0,
            5.0, // Size (width, height)
            0.0,
            maxhealth as f32,   // Range (min, max)
            self.health as f32, // Initial value
        );
        healthbar
    }

    pub fn reversereverse(&mut self, player_x: f32, player_y: f32, map: &map::Map, enemy_old_pos: Vec2) {
        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        self.movement = move_dir * self.move_speed * get_frame_time();

        if self.get_view_x() < player_x {
            move_dir.x -= 1.0; // Move right
        } else if self.get_view_x() > player_x {
            move_dir.x += 1.0; // Move left
        }
        if self.get_view_y() < player_y {
            move_dir.y -= 1.0; // Move down
        } else if self.get_view_y() > player_y {
            move_dir.y += 1.0; // Move up
        }
        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }
        self.set_x(enemy_old_pos.x + move_dir.x);
        if map.map_collision(self.view_enemy()).0 {
            //collision with map
            self.set_x(enemy_old_pos.x);
        }
        if self.get_view_x() > 930.0 {
            self.set_x(enemy_old_pos.x);
        }
        self.set_y(enemy_old_pos.y + move_dir.y);
        if map.map_collision(self.view_enemy()).0 {
            //collision with map
            self.set_y(enemy_old_pos.y);
        }
        let dx = player_x - self.get_x();
        let dy = player_y - self.get_y();
        if dx.abs() < 90.0 && dy.abs() < 90.0 {
            if self.get_x() < 512.0 && self.get_y() < 384.0 {
                self.set_position(550.0, 300.0);
            } else if self.get_x() < 512.0 && self.get_y() > 384.0 {
                self.set_position(450.0, 400.0);
            } else if self.get_x() > 512.0 && self.get_y() < 384.0 {
                self.set_position(550.0, 300.0);
            } else {
                self.set_position(450.0, 300.0);
            }
        }
    }

    pub fn pandemonium(&mut self, highesthealthenemypos: Vec2, enemy_old_pos: Vec2) {
        // Direction to move in
        let mut move_dir = vec2(0.0, 0.0);

        self.movement = move_dir * self.move_speed * get_frame_time();

        if self.get_view_x() < highesthealthenemypos.x {
            move_dir.x += 1.0; // Move right
        } else if self.get_view_x() > highesthealthenemypos.x {
            move_dir.x -= 1.0; // Move left
        }
        if self.get_view_y() < highesthealthenemypos.y {
            move_dir.y += 1.0; // Move down
        } else if self.get_view_y() > highesthealthenemypos.y {
            move_dir.y -= 1.0; // Move up
        }
        // Normalize the movement to prevent faster diagonal movement
        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        self.set_x(enemy_old_pos.x + move_dir.x / 2.0);
        self.set_y(enemy_old_pos.y + move_dir.y / 2.0);
    }

    pub fn check_collision(&self, img2: &StillImage) -> bool {
        let mut collided = false; // Placeholder for collision check
        if check_collision(self.view_enemy(), img2, 1) {
            collided = true;
        }
        collided
    }

    pub fn pushback(&mut self, enemy_old_pos: Vec2, other_pos: Vec2) {
        if enemy_old_pos.x < other_pos.x {
            self.set_x(self.get_x() - 40.0);
        }
        if enemy_old_pos.x > other_pos.x {
            self.set_x(self.get_x() + 40.0);
        }
        if enemy_old_pos.y < other_pos.y {
            self.set_y(self.get_y() - 40.0);
        }
        if enemy_old_pos.y > other_pos.y {
            self.set_y(self.get_y() + 40.0);
        }
        if self.get_x() < 70.0 {
            self.set_x(70.0);
        }
        if self.get_x() > 900.0 {
            self.set_x(930.0);
        }
        if self.get_y() < 50.0 {
            self.set_y(50.0);
        }
        if self.get_y() > 600.0 {
            self.set_y(600.0);
        }
    }

    pub fn sodapop(&mut self, first_pos: Vec2, map: &map::Map) {
        let min_y = first_pos.y - 20.0;
        let max_y = first_pos.y + 20.0;

        self.movement.y = self.move_speed * get_frame_time();
        let next_y = self.get_view_y() + self.movement.y;

        if next_y >= max_y {
            self.set_y(max_y);
            self.move_speed = -self.move_speed.abs();
        } else if next_y <= min_y {
            self.set_y(min_y);
            self.move_speed = self.move_speed.abs();
        } else {
            self.set_y(next_y);
        }

        if map.map_collision(self.view_enemy()).0 {
            self.set_y(first_pos.y);
        }
    }

    pub fn add_gold(&self, player: &mut Player) {
        let mut amount = (self.get_maxhealth() / 10.0).round() as i32;
        if amount <= 0 {
            amount = 1;
        }
        player.addcoins(amount);
    }
    #[allow(unused)]
    pub fn jeff_checkhit(&self, player: &mut Player, jeff_valid: bool, jeff_attackvalid: bool) -> (bool, bool) {
        let mut hit = jeff_valid;
        if check_collision(self.view_enemy_animated(), player.view_player(), 1) {
            hit = true;
        }
        (hit, hit)
    }

    pub fn jeff_choose_attack(&mut self) -> i32 {
        let attack = rand::gen_range(1, 4);
        attack
    }

    pub fn jeff_knifeattack1(&mut self) -> (i32, Label) {
        let mut lbl_warninglabel = Label::new("", 50.0, 100.0, 30);
        lbl_warninglabel.with_colors(RED, Some(RED));
        let wallchoice = rand::gen_range(1, 5);
        let walldistance_ud = rand::gen_range(150.0, VIRTUAL_WIDTH - 150.0);
        let walldistance_lr = rand::gen_range(150.0, VIRTUAL_HEIGHT - 150.0);
        match wallchoice {
            1 | 3 => {
                //north/south walls
                lbl_warninglabel.set_position(walldistance_ud, 0.0);
                lbl_warninglabel.with_fixed_size(150.0, VIRTUAL_HEIGHT + 100.0);
            }
            2 | 4 => {
                //east/west walls
                lbl_warninglabel.with_fixed_size(VIRTUAL_WIDTH + 100.0, 150.0);
                lbl_warninglabel.set_position(0.0, walldistance_lr);
            }
            _ => {}
        }
        lbl_warninglabel.draw();
        (wallchoice, lbl_warninglabel)
    }

    pub fn jeff_knifeattack2(&mut self, wallchoice: i32, warninglabel: &mut Label) -> Vec2 {
        let mut direction = Vec2::new(0.0, 0.0);
        match wallchoice {
            1 => {
                //north wall
                let position = warninglabel.get_position();
                self.set_position(position.x, position.y - 100.0);
                direction = Vec2::new(0.0, 1.0);
            }
            2 => {
                //east wall
                let position = warninglabel.get_position();
                self.set_position(position.x - 100.0, position.y);
                direction = Vec2::new(1.0, 0.0);
            }
            3 => {
                //south wall
                let position = warninglabel.get_position();
                self.set_position(position.x, VIRTUAL_HEIGHT + 100.0);
                direction = Vec2::new(0.0, -1.0);
            }
            4 => {
                //west wall
                let position = warninglabel.get_position();
                self.set_position(VIRTUAL_WIDTH + 100.0, position.y);
                direction = Vec2::new(-1.0, 0.0);
            }
            _ => {}
        }
        direction
    }

    pub fn jeff_knifeattack3(&mut self, player: &mut Player, direction: Vec2, musicdiscfunctions: &mut Musicdisc) -> bool {
        let mut attackend = false;
        self.movement = direction * self.move_speed * get_frame_time();
        self.set_x(self.get_x() + self.movement.x);
        self.set_y(self.get_y() + self.movement.y);
        if check_collision(self.view_enemy_animated(), player.view_player(), 1) {
            self.knockback(player, "player");
            let issactive = musicdiscfunctions.get_imstillstanding_active();
            player.dmgplayer(20.0, issactive, self);
            if player.get_x() < self.get_x() && player.get_y() < self.get_y() {
                player.set_position(player.get_x() - 150.0, player.get_y() - 150.0);
                if player.get_x() < 70.0 {
                    player.set_x(400.0);
                }
                if player.get_y() < 50.0 {
                    player.set_y(400.0);
                }
            } else if player.get_x() < self.get_x() && player.get_y() > self.get_y() {
                player.set_position(player.get_x() - 150.0, player.get_y() + 150.0);
                if player.get_x() < 70.0 {
                    player.set_x(400.0);
                }
                if player.get_y() > 600.0 {
                    player.set_y(300.0);
                }
            } else if player.get_x() > self.get_x() && player.get_y() < self.get_y() {
                player.set_position(player.get_x() + 150.0, player.get_y() - 150.0);
                if player.get_x() > 930.0 {
                    player.set_x(500.0);
                }
                if player.get_y() < 50.0 {
                    player.set_y(400.0);
                }
            } else {
                player.set_position(player.get_x() + 150.0, player.get_y() + 150.0);
                if player.get_x() > 930.0 {
                    player.set_x(500.0);
                }
                if player.get_y() > 600.0 {
                    player.set_y(300.0);
                }
            }
        }
        if self.get_x() < -100.0 || self.get_x() > VIRTUAL_WIDTH + 100.0 || self.get_y() < -100.0 || self.get_y() > VIRTUAL_HEIGHT + 100.0 {
            attackend = true;
        }
        attackend
    }

    pub fn jeff_normalidle(&mut self, player: &mut Player, tm: &TextureManager) {
        if self.get_x() < player.get_x() {
            self.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_idleR.gif").unwrap(), true);
        } else if self.get_x() > player.get_x() {
            self.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_idleL.gif").unwrap(), true);
        }
    }

    pub async fn jeff_cooldown(&mut self, tm: &TextureManager) -> (f64, AnimatedImage) {
        let cooldown = get_time();
        self.set_position(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0);
        let mut jeff_zzz = AnimatedImage::from_gif("", self.get_x() - 50.0, self.get_y() - 130.0, 50.0, 100.0, true).await;
        if let Some(preloaded) = tm.get_preloaded_animated_gif("assets/world1_boss/jeff_zzz3.gif") {
            jeff_zzz.set_preloaded_gif(preloaded, true);
        }
        self.set_position((VIRTUAL_WIDTH / 2.0) - 75.0, (VIRTUAL_HEIGHT / 2.0) - 75.0);
        self.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_tired.gif").unwrap(), true);
        (cooldown, jeff_zzz)
    }

    pub fn jeff_bubblebeam1(&mut self, tm: &TextureManager) -> Label {
        let mut lbl_warninglabel = Label::new("", 50.0, 100.0, 30);
        lbl_warninglabel.with_colors(RED, Some(RED));
        let sidechoice = rand::gen_range(1, 3);
        match sidechoice {
            1 => {
                //left
                self.set_position(100.0, (VIRTUAL_HEIGHT / 2.0) - 75.0);
                self.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_openmouth2R.gif").unwrap(), true);
                lbl_warninglabel.set_position(self.get_x() + 150.0, 250.0);
                lbl_warninglabel.with_fixed_size(VIRTUAL_WIDTH, VIRTUAL_HEIGHT - 420.0);
            }
            2 => {
                //right
                self.set_position(VIRTUAL_WIDTH - 200.0, (VIRTUAL_HEIGHT / 2.0) - 75.0);
                self.set_preload_gif(tm.get_preloaded_animated_gif("assets/world1_boss/jeff_openmouth2L.gif").unwrap(), true);
                lbl_warninglabel.set_position(-20.0, 250.0);
                lbl_warninglabel.with_fixed_size(VIRTUAL_WIDTH - 200.0, VIRTUAL_HEIGHT - 420.0);
            }
            _ => {}
        }
        lbl_warninglabel.draw();
        lbl_warninglabel
    }

    pub async fn jeff_bubblebeam2(
        &mut self,
        player: &mut Player,
        warninglabel: &mut Label,
        tm: &TextureManager,
        musicdiscfunctions: &mut Musicdisc,
    ) -> StillImage {
        let mut bubblebeam = StillImage::new(
            "",
            warninglabel.get_width().unwrap_or(0.0),  // width
            warninglabel.get_height().unwrap_or(0.0), // height
            warninglabel.get_x(),                     // x position
            warninglabel.get_y(),                     // y position
            true,                                     // Enable stretching
            1.0,                                      // Normal zoom (100%)
        )
        .await;
        bubblebeam.set_preload(tm.get_preload("assets/world1_boss/jeff_bubblebeam.png").unwrap());
        if check_collision(&bubblebeam, player.view_player(), 1) {
            let issactive = musicdiscfunctions.get_imstillstanding_active();
            player.dmgplayer(30.0, issactive, self);
        }
        bubblebeam
    }

    pub fn jeff_whirlpoolbounce(
        &mut self,
        player: &mut Player,
        whirlpool: &mut AnimatedImage,
        whirlpool_hitbox: &mut StillImage,
        map: &mut map::Map,
        whirlpool_direction: Vec2,
        musicdiscfunctions: &mut Musicdisc,
    ) -> Vec2 {
        let mut direction = whirlpool_direction;
        let move_speed = 400.0;
        let movement = direction * move_speed * get_frame_time();
        let whirlpool_old_pos = vec2(whirlpool.get_x(), whirlpool.get_y());
        let whirlpool_hitbox_old_pos = vec2(whirlpool_hitbox.get_x(), whirlpool_hitbox.get_y());
        let candidates = [
            vec2(1.0, 0.0),
            vec2(-1.0, 0.0),
            vec2(0.0, 1.0),
            vec2(0.0, -1.0),
            vec2(1.0, 1.0),
            vec2(1.0, -1.0),
            vec2(-1.0, 1.0),
            vec2(-1.0, -1.0),
        ];
        let mut bounced = false;
        whirlpool.set_x(whirlpool.get_x() + movement.x);
        whirlpool_hitbox.set_x(whirlpool_hitbox.get_x() + movement.x);
        if map.map_collision(whirlpool_hitbox).0 {
            bounced = true;
            whirlpool.set_x(whirlpool_old_pos.x);
            whirlpool_hitbox.set_x(whirlpool_hitbox_old_pos.x);
        }
        whirlpool.set_y(whirlpool.get_y() + movement.y);
        whirlpool_hitbox.set_y(whirlpool_hitbox.get_y() + movement.y);
        if map.map_collision(whirlpool_hitbox).0 {
            bounced = true;
            whirlpool.set_y(whirlpool_old_pos.y);
            whirlpool_hitbox.set_y(whirlpool_hitbox_old_pos.y);
        }
        if bounced {
            for _ in 0..candidates.len() * 2 {
                let candidate = candidates[rand::gen_range(0, candidates.len() as i32) as usize];
                let candidate_movement = candidate * move_speed * get_frame_time();
                let mut candidate_hitbox = whirlpool_hitbox.clone();
                candidate_hitbox.set_x(whirlpool_old_pos.x + candidate_movement.x);
                candidate_hitbox.set_y(whirlpool_old_pos.y + candidate_movement.y);
                if !map.map_collision(&candidate_hitbox).0 {
                    direction = candidate;
                    break;
                }
            }
        }
        if check_collision(whirlpool_hitbox, player.view_player(), 1) {
            self.knockback(player, "player");
            let issactive = musicdiscfunctions.get_imstillstanding_active();
            player.dmgplayer(20.0, issactive, self);
            if player.get_x() < self.get_x() && player.get_y() < self.get_y() {
                player.set_position(player.get_x() - 150.0, player.get_y() - 150.0);
                if player.get_x() < 70.0 {
                    player.set_x(400.0);
                }
                if player.get_y() < 50.0 {
                    player.set_y(400.0);
                }
            } else if player.get_x() < self.get_x() && player.get_y() > self.get_y() {
                player.set_position(player.get_x() - 150.0, player.get_y() + 150.0);
                if player.get_x() < 70.0 {
                    player.set_x(400.0);
                }
                if player.get_y() > 600.0 {
                    player.set_y(300.0);
                }
            } else if player.get_x() > self.get_x() && player.get_y() < self.get_y() {
                player.set_position(player.get_x() + 150.0, player.get_y() - 150.0);
                if player.get_x() > 930.0 {
                    player.set_x(500.0);
                }
                if player.get_y() < 50.0 {
                    player.set_y(400.0);
                }
            } else {
                player.set_position(player.get_x() + 150.0, player.get_y() + 150.0);
                if player.get_x() > 930.0 {
                    player.set_x(500.0);
                }
                if player.get_y() > 600.0 {
                    player.set_y(300.0);
                }
            }
        }
        whirlpool.draw();
        direction
    }

    pub async fn plant_boss_action(
        &mut self,
        player: &mut Player,
        tm: &TextureManager,
        attack: &mut bool,
        timer: &mut f64,
        shoot: &mut bool,
        chomp: &mut bool,
        attack_choice: &mut i32,
        dig: &mut bool,
        musicdiscfunctions: &mut Musicdisc,
        hit: &mut bool,
    ) {
        let way: &str;
        if player.get_x() < self.get_x() {
            way = "L";
        } else {
            way = "R";
        }

        if *attack == true {
            self.cooldown = get_time();
        }
        if get_time() > self.cooldown + 5.0 {
            if get_time() >= self.cooldown + 5.1 && get_time() < self.cooldown + 5.2 {
                rand::srand(date::now() as u64);
                *attack_choice = rand::gen_range(0, 2);
                *hit = false;
            }

            if *attack_choice == 0 {
                self.plant_boss_shoot(player, tm, way, shoot, dig, timer).await;
            } else {
                self.plant_boss_chomp(player, tm, way, chomp, dig, timer).await;
            }
            if get_time() > self.cooldown + 9.0 {
                self.cooldown = get_time();
                self.set_preload_gif(
                    tm.get_preloaded_animated_gif(format!("assets/world2_boss/boss_idle{}.gif", way).as_str())
                        .unwrap(),
                    true,
                );
                *attack = true;
                *dig = false;
                *shoot = false;
                *chomp = false;
            }
        }
        *attack = false;

        let collision = check_collision(self.view_enemy_animated(), player.view_player(), 1);
        if collision && *hit == false {
            let issactive = musicdiscfunctions.get_imstillstanding_active();
            player.dmgplayer(self.get_dmg(), issactive, self);
            self.knockback(player, "enemy");
            *hit = true;
        }
    }

    pub async fn plant_boss_shoot(&mut self, player: &mut Player, tm: &TextureManager, way: &str, shoot: &mut bool, dig: &mut bool, timer: &mut f64) {
        if *shoot == false {
            self.cooldown2 = get_time();

            *shoot = true;
        }
        self.plant_boss_dig_shoot(player, tm, way, dig, timer).await;
        if get_time() > self.cooldown2 + 2.0 && get_time() < self.cooldown2 + 2.1 {
            self.set_preload_gif(
                tm.get_preloaded_animated_gif(format!("assets/world2_boss/boss_shoot{}.gif", way).as_str())
                    .unwrap(),
                false,
            );
        }
        if get_time() > self.cooldown2 + 2.5 {
            if *shoot == true {
                self.shoot(player, 100.0, 100.0).await;
                *shoot = false;
            }
        }
    }

    pub async fn plant_boss_chomp(&mut self, player: &mut Player, tm: &TextureManager, way: &str, chomp: &mut bool, dig: &mut bool, timer: &mut f64) {
        if *chomp == false {
            self.cooldown2 = get_time();

            *chomp = true;
        }
        self.plant_boss_dig_bite(player, tm, way, dig, timer).await;
        if get_time() > self.cooldown2 + 2.0 && get_time() < self.cooldown2 + 2.1 {
            self.set_preload_gif(
                tm.get_preloaded_animated_gif(format!("assets/world2_boss/boss_bite{}.gif", way).as_str())
                    .unwrap(),
                false,
            );
        }
    }

    pub async fn plant_boss_dig_shoot(&mut self, player: &mut Player, tm: &TextureManager, way: &str, dig: &mut bool, timer: &mut f64) {
        if *dig == false {
            *timer = get_time();
            *dig = true;
        }
        if get_time() > *timer + 0.1 && get_time() < *timer + 0.2 {
            self.set_preload_gif(
                tm.get_preloaded_animated_gif(format!("assets/world2_boss/boss_dig{}.gif", way).as_str())
                    .unwrap(),
                false,
            );
        }

        if get_time() > *timer + 1.0 && get_time() < *timer + 1.1 {
            self.set_preload_gif(
                tm.get_preloaded_animated_gif(format!("assets/world2_boss/boss_dig_up{}.gif", way).as_str())
                    .unwrap(),
                false,
            );

            rand::srand(date::now() as u64);
            let rand_x = rand::gen_range(70.0, 900.0);
            let rand_y = player.get_y();
            self.set_position(rand_x, rand_y);
        }
    }
    pub async fn plant_boss_dig_bite(&mut self, player: &mut Player, tm: &TextureManager, way: &str, dig: &mut bool, timer: &mut f64) {
        if *dig == false {
            *timer = get_time();
            *dig = true;
        }
        if get_time() > *timer + 0.1 && get_time() < *timer + 0.2 {
            self.set_preload_gif(
                tm.get_preloaded_animated_gif(format!("assets/world2_boss/boss_dig{}.gif", way).as_str())
                    .unwrap(),
                false,
            );
        }

        if get_time() > *timer + 1.0 && get_time() < *timer + 1.1 {
            self.set_preload_gif(
                tm.get_preloaded_animated_gif(format!("assets/world2_boss/boss_dig_up{}.gif", way).as_str())
                    .unwrap(),
                false,
            );

            rand::srand(date::now() as u64);
            let rand_x = rand::gen_range(0, 2);
            let away;
            if rand_x == 0 {
                away = 1.0;
            } else {
                away = -1.0;
            }

            let pos_x = player.get_x() + away * 50.0; // Adjust the offset as needed
            let rand_y = player.get_y();
            self.set_position(pos_x, rand_y);
        }
    }

    pub fn knockback(&mut self, player: &mut Player, target: &str) {
        let player_pos = player.get_oldpos();
        let enemy_pos = self.get_pos();
        let direction = (enemy_pos - player_pos).normalize();
        let knockback_distance = 100.0; // Adjust this value as needed
        let knockback_vector = direction * knockback_distance;
        /*
        if target == "player" {
            player.set_position((player.get_x() - knockback_vector.x), (player.get_y() - knockback_vector.y));
            if player.get_x() < 40.0 {
                player.set_position(50.0, player.get_y());
            }
            if player.get_x() > VIRTUAL_WIDTH - 40.0 {
                player.set_position(VIRTUAL_WIDTH - 50.0, self.get_y());
            }
            if player.get_y() < 60.0 {
                player.set_position(player.get_x(), 70.0);
            }
            if player.get_y() > VIRTUAL_HEIGHT - 60.0 {
                player.set_position(player.get_x(), VIRTUAL_HEIGHT - 70.0);
            }
        }
            */
        if target == "enemy" {
            self.set_x(self.get_x() + knockback_vector.x);
            self.set_y(self.get_y() + knockback_vector.y);

            if self.get_x() < 40.0 {
                self.set_x(50.0);
            }
            if self.get_x() > VIRTUAL_WIDTH - 40.0 {
                self.set_x(VIRTUAL_WIDTH - 50.0);
            }
            if self.get_y() < 60.0 {
                self.set_y(70.0);
            }
            if self.get_y() > VIRTUAL_HEIGHT - 60.0 {
                self.set_y(VIRTUAL_HEIGHT - 70.0);
            }
        }
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
