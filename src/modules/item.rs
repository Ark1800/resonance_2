use macroquad::prelude::*;

pub struct Item {
    imgpath: String,
    title: String,
    description: String,
    mledmg: i32,
    rngdmg: i32,
    cooldownmult: f32,
    movespeedmult: f32,
    hpchng: i32,
    armor: i32,
}

impl Item {
    pub async fn new (imgpath: String, title: String, description: String, mledmg: i32, rngdmg: i32, cooldownmult: f32, movespeedmult: f32, hpchng: i32, armor: i32) -> Self {
        Item {
            imgpath,
            title,
            description,
            mledmg,
            rngdmg,
            cooldownmult,
            movespeedmult,
            hpchng,
            armor,
        }
    }
}

pub fn get_allitemstats(item: &Item) -> (String, String, i32, i32, f32, f32, i32, i32) {
    (item.title.clone(), item.description.clone(), item.mledmg, item.rngdmg, item.cooldownmult, item.movespeedmult, item.hpchng, item.armor)
}

pub fn get_itemimgpath(item: &Item) -> String {
    item.imgpath.clone()
}

pub fn get_itemtitle(item: &Item) -> String {
    item.title.clone()
}

pub fn get_itemdescription(item: &Item) -> String {
    item.description.clone()
}

pub fn get_itemmledmg(item: &Item) -> i32 {
    item.mledmg
}

pub fn get_itemrngdmg(item: &Item) -> i32 {
    item.rngdmg
}

pub fn get_itemcooldownmult(item: &Item) -> f32 {
    item.cooldownmult
}

pub fn get_itemmovespeedmult(item: &Item) -> f32 {
    item.movespeedmult
}

pub fn get_itemhpchng(item: &Item) -> i32 {
    item.hpchng
}

pub fn get_itemarmor(item: &Item) -> i32 { 
    item.armor
}