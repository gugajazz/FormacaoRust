use ordered_float::OrderedFloat;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use uuid::Uuid;

// type BookId = usize; // Type alias for book ID

// Helper functions for the terminal interface
fn prompt_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn prompt_numeric<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<T>() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn _prompt_yes_no(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid input. Please enter 'y' or 'n'."),
        }
    }
}

// New function to handle keyword input as an array
fn prompt_keywords(prompt: &str) -> Vec<String> {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Split the input by commas and convert to trimmed Strings
    input
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// An inverted index mapping keywords to sets of book IDs.
pub struct InvertedIndex {
    index: HashMap<String, HashSet<Uuid>>,
}

impl InvertedIndex {
    /// Creates a new empty inverted index.
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    /// Adds a book and updates the index with its keywords.
    pub fn add_item(&mut self, item: &Item) {
        match item {
            Item::Book(book) => {
                for keyword in &book.textual_or_auditive_media.keywords {
                    self.index
                        .entry(keyword.to_lowercase())
                        .or_default()
                        .insert(book.uuid);
                }
            }
            Item::AudioBook(audiobook) => {
                for keyword in &audiobook.textual_or_auditive_media.keywords {
                    self.index
                        .entry(keyword.to_lowercase())
                        .or_default()
                        .insert(audiobook.uuid);
                }
            }
            _ => panic!("Unsupported item type for indexing."),
        }
    }

    /// Searches for books by a keyword (case-insensitive).
    pub fn search(&self, keyword: &str) -> Vec<&Uuid> {
        match self.index.get(&keyword.to_lowercase()) {
            Some(book_ids) => {
                // book_ids.iter().map(|&id| &self.books[id]).collect();
                return book_ids.iter().collect();
            }
            None => Vec::new(),
        }
    }
}

// Item enum to store all possible library items
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Book(Book),
    AudioBook(AudioBook),
    Statue(Statue),
    Painting(Painting),
}

// Common traits and structures
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GeneralMedia {
    title: String,
    author: String,
}
impl GeneralMedia {
    fn terminal_interface_new() -> Self {
        // Collect GeneralMedia information
        println!("\n-- General Media Information --");
        let title = prompt_input("Title: ");
        let author = prompt_input("Author: ");
        GeneralMedia { title, author }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct TextualOrAuditiveMedia {
    keywords: Vec<String>,
}
impl TextualOrAuditiveMedia {
    fn terminal_interface_new() -> Self {
        // Collect TextualOrAuditiveMedia information
        println!("\n-- Textual Media Information --");
        let keywords = prompt_keywords("Enter keywords (comma-separated): ");
        TextualOrAuditiveMedia { keywords }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PhysicalMedia {
    location: String,
    condition: String,
}
impl PhysicalMedia {
    fn terminal_interface_new() -> Self {
        // Collect PhysicalMedia information
        println!("\n-- Physical Media Information --");
        let location = prompt_input("Location (shelf/section): ");
        let condition = prompt_input("Condition (new/good/fair/poor): ");
        PhysicalMedia {
            location,
            condition,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BorrowableMedia {
    copies_in_stock: u32,
    copies_borrowed: u32,
}
impl BorrowableMedia {
    fn terminal_interface_new() -> Self {
        // Collect BorrowableMedia information
        println!("\n-- Borrowable Media Information --");
        let copies_in_stock = prompt_numeric("Copies in stock: ");
        BorrowableMedia {
            copies_in_stock,
            copies_borrowed: 0,
        }
    }

    fn borrow(&mut self, quantity: Option<u32>) -> bool {
        let quantity = quantity.unwrap_or(1);
        if self.copies_in_stock >= quantity {
            self.copies_in_stock -= quantity;
            self.copies_borrowed += quantity;
            true
        } else {
            false
        }
    }

    fn return_item(&mut self, quantity: Option<u32>) -> bool {
        let quantity = quantity.unwrap_or(1);
        if self.copies_borrowed >= quantity {
            self.copies_borrowed -= quantity;
            self.copies_in_stock += quantity;
            true
        } else {
            false
        }
    }
}

// Specific media types
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Book {
    general_media: GeneralMedia,
    textual_or_auditive_media: TextualOrAuditiveMedia,
    borrowable_media: BorrowableMedia,
    physical_media: PhysicalMedia,
    isbn: String,
    uuid: Uuid,
}
impl Book {
    fn terminal_interface_new() -> Self {
        println!("=== Create New Book ===");

        let general_media = GeneralMedia::terminal_interface_new();
        let textual_or_auditive_media = TextualOrAuditiveMedia::terminal_interface_new();
        let borrowable_media = BorrowableMedia::terminal_interface_new();
        let physical_media = PhysicalMedia::terminal_interface_new();

        // Collect Book-specific information
        println!("\n-- Book-specific Information --");
        let isbn = prompt_input("ISBN: ");
        let uuid = Uuid::new_v4();
        println!("Generated UUID: {}", uuid);

        // Create and return Book instance
        Book {
            general_media: general_media,
            textual_or_auditive_media,
            borrowable_media: borrowable_media,
            physical_media: physical_media,
            isbn,
            uuid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AudioBook {
    general_media: GeneralMedia,
    textual_or_auditive_media: TextualOrAuditiveMedia,
    borrowable_media: BorrowableMedia,
    duration: OrderedFloat<f32>, // duration in hours
    uuid: Uuid,
}
impl AudioBook {
    fn terminal_interface_new() -> Self {
        println!("=== Create New AudioBook ===");

        let general_media = GeneralMedia::terminal_interface_new();
        let textual_or_auditive_media = TextualOrAuditiveMedia::terminal_interface_new();
        let borrowable_media = BorrowableMedia::terminal_interface_new();

        // Collect Book-specific information
        println!("\n-- Audiobook-specific Information --");
        let duration = prompt_numeric("Duration: ");
        let uuid = Uuid::new_v4();
        println!("Generated UUID: {}", uuid);

        // Create and return Book instance
        AudioBook {
            general_media: general_media,
            textual_or_auditive_media: textual_or_auditive_media,
            borrowable_media: borrowable_media,
            duration: duration,
            uuid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statue {
    general_media: GeneralMedia,
    physical_media: PhysicalMedia,
    material: String,
    uuid: Uuid,
}
impl Statue {
    fn terminal_interface_new() -> Self {
        println!("=== Create New Statue ===");

        let general_media = GeneralMedia::terminal_interface_new();
        let physical_media = PhysicalMedia::terminal_interface_new();

        // Collect Book-specific information
        println!("\n-- Statue-specific Information --");
        let material = prompt_input("Material (stone, wood): ");
        let uuid = Uuid::new_v4();
        println!("Generated UUID: {}", uuid);

        // Create and return Book instance
        Statue {
            general_media: general_media,
            physical_media: physical_media,
            material,
            uuid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Painting {
    general_media: GeneralMedia,
    physical_media: PhysicalMedia,
    style: String,
    uuid: Uuid,
}
impl Painting {
    fn terminal_interface_new() -> Self {
        println!("=== Create New Painting ===");

        let general_media = GeneralMedia::terminal_interface_new();
        let physical_media = PhysicalMedia::terminal_interface_new();

        // Collect Book-specific information
        println!("\n-- Painting-specific Information --");
        let style = prompt_input("Style (Modern, Post-Modern, Baroque): ");
        let uuid = Uuid::new_v4();
        println!("Generated UUID: {}", uuid);

        // Create and return Book instance
        Painting {
            general_media: general_media,
            physical_media: physical_media,
            style,
            uuid,
        }
    }
}

// Library structure to manage all items
struct Library {
    items: HashMap<Uuid, Item>, // key: ISBN
    inverted_index: InvertedIndex,
}

impl Library {
    fn new() -> Self {
        Library {
            items: HashMap::new(),
            inverted_index: InvertedIndex::new(),
        }
    }

    fn add_item(&mut self, item: Item) {
        match item {
            Item::Book(ref book) => {
                self.inverted_index.add_item(&item);
                self.items.insert(book.uuid, item);
            }
            Item::AudioBook(ref audiobook) => {
                self.inverted_index.add_item(&item);
                self.items.insert(audiobook.uuid, item);
            }
            Item::Statue(ref statue) => {
                self.items.insert(statue.uuid, item);
            }
            Item::Painting(ref painting) => {
                self.items.insert(painting.uuid, item);
            }
        }
    }

    fn remove_item(&mut self, uuid: &str) {
        if let Ok(uuid) = Uuid::parse_str(uuid) {
            if self.items.remove(&uuid).is_some() {
                println!("Item with UUID {} removed successfully.", uuid);
            } else {
                println!("Item with UUID {} not found.", uuid);
            }
        } else {
            println!("Invalid UUID format.");
        }
    }

    fn add_item_terminal_interface(&mut self) {
        println!("=== Add New Item ===");
        println!("╔════════════════════════════════════════╗");
        println!("║              ADD NEW ITEM              ║");
        println!("╠════════════════════════════════════════╣");
        println!("║                                        ║");
        println!("║ ITEM MANAGEMENT:                       ║");
        println!("║  1. Add a Book                         ║");
        println!("║  2. Add a AudioBook                    ║");
        println!("║  3. Add a Statue                       ║");
        println!("║  4. Add a Painting                     ║");
        println!("║                                        ║");
        println!("╚════════════════════════════════════════╝");
        let item_type: u16 = prompt_numeric("Option: ");
        match item_type {
            1 => {
                let book = Book::terminal_interface_new();
                self.add_item(Item::Book(book));
            }
            2 => {
                let audiobook = AudioBook::terminal_interface_new();
                self.add_item(Item::AudioBook(audiobook));
            }
            3 => {
                let statue = Statue::terminal_interface_new();
                self.add_item(Item::Statue(statue));
            }
            4 => {
                let painting = Painting::terminal_interface_new();
                self.add_item(Item::Painting(painting));
            }
            _ => println!("Invalid item type."),
        }
    }
}

fn main() {
    let mut library = Library::new();

    // let new_book = Book::terminal_interface_new();
    // library.add_item(Item::Book(new_book));

    // println!("Exiting the program. Goodbye!");
    // std::process::exit(0);
    // }
    loop {
        print_main_menu();
        let choice = prompt_input("Enter your choice: ");

        match choice.as_str() {
            // Book operations
            "1" => library.add_item_terminal_interface(),

            "2" => {
                let uuid = prompt_input("Enter the UUID of the item to remove: ");
                library.remove_item(&uuid);
            }

            "3" => {
                let keyword = prompt_input("Enter a keyword to search: ");
                let results = library.inverted_index.search(&keyword);
                if results.is_empty() {
                    println!("No items found with the keyword '{}'.", keyword);
                } else {
                    println!("Items found with the keyword '{}':", keyword);
                    for uuid in results {
                        if let Some(item) = library.items.get(uuid) {
                            println!("{:#?}", item);
                        }
                    }
                }
            }
            "4" => {
                let uuid = prompt_input("Enter the UUID of the item to find: ");
                if let Ok(uuid) = Uuid::parse_str(&uuid) {
                    library.items.get(&uuid);
                } else {
                    println!("Invalid UUID format.");
                }
            }
            "5" => {
                // print all items
                println!("All items in the library:");
                for (uuid, item) in &library.items {
                    println!("UUID: {}, Item: {:#?}", uuid, item);
                }
            }
            "6" => {
                // borrow item
                let uuid = prompt_input("Enter the UUID of the item to borrow: ");
                if let Ok(uuid) = Uuid::parse_str(&uuid) {
                    if let Some(item) = library.items.get_mut(&uuid) {
                        match item {
                            Item::Book(book) => {
                                if book.borrowable_media.borrow(None) {
                                    println!("Borrowed Book: {:#?}", book);
                                } else {
                                    println!("No copies available for borrowing.");
                                }
                            }
                            Item::AudioBook(audiobook) => {
                                if audiobook.borrowable_media.borrow(None) {
                                    println!("Borrowed Audiobook: {:#?}", audiobook);
                                } else {
                                    println!("No copies available for borrowing.");
                                }
                            }
                            _ => println!("Item is not borrowable."),
                        }
                    } else {
                        println!("Item with UUID {} not found.", uuid);
                    }
                } else {
                    println!("Invalid UUID format.");
                }
            }
            "7" => {
                // return item
                let uuid = prompt_input("Enter the UUID of the item to return: ");
                if let Ok(uuid) = Uuid::parse_str(&uuid) {
                    if let Some(item) = library.items.get_mut(&uuid) {
                        match item {
                            Item::Book(book) => {
                                if book.borrowable_media.return_item(None) {
                                    println!("Returned Book: {:#?}", book);
                                } else {
                                    println!("No copies borrowed.");
                                }
                            }
                            Item::AudioBook(audiobook) => {
                                if audiobook.borrowable_media.return_item(None) {
                                    println!("Returned Audiobook: {:#?}", audiobook);
                                } else {
                                    println!("No copies borrowed.");
                                }
                            }
                            _ => println!("Item is not borrowable."),
                        }
                    } else {
                        println!("Item with UUID {} not found.", uuid);
                    }
                } else {
                    println!("Invalid UUID format.");
                }
            }

            // Exit
            "0" => {
                println!("Exiting Library Management System...");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }

        println!("\nPress Enter to continue...");
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}

fn print_main_menu() {
    // clear_screen();
    println!("╔════════════════════════════════════════╗");
    println!("║  MAIN MENU LIBRARY MANAGEMENT SYSTEM   ║");
    println!("╠════════════════════════════════════════╣");
    println!("║                                        ║");
    println!("║ ITEM MANAGEMENT:                       ║");
    println!("║  1. Add a Item                         ║");
    println!("║  2. Remove a Item                      ║");
    println!("║  3. Find a Item (keyword)              ║");
    println!("║  4. Find a Item (UUID)                 ║");
    println!("║  5. Print all items                    ║");
    println!("║  6. Borrow Item                        ║");
    println!("║  7. Return Item                        ║");
    println!("║                                        ║");
    println!("╚════════════════════════════════════════╝");
}

fn _clear_screen() {
    // On Windows
    if cfg!(windows) {
        let _ = std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .status();
    }
    // On Unix-like systems
    else {
        let _ = std::process::Command::new("clear").status();
    }

    // Fallback clear method
    println!("\x1B[2J\x1B[1;1H");
}
