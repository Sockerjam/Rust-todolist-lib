pub mod status;
use crate::todoitem::status::Status;

#[derive(uniffi::Record, Debug, Clone)]
pub struct ToDoItem {
    pub(crate) id: u32,
    pub description: String,
    pub status: Status,
}
