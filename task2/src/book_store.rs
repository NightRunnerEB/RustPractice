use actix::prelude::*;
use std::collections::HashMap;
use crate::book::Book;

pub struct BookStoreActor {
    books: HashMap<usize, Book>,
    next_id: usize,
}

impl BookStoreActor {
    pub fn new() -> Self {
        BookStoreActor {
            books: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Actor for BookStoreActor {
    type Context = Context<Self>;
}

// Сообщения для управления книгами
#[derive(Message)]
#[rtype(result = "Vec<Book>")]
pub struct GetBooks;

#[derive(Message)]
#[rtype(result = "Option<Book>")]
pub struct GetBook {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddBook {
    pub title: String,
    pub author: String,
    pub year: u32,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct UpdateBook {
    pub id: usize,
    pub title: String,
    pub author: String,
    pub year: u32,
}

#[derive(Message)]
#[rtype(result = "bool")]
pub struct DeleteBook {
    pub id: usize,
}

// Реализация обработки сообщений

impl Handler<GetBooks> for BookStoreActor {
    type Result = Vec<Book>;

    fn handle(&mut self, _msg: GetBooks, _ctx: &mut Self::Context) -> Self::Result {
        self.books.values().cloned().collect()
    }
}

impl Handler<GetBook> for BookStoreActor {
    type Result = Option<Book>;

    fn handle(&mut self, msg: GetBook, _ctx: &mut Self::Context) -> Self::Result {
        self.books.get(&msg.id).cloned()
    }
}

impl Handler<AddBook> for BookStoreActor {
    type Result = ();

    fn handle(&mut self, msg: AddBook, _ctx: &mut Self::Context) {
        // Присваиваем уникальный id книге
        let id = self.next_id;
        self.next_id += 1;

        let book = Book {
            id,
            title: msg.title,
            author: msg.author,
            year: msg.year,
        };

        self.books.insert(id, book);
    }
}

impl Handler<UpdateBook> for BookStoreActor {
    type Result = bool;

    fn handle(&mut self, msg: UpdateBook, _ctx: &mut Self::Context) -> Self::Result {
        if let Some(book) = self.books.get_mut(&msg.id) {
            book.title = msg.title;
            book.author = msg.author;
            book.year = msg.year;
            true
        } else {
            false
        }
    }
}


impl Handler<DeleteBook> for BookStoreActor {
    type Result = bool;

    fn handle(&mut self, msg: DeleteBook, _ctx: &mut Self::Context) -> Self::Result {
        self.books.remove(&msg.id).is_some()
    }
}
