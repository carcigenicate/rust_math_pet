use std::collections::HashMap;
use crate::game_state::LiveGameState;

pub struct ShopItem {
    pub name: String,
    pub price: u32,
    pub effect: fn(&mut LiveGameState) -> (),
}

impl ShopItem {
    pub fn new(name: impl Into<String>, price: u32, effect: fn(&mut LiveGameState) -> ()) -> Self {
        return Self {
            name: name.into(),
            price: price,
            effect: effect,
        };
    }

    pub fn buy_and_apply(&self, game_state: &mut LiveGameState) -> bool {
        return if game_state.money >= self.price {
            game_state.money -= self.price;
            (self.effect)(game_state);
            true
        } else {
            false
        }
    }
}

pub fn get_shop_inventory() -> Vec<ShopItem> {
    return vec![
        ShopItem::new("10 HP", 10, |s| s.pet.heal(10.0)),
        ShopItem::new("10 SAT", 20, |s| s.pet.feed(20.0)),
        ShopItem::new("Increase Max HP (+2)", 10, |s| s.pet.health_max += 2.0),
        ShopItem::new("Increase Max SAT (+2)", 10, |s| s.pet.satiation_max += 2.0),
    ];
}

pub fn format_inventory(inventory: &Vec<ShopItem>) -> String {
    let mut shop_str = String::new();
    let mut i = 1;
    for item in inventory {
        let formatted_item = format!("{i}. Name: {}, Price: {}\n", item.name, item.price);
        shop_str.push_str(formatted_item.as_str());
        i += 1;
    }
    return shop_str;
}