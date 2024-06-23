#!/usr/bin/env bash

# Initialize variables
ENVIRONMENT=""

# Parse command-line options
while getopts e: option
do
    case "${option}" in
        e) ENVIRONMENT=${OPTARG};;
    esac
done

# Check required options
if [ -z "$ENVIRONMENT" ]; then
    echo "Usage: $0 -e [node|browser]"
    exit 1
fi

# Define test environment options
NODE_ENV="--environment node --run"
BROWSER_ENV="--environment jsdom --browser --browser.name=chrome --run"

# Check which environment to test based on the options provided
if [ "$ENVIRONMENT" = "node" ]; then
    echo "Testing ESM Version on Node"
    npx vitest --config "vitest.config.ts" $NODE_ENV tests/node.test.ts
elif [ "$ENVIRONMENT" = "browser" ]; then
    echo "Testing ESM Version in Browser"
    npx vitest --config "vitest.config.ts" $BROWSER_ENV tests/browser.test.ts
else
    echo "Error: Unknown environment specified. Please use '-e node' or '-e browser'."
    exit 1
fi
