//pub mod enemy;

//use crate::modules::enemy::Enemy;

use crate::modules::projectile::Projectile;
use crate::modules::still_image::StillImage;
use crate::modules::player::Player;
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
   pub async fn archer_img_change(&mut self, playerx: f32, archerx: f32, action: &str) -> &Enemy {
    let mut way = "";
    if archerx < playerx {
        way = "R";
    } else {
        way = "L";
    }
    match action {
        "move" => {
            self.set_image(format!("assets/archer_files/archer_run{}.png", way).as_str()).await;
        }
        "ready" => {
            self.set_image(format!("assets/archer_files/archer_ready{}.png", way).as_str()).await;
        }
        "attack" => {
            self.set_image(format!("assets/archer_files/archer_shoot{}.png", way).as_str()).await;
        }
        _ => {}
    }
    self
}
pub async fn mage_img_change(&mut self, playerx: f32, magex: f32, action: &str) -> &Enemy {
    let mut way = "";
    if magex < playerx {
        way = "R";
    } else {
        way = "L";
    }
    match action {
        "ready" => {
            self.set_image(format!("assets/mage_files/mage_stand{}.png", way).as_str()).await;
        }
        "attack" => {
            self.set_image(format!("assets/mage_files/mage_shoot{}.png", way).as_str()).await;
        }
        _ => {}
    }
    self
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

    pub async fn shoot(&mut self,  player: &mut Player) -> &Vec<Projectile>  {
           
        let mut projectile = Projectile::new("assets/arrow.png", 50.0, 50.0, self.get_x(), self.get_y(), true, 1.0).await;
                
                let angle = projectile.set_rotation(player.get_x(), player.get_y(), self.get_x(), self.get_y());
                projectile.set_angle(angle);
                projectile.set_direction(player.get_oldpos());
                self.projectiles.push(projectile);
    
//projectile_list.push(projectile);

&self.projectiles
    }
     


    pub async fn draw_bullet(&mut self) {
       for projectile in &mut self.projectiles {
            projectile.draw();
        }
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

