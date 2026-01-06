use std::io;
use std::io::stdin;


#[derive(Debug)] //REQUIRED for printing
enum MediaType{
    Book,
    Disc,
    Movie
}

#[derive(Debug, PartialEq)]  //partialeq for comparing te enum
enum Status{
    Available,
    NotInStock,
    Borrowed,
    Archived
}

#[derive(Debug)]
struct LibraryItem{
    id: u32,
    name: String,
    media: MediaType,
    status: Status,
    shelf_location: ShelfLocation
}

#[derive(Debug)]
struct ShelfLocation(String, u8,u8); //String is the area(Like K for Kunst, u8-shelfNumber, u8-Rownumber

struct DatabaseConnection;  // UnitLikeStruct

impl DatabaseConnection {
    fn connect() -> Self {
        // Future: actually connect to database
        // For now: just return the struct itself
        DatabaseConnection
    }
}

impl LibraryItem{
    fn new(id: u32, name: String, media: MediaType, status: Status, shelf_location: ShelfLocation)->Self{
        return Self{
            id,
            name,
            media,
            status,
            shelf_location
        };
    }

    fn get_title(&self)->String{
        println!("Characters: {}", self.name.chars().count());
        println!("Bytes: {}", self.name.len());
        return self.name.clone(); //does a deep copy of the String without actually losing the ownership (unlike move for primitive Datatypes)
    }
    fn set_title(&mut self, new_title: String){
        self.name = new_title;
    }
    fn archive (mut self){
        self.status = Status::Archived;
        println!("Archived {:?}: {} \n.Media no longer available.", self.media, self.name);
    }
}
fn print_item_info(item: &LibraryItem){ //immutable borrwing
    println!("{:#?}",  item);
}

fn search_by_title(library: &Vec<LibraryItem>, title:String) -> Option<&LibraryItem>{
    let search_term = title.to_lowercase();
    for item in library.iter() {
        if item.name.to_lowercase().contains(&search_term) && item.status == Status::Available
        {
                print_item_info(item);
                return Some(item);
        }else{
        println!("Item {} is either Not in Stock or currently unavailable.", item.name);
        return None;
    }
    }
    println!("Did not find the item you are looking for :(");
    None
}

fn main() {
    let mut first = LibraryItem::new(1,String::from("Da Vinci Code"), MediaType::Book, Status::Available,ShelfLocation(String::from("W02"), 2,6)) ;
    let second = LibraryItem::new(3,String::from("Pink Floyd: Dark Side of the Moon"), MediaType::Disc, Status::Borrowed,ShelfLocation(String::from("K02"), 5,9)) ;

    println!("{:?}", first.get_title());
    println!("{:?}", second.get_title());
    second.archive();
    //println!("{:?}", second.get_title()); //Error because the value has been moved in the previous line

    print_item_info(&first);
    println!("Choose a new name to replace: ");

    let mut new_title = String::new();
    io::stdin().read_line(&mut new_title).unwrap();
    let new_title = new_title.trim().to_string();
    first.set_title(new_title);

    print_item_info(&first);


    let mut library: Vec<LibraryItem> = Vec::new();
    // Books
    let book1 = LibraryItem::new(
        1001,
        String::from("Kafka: Die Erzählungen"),
        MediaType::Book,
        Status::Available,
        ShelfLocation(String::from("LIT"), 3, 15)
    );

    let book2 = LibraryItem::new(
        1002,
        String::from("Faust: Eine Tragödie"),
        MediaType::Book,
        Status::Borrowed,
        ShelfLocation(String::from("LIT"), 2, 8)
    );

    let book3 = LibraryItem::new(
        1003,
        String::from("Grundlagen der Informatik"),
        MediaType::Book,
        Status::Available,
        ShelfLocation(String::from("INF"), 5, 22)
    );

    let book4 = LibraryItem::new(
        1004,
        String::from("Dresden: Eine Stadtgeschichte"),
        MediaType::Book,
        Status::Available,
        ShelfLocation(String::from("HIS"), 1, 12)
    );

    // Music/Discs
    let disc1 = LibraryItem::new(
        2001,
        String::from("Bach: Goldberg Variationen"),
        MediaType::Disc,
        Status::Available,
        ShelfLocation(String::from("MUS"), 4, 7)
    );

    let disc2 = LibraryItem::new(
        2002,
        String::from("Beethoven: Sinfonien"),
        MediaType::Disc,
        Status::Borrowed,
        ShelfLocation(String::from("MUS"), 4, 18)
    );

    // Movies
    let movie1 = LibraryItem::new(
        3001,
        String::from("Das Leben der Anderen"),
        MediaType::Movie,
        Status::Available,
        ShelfLocation(String::from("FILM"), 2, 9)
    );

    let movie2 = LibraryItem::new(
        3002,
        String::from("Lola rennt"),
        MediaType::Movie,
        Status::NotInStock,
        ShelfLocation(String::from("FILM"), 2, 14)
    );
    library.push(book1);
    library.push(book2);
    library.push(book3);
    library.push(book4);
    library.push(disc1);
    library.push(disc2);
    library.push(movie1);
    library.push(movie2);

    println!("What are you looking for?");
    let mut search = String::new();
    stdin().read_line(&mut search).unwrap();
    search = search.trim().to_string();

    search_by_title(&library, search);
}
