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
#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn get_cle_absente()
    {
        let store = Store::new();
        assert_eq!(store.get("a"), None);
    }
    #[test]
    fn set_puis_get()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        assert_eq!(store.get("pseudo"), Some("paul"));
    }
    #[test]
    fn set_rend_ancienne_valeur()
    {
        let mut store = Store::new();
        assert_eq!(store.set("pseudo", "paul"), None);
        assert_eq!(store.set("pseudo", "paula"), Some("paul".to_string()));
        assert_eq!(store.get("pseudo"), Some("paula"));
    }
    #[test]
    fn del_cle_absente()
    {
        let mut store = Store::new();
        assert_eq!(store.del("inconnue"), None);
    }
    #[test]
    fn del_rend_la_valeur_puis_get_est_vide()
    {
        let mut store = Store::new();
        store.set("pseudo", "paul");
        assert_eq!(store.del("pseudo"), Some("paul".to_string()));
        assert_eq!(store.get("pseudo"), None);
    }
}
