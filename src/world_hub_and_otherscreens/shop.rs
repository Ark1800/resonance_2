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
        "attack".to_string(), // Type
        25, // Melee
        0, // Ranged
        1.0, // Move speed mult
        -0.1, // Cooldown mult
        0, // Health
        0, // Armour
    )
    .await;

    let future_bow = Item::new(
        tm.get_preload("assets/slime.png").unwrap(), // Preload
        "assets/slime.png".to_string(), // Image path
        "Future Bow".to_string(), // Name
        "A bow that, due to its enchantments, can fire its arrows into the future".to_string(), // Description
        "attack".to_string(), // Type
        0, // Melee
        20, // Ranged
        1.15, // Move speed mult
        -0.15, // Cooldown mult
        0, // Health
        0, // Armour
    )
    .await;

    let mut shop_stock: Vec<Item> = vec![diamond_armour, time_sword, future_bow];
    let mut shop_names: Vec<String> = vec![];
    for i in 0..shop_stock.len() {
        shop_names.push(shop_stock[i].get_itemtitle().clone());
    }
    let mut shop_view = ListView::new(&shop_names, 500.0, 500.0, 50);
    let mut lbl_desc = Label::new("", 250.0, 500.0, 50);
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
            let mut selected_item = 0;
            for i in 0..shop_stock.len() {
                if &selected_record == &shop_stock[i].get_itemtitle() {
                    selected_item = i;
                    break;
                }
            }

            if shop_stock[selected_item].get_itemtype() == "armour" {
                lbl_desc.set_text(format!("Name: {}\nItem type: {}\nDescription: {}\nArmour: {}\nHealth: {}\nSpeed: {}", shop_stock[selected_item].get_itemtitle(), shop_stock[selected_item].get_itemtype(), shop_stock[selected_item].get_itemdescription(), shop_stock[selected_item].get_itemarmor(), shop_stock[selected_item].get_itemhpchng(), shop_stock[selected_item].get_itemmovespeedmult()));
            } 
            lbl_price.set_text(&item_price[selected_item].to_string());
        }

        if btn_buy.click() && shop_view.selected_item().is_some() {
            let mut item_wanted = 0;
            for i in 0..shop_stock.len() {
                if &selected_record == &shop_stock[i].get_itemtitle() {
                    item_wanted = i;
                    break;
                }
            }

            if player.getcoins() >= item_price[item_wanted] {
                player.addcoins(-item_price[item_wanted]);
                player.add_inventory_item(shop_stock[item_wanted].clone());
                shop_stock.remove(item_wanted);
                shop_names.remove(item_wanted);
                shop_view = ListView::new(&shop_names, 500.0, 500.0, 50);
            }
        } else if btn_buy.click() && shop_view.selected_item().is_none() {
            lbl_error.set_text("Please select an item first");
        }

        shop_view.draw();
        next_frame().await;
    }
}
