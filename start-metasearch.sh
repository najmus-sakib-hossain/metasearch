#!/bin/bash

echo "Starting Metasearch Server..."
echo "=============================="
echo ""
echo "Building and starting on http://localhost:8888"
echo ""

cargo run --release --bin metasearch -- serve --host 127.0.0.1 --port 8888
