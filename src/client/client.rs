use anyhow::Result;
use reqwest::Client;
use crate::client::{
    auth::TandoorAuth,
    types::*,
};

pub struct TandoorClient {
    base_url: String,
    client: Client,
    auth: TandoorAuth,
}

impl TandoorClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            auth: TandoorAuth::new(base_url.clone()),
            base_url,
        }
    }

    pub async fn authenticate(&mut self, username: String, password: String) -> Result<()> {
        self.auth.authenticate(username, password).await
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.auth.is_authenticated()
    }
    
    pub fn get_token_preview(&self) -> Option<String> {
        self.auth.get_token().map(|t| format!("{}...", &t[..t.len().min(10)]))
    }
    
    pub fn set_token(&mut self, token: String) {
        self.auth.set_token(token);
    }
    
    pub fn get_token(&self) -> Option<&str> {
        self.auth.get_token()
    }

    fn get_auth_header(&self) -> Result<String> {
        match self.auth.get_token() {
            Some(token) => {
                tracing::debug!("Using authentication token: {}...", &token[..token.len().min(10)]);
                Ok(format!("Bearer {}", token))
            },
            None => {
                tracing::error!("Attempted to make authenticated request without valid token");
                anyhow::bail!("Not authenticated - please verify credentials and re-authenticate")
            },
        }
    }

    // Recipe operations
    pub async fn search_recipes(&self, query: Option<&str>, limit: Option<i32>) -> Result<PaginatedResponse<Recipe>> {
        let auth_header = self.get_auth_header()?;
        let mut url = format!("{}/api/recipe/", self.base_url);
        
        let mut params = vec![];
        if let Some(q) = query {
            params.push(format!("query={}", urlencoding::encode(q)));
        }
        if let Some(l) = limit {
            params.push(format!("page_size={}", l));
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to search recipes: {}", response.status());
        }

        let recipes = response.json().await?;
        Ok(recipes)
    }

    pub async fn get_recipe(&self, id: i32) -> Result<Recipe> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/recipe/{}/", self.base_url, id);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get recipe: {}", response.status());
        }

        let recipe = response.json().await?;
        Ok(recipe)
    }

    pub async fn create_recipe(&self, request: CreateRecipeRequest) -> Result<Recipe> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/recipe/", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to create recipe: {}", response.status());
        }

        let recipe = response.json().await?;
        Ok(recipe)
    }

    pub async fn import_recipe_from_url(&self, url: &str) -> Result<Recipe> {
        let auth_header = self.get_auth_header()?;
        let import_url = format!("{}/api/recipe-from-source/", self.base_url);
        let request = RecipeImport { url: url.to_string() };

        let response = self
            .client
            .post(&import_url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to import recipe: {}", response.status());
        }

        let recipe = response.json().await?;
        Ok(recipe)
    }

    // Food operations
    pub async fn search_foods(&self, query: &str, limit: Option<i32>) -> Result<PaginatedResponse<Food>> {
        let auth_header = self.get_auth_header()?;
        let mut url = format!("{}/api/food/?query={}", self.base_url, urlencoding::encode(query));
        
        if let Some(l) = limit {
            url.push_str(&format!("&page_size={}", l));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to search foods: {}", response.status());
        }

        let foods = response.json().await?;
        Ok(foods)
    }

    pub async fn update_food_availability(&self, food_id: i32, available: bool) -> Result<Food> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/food/{}/", self.base_url, food_id);
        let request = UpdateFoodRequest { food_onhand: Some(available) };

        let response = self
            .client
            .patch(&url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to update food availability: {}", response.status());
        }

        let food = response.json().await?;
        Ok(food)
    }

    // Shopping list operations
    pub async fn get_shopping_list(&self) -> Result<PaginatedResponse<ShoppingListEntry>> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/shopping-list-entry/", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get shopping list: {}", response.status());
        }

        let shopping_list = response.json().await?;
        Ok(shopping_list)
    }

    pub async fn add_to_shopping_list(&self, request: CreateShoppingListEntryRequest) -> Result<ShoppingListEntry> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/shopping-list-entry/", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to add to shopping list: {}", response.status());
        }

        let entry = response.json().await?;
        Ok(entry)
    }

    pub async fn add_bulk_to_shopping_list(&self, entries: Vec<CreateShoppingListEntryRequest>) -> Result<Vec<ShoppingListEntry>> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/shopping-list-entry/bulk/", self.base_url);
        let request = BulkShoppingListRequest { entries };

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to add bulk to shopping list: {}", response.status());
        }

        let entries = response.json().await?;
        Ok(entries)
    }

    pub async fn update_shopping_list_entry(&self, entry_id: i32, request: UpdateShoppingListEntryRequest) -> Result<ShoppingListEntry> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/shopping-list-entry/{}/", self.base_url, entry_id);

        let response = self
            .client
            .patch(&url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to update shopping list entry: {}", response.status());
        }

        let entry = response.json().await?;
        Ok(entry)
    }

    pub async fn delete_shopping_list_entry(&self, entry_id: i32) -> Result<()> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/shopping-list-entry/{}/", self.base_url, entry_id);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to delete shopping list entry: {}", response.status());
        }

        Ok(())
    }

    // Meal planning operations
    pub async fn get_meal_plans(&self, from_date: Option<&str>, to_date: Option<&str>) -> Result<PaginatedResponse<MealPlan>> {
        let auth_header = self.get_auth_header()?;
        let mut url = format!("{}/api/meal-plan/", self.base_url);
        
        let mut params = vec![];
        if let Some(from) = from_date {
            params.push(format!("from_date={}", from));
        }
        if let Some(to) = to_date {
            params.push(format!("to_date={}", to));
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get meal plans: {}", response.status());
        }

        let meal_plans = response.json().await?;
        Ok(meal_plans)
    }

    pub async fn create_meal_plan(&self, request: CreateMealPlanRequest) -> Result<MealPlan> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/meal-plan/", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to create meal plan: {}", response.status());
        }

        let meal_plan = response.json().await?;
        Ok(meal_plan)
    }

    pub async fn delete_meal_plan(&self, plan_id: i32) -> Result<()> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/meal-plan/{}/", self.base_url, plan_id);

        let response = self
            .client
            .delete(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to delete meal plan: {}", response.status());
        }

        Ok(())
    }

    // Meal types
    pub async fn get_meal_types(&self) -> Result<PaginatedResponse<MealType>> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/meal-type/", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get meal types: {}", response.status());
        }

        let meal_types = response.json().await?;
        Ok(meal_types)
    }

    // Cook log operations
    pub async fn get_cook_log(&self, recipe_id: Option<i32>, days_back: Option<i32>) -> Result<PaginatedResponse<CookLog>> {
        let auth_header = self.get_auth_header()?;
        let mut url = format!("{}/api/cook-log/", self.base_url);
        
        let mut params = vec![];
        if let Some(recipe) = recipe_id {
            params.push(format!("recipe={}", recipe));
        }
        if let Some(days) = days_back {
            // Calculate from_date based on days_back
            let from_date = chrono::Utc::now() - chrono::Duration::days(days as i64);
            params.push(format!("from_date={}", from_date.format("%Y-%m-%d")));
        }
        
        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get cook log: {}", response.status());
        }

        let cook_log = response.json().await?;
        Ok(cook_log)
    }

    pub async fn log_cooked_recipe(&self, request: CreateCookLogRequest) -> Result<CookLog> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/cook-log/", self.base_url);

        let response = self
            .client
            .post(&url)
            .header("Authorization", auth_header)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to log cooked recipe: {}", response.status());
        }

        let cook_log = response.json().await?;
        Ok(cook_log)
    }

    // Utility operations
    pub async fn get_keywords(&self) -> Result<PaginatedResponse<Keyword>> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/keyword/", self.base_url);
        
        tracing::debug!("Making request to get keywords: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Network error getting keywords: {}", e);
                anyhow::anyhow!("Failed to connect to Tandoor API: {}", e)
            })?;

        let status = response.status();
        tracing::debug!("Keywords response status: {}", status);
        
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error response".to_string());
            tracing::error!("Failed to get keywords with status {}: {}", status, error_body);
            
            match status.as_u16() {
                401 => anyhow::bail!("Authentication expired or invalid. Please re-authenticate."),
                403 => anyhow::bail!("Access denied to keywords endpoint. Check user permissions."),
                404 => anyhow::bail!("Keywords endpoint not found. Check Tandoor version and API availability."),
                500..=599 => anyhow::bail!("Tandoor server error getting keywords ({}): {}", status, error_body),
                _ => anyhow::bail!("Failed to get keywords with status {}: {}", status, error_body),
            }
        }

        let keywords: PaginatedResponse<Keyword> = response.json().await
            .map_err(|e| {
                tracing::error!("Failed to parse keywords response: {}", e);
                anyhow::anyhow!("Invalid response format from Tandoor server: {}", e)
            })?;
        
        tracing::debug!("Successfully retrieved {} keywords", keywords.count);
        Ok(keywords)
    }

    pub async fn get_units(&self) -> Result<PaginatedResponse<Unit>> {
        let auth_header = self.get_auth_header()?;
        let url = format!("{}/api/unit/", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", auth_header)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get units: {}", response.status());
        }

        let units = response.json().await?;
        Ok(units)
    }
}