use crate::catalog::Catalog;
use regex::Regex;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct ShoppingCart {
    id: Uuid,
    customer_id: String,
    items: HashMap<String, u32>,
    catalog: Catalog,
}

impl ShoppingCart {
    pub fn new(customer_id: &str) -> Result<Self, String> {
        if !Self::is_valid_customer_id(customer_id) {
            return Err("Invalid customer ID format".to_string());
        }
        Ok(Self {
            id: Uuid::new_v4(),
            customer_id: customer_id.to_string(),
            items: HashMap::new(),
            catalog: Catalog::new(),
        })
    }

    fn is_valid_customer_id(customer_id: &str) -> bool {
        let re = Regex::new(r"^\p{L}{3}\d{5}\p{L}{2}-[AQ]$").unwrap();
        re.is_match(customer_id)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn items(&self) -> &HashMap<String, u32> {
        &self.items
    }

    pub fn add_item(&mut self, name: &str, quantity: u32) -> Result<(), String> {
        if quantity == 0 {
            return Err("Quantity must be nonzero".to_string());
        }
        if let Some(&current_quantity) = self.items.get(name) {
            if current_quantity + quantity > 100 {
                return Err("Quantity exceeds the maximum limit of 100".to_string());
            }
        }
        if self.catalog.has_item(name) {
            let counter = self.items.entry(name.to_string()).or_insert(0);
            *counter += quantity;
            Ok(())
        } else {
            Err(format!("Item not found in the catalog: {}", name))
        }
    }

    pub fn update_item(&mut self, name: &str, quantity: u32) -> Result<(), String> {
        if quantity == 0 {
            return Err("Quantity must be nonzero".to_string());
        }
        if quantity > 100 {
            return Err("Quantity exceeds the maximum limit of 100".to_string());
        }
        if let Some(counter) = self.items.get_mut(name) {
            *counter = quantity;
            Ok(())
        } else {
            Err(format!("Item not found in the cart: {}", name))
        }
    }

    pub fn remove_item(&mut self, name: &str) -> Result<(), String> {
        if self.items.remove(name).is_some() {
            Ok(())
        } else {
            Err(format!("Item not found in the cart: {}", name))
        }
    }

    pub fn get_total_cost(&self) -> f64 {
        self.items.iter().fold(0.0, |total, (name, &quantity)| {
            if let Some(price) = self.catalog.get_price(name) {
                total + price * quantity as f64
            } else {
                total
            }
        })
    }
}
