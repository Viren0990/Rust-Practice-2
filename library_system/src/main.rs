use std::collections::HashMap;


enum Status {
    Available,
    Borrowed(String), 
}


struct Book {
    id: u32,
    title: String,
    author: String,
    status: Status,
}

fn main() {
    let mut books: Vec<Book> = Vec::new();
    books.push(Book {
        id: 1,
        title: String::from("Book A"),
        author: String::from("Viren1"),
        status: Status::Available,
    });


    books.push(Book {
        id: 2,
        title: String::from("Book B"),
        author: String::from("Viren2"),
        status: Status::Available,
    });


    books.push(Book {
        id: 3,
        title: String::from("Book C"),
        author: String::from("Viren3"),
        status: Status::Available,
    });

    println!("");

    let mut borrow_map: HashMap<u32, String> = HashMap::new();

    borrow_book(&mut books,&mut borrow_map, 1, "Alice".to_string());
    borrow_book(&mut books,&mut borrow_map, 1, "Bob".to_string());
    return_book(&mut books,&mut borrow_map, 1);
    borrow_book(&mut books,&mut borrow_map, 1, "Bob".to_string());
}

fn borrow_book(books: &mut Vec<Book>, borrow_map: &mut HashMap<u32, String>, id:u32, user: String){
    if let Some(book) = books.iter_mut().find(|b| b.id==id){
        match &book.status {
            Status::Available => {
                book.status = Status::Borrowed(user.clone());
                borrow_map.insert(id, user.clone());
                println!("'{}' by {} borrowed by {}", 
                    book.title, book.author, user);
            },
            Status::Borrowed(borrower) => {
                println!("Cannot borrow '{}': Already borrowed by {}", book.title, borrower);
            }
        }
    } else {
        println!("Book with ID {} not found", id);
    }
}


fn return_book(books: &mut Vec<Book>, borrow_map: &mut HashMap<u32,String>, id:u32){
    if let Some(book) = books.iter_mut().find(|b| b.id == id){
        match &book.status {
            Status::Borrowed(_) => {
                book.status = Status::Available;
                borrow_map.remove(&id);
                println!("Book '{}' has been returned", book.title);
            },
            Status::Available => {
                println!("Book '{}' is already available", book.title);
            }
        }
    }else {
        println!("Book with ID {} not found", id);
    }
}