/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/
/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/

use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::item::Item;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use macroquad::prelude::*;

pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager) -> String {

    let diamond_armour = Item::new(
        tm.get_preload("assets/slime.png").unwrap(), // Preload
        "assets/slime.png".to_string(), // Image path
        "Diamond Armour".to_string(), // Name
        "Armour made of the toughest gemstone around!".to_string(), // Description
        "armour".to_string(), // Type
        0, // Melee
        0, // Ranged
        0.9, // Move speed mult
        0.0, // Cooldown mult
        50, // Health
        20, // Armour
    )
    .await;

    let time_sword = Item::new(
        tm.get_preload("assets/slime.png").unwrap(), // Preload
        "assets/slime.png".to_string(), // Image path
        "Time Sword".to_string(), // Name
        "A weapon made to slice through time itself".to_string(), // Description
        "armour".to_string(), // Type
        25, // Melee
        0, // Ranged
        1.0, // Move speed mult
        -0.1, // Cooldown mult
        0, // Health
        0, // Armour
    )
    .await;

    let shop_stock: Vec<Item> = vec![diamond_armour, time_sword];
    let mut item_type: Vec<String> = vec![];
    let mut item_descs: Vec<String> = vec![];
    let mut shop_names: Vec<String> = vec![];
    for i in 0..shop_stock.len() {
        item_type.push(shop_stock[i].get_itemtype().clone());
        item_descs.push(shop_stock[i].get_itemdescription().clone());
        shop_names.push(shop_stock[i].get_itemtitle().clone());
    }
    let mut shop_view = ListView::new(&shop_names, 500.0, 500.0, 50);
    let mut lbl_desc = Label::new("", 250.0, 500.0, 50);
    let item_atk_def = vec![99, 25, 15, 10, 0];
    let mut lbl_atk_def = Label::new("", 250.0, 550.0, 50);
    let item_spd_res = vec![0.1, 0.2, 0.25, 0.05, 0.0];
    let mut lbl_spd_res = Label::new("", 250.0, 600.0, 50);
    let item_price = vec![1000, 500, 500, 750, 9999];
    let mut lbl_price = Label::new("", 250.0, 450.0, 50);
    let btn_buy = TextButton::new(
        100.0,
        200.0,
        200.0,
        60.0,
        "Buy",
        BLUE,
        GREEN,
        30
    );
    let mut lbl_error = Label::new("", 250.0, 650.0, 50);

    let mut selected_record: String = "None".to_string();

    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);

        if shop_view.selected_item().is_some() && &selected_record != shop_view.selected_item().unwrap() {
            selected_record = shop_view.selected_item().unwrap().clone();
            if selected_record == shop_names[0] {
                //lbl_desc.set_text(item_descs[0]);
                lbl_atk_def.set_text(item_atk_def[0].to_string());
                lbl_spd_res.set_text(item_spd_res[0].to_string());
                lbl_price.set_text(item_price[0].to_string());
            } else if selected_record == shop_names[1] {
                //lbl_desc.set_text(item_descs[1]);
                lbl_atk_def.set_text(item_atk_def[1].to_string());
                lbl_spd_res.set_text(item_spd_res[1].to_string());
                lbl_price.set_text(item_price[1].to_string());
            } else if selected_record == shop_names[2] {
                //lbl_desc.set_text(item_descs[2]);
                lbl_atk_def.set_text(item_atk_def[2].to_string());
                lbl_spd_res.set_text(item_spd_res[2].to_string());
                lbl_price.set_text(item_price[2].to_string());
            } else if selected_record == shop_names[3] {
                //lbl_desc.set_text(item_descs[3]);
                lbl_atk_def.set_text(item_atk_def[3].to_string());
                lbl_spd_res.set_text(item_spd_res[3].to_string());
                lbl_price.set_text(item_price[3].to_string());
            } else if selected_record == shop_names[4] {
                //lbl_desc.set_text(item_descs[4]);
                lbl_atk_def.set_text(item_atk_def[4].to_string());
                lbl_spd_res.set_text(item_spd_res[4].to_string());
                lbl_price.set_text(item_price[4].to_string());
            }
        }

        if btn_buy.click() && shop_view.selected_item().is_some() {
            for i in 0..shop_stock.len() {
        }
        } else if btn_buy.click() && shop_view.selected_item().is_none() {
            lbl_error.set_text("Please select an item first");
        }

        shop_view.draw();
        next_frame().await;
    }
}
