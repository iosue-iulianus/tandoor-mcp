# Tandoor MCP Server

A [Model Context Protocol (MCP)](https://modelcontextprotocol.io) server for [Tandoor Recipes](https://tandoor.dev). Connects AI assistants (Claude, etc.) to your Tandoor instance for recipe management, shopping lists, meal planning, and inventory tracking.

## Requirements

- A running [Tandoor](https://tandoor.dev) instance

## Quick Start

### Option 1: Download a binary (recommended)

Download the latest binary for your platform from the [Releases page](https://github.com/iosue-iulianus/tandoor-mcp/releases):

| Platform | File |
|----------|------|
| Linux x86_64 | `mcp-tandoor-linux-x86_64` |
| Linux ARM64 | `mcp-tandoor-linux-aarch64` |
| macOS Intel | `mcp-tandoor-macos-x86_64` |
| macOS Apple Silicon | `mcp-tandoor-macos-aarch64` |
| Windows | `mcp-tandoor-windows-x86_64.exe` |

```bash
# Linux / macOS — make executable and run
chmod +x mcp-tandoor-*
./mcp-tandoor-linux-x86_64
```

### Option 2: Build from source

> This server is written in Rust, which is less common for MCP servers. Building from source requires Cargo and takes a few minutes on first run while dependencies compile, but produces a self-contained binary with no runtime.

**Requirements:**
- Rust 1.85+ via [rustup](https://rustup.rs): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Linux only: `sudo apt install pkg-config libssl-dev`

```bash
git clone https://github.com/iosue-iulianus/tandoor-mcp.git
cd tandoor-mcp
cargo build --release
# binary output: target/release/mcp-tandoor
```

### Configuration

Either way, create a `.env` file before running:

```bash
cp .env.example .env
# edit .env with your Tandoor URL, username, and password
```

The server auto-loads `.env` on startup and listens on `127.0.0.1:3001` by default.

## Environment Variables

| Variable              | Description                                          | Default                 |
|-----------------------|------------------------------------------------------|-------------------------|
| `TANDOOR_BASE_URL`    | URL of your Tandoor instance                         | `http://localhost:8080` |
| `TANDOOR_USERNAME`    | Tandoor username                                     | `admin`                 |
| `TANDOOR_PASSWORD`    | Tandoor password                                     | `admin`                 |
| `TANDOOR_AUTH_TOKEN`  | Pre-set auth token — skips username/password auth    | —                       |
| `BIND_ADDR`           | Address and port for the MCP server                  | `127.0.0.1:3001`        |
| `RUST_LOG`            | Log level (`info`, `debug`, `trace`)                 | `info`                  |

### Rate limiting

Tandoor limits authentication to **10 requests per day**. If you hit this, get a token once and reuse it via `TANDOOR_AUTH_TOKEN`:

```bash
TOKEN=$(curl -s -X POST http://localhost:8080/api-token-auth/ \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"your-password"}' | jq -r .token)

# Add to your .env file
echo "TANDOOR_AUTH_TOKEN=$TOKEN" >> .env

cargo run --release
```

## MCP Client Configuration

### Claude Desktop

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "tandoor": {
      "url": "http://127.0.0.1:3001/sse"
    }
  }
}
```

### Claude Code

```bash
claude mcp add tandoor --transport sse http://127.0.0.1:3001/sse
```

## Available Tools

### Recipe Management
| Tool | Description |
|------|-------------|
| `search_recipes` | Search recipes by keyword, ingredient, or name |
| `get_recipe_details` | Get full recipe with scaled ingredients |
| `create_recipe` | Create a new recipe |

### Shopping Lists
| Tool | Description |
|------|-------------|
| `add_to_shopping_list` | Add items to the shopping list |
| `get_shopping_list` | Get current shopping list |
| `check_shopping_items` | Mark items as checked/purchased |
| `clear_shopping_list` | Clear checked items and update pantry |

### Food & Inventory
| Tool | Description |
|------|-------------|
| `search_foods` | Search foods/ingredients |
| `update_pantry` | Update pantry inventory |
| `suggest_from_inventory` | Get recipe suggestions based on what you have |

### Meal Planning
| Tool | Description |
|------|-------------|
| `get_meal_plans` | Get meal plans for a date range |
| `create_meal_plan` | Create a new meal plan entry |
| `delete_meal_plan` | Delete a meal plan entry |
| `get_meal_types` | List available meal types |

### Metadata
| Tool | Description |
|------|-------------|
| `get_keywords` | List all recipe keywords/tags |
| `get_units` | List available measurement units |

### Cooking History
| Tool | Description |
|------|-------------|
| `get_cook_log` | Get cooking history |
| `log_cooked_recipe` | Log a cooked recipe |

## Tandoor Setup

Tandoor uses a multi-tenant permission system. API access requires a user to be assigned to a Space with a group role.

### Web Interface (Recommended)

1. Go to `http://your-tandoor/admin/`
2. **Groups** — ensure `admin`, `user`, and `guest` groups exist
3. **Spaces** — create a Space (unlimited values: set to 0)
4. **User spaces** — link your user to the Space with the `admin` group and **Active** checked

### Command Line

```bash
docker exec -it your-tandoor-container \
  /opt/recipes/venv/bin/python manage.py shell
```

```python
from cookbook.models import Space, UserSpace
from django.contrib.auth.models import User, Group

user = User.objects.get(username='admin')
space, _ = Space.objects.get_or_create(
    name='Default',
    defaults={'created_by': user, 'max_recipes': 0, 'max_users': 0,
              'max_file_storage_mb': 0, 'allow_sharing': True}
)
us, _ = UserSpace.objects.get_or_create(user=user, space=space, defaults={'active': True})
us.active = True
us.save()
us.groups.add(Group.objects.get(name='admin'))
```

### Verify API Access

```bash
curl -X GET http://localhost:8080/api/keyword/ \
  -H "Authorization: Bearer YOUR_TOKEN"
```

## Troubleshooting

**"Authentication credentials were not provided"** — token is missing or in the wrong format. Ensure you're using `Bearer YOUR_TOKEN`.

**"You do not have permission to perform this action"** — the user isn't assigned to an active Space with a group. Follow the Tandoor Setup section above.

**"Request was throttled" / 429** — you've hit the 10 auth requests/day limit. Use `TANDOOR_AUTH_TOKEN` with a saved token instead of username/password.

**`ScopeError: A scope on dimension(s) space needs to be active`** — the UserSpace either doesn't exist or `active` is not set to true.

## Development

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests (requires Docker)
./scripts/test.sh test

# Start/stop Tandoor for local dev
./scripts/test.sh up
./scripts/test.sh down
```

Test script requires Docker, Docker Compose, and ports 8080 and 5432 available.
