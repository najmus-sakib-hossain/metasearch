#!/bin/bash
# Test script to verify OpenSearch and autocomplete endpoints

echo "Testing OpenSearch and Autocomplete endpoints..."
echo ""

# Test OpenSearch descriptor
echo "1. Testing /opensearch.xml endpoint:"
curl -s http://localhost:8888/opensearch.xml | head -20
echo ""
echo ""

# Test autocomplete
echo "2. Testing /autocomplete endpoint with query 'rust':"
curl -s "http://localhost:8888/autocomplete?q=rust" | jq .
echo ""
echo ""

# Test autocomplete with another query
echo "3. Testing /autocomplete endpoint with query 'python':"
curl -s "http://localhost:8888/autocomplete?q=python" | jq .
echo ""

echo "Done! If you see XML and JSON responses above, the endpoints are working."
