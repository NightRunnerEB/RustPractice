use crate::book_store::{BookStoreActor, AddBook, GetBooks, GetBook, UpdateBook, DeleteBook};
use crate::book::Book;
use actix::prelude::*;

#[actix_rt::test]
async fn test_add_book() {
    let mut book_store = BookStoreActor::new();
    let ctx = &mut Context::new();

    let msg = AddBook {
        title: "Book Title".to_string(),
        author: "Author Name".to_string(),
        year: 2021,
    };

    book_store.handle(msg, ctx);

    let books = book_store.handle(GetBooks, ctx);
    assert_eq!(books.len(), 1);
    assert_eq!(books[0].title, "Book Title");
    assert_eq!(books[0].author, "Author Name");
    assert_eq!(books[0].year, 2021);
}

#[actix_rt::test]
async fn test_get_book() {
    let mut book_store = BookStoreActor::new();
    let ctx = &mut Context::new();

    let msg = AddBook {
        title: "Book Title".to_string(),
        author: "Author Name".to_string(),
        year: 2021,
    };

    book_store.handle(msg, ctx);

    let book = book_store.handle(GetBook { id: 1 }, ctx);
    assert!(book.is_some());
    assert_eq!(book.unwrap().title, "Book Title");
}

#[actix_rt::test]
async fn test_update_book() {
    let mut book_store = BookStoreActor::new();
    let ctx = &mut Context::new();

    let msg = AddBook {
        title: "Old Title".to_string(),
        author: "Old Author".to_string(),
        year: 2021,
    };

    book_store.handle(msg, ctx);

    let update_msg = UpdateBook {
        id: 1,
        title: "New Title".to_string(),
        author: "New Author".to_string(),
        year: 2022,
    };

    let updated = book_store.handle(update_msg, ctx);
    assert!(updated);

    let book: Option<Book> = book_store.handle(GetBook { id: 1 }, ctx);
    assert!(book.is_some());

    let book_ref = book.as_ref().unwrap();
    assert_eq!(book_ref.title, "New Title");
    assert_eq!(book_ref.author, "New Author");
    assert_eq!(book_ref.year, 2022);
}

#[actix_rt::test]
async fn test_delete_book() {
    let mut book_store = BookStoreActor::new();
    let ctx = &mut Context::new();

    let msg = AddBook {
        title: "Book to Delete".to_string(),
        author: "Author Name".to_string(),
        year: 2021,
    };

    book_store.handle(msg, ctx);

    let deleted = book_store.handle(DeleteBook { id: 1 }, ctx);
    assert!(deleted);

    let book = book_store.handle(GetBook { id: 1 }, ctx);
    assert!(book.is_none());
}
