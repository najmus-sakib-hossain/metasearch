#!/bin/bash

echo "Starting SearXNG with Docker..."
echo "================================"
echo ""
echo "This will:"
echo "1. Pull the latest SearXNG Docker image"
echo "2. Start SearXNG on http://localhost:8080"
echo ""

# Create config directory if it doesn't exist
mkdir -p searxng

# Start SearXNG
docker-compose -f docker-compose.searxng.yml up -d

echo ""
echo "✓ SearXNG is starting..."
echo "✓ Access it at: http://localhost:8080"
echo ""
echo "To view logs: docker-compose -f docker-compose.searxng.yml logs -f"
echo "To stop: docker-compose -f docker-compose.searxng.yml down"
