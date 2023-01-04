pub(crate) mod conjured_item;
mod generic_item;
mod legendary_item;
pub(crate) mod brie_item;
pub(crate) mod pass_item;
mod factory;
pub(crate) mod item;

use factory::factory;

use crate::gildedrose::item::Item;


pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            item.sell_in = factory(item).sell_in;
            item.quality = factory(item).quality;
        }
    }

}



