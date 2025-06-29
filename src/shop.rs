use crate::game_state::LiveGameState;

pub struct ShopItem {
    pub name: String,
    pub price: f64,
    pub effect: fn(&mut LiveGameState) -> (),
}

impl ShopItem {
    pub fn new(name: impl Into<String>, price: f64, effect: fn(&mut LiveGameState) -> ()) -> Self {
        Self {
            name: name.into(),
            price: price,
            effect: effect,
        }
    }

    pub fn buy_and_apply(&self, game_state: &mut LiveGameState) -> bool {
        if game_state.pet.satiation >= self.price {
            game_state.pet.starve(self.price);
            (self.effect)(game_state);
            true
        } else {
            false
        }
    }
}

pub fn get_shop_inventory() -> Vec<ShopItem> {
    vec![
        ShopItem::new("Heal", 50.0, |s| s.pet.heal(20.0)),
        // ShopItem::new("Increase Max HP", 10.0, |s| s.pet.health_max += 2.0),
        // ShopItem::new("Increase Max SAT", 10.0, |s| s.pet.satiation_max += 2.0),
        ShopItem::new("Increase Time to Starve", 10.0, |s| s.tweaks.starve_per_tick *= 0.995),
        ShopItem::new("Increase Risk", 30.0, |s| {
            s.tweaks.food_per_correct += 0.2;
            s.tweaks.damage_per_wrong += 0.5;
        }),
    ]
}

pub fn format_inventory(inventory: &Vec<ShopItem>) -> String {
    let mut shop_str = String::new();
    let mut i = 1;
    for item in inventory {
        let formatted_item = format!("{i}. Name: {}, Price: {}\n", item.name, item.price);
        shop_str.push_str(formatted_item.as_str());
        i += 1;
    }

    shop_str
}