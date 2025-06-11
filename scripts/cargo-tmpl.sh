#!/bin/bash

[ -z "$1" ] && { echo "Usage: $0 <project_name> [template_path]"; exit 1; }

PROJECT="$1"
TEMPLATE="${2:-./scripts/solution-tmpl.rs}"
MAIN="./$PROJECT/src/main.rs"

[ -d "$PROJECT" ] && { echo "Directory '$PROJECT' already exists."; exit 1; }
cargo new "$PROJECT" || { echo "Failed project creation"; exit 1; }

[ -f "$TEMPLATE" ] && cp "$TEMPLATE" "$MAIN" && echo "main.rs replaced" || { echo "Template not found at '$TEMPLATE'."; exit 1; }

echo "'$PROJECT' created."