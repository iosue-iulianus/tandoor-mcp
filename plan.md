# Tandoor MCP Server Implementation Plan

## Authentication & API Integration Strategy

### Authentication Flow
- **Token-based authentication**: Tandoor uses REST API tokens via `/api-token-auth/` endpoint
- **Required credentials**: username + password → receive token
- **Token usage**: Bearer token in Authorization header for all subsequent API calls
- **Configuration**: Store base URL and credentials in MCP server config

### Core API Endpoints Mapping

Based on OpenAPI analysis, key endpoints for our MCP tools:

#### Recipe Management
- `GET /api/recipe/` - Search recipes (supports query params)
- `GET /api/recipe/{id}/` - Get recipe details
- `POST /api/recipe/` - Create new recipe
- `PATCH /api/recipe/{id}/` - Update recipe
- `POST /api/recipe-from-source/` - Import recipe from URL

#### Food & Ingredients
- `GET /api/food/` - Search foods/ingredients
- `POST /api/food/` - Create new food
- `PATCH /api/food/{id}/` - Update food availability

#### Shopping Lists
- `GET /api/shopping-list-entry/` - Get shopping list
- `POST /api/shopping-list-entry/` - Add single item
- `POST /api/shopping-list-entry/bulk/` - Add multiple items
- `PATCH /api/shopping-list-entry/{id}/` - Update item (mark checked)
- `DELETE /api/shopping-list-entry/{id}/` - Remove item

#### Meal Planning
- `GET /api/meal-plan/` - Get meal plans
- `POST /api/meal-plan/` - Create meal plan
- `DELETE /api/meal-plan/{id}/` - Remove meal plan
- `GET /api/meal-type/` - Get available meal types

#### Recipe History
- `GET /api/cook-log/` - Get cooking history
- `POST /api/cook-log/` - Log cooked recipe

#### Utilities
- `GET /api/keyword/` - Get recipe keywords/tags
- `GET /api/unit/` - Get measurement units
- `GET /api/recipe-book/` - Get recipe collections

## Implementation Architecture

### Project Structure
```
src/
├── main.rs              # MCP server entry point
├── lib.rs              # Library root
├── client/             # Tandoor API client
│   ├── mod.rs
│   ├── auth.rs         # Authentication handling
│   ├── types.rs        # API response types
│   └── client.rs       # HTTP client implementation
├── tools/              # MCP tool implementations
│   ├── mod.rs
│   ├── recipes.rs      # Recipe search/management tools
│   ├── shopping.rs     # Shopping list tools
│   ├── meals.rs        # Meal planning tools
│   ├── inventory.rs    # Inventory management tools
│   ├── history.rs      # Recipe history tools
│   └── utils.rs        # Utility tools
└── server.rs           # MCP server setup
```

### Dependencies Required
```toml
[dependencies]
rmcp = { version = "0.3", features = ["server"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
```

### Configuration
- Server will accept Tandoor base URL and credentials as config
- Use environment variables for sensitive data
- Support both SSE and stdio transports

### Error Handling Strategy
- Graceful handling of authentication failures
- Network timeout and retry logic
- Clear error messages for users
- Fallback behavior when Tandoor is unavailable

## Implementation Priority

### Phase 1: Core Infrastructure
1. Set up project dependencies
2. Implement Tandoor API client with authentication
3. Create basic MCP server structure
4. Add configuration handling

### Phase 2: Essential Tools
1. Recipe search and details (`search_recipes`, `get_recipe_details`)
2. Shopping list management (`add_to_shopping_list`, `get_shopping_list`)
3. Basic meal planning (`plan_meals`, `get_meal_plans`)

### Phase 3: Advanced Features
1. Recipe creation and management
2. Inventory tracking
3. Recipe history and logging
4. Import from external URLs

### Phase 4: Optimization
1. Caching for frequent API calls
2. Batch operations where possible
3. Rate limiting compliance
4. Performance testing

## Development Notes

- Tandoor API appears to be Django REST Framework based
- No explicit rate limiting visible in OpenAPI spec
- Some endpoints support bulk operations (shopping list bulk add)
- Food items can be hierarchical (move/merge operations available)
- Recipe scaling supported through servings parameter