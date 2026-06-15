/*
Leo
April 22nd, 2026
Randomized map generation based on set objects


0 - Empty
1 - Wall
2 - Chest

map.create_map_array(chest_num, entrance_num, wall_num, entrance_sides)

// Up entrance: map.change_map(vec![0, 0], vec![vec![7, 0], vec![6, 0]]);
// Left entrance: map.change_map(vec![0, 0], vec![vec![0, 4], vec![0, 5]]);
// Down entrance: map.change_map(vec![0, 0], vec![vec![7, 9], vec![6, 9]]);
// Right entrance: map.change_map(vec![0, 0], vec![vec![14, 4], vec![14, 5]]);


// 1 = up, 2 = left, 3 = down, 4 = right
*/

use crate::modules::collision::check_collision;
use crate::modules::preload_image::TextureManager;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use miniquad::date;
#[derive(Clone)]
pub struct Map {
    map_array: [[i32; 10]; 15],  // Map array
    wall_list: Vec<StillImage>,  // List of all objects that are walls
    change_wall: bool,           // Boolean to check if the wall list needs to be changed
    chest_list: Vec<StillImage>, // List of all objects that are chests
    change_chest: bool,          // Boolean to check if the chest list needs to be changed
    map_dimensions: Vec2, // Dimensions of the map., While the grid is 15 by 10, the dimensions is the height and width of the screen, the map will scale to fit the dimensions appropriately
    image_list: Vec<String>, // List of all images to be used in the map, 0 is wall, 1 is chest
}
impl Map {
    pub async fn new(width: f32, height: f32, image_list: Vec<String>) -> Self {
        Map {
            map_array: [[0; 10]; 15],
            wall_list: Vec::new(),
            change_wall: true,
            chest_list: Vec::new(),
            change_chest: true,
            map_dimensions: vec2(width, height),
            image_list: image_list,
        }
    }
    #[allow(unused)]
    pub async fn create_map_array(&mut self, chest_num: i32, entrance_num: i32, wall_num: i32, entrance_sides: Vec<i32>) {
        rand::srand(date::now() as u64);
        // Clears the map array, setting all spaces to 0
        for x in 0..self.map_array.len() {
            for y in 0..self.map_array[x].len() {
                if self.map_array[x][y] != 0 {
                    self.map_array[x][y] = 0;
                }
            }
        }
        // Sets every edge space to 1, a wall
        for x in 0..self.map_array.len() {
            for y in 0..self.map_array[0].len() {
                if x == 0 || x == self.map_array.len() - 1 {
                    self.map_array[x][y] = 1;
                } else if y == 0 || y == self.map_array[0].len() - 1 {
                    self.map_array[x][y] = 1;
                }
            }
        }
        // Randomly generates walls based on how many the user wants, detailed in wall_num
        for _wall in 0..wall_num {
            let mut pass = false;
            // Rerolls random coordinates until it finds an empty space to put a wall. Only puts a wall if there's not a chest there
            while !pass {
                let rand_num: Vec2 = vec2(rand::gen_range(1.0, 8.0), rand::gen_range(1.0, 13.0));
                if self.map_array[rand_num.y as usize][rand_num.x as usize] == 0 {
                    self.map_array[rand_num.y as usize][rand_num.x as usize] = 1;
                    pass = true;
                }
            }
        }
        // Randomly generates chests based on how many the user wants, detailed in chest_num
        for _chest in 0..chest_num {
            let mut pass = false;
            // Rerolls random coordinates until it finds an empty space to put a chest. Only puts a chest if there's not a wall there
            while !pass {
                let rand_num: Vec2 = vec2(rand::gen_range(1.0, 8.0), rand::gen_range(1.0, 13.0));
                if self.map_array[rand_num.y as usize][rand_num.x as usize] == 0 {
                    self.map_array[rand_num.y as usize][rand_num.x as usize] = 2;
                    pass = true;
                }
            }
        }
        // Generates entrances based on how many are wanted and where the user wants them
        for entrance in 0..entrance_num {
            // 1 = up, 2 = left, 3 = down, 4 = right
            match entrance_sides[entrance as usize] {
                1 => self.map_array[7][0] = 0,
                2 => self.map_array[0][4] = 0,
                3 => self.map_array[7][9] = 0,
                4 => self.map_array[14][4] = 0,
                _ => {},
            }
            // Runs twice to make two open spaces
            match entrance_sides[entrance as usize] {
                1 => self.map_array[6][0] = 0,
                2 => self.map_array[0][5] = 0,
                3 => self.map_array[6][9] = 0,
                4 => self.map_array[14][5] = 0,
                _ => {},
            }
        }
    }
    #[allow(unused)]
    pub async fn draw_map(&mut self, tm: &TextureManager) {
        // changed wall and chest are used to check if the change to the wall or chest list has been made
        let mut changed_wall = false;
        let mut changed_chest = false;

        if self.change_wall {
            self.wall_list.clear();
        }
        if self.change_chest {
            self.chest_list.clear();
        }

        for x in 0..self.map_array.len() {
            for y in 0..self.map_array[x].len() {
                // Pushes new walls into the list, replacing air with wall. If no changes need to be made, doesn't run
                if self.change_wall {
                    if self.map_array[x][y] == 1 {
                        self.wall_list.push(
                            StillImage::new(
                                "",
                                self.map_dimensions.x / 15.0,
                                self.map_dimensions.y / 10.0,
                                x as f32 * self.map_dimensions.x / 15.0,
                                y as f32 * self.map_dimensions.y / 10.0,
                                true,
                                1.0,
                            )
                            .await,
                        );
                        let wall_list_len = self.wall_list.len() - 1;
                        self.wall_list[wall_list_len].set_preload(tm.get_preload(format!("{}", self.image_list[0]).as_str()).unwrap());
                    }
                    changed_wall = true;
                }
                // Pushes new chests into the list, replacing air with chest. If no changes need to be made, doesn't run
                if self.change_chest {
                    if self.map_array[x][y] == 2 {
                        self.chest_list.push(
                            StillImage::new(
                                "",
                                self.map_dimensions.x / 15.0,
                                self.map_dimensions.y / 10.0,
                                x as f32 * self.map_dimensions.x / 15.0,
                                y as f32 * self.map_dimensions.y / 10.0,
                                true,
                                1.0,
                            )
                            .await,
                        );
                        let chest_list_len = self.chest_list.len() - 1;
                        self.chest_list[chest_list_len].set_preload(tm.get_preload(format!("{}", self.image_list[1]).as_str()).unwrap());
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

        // Draws the walls and chests
        for i in 0..self.wall_list.len() {
            self.wall_list[i].draw();
        }
        for i in 0..self.chest_list.len() {
            self.chest_list[i].draw();
        }
    }

    #[allow(unused)]
    pub fn map_collision(&self, player: &StillImage) -> (bool, bool) {
        // If the player enters a wall space, it returns true
        let mut wall = false;
        // If the player enters a chest space, it returns true
        let mut chest = false;

        // Checks collision with walls
        for i in 0..self.wall_list.len() {
            if check_collision(player, &self.wall_list[i], 1) {
                wall = true;
            }
        }

        // Checks collision with chests
        for i in 0..self.chest_list.len() {
            if check_collision(player, &self.chest_list[i], 1) {
                chest = true;
            }
        }

        (wall, chest)
    }
    #[allow(unused)]
    // Change list is a list of what the changes are, change coords is a list of what coords to change based on the change list
    pub fn change_map(&mut self, change_list: Vec<i32>, change_coords: Vec<Vec<i32>>) {
        // Goes through the change list and makes appropriate shifts
        for i in 0..change_list.len() {
            self.map_array[change_coords[i as usize][0] as usize][change_coords[i as usize][1] as usize] = change_list[i];
            if change_list[i] == 0 && !self.change_wall && !self.change_chest {
                self.change_wall = true;
                self.change_chest = true;
            }
            if change_list[i] == 1 && !self.change_wall {
                self.change_wall = true;
            }
            if change_list[i] == 2 && !self.change_chest {
                self.change_chest = true;
            }
        }
    }
    #[allow(unused)]
    pub fn get_map_rows(&self) -> &[[i32; 10]; 15] {
        &self.map_array
    }

    pub fn get_map_columns(&self) -> &[i32; 10] {
        &self.map_array[0]
    }

    #[allow(unused)]
    pub fn get_wall_list(&self) -> &Vec<StillImage> {
        &self.wall_list
    }
    #[allow(unused)]
    pub fn get_chest_list(&self) -> &Vec<StillImage> {
        &self.chest_list
    }
}
