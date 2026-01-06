# Library Management System in Rust

A simple command-line library management system written in **Rust**.
This program allows you to manage books, discs, and movies, track their availability, and search by title.

## Features

* Add and store library items: **Books, Discs, Movies**
* Track **status** of items: Available, Borrowed, Not in Stock, Archived
* Search library items by title (case-insensitive)
* Update item titles
* Archive items to mark them as unavailable
* Display detailed item information

## Project Structure

* `main.rs` – Contains the main program logic and `LibraryItem` implementation.

* `LibraryItem` struct – Represents each library item with fields:

  * `id`: Unique numeric identifier
  * `name`: Title of the item
  * `media`: Type of media (`Book`, `Disc`, `Movie`)
  * `status`: Current status (`Available`, `Borrowed`, `NotInStock`, `Archived`)
  * `shelf_location`: Location in the library (area, shelf number, row number)

* `DatabaseConnection` – Placeholder for future database connectivity

## Dependencies

* Rust 1.70+ (no external crates required)

## How to Run

1. Clone the repository:

```bash
git clone https://github.com:dikshita0719/LibraryMediaManagementSystem.git
cd LibraryMediaManagementsystem
```

2. Build and run the project using Cargo:

```bash
cargo run
```

3. Follow the on-screen prompts to:

* View library items
* Change titles
* Search for items by title
* Archive items

## Example Usage

```text
Characters: 14
Bytes: 14
"Da Vinci Code"
Choose a new name to replace:
> Da Vinci's Secret
LibraryItem { id: 1, name: "Da Vinci's Secret", media: Book, status: Available, shelf_location: ShelfLocation("W02", 2, 6) }
What are you looking for?
> Kafka
LibraryItem { id: 1001, name: "Kafka: Die Erzählungen", media: Book, status: Available, shelf_location: ShelfLocation("LIT", 3, 15) }
```


