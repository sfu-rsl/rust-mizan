#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"
cd "$BASE_DIR"

MUTATIONS=(
    "format-compact"
    "format-expanded"
    "benign-comments"
    "benign-blocks"
    "malignant-comments"
    "malignant-blocks"
    "mizan-mut-for-to-while"
    "mizan-mut-while-to-loop"
    "mizan-mut-if-else-reorder"
    "mizan-mut-derive-reorder"
    "mizan-mut-trait-bound-reorder"
    "mizan-mut-use-reorder"
    "mizan-mut-arithmetic-identity"
    "benign-rename-fn"
    "benign-rename-var"
    "malignant-rename-fn"
    "malignant-rename-var"
    "remove-comments"
)

# Run checkout once to get total number of samples
echo "Running initial checkout to count samples..."
rm -rf output
CHECKOUT_OUTPUT=$(mizan checkout --include-fixed 2>&1)
TOTAL_SAMPLES=$(echo "$CHECKOUT_OUTPUT" | grep "Selected" | grep -oE '[0-9]+' | head -1)
echo "Total samples: $TOTAL_SAMPLES"

echo "# Mutation Report" > mutation-summary.md
echo "" >> mutation-summary.md
echo "Total samples: $TOTAL_SAMPLES" >> mutation-summary.md
echo "" >> mutation-summary.md
echo "| Mutation | Partial Mutations | Skipped |" >> mutation-summary.md
echo "|----------|-------------------|---------|" >> mutation-summary.md

for mutation in "${MUTATIONS[@]}"; do
    echo ""
    echo "Testing mutation: $mutation"
    
    # Clean and checkout
    rm -rf output
    mizan checkout --include-fixed > /dev/null 2>&1
    
    cd output
    
    # Run mutation
    mizan mutate -m "$mutation" > /dev/null 2>&1 || true
    
    # Parse results
    if [[ -f "mizan_mutations.json" ]]; then
        COUNTS=$(python3 -c "
import json

with open('mizan_mutations.json', 'r') as f:
    data = json.load(f)

mutation = '$mutation'
# Support both old and new field names for backward compatibility
partial_mutations = data.get('partial_mutations', data.get('partial_applications', {}))
skipped = data.get('skipped', data.get('failures', {}))

partial_count = len(partial_mutations.get(mutation, []))
skipped_count = len(skipped.get(mutation, []))

print(f'{partial_count},{skipped_count}')
")
        
        PARTIAL=$(echo "$COUNTS" | cut -d',' -f1)
        SKIPPED=$(echo "$COUNTS" | cut -d',' -f2)
        
        echo "  Partial: $PARTIAL, Skipped: $SKIPPED"
        echo "| $mutation | $PARTIAL | $SKIPPED |" >> "$BASE_DIR/mutation-summary.md"
    else
        echo "  No results file found"
        echo "| $mutation | N/A | N/A |" >> "$BASE_DIR/mutation-summary.md"
    fi
    
    cd "$BASE_DIR"
done

rm -rf output

echo ""
echo "Report saved to mutation-summary.md"
