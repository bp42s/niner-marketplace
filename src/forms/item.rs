#[allow(dead_code)]
pub mod nm_item {
    // generic type for listing
    pub struct Item {
        item_type: ItemType,
    }
    impl Item {}

    pub enum ItemType {
        Clothing,
        Decoration,
        Furniture,
        Technology,
    }

    pub enum Clothing {
        Shirt,
        Pants,
        Jacket,
        Dress,
        Socks,
        Shoes,
    }
    pub enum Shirt {
        Generic,
        TShirt,
        LongSleeve,
        LongSleeveCollared,
        Polo,
    }
    pub enum Pants {
        Generic,
        Shorts,
        Jeans,
        Jorts,
    }
    pub enum Jacket {
        Generic,
        Hoodie,
        Pullover,
        Windbreaker,
        Raincoat,
        Tuxedo,
        Sport,
    }
    pub enum Dress {
        Generic,
        Long,
        Short,
        Sundress,
    }
    pub enum Socks {
        Generic,
        Long,
        Short,
        Compression,
        Stockings,
        Graphic,
    }
    pub enum Shoes {
        Sneakers,
        Dress,
        Boots,
        Hiking,
        Shower,
        Sandals,
        FlipFlops,
        Ballet,
    }

    pub enum Decoration {}

    pub enum Furniture {
        Seat,
        Table,
        Shelf,
    }
    pub enum Seat {
        Generic,
        Chair,
        Stool,
        Couch,
    }
    pub enum Table {
        Generic,
        Dining,
        Study,
        Ottoman,
        Coffee,
    }
    pub enum Shelf {
        Generic,
        Bookshelf,
        Study,
    }

    pub enum Technology {
        Computer,
        ComputerPart,
        Device,
        Phone,
        Cable,
        Console,
    }
    pub enum Computer {
        Generic,
        Laptop,
        Desktop,
    }
    pub enum ComputerPart {
        Generic,
        Mouse,
        Keyboard,
        Monitor,
    }
    pub enum Device {
        Generic,
        Headphones,
    }
    pub enum Phone {
        Generic,
    }
    pub enum Cable {
        Generic,
        HDMI,
        Charger,
    }
    pub enum Console {
        Generic,
        Xbox,
        Playstation,
        Switch,
    }

    pub enum Generic {
        Generic,
    }
}
