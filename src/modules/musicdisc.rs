use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, PlaySoundParams};

#[derive(Clone)]
pub struct Musicdisc {
    //musicpath: &str,
    imgpath: (Texture2D, Option<Vec<u8>>, String),
}

impl Musicdisc {
    pub async fn new (imgpath: (Texture2D, Option<Vec<u8>>, String), musicpath: &str) -> Self {
        Musicdisc {
            imgpath,
            //musicpath
        }
    }

    pub async fn test_musicdisc(&self) {
      //  println!("Musicdisc: {}, {}", self.imgpath.2, self.musicpath);
       // let sound_effect = load_sound(self.musicpath).await.unwrap();
      //  play_sound(&sound_effect, PlaySoundParams::default());
    }
}

