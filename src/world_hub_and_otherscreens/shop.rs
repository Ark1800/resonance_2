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

use crate::modules::scale::use_virtual_resolution;
use macroquad::prelude::*;
use crate::modules::map::Map;
use crate::modules::preload_image::TextureManager;
use crate::modules::listview::ListView;
use crate::modules::text_button::TextButton;
use crate::modules::label::Label;
use crate::modules::still_image::StillImage;


pub async fn run(virtual_width: f32, virtual_height: f32, player: &mut crate::modules::player::Player, tm: &TextureManager) -> String {

    let mut shop_stock: Vec<&str> = vec!["Diamond Armour", "Time sword", "Future Bow", "Chainmail of Hermes", "Bike"];
    let mut shop_view = ListView::new(&shop_stock, 500.0, 500.0, 50);
    let mut item_type = vec!["Armour", "Weapon", "Weapon", "Armour", "Vehicle"];
    let mut item_descs = vec!["Diamond armour desc", "Time sword desc", "Future bow desc", "Chainmail of Hermes desc", "Bike desc"];
    let mut lbl_desc = Label::new("", 250.0, 500.0, 50);
    let mut item_atk_def = vec![99, 25, 15, 10, 0];
    let mut lbl_atk_def = Label::new("", 250.0, 550.0, 50);
    let mut item_spd_res = vec![0.1, 0.2, 0.25, 0.05, 0.0];
    let mut lbl_spd_res = Label::new("", 250.0, 600.0, 50);
    let mut item_price = vec![1000, 500, 500, 750, 9999];
    let mut lbl_price = Label::new("", 250.0, 450.0, 50);
    
    let mut selected_record: String = "None".to_string();

    loop {
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(RED);

        if shop_view.selected_item().is_some() && &selected_record != shop_view.selected_item().unwrap() {
            selected_record = shop_view.selected_item().unwrap().clone();
            if selected_record == "Diamond Armour" {
                lbl_desc.set_text(item_descs[0]);
                lbl_atk_def.set_text(item_atk_def[0].to_string());
                lbl_spd_res.set_text(item_spd_res[0].to_string());
                lbl_price.set_text(item_price[0].to_string());
            }
        }

        shop_view.draw();
        next_frame().await;
    }
}