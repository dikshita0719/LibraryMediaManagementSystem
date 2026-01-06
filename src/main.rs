enum MediaType{
    Book,
    Disc,
    Movie
}

enum Status{
    Available,
    NotInStock,
    Borrowed
}

struct LibraryItem{
    id: u32,
    name: String,
    media: MediaType,
    status: Status
}

struct ShelfLocation(String, u8,u8); //String is the area(Like K for Kunst, u8-shelfNumber, u8-Rownumber

struct DatabaseConnection;  // UnitLikeStruct

impl DatabaseConnection {
    fn connect() -> Self {
        // Future: actually connect to database
        // For now: just return the struct itself
        DatabaseConnection
    }
}

fn main() {
    println!("Hello, world!");
}
