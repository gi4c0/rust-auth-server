use parse_display::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SearchType<T> {
    pub results: Vec<T>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize, Display)]
pub enum SortingDirection {
    ASC,
    DESC,
}
