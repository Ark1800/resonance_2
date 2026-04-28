//pub mod enemy;

//use crate::modules::enemy::Enemy;

use crate::modules::projectile::Projectile;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
pub struct Enemy {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
    health: i32,
    dmg: i32,
    projectiles: Vec<Projectile>,
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
    ) -> Enemy {
        Enemy {
            view: StillImage::new(asset_path, width, height, x, y, stretch_enabled, zoom_level).await,
            move_speed: 200.0, // Default speed
            movement: Vec2::ZERO,
            health,
            dmg,
            projectiles: Vec::new(),
        }
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

    #[allow(unused)]
    pub fn set_dmg(&mut self, dmg: i32) -> &mut Self {
        self.dmg = dmg;
        self
    }
pub async fn set_image(&mut self, image_path: &str) {
        self.view.set_texture(image_path).await;
    }
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
    pub fn pos(&self) -> Vec2 {
        vec2(self.view.get_x(), self.view.get_y())
    }
        
        #[allow(unused)]
        pub fn set_direction(&self, player_pos: Vec2) -> Vec2 {
        let direction = (player_pos - self.get_pos()).normalize();
        let movement = direction * self.move_speed * get_frame_time();

        movement
    }

    pub async fn shoot(&self,  mut projectiles_list: Vec<Projectile>,) -> Vec<Projectile> {
        // Implement shooting logic here
        let img = Projectile::new(
        "assets/slime.png",
        25.0,  // width
        25.0,  // height
        self.get_x(),  // x position
        self.get_y(),   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
        
    ).await;
    
projectiles_list.push(img);
projectiles_list
    }


    pub async fn draw_bullet(&mut self, enemy_type: &str) {
        let bullet = Projectile::new(
        "assets/slime.png",
        25.0,  // width
        25.0,  // height
        self.get_x(),  // x position
        self.get_y(),   // y position
        true,   // Enable stretching
        1.0,    // Normal zoom (100%)
        
    ).await;
        self.projectiles.push(bullet);
    }
   
    pub fn get_projectiles(&self) -> &Vec<Projectile> {
        &self.projectiles
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

