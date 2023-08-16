use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Page<T> {
    pub total: usize,
    pub page: usize,
    pub size: usize,
    pub content: Vec<T>,
}
#[derive(Serialize, Deserialize)]
pub struct PageRequest {
    pub page: usize,
    pub size: usize,
}
