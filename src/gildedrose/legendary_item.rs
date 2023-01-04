use crate::gildedrose::item::{InventoryItem, Item};


pub struct LegendaryItem {
    pub item: Item
}

impl InventoryItem for LegendaryItem {
    fn update(&mut self) -> Item{
        self.item.quality = 80;
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