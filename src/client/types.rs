use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub servings: Option<i32>,
    pub working_time: Option<i32>,
    pub waiting_time: Option<i32>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub internal: bool,
    pub show_ingredient_overview: bool,
    pub keywords: Vec<Keyword>,
    pub steps: Vec<Step>,
    pub nutrition: Option<Nutrition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "created_at")]
    pub created: DateTime<Utc>,
    #[serde(rename = "updated_at")]
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Step {
    pub id: i32,
    pub name: String,
    pub instruction: String,
    pub ingredients: Vec<StepIngredient>,
    pub time: Option<i32>,
    pub order: i32,
    pub file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepIngredient {
    pub id: i32,
    pub food: Food,
    pub unit: Option<Unit>,
    pub amount: f64,
    pub note: Option<String>,
    pub order: i32,
    pub is_header: bool,
    pub no_amount: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Food {
    pub id: i32,
    pub name: String,
    pub plural_name: Option<String>,
    pub description: Option<String>,
    pub recipe: Option<i32>,
    pub food_onhand: bool,
    pub supermarket_category: Option<i32>,
    pub inherit_fields: Vec<InheritField>,
    pub properties: Vec<FoodProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub plural_name: Option<String>,
    pub description: Option<String>,
    pub base_unit: Option<String>,
    pub type_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InheritField {
    pub id: i32,
    pub name: String,
    pub field: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FoodProperty {
    pub id: i32,
    pub property_amount: f64,
    pub property_type: PropertyType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyType {
    pub id: i32,
    pub name: String,
    pub unit: String,
    pub order: i32,
    pub fdc_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nutrition {
    pub calories: Option<f64>,
    pub proteins: Option<f64>,
    pub fats: Option<f64>,
    pub carbs: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShoppingListEntry {
    pub id: i32,
    pub food: Food,
    pub unit: Option<Unit>,
    pub amount: f64,
    pub order: i32,
    pub checked: bool,
    pub created: DateTime<Utc>,
    pub completed: Option<DateTime<Utc>>,
    pub delay_until: Option<DateTime<Utc>>,
    pub created_by: i32,
    pub completed_by: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealPlan {
    pub id: i32,
    pub title: Option<String>,
    pub recipe: Option<Recipe>,
    pub servings: i32,
    pub note: Option<String>,
    pub date: chrono::NaiveDate,
    pub meal_type: MealType,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MealType {
    pub id: i32,
    pub name: String,
    pub order: i32,
    pub color: String,
    pub default: bool,
    pub created_by: i32,
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CookLog {
    pub id: i32,
    pub recipe: Recipe,
    pub servings: i32,
    pub rating: Option<i32>,
    pub comment: Option<String>,
    pub created: DateTime<Utc>,
    pub created_by: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeImport {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

// Request types for creating/updating resources
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRecipeRequest {
    pub name: String,
    pub description: Option<String>,
    pub servings: Option<i32>,
    pub working_time: i32,
    pub waiting_time: i32,
    pub keywords: Vec<CreateKeywordRequest>,
    pub steps: Vec<CreateStepRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateKeywordRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStepRequest {
    pub name: Option<String>,
    pub instruction: String,
    pub ingredients: Vec<CreateStepIngredientRequest>,
    pub time: Option<i32>,
    pub order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStepIngredientRequest {
    pub food: CreateFoodRequest,
    pub unit: Option<CreateUnitRequest>,
    pub amount: String,
    pub note: Option<String>,
    pub order: i32,
    pub is_header: bool,
    pub no_amount: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFoodRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUnitRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateShoppingListEntryRequest {
    pub food: i32,
    pub unit: Option<i32>,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMealPlanRequest {
    pub recipe: Option<i32>,
    pub title: Option<String>,
    pub servings: i32,
    pub date: chrono::NaiveDate,
    pub meal_type: i32,
    pub note: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCookLogRequest {
    pub recipe: i32,
    pub servings: i32,
    pub rating: Option<i32>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateFoodRequest {
    pub food_onhand: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateShoppingListEntryRequest {
    pub checked: Option<bool>,
    pub amount: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct BulkShoppingListRequest {
    pub entries: Vec<CreateShoppingListEntryRequest>,
}