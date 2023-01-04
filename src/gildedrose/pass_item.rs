use crate::gildedrose::item::{increase_quality, InventoryItem, is_expired, Item};

pub struct PassItem {
    pub item: Item
}

impl InventoryItem for PassItem {
    fn update(&mut self) -> Item{
        if is_expired(self.item.sell_in) {
            self.item.quality = 0;
        } else if self.item.sell_in < 6 {
            increase_quality(& mut self.item,3);
        } else if self.item.sell_in < 11 {
            increase_quality(&mut self.item,2);
        } else {
            increase_quality(&mut self.item, 1);
        }
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
