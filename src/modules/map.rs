/*
Leo
April 22nd, 2026
Randomized map generation based on set objects


0 - Empty
1 - Wall
2 - Chest
3 - Enemy spawn
*/

use crate::modules::collision::check_collision;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub struct Map {
    map_array: [[i32; 10]; 15],
    wall_list: Vec<StillImage>,
    change_wall: bool,
    chest_list: Vec<StillImage>,
    change_chest: bool,
}

impl Map {
    pub async fn new() -> Self {
        Map {
            map_array: [[0; 10]; 15],
            wall_list: Vec::new(),
            change_wall: true,
            chest_list: Vec::new(),
            change_chest: true,
        }
    }

    pub async fn create_map_array(&mut self, enemy_num: i32, chest_num: i32, entrance_num: i32, wall_num: i32, entrance_sides: Vec<i32>) {
        for x in 0..self.map_array.len() {
            for y in 0..self.map_array[0].len() {
                if x == 0 || x == self.map_array.len() - 1 {
                    self.map_array[x][y] = 1;
                } else {
                    if y == 0 || y == self.map_array[0].len() - 1 {
                        self.map_array[x][y] = 1;
                    }
                }
            }
        }
        for _wall in 0..wall_num {
            let mut pass = false;
            while !pass {
                let rand_num: Vec2 = vec2(rand::gen_range(1.0, 8.0), rand::gen_range(1.0, 13.0));
                if self.map_array[rand_num.y as usize][rand_num.x as usize] == 0 {
                    self.map_array[rand_num.y as usize][rand_num.x as usize] = 1;
                    pass = true;
                }
            }
        }
        for _chest in 0..chest_num {
            let mut pass = false;
            while !pass {
                let rand_num: Vec2 = vec2(rand::gen_range(1.0, 8.0), rand::gen_range(1.0, 13.0));
                if self.map_array[rand_num.y as usize][rand_num.x as usize] == 0 {
                    self.map_array[rand_num.y as usize][rand_num.x as usize] = 2;
                    pass = true;
                }
            }
        }
        for _enemy in 0..enemy_num {
            let mut pass = false;
            while !pass {
                let rand_num: Vec2 = vec2(rand::gen_range(1.0, 8.0), rand::gen_range(1.0, 13.0));
                if self.map_array[rand_num.y as usize][rand_num.x as usize] == 0 {
                    self.map_array[rand_num.y as usize][rand_num.x as usize] = 3;
                    pass = true;
                }
            }
        }

        for entrance in 0..entrance_num {
            // 1 = left, 2 = up, 3 = right, 4 = down
            match entrance_sides[entrance as usize] {
                1 => self.map_array[6][0] = 0,
                2 => self.map_array[0][4] = 0,
                3 => self.map_array[7][9] = 0,
                4 => self.map_array[14][4] = 0,
                _ => println!("Invalid num"),
            }
            match entrance_sides[entrance as usize] {
                1 => self.map_array[7][0] = 0,
                2 => self.map_array[0][5] = 0,
                3 => self.map_array[6][9] = 0,
                4 => self.map_array[14][5] = 0,
                _ => println!("Invalid num"),
            }
        }
    }

    pub async fn draw_map(&mut self) {
        let mut changed_wall = false;
        let mut changed_chest = false;
        for x in 0..self.map_array.len() {
            for y in 0..self.map_array[0].len() {
                if self.change_wall {
                    if self.map_array[x][y] == 1 {
                        self.wall_list
                            .push(StillImage::new("assets/fireball.png", 80.0, 80.0, y as f32 * 80.0, x as f32 * 80.0, true, 1.0).await);
                    }
                    changed_wall = true;                       //sorry leo had to change so we could run
                }                                             //add yo assets gangggggggggggggggggggggg
                if self.change_chest {
                    if self.map_array[x][y] == 2 {
                        self.chest_list
                            .push(StillImage::new("assets/fireball.png", 80.0, 80.0, y as f32 * 80.0, x as f32 * 80.0, true, 1.0).await);
                    }
                    changed_chest = true;
                }
            }
        }
        if changed_wall {
            self.change_wall = false;
        }
        if changed_chest {
            self.change_chest = false;
        }

        for i in 0..self.wall_list.len() {
            self.wall_list[i].draw();
        }
        for i in 0..self.chest_list.len() {
            self.chest_list[i].draw();
        }
    }

    pub fn map_collision(&self, player: &StillImage) -> (bool, bool) {
        // If the player enters a wall space, it returns true
        let mut wall = false;
        // If the player enters a chest space, it returns true
        let mut chest = false;

        for i in 0..self.wall_list.len() {
            if check_collision(player, &self.wall_list[i], 1) {
                wall = true;
            }
        }
        for i in 0..self.chest_list.len() {
            if check_collision(player, &self.chest_list[i], 1) {
                chest = true;
            }
        }

        (wall, chest)
    }

    // Change list is a list of what the changes are, change coords is a list of what coords to change based on the change list
    pub fn change_map(&mut self, change_list: Vec<i32>, change_coords: Vec<Vec<i32>>) {
        for i in 0..change_list.len() {
            self.map_array[change_coords[i as usize][0] as usize][change_coords[i as usize][1] as usize] = change_list[i];
            if change_list[i] == 1 && !self.change_wall {
                self.change_wall = true;
            }
            if change_list[i] == 2 && !self.change_chest {
                self.change_chest = true;
            }
        }
    }

    pub fn get_map_array(&self) -> &[[i32; 10]; 15] {
        &self.map_array
    }
}
