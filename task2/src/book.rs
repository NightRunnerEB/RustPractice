use serde::{Deserialize, Serialize};

// Структура для книги
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub id: usize,
    pub title: String,
    pub author: String,
    pub year: u32,
}
