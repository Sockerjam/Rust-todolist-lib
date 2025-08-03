#[derive(uniffi::Enum, Clone, Debug, PartialEq)]
pub enum Status {
    Done,
    InProgress,
    Deleted,
}
