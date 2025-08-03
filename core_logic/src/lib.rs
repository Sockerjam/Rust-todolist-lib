uniffi::setup_scaffolding!();
mod errors;
mod generator;
mod todoitem;
use errors::ToDoError;
use generator::IDGenerator;
use std::fmt;
use std::sync::Mutex;
use todoitem::ToDoItem;
use todoitem::status::Status;

#[derive(uniffi::Object)]
pub struct ToDoList {
    items: Mutex<Vec<ToDoItem>>,
    id_generator: Mutex<IDGenerator>,
}

#[uniffi::export]
impl ToDoList {
    #[uniffi::constructor]
    pub fn new() -> Self {
        ToDoList {
            items: Mutex::new(Vec::new()),
            id_generator: Mutex::new(IDGenerator::new()),
        }
    }

    pub fn get_items(&self) -> Vec<ToDoItem> {
        let items = self.items.lock().unwrap();
        items.clone()
    }

    pub fn add_item(&self, description: String, status: Status) {
        let id = self.id_generator.lock().unwrap().get_id();
        let to_do_item = ToDoItem {
            id: id,
            description: description,
            status: status,
        };
        let mut items = self.items.lock().unwrap();
        items.push(to_do_item);
    }

    pub fn update_item(&self, item: ToDoItem) -> Result<(), ToDoError> {
        let mut items = self.items.lock().unwrap();
        match items.iter_mut().find(|i| i.id == item.id) {
            Some(existing) => {
                *existing = item;
                Ok(())
            }
            None => Err(ToDoError::ItemNotFound(item.id)),
        }
    }

    pub fn delete_item(&self, item: ToDoItem) -> Result<(), ToDoError> {
        let mut items = self.items.lock().unwrap();
        for (i, e) in items.iter_mut().enumerate() {
            if e.id == item.id {
                items.remove(i);
                self.id_generator.lock().unwrap().add_id_to_pool(item.id);
                return Ok(());
            }
        }
        Err(ToDoError::ItemNotFound(item.id))
    }
}

impl fmt::Display for ToDoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::todoitem::status::Status;

    #[test]
    fn new_returns_empty_items() {
        let to_do_list = ToDoList::new();
        assert_eq!(to_do_list.get_items().len(), 0);
    }

    #[test]
    fn adding_item() {
        let to_do_list = ToDoList::new();
        assert_eq!(to_do_list.get_items().len(), 0);

        to_do_list.add_item("Buy eggs".to_string(), Status::InProgress);
        assert_eq!(to_do_list.get_items().len(), 1);
    }

    #[test]
    fn update_item() {
        let to_do_list = ToDoList::new();
        to_do_list.add_item("Buy eggs".to_string(), Status::InProgress);

        let mut to_do_list_item = to_do_list
            .get_items()
            .first()
            .expect("Should contain one element")
            .clone();

        assert_eq!(to_do_list_item.description, "Buy eggs");

        to_do_list_item.description = "Buy chicken".to_string();
        to_do_list.update_item(to_do_list_item);

        let mut to_do_list_item = to_do_list
            .get_items()
            .first()
            .expect("Should contain one element")
            .clone();

        assert_eq!(to_do_list_item.description, "Buy chicken");
    }

    #[test]
    fn delete_item() {
        let to_do_list = ToDoList::new();
        to_do_list.add_item("Buy milk".to_string(), Status::InProgress);
        to_do_list.add_item("Buy chicken".to_string(), Status::InProgress);

        assert_eq!(to_do_list.get_items().len(), 2);

        let to_do_list_item = to_do_list
            .get_items()
            .first()
            .expect("Should contain one element")
            .clone();

        to_do_list.delete_item(to_do_list_item);

        assert_eq!(to_do_list.get_items().len(), 1);
    }

    #[test]
    fn delete_item_should_throw_error() {
        let to_do_list = ToDoList::new();
        to_do_list.add_item("Buy milk".to_string(), Status::InProgress);

        assert_eq!(to_do_list.get_items().len(), 1);

        let to_do_item = ToDoItem {
            id: 9,
            description: "test".to_string(),
            status: Status::Done,
        };

        assert!(matches!(
            to_do_list.delete_item(to_do_item),
            Err(ToDoError::ItemNotFound(9))
        ));
    }
}
