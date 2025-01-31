use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Catalog {
    items: HashMap<&'static str, f64>,
}

impl Catalog {
    pub fn new() -> Self {
        let mut catalog = Self {
            items: HashMap::new(),
        };

        catalog.add_item("Laptop", 999.99);
        catalog.add_item("Mouse", 25.99);
        catalog.add_item("Keyboard", 49.99);
        catalog.add_item("Monitor", 199.99);
        catalog.add_item("Headphones", 89.99);

        catalog
    }

    fn add_item(&mut self, item_name: &'static str, price: f64) {
        if !Self::is_valid_item_name(item_name) {
            panic!("Invalid item name, must be 3-20 characters long and contain only letters");
        }
        if !Self::is_valid_item_price(price) {
            panic!("Price must be more than $0 and less than $10,000");
        }
        self.items.insert(item_name, price);
    }

    fn is_valid_item_price(price: f64) -> bool {
        price > 0.0 && price < 10000.0
    }

    fn is_valid_item_name(item_name: &str) -> bool {
        let re = Regex::new(r"^[\p{L} ]{3,20}$").unwrap();
        re.is_match(item_name)
    }

    pub fn has_item(&self, item_name: &str) -> bool {
        self.items.contains_key(item_name)
    }

    pub fn get_price(&self, item_name: &str) -> Option<f64> {
        self.items.get(item_name).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_has_item() {
        let catalog = Catalog::new();
        assert!(catalog.has_item("Laptop"));
        assert!(catalog.has_item("Mouse"));
        assert!(catalog.has_item("Keyboard"));
        assert!(catalog.has_item("Monitor"));
        assert!(catalog.has_item("Headphones"));
    }

    #[test]
    fn test_catalog_get_price() {
        let catalog = Catalog::new();
        assert_eq!(catalog.get_price("Laptop"), Some(999.99));
        assert_eq!(catalog.get_price("Mouse"), Some(25.99));
        assert_eq!(catalog.get_price("Keyboard"), Some(49.99));
        assert_eq!(catalog.get_price("Monitor"), Some(199.99));
        assert_eq!(catalog.get_price("Headphones"), Some(89.99));
    }

    #[test]
    #[should_panic]
    fn test_catalog_invalid_item_name() {
        let mut catalog = Catalog::new();
        catalog.add_item("T", 999.99);
    }

    #[test]
    #[should_panic]
    fn test_catalog_invalid_item_price() {
        let mut catalog = Catalog::new();
        catalog.add_item("Laptop", 0.0);
    }

    #[test]
    fn test_non_english() {
        let mut catalog = Catalog::new();
        catalog.add_item("الهاتف الذكي", 999.99);
        assert!(catalog.has_item("الهاتف الذكي"));
    }
}
