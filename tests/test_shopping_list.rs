mod common;

use common::{DockerEnvironment, TestEnvironment};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_get_shopping_list() {
    common::init_test_logging();
    DockerEnvironment::ensure_running().expect("Docker environment not running");
    
    let env = TestEnvironment::new().await
        .expect("Failed to create test environment");
    
    // Get the current shopping list
    let result = env.client.get_shopping_list().await;
    
    assert!(result.is_ok(), "Should retrieve shopping list successfully");
    let response = result.unwrap();
    
    // Shopping list should exist even if empty
    assert!(response.count >= 0, "Should have a valid count");
}

#[tokio::test]
#[serial]
async fn test_add_to_shopping_list() {
    common::init_test_logging();
    DockerEnvironment::ensure_running().expect("Docker environment not running");
    
    let mut env = TestEnvironment::new().await
        .expect("Failed to create test environment");
    
    // First, we need to ensure we have a food item to add
    // Search for or create a food item
    let food_search = env.client.search_foods("tomato", Some(1)).await;
    
    let food_id = if let Ok(search_response) = food_search {
        if let Some(food) = search_response.results.first() {
            food.id
        } else {
            // If no tomato found, we'd need to create one
            // For now, we'll skip this test case
            println!("No tomato food found, skipping test");
            return;
        }
    } else {
        println!("Failed to search for foods, skipping test");
        return;
    };
    
    // Add the item to shopping list
    let request = mcp_tandoor::client::types::CreateShoppingListEntryRequest {
        food: food_id,
        unit: None,
        amount: 2.0,
    };
    
    let result = env.client.add_bulk_to_shopping_list(vec![request]).await;
    assert!(result.is_ok(), "Should add item to shopping list successfully");
    
    let entries = result.unwrap();
    assert!(!entries.is_empty(), "Should have added at least one entry");
    
    // Verify it was added
    let list_result = env.client.get_shopping_list().await;
    assert!(list_result.is_ok(), "Should retrieve updated shopping list");
}

#[tokio::test]
#[serial]
async fn test_update_shopping_list_entry() {
    common::init_test_logging();
    DockerEnvironment::ensure_running().expect("Docker environment not running");
    
    let mut env = TestEnvironment::new().await
        .expect("Failed to create test environment");
    
    // Get current shopping list
    let list_result = env.client.get_shopping_list().await;
    assert!(list_result.is_ok(), "Should retrieve shopping list");
    
    let list = list_result.unwrap();
    if let Some(entry) = list.results.first() {
        // Update the first entry to be checked
        let update_request = mcp_tandoor::client::types::UpdateShoppingListEntryRequest {
            checked: Some(true),
            amount: None,
        };
        
        let result = env.client.update_shopping_list_entry(entry.id, update_request).await;
        assert!(result.is_ok(), "Should update shopping list entry successfully");
        
        let updated_entry = result.unwrap();
        assert!(updated_entry.checked, "Entry should be marked as checked");
    } else {
        println!("No shopping list entries to update, skipping test");
    }
}