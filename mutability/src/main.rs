#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // &'static str is a reference to a string
    // allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book 
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title,
        book.year);
}

// this fn takes a mutable ref to a book
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title,
        book.year);
}

fn main() {
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);

    // new_edition(&mut immutabook);
        
}

