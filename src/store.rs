use std::collections::HashMap;
#[derive(Debug)]
pub struct Store
{
    stock: HashMap<String, String>,
}

impl Store
{
    pub fn new() -> Self
    {
        Store {
            stock: HashMap::new(),
        }
    }
    pub fn set(&mut self, key: &str, value: &str) -> Option<String>
    {
        self.stock.insert(key.to_string(), value.to_string())
    }
    pub fn get(&self, key: &str) -> Option<&str>
    {
        self.stock.get(key).map(|s| s.as_str())
    }
    pub fn del(&mut self, key: &str) -> Option<String>
    {
        self.stock.remove(key)
    }
}
