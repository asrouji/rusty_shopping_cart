use crate::catalog::Catalog;
use regex::Regex;
use std::collections::HashMap;
use uuid::Uuid;

/// The maximum number of any given item that can be added to the shopping cart.
const MAX_ITEM_COUNT: u32 = 100;

/// Represents a shopping cart with a unique ID, customer ID, items, and a catalog.
#[derive(Debug)]
pub struct ShoppingCart {
    id: Uuid,
    customer_id: String,
    items: HashMap<String, u32>,
    catalog: Catalog,
}

impl ShoppingCart {
    /// Creates a new shopping cart for the given customer ID.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - A string slice that holds the customer ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the customer ID format is invalid.
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

    /// Validates the format of the customer ID.
    ///
    /// # Arguments
    ///
    /// * `customer_id` - A string slice that holds the customer ID.
    ///
    /// # Returns
    ///
    /// `true` if the customer ID format is valid, otherwise `false`.
    fn is_valid_customer_id(customer_id: &str) -> bool {
        let re = Regex::new(r"^\p{L}{3}\d{5}\p{L}{2}-[AQ]$").unwrap();
        re.is_match(customer_id)
    }

    /// Returns the unique ID of the shopping cart.
    pub fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the customer ID associated with the shopping cart.
    pub fn customer_id(&self) -> &str {
        &self.customer_id
    }

    /// Returns a reference to the items in the shopping cart.
    pub fn items(&self) -> &HashMap<String, u32> {
        &self.items
    }

    /// Adds an item to the shopping cart.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the item to add.
    /// * `quantity` - The quantity of the item to add.
    ///
    /// # Errors
    ///
    /// Returns an error if the quantity is zero, the item is not found in the catalog,
    /// or the quantity exceeds the maximum limit.
    pub fn add_item(&mut self, name: &str, quantity: u32) -> Result<(), String> {
        if quantity == 0 {
            return Err("Quantity must be nonzero".into());
        }
        if !self.catalog.has_item(name) {
            return Err(format!("Item not found in the catalog: {}", name));
        }
        let counter = self.items.entry(name.to_string()).or_insert(0);
        if *counter + quantity > MAX_ITEM_COUNT {
            return Err(format!("Quantity exceeds the limit of {}", MAX_ITEM_COUNT));
        }
        *counter += quantity;
        Ok(())
    }

    /// Updates the quantity of an item in the shopping cart.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the item to update.
    /// * `quantity` - The new quantity of the item.
    ///
    /// # Errors
    ///
    /// Returns an error if the item is not found in the cart or the quantity is out of range.
    pub fn update_item(&mut self, name: &str, quantity: u32) -> Result<(), String> {
        match self.items.get_mut(name) {
            Some(counter) if (1..=MAX_ITEM_COUNT).contains(&quantity) => {
                *counter = quantity;
                Ok(())
            }
            Some(_) => Err(format!("Quantity must be between 1 and {}", MAX_ITEM_COUNT)),
            None => Err(format!("Item not found in the cart: {}", name)),
        }
    }

    /// Removes an item from the shopping cart.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the item to remove.
    ///
    /// # Errors
    ///
    /// Returns an error if the item is not found in the cart.
    pub fn remove_item(&mut self, name: &str) -> Result<(), &'static str> {
        if self.items.remove(name).is_some() {
            Ok(())
        } else {
            Err("Item not found in the cart")
        }
    }

    /// Calculates the total cost of the items in the shopping cart.
    ///
    /// # Returns
    ///
    /// The total cost of the items in the shopping cart.
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
