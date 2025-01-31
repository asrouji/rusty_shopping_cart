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
    pub fn new(customer_id: &str) -> Result<Self, &'static str> {
        if !Self::is_valid_customer_id(customer_id) {
            return Err("Invalid customer ID format");
        }
        Ok(Self {
            id: Uuid::new_v4(),
            customer_id: customer_id.to_string(),
            items: HashMap::default(),
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
            return Err("Quantity must be nonzero".into());
        }
        if !self.catalog.has_item(name) {
            return Err(format!("Item not found in the catalog: {}", name));
        }
        let counter = self.items.entry(name.to_string()).or_insert(0);
        if *counter + quantity > 100 {
            return Err("Quantity exceeds the maximum limit of 100".into());
        }
        *counter += quantity;
        Ok(())
    }

    pub fn update_item(&mut self, name: &str, quantity: u32) -> Result<(), &'static str> {
        match self.items.get_mut(name) {
            Some(counter) if (1..=100).contains(&quantity) => {
                *counter = quantity;
                Ok(())
            }
            Some(_) => Err("Quantity must be between 1 and 100"),
            None => Err("Item not found in the cart"),
        }
    }

    pub fn remove_item(&mut self, name: &str) -> Result<(), &'static str> {
        if self.items.remove(name).is_some() {
            Ok(())
        } else {
            Err("Item not found in the cart")
        }
    }

    pub fn get_total_cost(&self) -> f64 {
        self.items
            .iter()
            .filter_map(|(name, &quantity)| {
                self.catalog
                    .get_price(name)
                    .map(|price| price * quantity as f64)
            })
            .sum()
    }
}
