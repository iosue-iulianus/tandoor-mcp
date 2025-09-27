set -e

# Get a single auth token from Tandoor to avoid rate limiting
# The token endpoint is limited to 10 requests per day!

USERNAME="${1:-admin}"
PASSWORD="${2:-testing1}"
BASE_URL="${3:-http://localhost:8080}"

TOKEN=$(curl -s -X POST "${BASE_URL}/api-token-auth/" \
  -H "Content-Type: application/json" \
  -d "{\"username\": \"${USERNAME}\", \"password\": \"${PASSWORD}\"}" \
  | jq -r '.token')

if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
  echo "Failed to get token. You may be rate-limited (10 requests/day)." >&2
  exit 1
fi

echo "$TOKEN"