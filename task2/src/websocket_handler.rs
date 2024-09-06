use actix::prelude::*;
use actix_web::{HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use serde_json::json;
use crate::book_store::{BookStoreActor, GetBooks, GetBook, AddBook, UpdateBook, DeleteBook};

// Структура WebSocket-актора
pub struct MyWebSocket {
    store: Addr<BookStoreActor>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

// Сообщение от клиента
#[derive(Debug, serde::Deserialize)]
struct ClientMessage {
    action: String,
    id: Option<usize>,
    book: Option<ClientBook>,
}

#[derive(Debug, serde::Deserialize, Clone)]
pub struct ClientBook {
    pub title: String,
    pub author: String,
    pub year: u32,
}

// Обработка WebSocket сообщений
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let client_msg: ClientMessage = serde_json::from_str(&text).unwrap_or_else(|_| {
                    ClientMessage {
                        action: "invalid".to_string(),
                        id: None,
                        book: None,
                    }
                });
                
                match client_msg.action.as_str() {
                    "get_books" => {
                        let store = self.store.clone();
                        let fut = async move {
                            let books = store.send(GetBooks).await.unwrap();
                            books
                        };
                        ctx.spawn(fut.into_actor(self).map(|books, _act, ctx| {
                            ctx.text(serde_json::to_string(&books).unwrap());
                        }));
                    }
                    "get_book" => {
                        if let Some(id) = client_msg.id {
                            let store = self.store.clone();
                            let fut = async move {
                                let book = store.send(GetBook { id }).await.unwrap();
                                book
                            };
                            ctx.spawn(fut.into_actor(self).map(|book, _act, ctx| {
                                if let Some(b) = book {
                                    ctx.text(serde_json::to_string(&b).unwrap());
                                } else {
                                    ctx.text(json!({"error": "Book not found"}).to_string());
                                }
                            }));
                        }
                    }
                    "add_book" => {
                        if let Some(client_book) = client_msg.book {
                            self.store.do_send(AddBook {
                                title: client_book.title,
                                author: client_book.author,
                                year: client_book.year,
                            });
                            ctx.text(json!({"status": "Book added"}).to_string());
                        }
                    }
                    "update_book" => {
                        if let Some(id) = client_msg.id {
                            if let Some(client_book) = client_msg.book {
                                // Отправляем сообщение UpdateBook с новыми данными
                                let store = self.store.clone();
                                let fut = async move {
                                    store.send(UpdateBook {
                                        id,
                                        title: client_book.title,
                                        author: client_book.author,
                                        year: client_book.year,
                                    }).await.unwrap()
                                };
                                ctx.spawn(fut.into_actor(self).map(|success, _act, ctx| {
                                    if success {
                                        ctx.text(json!({"status": "Book updated"}).to_string());
                                    } else {
                                        ctx.text(json!({"error": "Book not found"}).to_string());
                                    }
                                }));
                            } else {
                                ctx.text(json!({"error": "Missing book data"}).to_string());
                            }
                        } else {
                            ctx.text(json!({"error": "Missing book id"}).to_string());
                        }
                    }
                    "delete_book" => {
                        if let Some(id) = client_msg.id {
                            let store = self.store.clone();
                            let fut = async move {
                                store.send(DeleteBook { id }).await.unwrap()
                            };
                            ctx.spawn(fut.into_actor(self).map(|success, _act, ctx| {
                                if success {
                                    ctx.text(json!({"status": "Book deleted"}).to_string());
                                } else {
                                    ctx.text(json!({"error": "Book not found"}).to_string());
                                }
                            }));
                        }
                    }
                    _ => {
                        println!("Invalid action received: {}", client_msg.action);
                        ctx.text(json!({"error": "Invalid action"}).to_string())
                    }
                }
            }
            _ => (),
        }
    }
}

// Создание WebSocket соединения
pub async fn ws_index(r: HttpRequest, stream: web::Payload, store: web::Data<Addr<BookStoreActor>>) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(MyWebSocket { store: store.get_ref().clone() }, &r, stream);
    resp
}
