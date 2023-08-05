pub struct Page<T> {
    pub total: i32,
    pub page: i32,
    pub size: i32,
    pub content : Vec<T>,
}
