#[cfg(test)]
mod tests {
    use crate::gildedrose::brie_item::BrieItem;
    use crate::gildedrose::conjured_item::ConjuredItem;
    use crate::gildedrose::item::InventoryItem;
    use crate::gildedrose::pass_item::PassItem;
    use crate::{GildedRose, Item};

    #[test]
    pub fn test_item_name_stays_same() {
        let items = vec![Item::new("foo", 0, 0)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!("foo", rose.items[0].name);
    }

    #[test]
    pub fn test_quality_degrades_fast_after_sell_date_is_past() {
        let items = vec![Item::new("foo", 0, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(18, rose.items[0].quality);
    }

    #[test]
    pub fn test_quality_degrades() {
        let items = vec![Item::new("foo", 10, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(19, rose.items[0].quality);
    }

    #[test]
    pub fn test_quality_is_not_negative() {
        let items = vec![Item::new("Conjured foo", 0, 1)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert!(0 <= rose.items[0].quality);
    }

    #[test]
    pub fn test_aged_brie_quality_increases() {
        let items = vec![Item::new("Aged Brie", 20, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(11, rose.items[0].quality);
    }

    #[test]
    pub fn test_aged_brie_quality_increases_when_sell_in_under_0() {
        let items = vec![Item::new("Aged Brie", 0, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(11, rose.items[0].quality);
    }

    #[test]
    pub fn test_quality_not_more_than_50() {
        let items = vec![Item::new("Aged Brie", 20, 50)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert!(50 >= rose.items[0].quality);
    }

    #[test]
    pub fn test_sulfuras_not_decrease_sell_in() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 20, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(20, rose.items[0].sell_in);
    }

    #[test]
    pub fn test_sulfuras_not_decrease_quality() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 20, 80)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(80, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_pass_increase_quality() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 20, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(11, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_pass_increase_quality_less_than_11_days() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(12, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_pass_increase_quality_less_than_6_days() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(13, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_pass_zero_quality_after_concert() {
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn test_conjured_items_loose_quality_twice_as_fast() {
        let items = vec![Item::new("Conjured Cake", 10, 10)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();

        assert_eq!(8, rose.items[0].quality);
    }

    #[test]
    pub fn test_conjured_item_struct_quality_update() {
        let mut items = ConjuredItem::new("Conjured Cake", 10, 20);
        items.update();
        let quality = items.item.quality;

        assert_eq!(18, quality);
    }
    #[test]
    pub fn test_conjured_item_struct_sell_in_update() {
        let mut items = ConjuredItem::new("Conjured Cake", 10, 20);
        items.update();
        let sell_in = items.item.sell_in;

        assert_eq!(9, sell_in);
    }

    #[test]
    pub fn test_brie_item_struct_quality_update() {
        let mut items = BrieItem::new("Aged Brie", 10, 20);
        items.update();
        let quality = items.item.quality;

        assert_eq!(21, quality);
    }

    #[test]
    pub fn test_backstage_pass_struct_sell_in_update() {
        let mut items = PassItem::new("Backstage Pass",9,20);
        items.update();
        let quality = items.item.quality;

        assert_eq!(22, quality);
    }

}