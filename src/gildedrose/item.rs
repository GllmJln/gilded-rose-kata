use std::fmt;
use std::fmt::Display;


pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

pub trait InventoryItem {
    fn update(&mut self) -> Item;
    fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Self;
}


impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub fn is_legendary(item: &String) -> bool{
    let legendary_items = [
        "Sulfuras, Hand of Ragnaros",
    ];
    return legendary_items.contains(&&**item)
}

pub fn increase_quality(item: &mut Item, qty: i32) {
    if item.quality + qty <= 50 && item.quality + qty >= 0 {
        item.quality += qty;
    }
}

pub fn is_expired(date: i32) -> bool{
    return date <= 0
}