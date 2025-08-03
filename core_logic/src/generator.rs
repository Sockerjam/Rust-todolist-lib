use std::collections::VecDeque;
use std::sync::Mutex;

#[derive(uniffi::Object)]
pub struct IDGenerator {
    next_id: Mutex<u32>,
    available_ids: Mutex<VecDeque<u32>>,
}

#[uniffi::export]
impl IDGenerator {
    #[uniffi::constructor]
    pub const fn new() -> Self {
        IDGenerator {
            next_id: Mutex::new(0),
            available_ids: Mutex::new(VecDeque::new()),
        }
    }

    pub fn get_id(&self) -> u32 {
        if let Some(available_id) = self.available_ids.lock().unwrap().pop_front() {
            available_id
        } else {
            let mut guard = self.next_id.lock().unwrap();
            let id = *guard;
            *guard += 1;
            id
        }
    }

    pub fn add_id_to_pool(&self, id: u32) {
        self.available_ids.lock().unwrap().push_back(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_id_returns_incremented_id() {
        let id_generator = IDGenerator::new();
        assert_eq!(id_generator.get_id(), 0);
        assert_eq!(id_generator.get_id(), 1);
        assert_eq!(id_generator.get_id(), 2);
    }

    #[test]
    fn get_id_returns_from_pool() {
        let id_generator = IDGenerator::new();
        id_generator.add_id_to_pool(10);
        id_generator.add_id_to_pool(55);
        assert_eq!(id_generator.get_id(), 10);
        assert_eq!(id_generator.get_id(), 55);
    }

    #[test]
    fn get_id_returns_new_id_when_pool_is_empty() {
        let id_generator = IDGenerator::new();
        id_generator.add_id_to_pool(10);
        id_generator.add_id_to_pool(55);
        assert_eq!(id_generator.get_id(), 10);
        assert_eq!(id_generator.get_id(), 55);
        assert_eq!(id_generator.get_id(), 0);
    }
}
