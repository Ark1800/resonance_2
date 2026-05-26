// pub mod projectile;
// use crate::modules::projectile::Projectile;

use crate::modules::collision::check_collision;
use crate::modules::player::Player;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

#[derive(Clone)]

pub struct Projectile {
    view: StillImage,
    move_speed: f32,
    direction: Vec2,
}

impl Projectile {
    // Creates a new projectile with the given parameters and returns it
    #[allow(unused)]
    pub async fn new(asset_path: StillImage, width: f32, height: f32, x: f32, y: f32, stretch_enabled: bool, zoom_level: f32) -> Projectile {
        let mut bob = asset_path;
        bob.set_x(x);
        bob.set_y(y);
        bob.set_size(width, height);
        bob.set_stretch(stretch_enabled);
        bob.set_zoom(zoom_level);
        Projectile {
            view: bob,
            move_speed: 400.0,
            direction: vec2(0.0, 0.0),
        }
    }

    #[allow(unused)]
    pub fn draw(&mut self) {
        self.view.draw();
    }
    #[allow(unused)]
    // Sets the speed, despite the current amount
    pub fn set_speed(&mut self, speed: f32) {
        self.move_speed = speed;
    }

    #[allow(unused)]
    // Adds speed to the current amount
    pub fn add_speed(&mut self, amount: f32) {
        self.move_speed += amount;
    }
    #[allow(unused)]
    // Returns the current speed
    pub fn get_speed(&self) -> f32 {
        self.move_speed
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.view.set_angle(angle);
    }

    pub fn get_angle(&self) -> f32 {
        self.view.get_angle()
    }

    
    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.view.set_x(x);
        self.view.set_y(y);
    }

    #[allow(unused)]
    // Gets X and Y
    pub fn get_pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }

    #[allow(unused)]
    // Gets X
    pub fn get_x(&self) -> f32 {
        self.view.get_x()
    }

    #[allow(unused)]
    pub fn set_x(&mut self, x: f32) {
        self.view.set_x(x);
    }

    #[allow(unused)]
    // Gets Y
    pub fn get_y(&self) -> f32 {
        self.view.get_y()
    }
    #[allow(unused)]
    pub fn set_y(&mut self, y: f32) {
        self.view.set_y(y);
    }
    #[allow(unused)]
    // Lets us use the player as a StillImage
    pub fn view_player(&self) -> &StillImage {
        &self.view
    }
    #[allow(unused)]
    pub fn get_direction(&self) -> Vec2 {
        self.direction
    }

    
    #[allow(unused)]
    pub fn set_preload(&mut self, preloaded: (Texture2D, Option<Vec<u8>>, String)) {
        let (texture, mask, filename) = preloaded;
        self.view.texture = texture;
        self.view.transparency_mask = mask;
        self.view.filename = filename;
    }
    // Sets the rotation of the projectile based on the player's and enemy's positions
    pub fn set_rotation(&mut self, playerx: f32, playery: f32, enemyx: f32, enemyy: f32) -> f32 {
        // Calculate the angle using atan2 to get the correct quadrant
        let oppositelen = playery - enemyy;
        let adjacentlen = playerx - enemyx;
        let result = oppositelen / adjacentlen;
        let mut angle = result.atan();
        if playerx < enemyx {
            // If the player is to the left of the enemy, we need to add 180 degrees (PI radians) to the angle
            angle += std::f32::consts::PI;
        }

        return angle;
    }
    // Moves the projectile in the direction it's facing, multiplied by the move speed and frame time
    #[allow(unused)]
    pub fn move_projectiles(&mut self, player_pos: Vec2) {
        let movement = self.direction * self.move_speed * get_frame_time();

        self.set_x(self.get_x() + movement.x);
        self.set_y(self.get_y() + movement.y);
    }
    // Sets the direction of the projectile based on the player's position and the projectile's current position
    pub fn set_direction(&mut self, player_pos: Vec2) {
        self.direction = (player_pos - self.get_pos()).normalize();
    }
    // Checks for collision between the projectile and the player, returns true if there is a collision
    #[allow(unused)]
    pub fn check_collide(&mut self, object: &Player) -> bool {
        let mut collide = false;
        if check_collision(&self.view, object.view_player(), 1) {
            collide = true;
        }
        collide
    }
}
