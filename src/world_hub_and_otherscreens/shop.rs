/*
By: Andrew Campbell, Dradon L, Leo Allison
Date: 2026-04-14
Program Details:
*/


use crate::modules::grid::draw_grid;
use crate::modules::item::Item;
use crate::modules::label::Label;
use crate::modules::listview::ListView;
use crate::modules::preload_image::TextureManager;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::still_image::StillImage;
use crate::modules::text_button::TextButton;
use crate::modules::database::{DatabaseClient, DatabaseTable};
use macroquad::prelude::*;

pub async fn run(
    virtual_width: f32,
    virtual_height: f32,
    player: &mut crate::modules::player::Player,
    tm: &TextureManager,
    _pause: &mut bool,
    _musicdiscfunctions: &mut crate::modules::musicdisc::Musicdisc,
) -> String {
    player.set_currentscreen("shop".to_string());
    let diamond_armour = Item::new(
        tm.get_preload("assets/item_files/armour/diamond_armor.png").unwrap(), // Preload
        "assets/item_files/armour/diamond_armor.png".to_string(),              // Image path
        "Diamond Armour".to_string(),                                          // Name
        "Armour made of the toughest gemstone around!".to_string(),            // Description
        "armour".to_string(),                                                  // Type
        0,                                                                     // Melee
        0,                                                                     // Ranged
        0.9,                                                                   // Move speed mult
        0.0,                                                                   // Cooldown mult
        50,                                                                    // Health
        20,                                                                    // Armour
    )
    .await;

    let time_sword = Item::new(
        tm.get_preload("assets/item_files/weapons/time_sword.png").unwrap(), // Preload
        "assets/item_files/weapons/time_sword.png".to_string(),              // Image path
        "Time Sword".to_string(),                                            // Name
        "A weapon made to slice through time itself".to_string(),            // Description
        "attack".to_string(),                                                // Type
        25,                                                                  // Melee
        0,                                                                   // Ranged
        1.0,                                                                 // Move speed mult
        -0.1,                                                                // Cooldown mult
        0,                                                                   // Health
        0,                                                                   // Armour
    )
    .await;

    let future_bow = Item::new(
        tm.get_preload("assets/item_files/weapons/future_bow.png").unwrap(), // Preload
        "assets/item_files/weapons/future_bow.png".to_string(),              // Image path
        "Future Bow".to_string(),                                            // Name
        "A bow that, due to its enchantments, can fire its arrows into the future".to_string(), // Description
        "attack".to_string(),                                                // Type
        0,                                                                   // Melee
        20,                                                                  // Ranged
        1.15,                                                                // Move speed mult
        -0.15,                                                               // Cooldown mult
        0,                                                                   // Health
        0,                                                                   // Armour
    )
    .await;

    let hermes_armour = Item::new(
        tm.get_preload("assets/item_files/armour/hermes_armor.png").unwrap(), // Preload
        "assets/item_files/armour/hermes_armor.png".to_string(),              // Image path
        "Chainmail of Hermes".to_string(),                                    // Name
        "Armour blessed by the messenger god, Hermes. This armour provides the wearer a light feeling, and increases their movement speed"
            .to_string(), // Description
        "armour".to_string(),                                                 // Type
        0,                                                                    // Melee
        0,                                                                    // Ranged
        1.5,                                                                  // Move speed mult
        -0.25,                                                                // Cooldown mult
        10,                                                                   // Health
        5,                                                                    // Armour
    )
    .await;
    let mut shop_stock: Vec<Item> = vec![diamond_armour, time_sword, future_bow, hermes_armour];
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
    let item_price = vec![25, 500, 500, 750, 9999];
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
        150.0,   // x position
        0.0,   // y position
        false, // Enable stretching
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
        use_virtual_resolution(virtual_width, virtual_height);
        clear_background(WHITE);
        img_back.draw();
        draw_grid(50.0, BLACK);

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
