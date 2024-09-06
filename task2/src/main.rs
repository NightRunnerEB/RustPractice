pub mod book;
pub mod book_store;
mod websocket_handler;

#[cfg(test)]
mod book_store_tests;

use actix::Actor;
use actix_web::{web, App, HttpServer};
use websocket_handler::ws_index;
use book_store::BookStoreActor;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let store = BookStoreActor::new().start();

    // Запуск сервера
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(store.clone()))
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
