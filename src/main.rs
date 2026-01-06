use std::io;
use std::io::stdin;

#[derive(Debug)] //REQUIRED for printing
enum MediaType{
    Book,
    Disc,
    Movie
}

#[derive(Debug)]
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

fn change_title(item: &mut LibraryItem){

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
}
