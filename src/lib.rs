use crate::{
    lab1::{Student, StudentTrait},
    lab2::{Book, BookType, Library, LibraryTrait},
};

pub mod lab1;
pub mod lab2;
pub mod lab3;

pub fn proccess_lab1() {
    let mut student = Student::new("John".to_string(), vec![1, 2, 3]);
    student.add_grade(4);

    println!("Student: {:?}", student);
    println!("Average: {}", student.average().unwrap());
    println!("Highest grade: {}", student.highest_grade().unwrap());
}

pub fn proccess_lab2() {
    let book = Book {
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
        book_type: BookType::HardCover,
    };

    let book2 = Book {
        title: "To Kill a Mockingbird".to_string(),
        author: "Harper Lee".to_string(),
        book_type: BookType::EBook,
    };

    let book3 = Book {
        title: "1984".to_string(),
        author: "George Orwell".to_string(),
        book_type: BookType::Paperback,
    };

    let books = vec![book, book2, book3];

    let mut library = Library { books };

    let book_to_add = Book {
        title: "1234".to_string(),
        author: "George Orwell".to_string(),
        book_type: BookType::Paperback,
    };
    library.add_book(book_to_add);

    println!(
        "Books by author: {:?}",
        library.find_books_by_author("George Orwell").unwrap()
    );

    println!("Book counts: {:#?}", library.count_books_by_type());
}
