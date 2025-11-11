use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    genre: String,
    publication_year: u32,
    total_copies: u32,
    available_copies: u32,
}

#[derive(Debug, Clone)]
struct Patron {
    id: u32,
    name: String,
    email: String,
    phone: String,
    borrowed_books: Vec<String>, // ISBNs of borrowed books
    fines_owed: f64,
    is_active: bool,
}

#[derive(Debug, Clone)]
struct BorrowRecord {
    patron_id: u32,
    isbn: String,
    borrow_date: u64,
    due_date: u64,
    return_date: Option<u64>,
    fine_amount: f64,
}

#[derive(Debug)]
struct Library {
    name: String,
    books: HashMap<String, Book>,      // ISBN -> Book
    patrons: HashMap<u32, Patron>,     // ID -> Patron
    borrow_records: Vec<BorrowRecord>,
    next_patron_id: u32,
}

impl Book {
    fn new(
        isbn: String,
        title: String,
        author: String,
        genre: String,
        publication_year: u32,
        total_copies: u32,
    ) -> Self {
        // TODO: Create new book with all copies available
        Book {
            isbn,
            title,
            author,
            genre,
            publication_year,
            total_copies,
            available_copies: total_copies,
        }
    }

    fn is_available(&self) -> bool {
        // TODO: Check if book has available copies
        self.available_copies > 0

    }

    fn borrow_copy(&mut self) -> Result<(), String> {
        // TODO: Reduce available copies by 1 if possible
        if self.total_copies > 0 {
            self.available_copies -= 1;
            Ok(())
        } else {
            Err(String::from("No available copies to borrow"))
        }

    }

    fn return_copy(&mut self) -> Result<(), String> {
        // TODO: Increase available copies by 1 if not at max
        if self.total_copies > self.available_copies {
            self.available_copies += 1;
            Ok(())
        } else {
            Err(String::from("All copies are already in the library"))
        }

    }

    fn get_info(&self) -> String {
        // TODO: Return formatted book information
        format!(
            "📖 {} by {}\n\
             ISBN: {} | Genre: {} | Year: {}\n\
             Available: {}/{} copies",
            self.title,
            self.author,
            self.isbn,
            self.genre,
            self.publication_year,
            self.available_copies,
            self.total_copies
        )
    }
}

impl Patron {
    fn new(id: u32, name: String, email: String, phone: String) -> Self {
        // TODO: Create new patron
        Patron {
            id,
            name,
            email,
            phone,
            borrowed_books: Vec::new(),
            fines_owed: 0.0,
            is_active: true,
        }
    }

    fn can_borrow(&self) -> bool {
        // TODO: Check if patron can borrow (active, not too many books, low fines)
        self.is_active && self.borrowed_books.len() < 5 && self.fines_owed < 50.0
    }

    fn add_borrowed_book(&mut self, isbn: String) {
        // TODO: Add book to borrowed list
        self.borrowed_books.push(isbn);
    }

    fn remove_borrowed_book(&mut self, isbn: &str) -> bool {
        // TODO: Remove book from borrowed list, return true if found
        match self.borrowed_books.iter().position(|b| b == isbn) {
            Some(index) => {
                self.borrowed_books.remove(index);
                true
            }
            None => false,
        }
    }

    fn add_fine(&mut self, amount: f64) {
        // TODO: Add fine amount
        self.fines_owed += amount;
    }

    fn pay_fine(&mut self, amount: f64) -> f64 {
        // TODO: Pay fine, return amount actually paid
        let payment = self.fines_owed - amount.max(0.0);
        payment


    }

    fn get_summary(&self) -> String {
        // TODO: Return patron summary
        format!(
            "👤 {} (ID: {})\n\
             Email: {} | Phone: {}\n\
             Borrowed Books: {} | Fines Owed: ${:.2}\n\
             Status: {}",
            self.name,
            self.id,
            self.email,
            self.phone,
            self.borrowed_books.len(),
            self.fines_owed,
            if self.is_active { "Active" } else { "Inactive" }
        )
        String::new()
    }
}

impl BorrowRecord {
    fn new(patron_id: u32, isbn: String, borrow_date: u64, loan_period_days: u32) -> Self {
        // TODO: Create new borrow record with calculated due date
        BorrowRecord {
            patron_id,
            isbn,
            borrow_date,
            due_date: borrow_date + (loan_period_days as u64 * 24 * 3600),
            return_date: None,
            fine_amount: 0.0,
        }
    }

    fn mark_returned(&mut self, return_date: u64, daily_fine_rate: f64) {
        // TODO: Mark as returned and calculate fine if late
    }

    fn is_overdue(&self, current_date: u64) -> bool {
        // TODO: Check if book is overdue
        false
    }

    fn calculate_fine(&self, current_date: u64, daily_fine_rate: f64) -> f64 {
        // TODO: Calculate fine based on days overdue
        0.0
    }
}

impl Library {
    fn new(name: String) -> Self {
        // TODO: Create new library
        Library {
            name,
            books: HashMap::new(),
            patrons: HashMap::new(),
            borrow_records: Vec::new(),
            next_patron_id: 1,
        }
    }

    fn add_book(&mut self, book: Book) {
        // TODO: Add book to catalog
    }

    fn register_patron(&mut self, name: String, email: String, phone: String) -> u32 {
        // TODO: Register new patron and return their ID
        0
    }

    fn borrow_book(&mut self, patron_id: u32, isbn: &str) -> Result<String, String> {
        // TODO: Process book borrowing
        // Check patron can borrow, book is available
        // Create borrow record, update book and patron
        Ok(String::new())
    }

    fn return_book(&mut self, patron_id: u32, isbn: &str) -> Result<String, String> {
        // TODO: Process book return
        // Find borrow record, calculate fines, update records
        Ok(String::new())
    }

    fn search_books(&self, query: &str) -> Vec<&Book> {
        // TODO: Search books by title, author, or ISBN
        vec![]
    }

    fn get_overdue_books(&self, current_date: u64) -> Vec<&BorrowRecord> {
        // TODO: Return list of overdue borrow records
        vec![]
    }

    fn generate_report(&self) -> String {
        // TODO: Generate library statistics report
        String::new()
    }
}

fn get_current_timestamp() -> u64 {
    // Simplified timestamp for testing
    1640995200
}

fn main() {
    println!("=== Library Management System ===");

    // Create library
    let mut library = Library::new(String::from("City Central Library"));

    // Add some books
    library.add_book(Book::new(
        String::from("978-0-123456-78-9"),
        String::from("The Rust Programming Language"),
        String::from("Steve Klabnik"),
        String::from("Programming"),
        2018,
        3,
    ));

    library.add_book(Book::new(
        String::from("978-1-234567-89-0"),
        String::from("Blockchain Development"),
        String::from("Jane Smith"),
        String::from("Technology"),
        2021,
        2,
    ));

    // Register patrons
    let alice_id = library.register_patron(
        String::from("Alice Johnson"),
        String::from("alice@email.com"),
        String::from("555-0123"),
    );

    let bob_id = library.register_patron(
        String::from("Bob Wilson"),
        String::from("bob@email.com"),
        String::from("555-0456"),
    );

    // Borrow books
    match library.borrow_book(alice_id, "978-0-123456-78-9") {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => println!("❌ {}", e),
    }

    match library.borrow_book(bob_id, "978-1-234567-89-0") {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => println!("❌ {}", e),
    }

    // Search books
    let search_results = library.search_books("Rust");
    println!("\nSearch results for 'Rust':");
    for book in search_results {
        println!("{}", book.get_info());
    }

    // Generate report
    println!("\n=== Library Report ===");
    println!("{}", library.generate_report());
}
