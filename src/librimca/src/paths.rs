use crate::error::PathError;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Paths(pub HashMap<String, PathBuf>);

impl Paths {
    pub fn get(&self, key: &str) -> Result<&PathBuf, PathError> {
        self.0.get(key).ok_or(PathError::NotFound(String::from(key)))
    }

    pub fn new() -> Self {
        Paths(HashMap::new())
    }
}