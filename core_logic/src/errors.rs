#[derive(uniffi::Error, Debug, PartialEq)]
pub enum ToDoError {
    ItemNotFound(u32),
}
