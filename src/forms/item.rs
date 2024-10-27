#[allow(dead_code)]
pub mod item {
    // generic type for listing
    pub enum Item {
        Clothing,
        Decoration,
        Furniture,
        Technology,
    }

    pub enum Clothing {
        ShirtGeneric,
        ShirtLong,
        PantsGeneric,
        PantsShorts,
        PantsJeans,
        JacketGeneric,
        Dress,
        Socks,
        Shoes,
    }

    pub enum Decoration {}

    pub enum Furniture {
        SeatGeneric,
        SeatChair,
        SeatCouch,
        SeatStool,
        TableGeneric,
        TableDining,
        TableOttoman,
        ShelfGeneric,
        ShelfBooks,
    }

    pub enum Technology {
        ComputerDesktop,
        ComputerLaptop,
        ComputerPartMouse,
        ComputerPartKeyword,
        ComputerPartMonitor,
        Phone,
        Television,
        Speaker,
        Headphones,
    }
}
