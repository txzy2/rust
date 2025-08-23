// Создайте enum BookType { Paperback, HardCover, EBook }
// Создайте структуру Book с полями: title, author, book_type (BookType)
// Создайте структуру Library с полем books: Vec<Book>
// Реализуйте методы:
// - add_book()
// - find_books_by_author() -> &[Book] (возвращает срез)
// - count_books_by_type() -> HashMap<BookType, usize>

use std::collections::HashMap;

pub trait LibraryTrait {
    fn add_book(&mut self, book: Book) -> String;
    fn find_books_by_author(&self, author: &str) -> Result<Vec<&Book>, &'static str>;
    fn count_books_by_type(&self) -> HashMap<BookType, usize>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Добавляем необходимые трейты
pub enum BookType {
    Paperback,
    HardCover,
    EBook,
}

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub book_type: BookType,
}

#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
}

impl LibraryTrait for Library {
    fn add_book(&mut self, book: Book) -> String {
        if self.books.iter().any(|item| item.title == book.title) {
            return "Book is already added".to_string();
        }

        self.books.push(book);
        "added".to_string()
    }

    fn find_books_by_author(&self, author: &str) -> Result<Vec<&Book>, &'static str> {
        let books: Vec<&Book> = self
            .books
            .iter()
            .filter(|item| item.author == author)
            .collect();

        if books.is_empty() {
            Err("Books by this author not found")
        } else {
            Ok(books)
        }
    }

    fn count_books_by_type(&self) -> HashMap<BookType, usize> {
        let mut counts = HashMap::new();

        for book in &self.books {
            *counts.entry(book.book_type).or_insert(0) += 1;
        }

        counts
    }
}
