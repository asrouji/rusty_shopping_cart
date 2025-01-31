use rusty_shopping_cart::ShoppingCart;

#[test]
fn test_new_cart() {
    let cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.id(), cart.id());
    assert_eq!(cart.customer_id(), "abc12345de-A");
    assert_eq!(cart.items().len(), 0);
}

#[test]
fn test_id_immutable() {
    let cart = ShoppingCart::new("abc12345de-A").unwrap();
    let original_id = cart.id();
    // uuid implements the Copy trait, modifying this should not affect the original id
    let mut _id = cart.id();
    _id = uuid::Uuid::new_v4();
    // Ensure that the original id in the cart is not modified
    assert_eq!(cart.id(), original_id);
}

#[test]
fn test_customer_id_immutable() {
    let cart = ShoppingCart::new("abc12345de-A").unwrap();
    // &str is immutable, so we can't even modify it without calling to_string()
    let mut customer_id = cart.customer_id().to_string();
    customer_id.push_str("garbage");
    // ensure that the original customer_id in the cart is not modified
    assert_eq!(cart.customer_id(), "abc12345de-A");
}

#[test]
fn test_items_immutable() {
    let cart = ShoppingCart::new("abc12345de-A").unwrap();
    let items = cart.items();
    // without the clone line below, Rust will not allow us to modify the items!
    let mut items = items.clone();
    // because this is a clone, modifying it will not affect the original cart
    items.insert("Laptop".to_string(), 1);
    assert_eq!(cart.items().len(), 0);
}

#[test]
fn test_invalid_customer_id() {
    let result = ShoppingCart::new("abc12345de");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Invalid customer ID format");
}

#[test]
fn test_add_item() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.items().len(), 0);

    cart.add_item("Laptop", 1).unwrap();
    assert_eq!(cart.items().len(), 1);

    cart.add_item("Laptop", 1).unwrap();
    assert_eq!(cart.items().len(), 1);

    cart.add_item("Mouse", 1).unwrap();
    assert_eq!(cart.items().len(), 2);

    cart.add_item("Keyboard", 1).unwrap();
    assert_eq!(cart.items().len(), 3);

    cart.add_item("Monitor", 1).unwrap();
    assert_eq!(cart.items().len(), 4);

    cart.add_item("Headphones", 1).unwrap();
    assert_eq!(cart.items().len(), 5);

    assert_eq!(cart.items().get("Laptop").unwrap(), &2);
    assert_eq!(cart.items().get("Mouse").unwrap(), &1);
    assert_eq!(cart.items().get("Keyboard").unwrap(), &1);
    assert_eq!(cart.items().get("Monitor").unwrap(), &1);
    assert_eq!(cart.items().get("Headphones").unwrap(), &1);
}

#[test]
fn test_add_item_not_found() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.items().len(), 0);

    let result = cart.add_item("Tablet", 1);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Item not found in the catalog: 'Tablet'"
    );
}

#[test]
fn test_add_item_zero_quantity() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    let result = cart.add_item("Laptop", 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Quantity must be nonzero");
}

#[test]
fn test_add_item_exceeds_limit() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();

    cart.add_item("Laptop", 50).unwrap();
    let result = cart.add_item("Laptop", 51);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Quantity exceeds the limit of 100");

    let result = cart.add_item("Laptop", 101);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Quantity exceeds the limit of 100");
}

#[test]
fn test_update_item() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.items().len(), 0);

    cart.add_item("Laptop", 1).unwrap();
    cart.add_item("Mouse", 1).unwrap();
    cart.add_item("Keyboard", 1).unwrap();
    cart.add_item("Monitor", 1).unwrap();
    cart.add_item("Headphones", 1).unwrap();

    assert_eq!(cart.items().get("Laptop").unwrap(), &1);
    assert_eq!(cart.items().get("Mouse").unwrap(), &1);
    assert_eq!(cart.items().get("Keyboard").unwrap(), &1);
    assert_eq!(cart.items().get("Monitor").unwrap(), &1);
    assert_eq!(cart.items().get("Headphones").unwrap(), &1);

    cart.update_item("Laptop", 2).unwrap();
    cart.update_item("Mouse", 2).unwrap();
    cart.update_item("Keyboard", 2).unwrap();
    cart.update_item("Monitor", 2).unwrap();
    cart.update_item("Headphones", 2).unwrap();

    assert_eq!(cart.items().get("Laptop").unwrap(), &2);
    assert_eq!(cart.items().get("Mouse").unwrap(), &2);
    assert_eq!(cart.items().get("Keyboard").unwrap(), &2);
    assert_eq!(cart.items().get("Monitor").unwrap(), &2);
    assert_eq!(cart.items().get("Headphones").unwrap(), &2);
}

#[test]
fn test_update_item_not_found() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.items().len(), 0);

    let result = cart.update_item("Tablet", 1);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Item not found in the cart: 'Tablet'");
}

#[test]
fn test_update_item_zero_quantity() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    cart.add_item("Laptop", 1).unwrap();
    let result = cart.update_item("Laptop", 0);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Quantity must be between 1 and 100");
}

#[test]
fn test_update_item_exceeds_limit() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    cart.add_item("Laptop", 50).unwrap();
    let result = cart.update_item("Laptop", 101);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Quantity must be between 1 and 100");
}

#[test]
fn test_remove_item() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.items().len(), 0);

    cart.add_item("Laptop", 1).unwrap();
    cart.add_item("Mouse", 1).unwrap();
    cart.add_item("Keyboard", 1).unwrap();
    cart.add_item("Monitor", 1).unwrap();
    cart.add_item("Headphones", 1).unwrap();

    assert_eq!(cart.items().len(), 5);

    cart.remove_item("Laptop").unwrap();
    assert_eq!(cart.items().len(), 4);

    cart.remove_item("Mouse").unwrap();
    assert_eq!(cart.items().len(), 3);

    cart.remove_item("Keyboard").unwrap();
    assert_eq!(cart.items().len(), 2);

    cart.remove_item("Monitor").unwrap();
    assert_eq!(cart.items().len(), 1);

    cart.remove_item("Headphones").unwrap();
    assert_eq!(cart.items().len(), 0);
}

#[test]
fn test_remove_item_not_found() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.items().len(), 0);

    let result = cart.remove_item("Tablet");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Item not found in the cart: 'Tablet'");
}

#[test]
fn test_get_total_cost() {
    let mut cart = ShoppingCart::new("abc12345de-A").unwrap();
    assert_eq!(cart.get_total_cost(), 0.0);

    cart.add_item("Laptop", 1).unwrap();
    assert_eq!(cart.get_total_cost(), 999.99);

    cart.add_item("Mouse", 1).unwrap();
    assert_eq!(cart.get_total_cost(), 1025.98);

    cart.add_item("Keyboard", 1).unwrap();
    assert_eq!(cart.get_total_cost(), 1075.97);

    cart.add_item("Monitor", 1).unwrap();
    assert_eq!(cart.get_total_cost(), 1275.96);

    cart.add_item("Headphones", 1).unwrap();
    assert_eq!(cart.get_total_cost(), 1365.95);

    cart.add_item("Laptop", 1).unwrap();
    cart.add_item("Mouse", 1).unwrap();
    cart.add_item("Keyboard", 1).unwrap();
    cart.add_item("Monitor", 1).unwrap();
    cart.add_item("Headphones", 1).unwrap();

    assert_eq!(cart.get_total_cost(), 2731.90);
}
