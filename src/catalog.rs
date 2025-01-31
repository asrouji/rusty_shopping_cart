use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Catalog {
    items: HashMap<String, f64>,
}

impl Catalog {
    pub fn new() -> Self {
        let mut catalog = Self {
            items: HashMap::new(),
        };

        let default_items = [
            ("Laptop", 999.99),
            ("Mouse", 25.99),
            ("Keyboard", 49.99),
            ("Monitor", 199.99),
            ("Headphones", 89.99),
        ];

        for (name, price) in default_items {
            catalog.add_item(name, price).unwrap(); // Safe to unwrap since defaults are valid
        }

        catalog
    }

    pub fn add_item(&mut self, item_name: &str, price: f64) -> Result<(), String> {
        if !Self::is_valid_item_name(item_name) {
            return Err(
                "Invalid item name: must be 3-20 characters long and contain only letters".into(),
            );
        }
        if !Self::is_valid_item_price(price) {
            return Err("Invalid price: must be more than $1 and less than $10,000".into());
        }
        self.items.insert(item_name.to_string(), price);
        Ok(())
    }

    fn is_valid_item_price(price: f64) -> bool {
        (1.0..10000.0).contains(&price)
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
    fn test_add_valid_item() {
        let mut catalog = Catalog::new();
        assert!(catalog.add_item("Smartphone", 799.99).is_ok());
        assert!(catalog.has_item("Smartphone"));
    }

    #[test]
    fn test_add_invalid_item_name() {
        let mut catalog = Catalog::new();
        assert!(catalog.add_item("T", 999.99).is_err());
        assert!(catalog
            .add_item("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 999.99)
            .is_err());
    }

    #[test]
    fn test_add_invalid_item_price() {
        let mut catalog = Catalog::new();
        assert!(catalog.add_item("Laptop", 0.0).is_err());
        assert!(catalog.add_item("Laptop", 15000.0).is_err());
    }

    #[test]
    fn test_non_english() {
        let mut catalog = Catalog::new();
        assert!(catalog.add_item("الهاتف الذكي", 999.99).is_ok());
        assert!(catalog.has_item("الهاتف الذكي"));
    }
}
