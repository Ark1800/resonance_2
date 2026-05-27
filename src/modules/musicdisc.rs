use macroquad::prelude::*;
use macroquad::audio::{play_sound, PlaySoundParams, Sound};
use crate::modules::still_image::StillImage;
use crate::modules::preload_image::TextureManager;
/* 
run through player 

outside the loop...
musicdisc::get_musicdisc_time();

in loop...
use musicdisc::handle_musicdisccooldowns; in each loop

*/

#[derive(Clone)]
pub struct Musicdisc {
    //musicpath: &str,
    musicpaths: Vec<String>,
    sounds: Vec<Sound>,
    backinblack_starttime: f64,
    disc_elements: Vec<StillImage>,
}

impl Musicdisc {
    pub async fn new(tm: &TextureManager) -> Self {
        let musicpaths = vec!["assets/musicdisc_files/music/backinblack.ogg".to_string()];
        tm.preload_sound(musicpaths[0].as_str()).await;
        let backinblack_sound = tm.get_preloaded_sound(musicpaths[0].as_str()).unwrap();
        Musicdisc {
            musicpaths,
            sounds: vec![backinblack_sound],
            disc_elements: Musicdisc::create_disc_elements(&tm).await,
            backinblack_starttime: 0.0,
        }

    }


    pub async fn test_musicdisc(&self) {
        println!("Playing music disc: {}", self.musicpaths[0]);
        play_sound(&self.sounds[0], PlaySoundParams {looped: false, volume: 1.0 });
    }


    pub fn get_musicdisc_times(&mut self) {
        self.backinblack_starttime = get_time(); 
    }

    pub fn handle_musicdisccooldowns(&mut self, activedisc: String) -> String {
        let mut discmatch = activedisc.as_str();
        match discmatch {
            "backinblack" => {
                let time = get_time() - self.backinblack_starttime;
                if time >= 0.0 && time <= 2.0 {
                    println!("draw labels")
                }
                else if time > 2.0 && time <= 4.0 {
                    println!("hide labels")
                }
                else if time > 4.0 && time <= 6.0 {
                    println!("draw labels")
                }
                else if time > 6.0 && time <= 8.0 {
                    println!("hide labels")
                }
                else if time > 8.0 && time <= 10.0 {
                    println!("draw labels")
                }
                else if time > 10.0 && time <= 12.0 {
                    println!("hide labels")
                }
                else if time > 12.0 && time <= 15.0 {
                    println!("draw labels")
                }
                else if time > 15.0 {
                    println!("hide labels");
                    discmatch = "";
                }
            },
            _ => {}
        }
        discmatch.to_string()
    }

    pub async fn create_disc_elements(tm: &TextureManager) -> Vec<StillImage> {
        let mut bib_img1 = StillImage::new("", 200.0, 200.0, 20.0, 50.0, true, 1.0).await; 
        bib_img1.set_preload(tm.get_preload("assets/musicdisc_files/effectimages/bibimg.png").unwrap());

        vec![bib_img1]
    }
}

