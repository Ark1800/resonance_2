use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};
use crate::modules::still_image::StillImage;
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
    imgpath: (Texture2D, Option<Vec<u8>>, String),
    backinblack_starttime: f64,
    disc_elements: Vec<StillImage>,
}

impl Musicdisc {
    pub async fn new (imgpath: (Texture2D, Option<Vec<u8>>, String), musicpath: &str) -> Self {
        Musicdisc {
            imgpath,
            disc_elements: Musicdisc::create_disc_elements().await,
            backinblack_starttime: 0.0,
            //musicpath
        }

    }

    pub async fn test_musicdisc(&self) {
      //  println!("Musicdisc: {}, {}", self.imgpath.2, self.musicpath);
       // let sound_effect = load_sound(self.musicpath).await.unwrap();
      //  play_sound(&sound_effect, PlaySoundParams::default());
    }


    pub fn get_musicdisc_time(&mut self) {
        self.backinblack_starttime = get_time(); 
    }

    pub fn handle_musicdisccooldowns(&mut self, activedisc: String) -> String {
        match activedisc.as_str() {
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
            },
            _ => {}
        }
        activedisc
    }

    pub async fn create_disc_elements() -> Vec<StillImage> {
        let mut bib_img1 = StillImage::new("", 100.0, 100.0, 825.0, 50.0, true, 1.0).await;
        vec![bib_img1]
    }
}

