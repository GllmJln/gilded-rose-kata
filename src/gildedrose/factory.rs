use crate::gildedrose::conjured_item::ConjuredItem;
use crate::gildedrose::generic_item::GenericItem;
use crate::gildedrose::pass_item::PassItem;
use crate::gildedrose::brie_item::BrieItem;
use crate::gildedrose::legendary_item::LegendaryItem;
use crate::gildedrose::item::{InventoryItem, is_legendary, Item};

pub fn factory(item: &mut Item) -> Item{
    let name = item.name.to_string();
    if name.contains("Conjured") {
        ConjuredItem::new(name,item.sell_in,item.quality).update()
    } else if is_legendary(&name){
        LegendaryItem::new(name,item.sell_in,item.quality).update()
    } else if name == "Aged Brie" {
        BrieItem::new(name,item.sell_in,item.quality).update()
    } else if name.contains("Backstage passes") {
        PassItem::new(name,item.sell_in,item.quality).update()
    } else {
        GenericItem::new(name,item.sell_in,item.quality).update()
    }

}