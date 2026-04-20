// pub mod projectile;
// use crate::modules::projectile::Projectile;




use crate::modules::collision::check_collision;
use crate::modules::still_image::StillImage;
use crate::modules::player::Player;
use macroquad::prelude::*;
use crate::modules::enemy::Enemy;






pub struct Projectile {
    view: StillImage,
    move_speed: f32,
    direction: Vec2
    
}

impl Projectile {
    
    #[allow(unused)]
    pub async fn new(asset_path: &str, width: f32, height: f32, x: f32, y: f32, stretch_enabled: bool, zoom_level: f32) -> Projectile {
        Projectile { view: StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await, move_speed: 400.0, direction: vec2(0.0, 0.0) }
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

    #[allow(unused)]
    // Sets X and Y
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
    pub fn get_direction(&self) -> Vec2 {
        self.direction
    }

    #[allow(unused)]
     pub fn move_projectiles(&mut self, player_pos: Vec2) { 
           
 let movement = self.direction * self.move_speed * get_frame_time();

            self.set_x(self.get_x() + movement.x);
            self.set_y(self.get_y() + movement.y);
        
        
    }

    pub fn set_direction (&mut self, player_pos: Vec2)  {
        self.direction = (player_pos - self.get_pos()).normalize();
    }

    #[allow(unused)]
    pub fn check_collide(&mut self, object: &Player) -> bool {
        let mut collide = false;
        if check_collision(&self.view, object.view_player(), 1) {
            collide = true;
        }
        collide
    }
}
