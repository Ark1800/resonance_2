use macroquad::prelude::*;
#[derive(Clone)]
pub struct Item {
    imgpath: (Texture2D, Option<Vec<u8>>, String),
    assetpath: String,
    title: String,
    description: String,
    itemtype: String,
    mledmg: i32,
    rngdmg: i32,
    hpchng: i32,
    armor: i32,
    cooldownmult: f32,
    movespeedmult: f32,
}

impl Item {
    pub async fn new (imgpath: (Texture2D, Option<Vec<u8>>, String), assetpath: String, title: String, description: String, itemtype: String, mledmg: i32, rngdmg: i32, cooldownmult: f32, movespeedmult: f32, hpchng: i32, armor: i32) -> Self {
        Item {
            imgpath,
            assetpath,
            title,
            description,
            itemtype,
            mledmg,
            rngdmg,
            hpchng,
            armor,
            cooldownmult,
            movespeedmult,
        }
    }

    pub fn get_itemassetpath(&self) -> String {
        self.assetpath.clone()
    }

    pub fn get_itemimgpath(&self) -> (Texture2D, Option<Vec<u8>>, String) {
        self.imgpath.clone()
    }

    pub fn get_itemtitle(&self) -> String {
        self.title.clone()
    }

    pub fn get_itemdescription(&self) -> String {
        self.description.clone()
    }

    pub fn get_itemtype(&self) -> String {
        self.itemtype.clone()
    }

    pub fn get_itemmledmg(&self) -> i32 {
        self.mledmg
    }

    pub fn get_itemrngdmg(&self) -> i32 {
        self.rngdmg
    }

    pub fn get_itemcooldownmult(&self) -> f32 {
        self.cooldownmult
    }

    pub fn get_itemmovespeedmult(&self) -> f32 {
        self.movespeedmult
    }

    pub fn get_itemhpchng(&self) -> i32 {
        self.hpchng
    }

    pub fn get_itemarmor(&self) -> i32 {
        self.armor
    }
}