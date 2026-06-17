/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details: shop scene, leads to town when leaving
*/

use crate::modules::item::Item;
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
//use crate::modules::database::{DatabaseClient, DatabaseTable};
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    _pause: &mut bool,
    last_scene: &mut String,
    _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
) -> String {
    player.set_currentscreen("shop".to_string());
    let iron_armour = Item::new(
        tm.get_preload("assets/item_files/armour/iron_armor.png").unwrap(), // Preload
        "assets/item_files/armour/iron_armor.png".to_string(),              // Image path
        "Iron Armour".to_string(),                                          // Name
        "Armour made of the toughest iron around!".to_string(),            // Description
        "bodyarmor".to_string(),                                               // Type
        0,                                                                     // Melee
        0,                                                                     // Ranged
        0.0 ,                                                                   // Move speed mult
        0.0,                                                                   // Cooldown mult
        0,                                                                    // Health
        8,                                                                    // Armour
    )
    .await;
    let swift_sword = Item::new(
        tm.get_preload("assets/item_files/weapons/swift_sword.png").unwrap(), // Preload
        "assets/item_files/weapons/swift_sword.png".to_string(),              // Image path
        "Swift Sword".to_string(),                                            // Name
        "A weapon made to slice swfitly through enemies increasing damage".to_string(),            // Description
        "melee".to_string(),                                                // Type
        9,                                                                  // Melee
        0,                                                                   // Ranged
        0.0,                                                                 // Move speed mult
        0.0,                                                                // Cooldown mult
        0,                                                                   // Health
        0,                                                                   // Armour
    )
    .await;
    let crossbow = Item::new(
        tm.get_preload("assets/item_files/weapons/crossbow.png").unwrap(), // Preload
        "assets/item_files/weapons/crossbow.png".to_string(),              // Image path
        "Crossbow".to_string(),                                            // Name
        "A bow that, does moderate damage and increases cooldowns slightly".to_string(), // Description
        "ranged".to_string(),                                                // Type
        0,                                                                   // Melee
        15,                                                                  // Ranged
        1.2,                                                                // Move speed mult
        0.0,                                                               // Cooldown mult
        0,                                                                   // Health
        0,                                                                   // Armour
    )
    .await;
    let swift_sneakers = Item::new(
        tm.get_preload("assets/item_files/armour/swift_sneakers.png").unwrap(),
        "assets/item_files/armour/swift_sneakers.png".to_string(),
        "Swift Sneakers".to_string(),
        "A pair of boots with remnants of energy from the blue blur, increases movement speed and armor".to_string(),
        "boots".to_string(),
        0,
        0,
        0.0,
        2.0,
        0,
        3,
    )
    .await;

    let mut shop_stock: Vec<Item> = vec![];
    if !player.get_inventory_titles().contains(&"Iron Armour".to_string()) {
        shop_stock.push(iron_armour);
    }
    if !player.get_inventory_titles().contains(&"Swift Sword".to_string()) {
        shop_stock.push(swift_sword);
    }
    if !player.get_inventory_titles().contains(&"Crossbow".to_string()) {
        shop_stock.push(crossbow);
    }
    if !player.get_inventory_titles().contains(&"Swift Sneakers".to_string()) {
        shop_stock.push(swift_sneakers);
    }
    let mut shop_names: Vec<String> = vec![];
    for i in 0..shop_stock.len() {
        shop_names.push(shop_stock[i].get_itemtitle().clone());
    }
    let mut lbl_coins = Label::new(format!("Coins: {}", player.getcoins()), 50.0, 50.0, 30);
    lbl_coins.with_colors(WHITE, None);
    let mut shop_view = ListView::new(&shop_names, 650.0, 50.0, 40);
    shop_view.with_colors(WHITE, None, Some(LIGHTGRAY));
    let mut lbl_desc = Label::new("", 50.0, 300.0, 30);
    lbl_desc.with_colors(WHITE, None);
    let item_price = vec![50, 80, 80, 25];
    let mut lbl_price = Label::new("", 250.0, 450.0, 40);
    lbl_price.with_colors(WHITE, None);
    let mut btn_buy = TextButton::new(100.0, 200.0, 200.0, 60.0, "Buy", BLUE, GREEN, 30);
    btn_buy.with_text_color(WHITE);
    let mut btn_exit = TextButton::new(800.0, 700.0, 200.0, 60.0, "Exit", BLUE, GREEN, 30);
    btn_exit.with_text_color(WHITE);
    let mut lbl_error = Label::new("Click one of the options\nabove to see its stats!", 650.0, 300.0, 30);
    lbl_error.with_colors(WHITE, None);
    let mut item_img = StillImage::new(
        "", 160.0, // width
        160.0, // height
        150.0, // x position
        0.0,   // y position
        true, // Enable stretching
        1.0,   // Normal zoom (100%)
    )
    .await;
    let mut img_back = StillImage::new(
        "",
        virtual_width,  // width
        virtual_height, // height
        0.0,            // x position
        0.0,            // y position
        false,          // Enable stretching
        1.0,            // Normal zoom (100%)
    )
    .await;
    img_back.set_preload(tm.get_preload("assets/map_files/shop.png").unwrap());

    let mut selected_record: String = "None".to_string();

    loop {
        if last_scene == "title_screen" {
            player.show_player_messagebox();
        }
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(WHITE);
        img_back.draw();

        if shop_view.selected_item().is_some() && &selected_record != shop_view.selected_item().unwrap() {
            lbl_error.set_text("");
            selected_record = shop_view.selected_item().unwrap().clone();
            let mut selected_item = 0;
            for i in 0..shop_stock.len() {
                if &selected_record == &shop_stock[i].get_itemtitle() {
                    selected_item = i;
                    break;
                }
            }

            if shop_stock[selected_item].get_itemtype() == "armour" {
                lbl_desc.set_text(format!(
                    "Name: {}\nItem type: {}\nDescription: {}\nArmour: {}\nHealth: {}\nSpeed: {}",
                    shop_stock[selected_item].get_itemtitle(),
                    shop_stock[selected_item].get_itemtype(),
                    shop_stock[selected_item].get_itemdescription(),
                    shop_stock[selected_item].get_itemarmor(),
                    shop_stock[selected_item].get_itemhpchng(),
                    shop_stock[selected_item].get_itemmovespeedmult()
                ));
            } else if shop_stock[selected_item].get_itemtype() == "attack" {
                let dmg = shop_stock[selected_item].get_itemmledmg() + shop_stock[selected_item].get_itemrngdmg();
                lbl_desc.set_text(format!(
                    "Name: {}\nItem type: {}\nDescription: {}\nDamage: {}\nSpeed: {}\nCooldown: {}",
                    shop_stock[selected_item].get_itemtitle(),
                    shop_stock[selected_item].get_itemtype(),
                    shop_stock[selected_item].get_itemdescription(),
                    dmg,
                    shop_stock[selected_item].get_itemmovespeedmult(),
                    shop_stock[selected_item].get_itemcooldownmult()
                ));
            }
            lbl_price.set_text(&item_price[selected_item].to_string());
            item_img.set_preload(shop_stock[selected_item].get_itemimgpath());
        }

        if shop_view.selected_item().is_some() {
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
                    shop_view.clear();
                    lbl_coins.set_text(format!("Coins: {}", player.getcoins()));
                    for i in 0..shop_names.len() {
                        shop_view.add_item(&shop_names[i]);
                    }
                    selected_record = "None".to_string();
                }
            } else if btn_buy.click() && shop_view.selected_item().is_none() {
                lbl_error.set_text("Please select an item first");
            }
        }

        if btn_exit.click() {
            *last_scene = "Town".to_string();
            return "town".to_string();
        }

        lbl_desc.draw();
        lbl_price.draw();
        lbl_coins.draw();
        lbl_error.draw();
        shop_view.draw();
        item_img.draw();
        next_frame().await;
    }
}
