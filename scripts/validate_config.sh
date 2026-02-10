#!/usr/bin/env bash
# Validate Ripple configuration by running the application with --validate-config flag.
set -e

echo "Validating Ripple configuration..."
cargo run --quiet -- --validate-config

EXIT_CODE=$?
if [ $EXIT_CODE -eq 0 ]; then
    echo "Configuration valid ✓"
else
    echo "Configuration INVALID ✗ (exit code: $EXIT_CODE)"
    exit 1
fi
