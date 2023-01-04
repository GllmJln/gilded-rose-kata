use crate::gildedrose::item::{increase_quality, InventoryItem, Item};


pub struct BrieItem {
    pub item: Item
}

impl InventoryItem for BrieItem {
    fn update(&mut self) -> Item{
        self.item.sell_in -= 1;
        let base_rate = 1;
        increase_quality(& mut self.item, base_rate);
        Item{
            name: self.item.name.to_string(),
            sell_in: self.item.sell_in,
            quality: self.item.quality
        }
    }

    fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Self {
        Self {item: Item::new(name,sell_in,quality)}
    }
}