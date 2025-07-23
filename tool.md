# Tandoor MCP Tools

## Design Philosophy

These tools provide primitive operations that LLMs can orchestrate to create sophisticated meal planning experiences. Key principles:

1. **Simple primitives** - Each tool does one thing well, LLM handles complex orchestration
2. **Predictable behavior** - No hidden AI logic, just clean data operations
3. **Batch operations** - Support arrays to minimize round trips
4. **Smart consolidation** - Shopping lists auto-merge duplicates and check inventory
5. **LLM-friendly returns** - Structured data the LLM can reason about

The MCP server provides the building blocks; the LLM provides the intelligence to combine them based on user needs.

## Recipe Tools

### search_recipes

**Description**: Search for recipes with flexible querying. Supports both structured filters and natural language.

**Input Parameters**:

- `query` (string, optional): Natural language search like "quick vegetarian pasta" - will be parsed server-side
- `filters` (object, optional): Structured filters:
  - `keywords` (array of strings): Must have these keywords
  - `max_time` (integer): Maximum cooking time in minutes
  - `min_rating` (number): Minimum rating (0-5)
  - `has_ingredients` (array of strings): Must include these
  - `exclude_ingredients` (array of strings): Must not include these
- `context` (object, optional): Help the search be smarter:
  - `avoid_recent` (boolean): Skip recipes made in last 7 days
  - `prefer_available` (boolean): Prioritize recipes with on-hand ingredients
- `limit` (integer, optional, default: 20): Maximum results

**Returns**: 
```json
{
  "recipes": [
    {
      "id": 1,
      "name": "Recipe Name",
      "total_time": 30,
      "rating": 4.5,
      "keywords": ["vegetarian", "quick"],
      "missing_ingredients": ["basil"],
      "match_score": 0.95
    }
  ],
  "search_interpretation": "Searched for vegetarian recipes under 30 minutes"
}
```

**API Routes Used**:

- GET `/api/recipe/` (with query parameters)

### get_recipe_details

**Description**: Get comprehensive recipe information including scaled ingredients, nutritional data, and user history.

**Input Parameters**:

- `id` (integer, required): Recipe ID
- `servings` (integer, optional): Scale ingredients to this serving size

**Returns**: Full recipe with ingredients scaled, nutrition calculated, and personal history

**API Routes Used**:

- GET `/api/recipe/{id}/`
- GET `/api/cook-log/?recipe={id}` (for personal history)

## Shopping List Tools

### add_to_shopping_list

**Description**: Intelligently add items to shopping list. Handles consolidation, inventory checking, and recipe parsing.

**Input Parameters**:

- `request` (string, optional): Natural language like "add ingredients for chicken stir fry for 4 people"
- `items` (array, optional): Structured items:
  - `name` (string): Food name (will be fuzzy matched)
  - `amount` (number, optional): Quantity 
  - `unit` (string, optional): Unit of measurement
- `from_recipe` (object, optional): Add from recipe:
  - `recipe_id` (integer): Recipe to add
  - `servings` (integer, optional): Scale for this many servings
- `options` (object, optional):
  - `check_pantry` (boolean, default true): Skip items already available
  - `consolidate` (boolean, default true): Merge with existing list items

**Returns**:

```json
{
  "added": [
    { "food": "Tomatoes", "amount": 3, "unit": "lbs", "status": "added" }
  ],
  "skipped": [
    { "food": "Olive Oil", "reason": "already in pantry" }
  ],
  "consolidated": [
    { "food": "Onions", "previous": 2, "new_total": 4 }
  ],
  "summary": "Added 3 items, consolidated 2, skipped 1"
}
```

**API Routes Used**:

- GET `/api/food/` (to resolve food names)
- GET `/api/shopping-list-entry/`
- POST `/api/shopping-list-entry/bulk/`
- GET `/api/food/?food_onhand=true` (check inventory)

### get_shopping_list

**Description**: Get current shopping list organized by store section with availability status.

**Input Parameters**:

- `format` (string, optional): "grouped" (by category) or "flat" (default)

**Returns**: Shopping list with items marked as available/needed and grouped by category

**API Routes Used**:

- GET `/api/shopping-list-entry/`
- GET `/api/food/` (for category information)

### check_shopping_items

**Description**: Mark multiple shopping list items as purchased.

**Input Parameters**:

- `items` (array, required): Food names or IDs to mark as checked

**Returns**: Updated shopping list status

**API Routes Used**:

- PATCH `/api/shopping-list-entry/{id}/`

### clear_shopping_list

**Description**: Remove all checked items and update pantry inventory automatically.

**Built-in Logic**:

- Marks checked items as `food_onhand=true` in inventory
- Removes them from shopping list
- Returns summary of pantry updates

**Returns**:
```json
{
  "removed_items": [
    {
      "id": 123,
      "food": "Tomatoes",
      "amount": 3,
      "unit": "lbs",
      "was_checked": true
    }
  ],
  "pantry_updates": ["Tomatoes", "Onions"],
  "summary": "Removed 5 checked items, updated pantry"
}
```

**API Routes Used**:

- DELETE `/api/shopping-list-entry/{id}/`
- PATCH `/api/food/{id}/` (to update onhand status)

## Meal Planning Tools

### plan_meals

**Description**: Create meal plans with intelligent suggestions. Can handle requests like "plan 5 dinners this week" or specific assignments.

**Input Parameters**:

- `request` (string, optional): Natural language like "plan 5 dinners for next week, avoiding pasta"
- `specific_plans` (array, optional): Exact meal plans:
  - `recipe_id` (integer): Recipe to plan
  - `date` (string): Date YYYY-MM-DD
  - `meal_type` (string): "breakfast", "lunch", "dinner", "snack"
  - `servings` (integer, optional): Number of servings
- `constraints` (object, optional):
  - `start_date` (string): Week start date
  - `avoid_keywords` (array): Don't suggest recipes with these keywords
  - `prefer_quick` (boolean): Favor recipes under 30 minutes
  - `use_inventory` (boolean): Prioritize using on-hand ingredients

**Returns**:
```json
{
  "created": [
    {
      "id": 123,
      "date": "2024-01-22",
      "meal_type": "dinner", 
      "recipe_name": "Chicken Stir Fry",
      "servings": 4
    }
  ],
  "suggestions": [
    {
      "recipe_id": 456,
      "recipe_name": "Quick Pasta",
      "reason": "Uses pantry ingredients, 20 min cook time"
    }
  ],
  "shopping_needed": ["bell peppers", "ginger"]
}
```

**API Routes Used**:

- GET `/api/recipe/` (for suggestions)
- GET `/api/cook-log/` (avoid recent)
- POST `/api/meal-plan/` (create plans)

### get_meal_plans

**Description**: Get meal plans for a date range.

**Input Parameters**:

- `from_date` (string, required): Start date YYYY-MM-DD
- `to_date` (string, required): End date YYYY-MM-DD
- `meal_type` (string, optional): Filter by meal type

**Returns**: Array of meal plan objects

**API Routes Used**:

- GET `/api/meal-plan/`

### delete_meal_plan

**Description**: Remove a meal plan entry.

**Input Parameters**:

- `id` (integer, required): Meal plan ID

**Returns**: 
```json
{
  "deleted": {
    "id": 123,
    "date": "2024-01-22",
    "meal_type": "dinner",
    "recipe_id": 456,
    "recipe_name": "Chicken Stir Fry",
    "servings": 4
  },
  "success": true
}
```

**API Routes Used**:

- GET `/api/meal-plan/{id}/` (to capture before delete)
- DELETE `/api/meal-plan/{id}/`

### get_meal_types

**Description**: Get available meal types for planning.

**Input Parameters**: None

**Returns**: Array of meal types (breakfast, lunch, dinner, etc.)

**API Routes Used**:

- GET `/api/meal-type/`

## Inventory Management Tools

### update_pantry

**Description**: Update what's available in your pantry/fridge. Accepts partial names and handles common variations.

**Input Parameters**:

- `items` (array, required): Array of:
  - `food` (string): Food name (fuzzy matched)
  - `available` (boolean): Whether it's in stock
  - `amount` (number, optional): Quantity on hand
  - `expires` (string, optional): Expiration date

**Returns**: Updated inventory summary

**API Routes Used**:

- GET `/api/food/` (resolve names)
- PATCH `/api/food/{id}/`

### suggest_from_inventory

**Description**: Get recipes that use ingredients expiring soon or maximize use of current inventory.

**Input Parameters**:

- `mode` (string, optional): "expiring" or "maximum-use" (default)
- `days_until_expiry` (integer, optional): For expiring mode (default 3)

**Returns**: Recipes prioritized by inventory usage

**API Routes Used**:

- GET `/api/food/?food_onhand=true`
- GET `/api/recipe/?makenow=true`

## Recipe History Tools

### get_cook_log

**Description**: Get cooking history to check what's been made recently.

**Input Parameters**:

- `days_back` (integer, optional, default: 30): How many days of history
- `recipe_id` (integer, optional): Filter for specific recipe

**Returns**: Array of cook log entries with recipe_id, date, rating

**API Routes Used**:

- GET `/api/cook-log/`

### log_cooked_recipe

**Description**: Record that a recipe was cooked.

**Input Parameters**:

- `recipe_id` (integer, required): Recipe ID
- `date` (string, optional): Date cooked (defaults to today)
- `rating` (integer, optional): Rating 1-5

**Returns**: Created cook log entry

**API Routes Used**:

- POST `/api/cook-log/`


## Utility Tools

### import_recipe_from_url

**Description**: Import a recipe from an external URL.

**Input Parameters**:

- `url` (string, required): Recipe URL to import

**Returns**: Created recipe object or import status

**API Routes Used**:

- POST `/api/recipe-from-source/`

### get_keywords

**Description**: Get all available recipe keywords/tags.

**Input Parameters**: None

**Returns**: Array of keyword objects with id and name

**API Routes Used**:

- GET `/api/keyword/`

### get_recipe_books

**Description**: Get all recipe books/collections.

**Input Parameters**: None

**Returns**: Array of recipe book objects

**API Routes Used**:

- GET `/api/recipe-book/`

## Food Management Tools

### search_foods

**Description**: Find foods/ingredients with fuzzy name matching.

**Input Parameters**:

- `query` (string, required): Food name to search for
- `limit` (integer, optional, default: 10): Maximum results

**Returns**: Array of food objects with id, name, properties

**API Routes Used**:

- GET `/api/food/`

### create_food

**Description**: Add a new food/ingredient to the database.

**Input Parameters**:

- `name` (string, required): Food name
- `plural_name` (string, optional): Plural form
- `category` (string, optional): Food category
- `properties` (object, optional): Nutritional data per 100g

**Returns**: Created food object

**API Routes Used**:

- POST `/api/food/`

### update_food_availability

**Description**: Update what foods are currently available in pantry.

**Input Parameters**:

- `updates` (array, required): Array of:
  - `food_id` (integer): Food ID  
  - `available` (boolean): Whether it's in stock
  - `amount` (number, optional): Quantity on hand
  - `unit` (string, optional): Unit of measurement

**Returns**: Updated food availability status

**API Routes Used**:

- PATCH `/api/food/{id}/`

## Recipe Management Tools

### create_recipe

**Description**: Create a new recipe.

**Input Parameters**:

- `name` (string, required): Recipe name
- `description` (string, optional): Recipe description  
- `instructions` (string, required): Cooking steps
- `ingredients` (array, required): Array of:
  - `food` (string): Ingredient name
  - `amount` (number): Quantity
  - `unit` (string): Unit of measurement
- `prep_time` (integer, optional): Prep time in minutes
- `cook_time` (integer, optional): Cook time in minutes
- `servings` (integer, optional): Number of servings
- `keywords` (array of strings, optional): Tags like "vegetarian", "quick"

**Returns**: Created recipe object with assigned ID

**API Routes Used**:

- POST `/api/recipe/`

### update_recipe

**Description**: Modify an existing recipe.

**Input Parameters**:

- `recipe_id` (integer, required): Recipe ID
- Same optional fields as create_recipe for partial updates

**Returns**: Updated recipe object

**API Routes Used**:

- PATCH `/api/recipe/{id}/`

### rate_recipe

**Description**: Give a rating to a recipe you've tried.

**Input Parameters**:

- `recipe_id` (integer, required): Recipe ID
- `rating` (number, required): Rating from 1-5
- `comment` (string, optional): Optional review comment

**Returns**: Updated recipe with new rating

**API Routes Used**:

- PATCH `/api/recipe/{id}/`

## User Preferences

### get_user_preferences

**Description**: Get user settings that affect meal planning.

**Input Parameters**: None

**Returns**: 
```json
{
  "default_servings": 4,
  "dietary_restrictions": ["vegetarian"],
  "preferred_units": "metric",
  "shopping_day": "sunday",
  "meal_plan_start": "monday"
}
```

**API Routes Used**:

- GET `/api/user-preference/`

### update_user_preferences  

**Description**: Update user settings.

**Input Parameters**:

- `default_servings` (integer, optional): Default people to cook for
- `dietary_restrictions` (array, optional): Dietary needs
- `preferred_units` (string, optional): "metric" or "imperial"
- `shopping_day` (string, optional): Preferred shopping day
- `meal_plan_start` (string, optional): Week start day

**Returns**: Updated preferences

**API Routes Used**:

- PATCH `/api/user-preference/`

## Units & Conversions

### get_units

**Description**: Get available measurement units for recipes and shopping.

**Input Parameters**: None

**Returns**: Array of unit objects with name, type (weight, volume, count)

**API Routes Used**:

- GET `/api/unit/`

### convert_units

**Description**: Convert between measurement units for recipe scaling.

**Input Parameters**:

- `amount` (number, required): Quantity to convert
- `from_unit` (string, required): Source unit
- `to_unit` (string, required): Target unit
- `food_id` (integer, optional): Specific food for density conversions

**Returns**: Converted amount and unit

**API Routes Used**:

- GET `/api/unit-conversion/`

