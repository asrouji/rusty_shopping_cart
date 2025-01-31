use crate::ensure;
use regex::Regex;
use std::collections::HashMap;

const MIN_ITEM_NAME_LENGTH: usize = 3;
const MAX_ITEM_NAME_LENGTH: usize = 20;
const MIN_ITEM_PRICE: f64 = 1.0;
const MAX_ITEM_PRICE: f64 = 10000.0;

/// A catalog of items with their prices.
#[derive(Debug)]
pub struct Catalog {
    items: HashMap<String, f64>,
}

impl Catalog {
    /// Creates a new catalog with default items.
    ///
    /// # Returns
    ///
    /// A `Catalog` instance with default items.
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

    /// Adds an item to the catalog.
    ///
    /// # Arguments
    ///
    /// * `item_name` - The name of the item to add.
    /// * `price` - The price of the item.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the item was added successfully.
    /// * `Err(String)` if the item name or price is invalid.
    fn add_item(&mut self, item_name: &str, price: f64) -> Result<(), String> {
        ensure!(
            Self::is_valid_item_name(item_name),
            format!(
                "Invalid item name: must be {}-{} characters long and contain only letters",
                MIN_ITEM_NAME_LENGTH, MAX_ITEM_NAME_LENGTH
            )
        );
        ensure!(
            Self::is_valid_item_price(price),
            format!(
                "Invalid price: must be more than ${} and less than ${}",
                MIN_ITEM_PRICE, MAX_ITEM_PRICE
            )
        );
        self.items.insert(item_name.to_string(), price);
        Ok(())
    }

    /// Checks if the item price is valid.
    ///
    /// # Arguments
    ///
    /// * `price` - The price to check.
    ///
    /// # Returns
    ///
    /// `true` if the price is between $1 and $10,000, otherwise `false`.
    fn is_valid_item_price(price: f64) -> bool {
        (MIN_ITEM_PRICE..MAX_ITEM_PRICE).contains(&price)
    }

    /// Checks if the item name is valid.
    ///
    /// # Arguments
    ///
    /// * `item_name` - The name to check.
    ///
    /// # Returns
    ///
    /// `true` if the name is 3-20 characters long and contains only letters, otherwise `false`.
    fn is_valid_item_name(item_name: &str) -> bool {
        let re = Regex::new(&format!(
            r"^[\p{{L}} ]{{{},{}}}$",
            MIN_ITEM_NAME_LENGTH, MAX_ITEM_NAME_LENGTH
        ))
        .unwrap();
        re.is_match(item_name)
    }

    /// Checks if the catalog contains an item.
    ///
    /// # Arguments
    ///
    /// * `item_name` - The name of the item to check.
    ///
    /// # Returns
    ///
    /// `true` if the item is in the catalog, otherwise `false`.
    pub fn has_item(&self, item_name: &str) -> bool {
        self.items.contains_key(item_name)
    }

    /// Gets the price of an item.
    ///
    /// # Arguments
    ///
    /// * `item_name` - The name of the item.
    ///
    /// # Returns
    ///
    /// `Some(f64)` with the price if the item is found, otherwise `None`.
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
