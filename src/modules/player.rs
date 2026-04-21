use macroquad::prelude::*;
use crate::modules::still_image::StillImage;
use crate::modules::collision::check_collision;

//IMPLEMENTATION
//in every screen write
/* 
player.handle_keypresses().await;
player.move_player();
player.draw();
*/
pub struct Player {
    view: StillImage,
    move_speed: f32,
    movement: Vec2,
}

impl Player {
    pub async fn new(image_path: &str, x: f32, y: f32) -> Self {
        let view = StillImage::new(
            image_path,
            60.0,  // width 
            110.0,  // height
            x,     // x position
            y,     // y position
            true,   // Enable stretching
            1.0,    // Normal zoom (100%)

        ).await;

        Player {
            view,
            move_speed: 400.0, // Movement speed in pixels per second
            movement: vec2(0.0, 0.0),
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
    }

    pub async fn handle_image(&mut self) {
        if is_key_down(KeyCode::W) && is_key_down(KeyCode::D) {
            self.view.set_image("assets/player_files/player_tr.png").await;
        } else if is_key_down(KeyCode::W) && is_key_down(KeyCode::A) {
            self.view.set_image("assets/player_files/player_tl.png").await;
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::D) {
            self.view.set_image("assets/player_files/player_br.png").await;
        } else if is_key_down(KeyCode::S) && is_key_down(KeyCode::A) {
            self.view.set_image("assets/player_files/player_bl.png").await;
        }
          else if is_key_down(KeyCode::D) {
            self.view.set_image("assets/player_files/player_r.png").await;
        } else if is_key_down(KeyCode::A) {
            self.view.set_image("assets/player_files/player_l.png").await;
        } else if is_key_down(KeyCode::S) {
            self.view.set_image("assets/player_files/player_b.png").await;
        } else if is_key_down(KeyCode::W) {
            self.view.set_image("assets/player_files/player_t.png").await;
        }
    }

    pub fn move_x(&mut self) {
        self.view.set_x(self.view.get_x() + self.movement.x);
    }

    pub fn move_y(&mut self) {
        self.view.set_y(self.view.get_y() + self.movement.y);
    }

    pub fn move_player(&mut self) {
        self.move_x();
        self.move_y();
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
}