use crate::gildedrose::item::{increase_quality, InventoryItem, is_expired, Item};


pub struct GenericItem {
    pub item: Item
}

impl InventoryItem for GenericItem {
    fn update(& mut self) -> Item{
        let mut base_rate = 1;
        if is_expired(self.item.sell_in) {
            base_rate *=2
        }
        increase_quality(& mut self.item,-base_rate);
        self.item.sell_in -= 1;
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