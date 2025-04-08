#! /bin/sh
API_CONTAINER_NAME=$(docker ps --filter "name=sl_api-api-1" --format "{{.ID}}")

if [ -z "$API_CONTAINER_NAME" ]; then
    echo "ERROR: API container not found!" >&2
    exit 1
fi

docker exec $API_CONTAINER_NAME sea-orm-cli migrate "$@"
