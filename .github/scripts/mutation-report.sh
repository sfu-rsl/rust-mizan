#!/bin/bash

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
BASE_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"
cd "$BASE_DIR"

# Create datasets directory
DATASETS_DIR="datasets"
mkdir -p "$DATASETS_DIR"

# Check if mizan-mut is available
if ! command -v mizan-mut &> /dev/null; then
    echo "ERROR: mizan-mut not found in PATH. Please ensure mizan-mut is built and available."
    exit 1
fi

# Define all mutations to test
ALL_MUTATIONS=(
    "remove-comments"
    "format-compact"
    "mizan-mut-while-to-loop"
    "mizan-mut-for-to-while"
    "mizan-mut-if-else-reorder"
    "mizan-mut-derive-reorder"
    "mizan-mut-trait-bound-reorder"
    "mizan-mut-use-reorder"
    "mizan-mut-arithmetic-identity"
    "benign-comments"
    "benign-blocks"
    "benign-rename-fn"
    "benign-rename-var"
    "malignant-comments"
    "malignant-blocks"
    "malignant-rename-fn"
    "malignant-rename-var"
    "format-expanded"
)

echo "=== Mutation Report and Dataset Generation ==="
echo

# Run initial checkout to get total sample count
echo "Running initial checkout to count samples..."
rm -rf output
CHECKOUT_OUTPUT=$(mizan checkout --include-fixed 2>&1)
TOTAL_SAMPLES=$(echo "$CHECKOUT_OUTPUT" | grep "Selected" | grep -oE '[0-9]+' | head -1)
echo "Total samples: $TOTAL_SAMPLES"
echo

# Initialize markdown report
echo "# Mutation Report and Dataset Generation" > mutation-summary.md
echo "" >> mutation-summary.md
echo "Total samples: $TOTAL_SAMPLES" >> mutation-summary.md
echo "" >> mutation-summary.md
echo "## Individual Mutation Results" >> mutation-summary.md
echo "" >> mutation-summary.md
echo "| Mutation | Partial Mutations | Skipped |" >> mutation-summary.md
echo "|----------|-------------------|---------|" >> mutation-summary.md

apply_mutations_and_generate_dataset() {
    local mutations=("$@")
    local dataset_name="${mutations[-1]}"  # Last argument is the dataset name
    unset 'mutations[-1]'  # Remove dataset name from mutations array
    
    echo "Applying mutations: ${mutations[*]} -> $dataset_name"
    
    # Clean and checkout
    rm -rf output
    mizan checkout --include-fixed > /dev/null 2>&1
    cd output
    
    # Apply mutations in sequence
    local all_partial=0
    local all_skipped=0
    local mutation_success=true
    
    for mutation in "${mutations[@]}"; do
        echo "  Applying mutation: $mutation"
        if ! mizan mutate -m "$mutation" > /dev/null 2>&1; then
            echo "  ERROR: Failed to apply mutation $mutation"
            mutation_success=false
            break
        fi
        
        if [[ -f "mizan_mutations.json" ]]; then
            COUNTS=$(python3 -c "
import json
import sys

try:
    with open('mizan_mutations.json', 'r') as f:
        data = json.load(f)
    
    mutation = '$mutation'
    partial_mutations = data.get('partial_mutations', data.get('partial_applications', {}))
    skipped = data.get('skipped', data.get('failures', {}))
    
    partial_count = len(partial_mutations.get(mutation, []))
    skipped_count = len(skipped.get(mutation, []))
    
    print(f'{partial_count},{skipped_count}')
except Exception as e:
    print('0,0', file=sys.stderr)
    sys.exit(1)
")
            
            if [[ $? -eq 0 ]]; then
                PARTIAL=$(echo "$COUNTS" | cut -d',' -f1)
                SKIPPED=$(echo "$COUNTS" | cut -d',' -f2)
                all_partial=$((all_partial + PARTIAL))
                all_skipped=$((all_skipped + SKIPPED))
                echo "    Partial: $PARTIAL, Skipped: $SKIPPED"
            else
                echo "    Error parsing mutation results"
                mutation_success=false
                break
            fi
        else
            echo "    No results file found"
            mutation_success=false
            break
        fi
    done
    
    # Generate dataset if mutations were successful
    local dataset_generated="❌"
    if [[ "$mutation_success" == true ]]; then
        echo "  Generating dataset: $dataset_name"
        if mizan evaluate prepare-dataset -o "../$DATASETS_DIR/$dataset_name" > /dev/null 2>&1; then
            dataset_generated="✅"
            echo "  Dataset generated successfully: $DATASETS_DIR/$dataset_name"
        else
            echo "  ERROR: Failed to generate dataset $dataset_name"
        fi
    else
        echo "  Skipping dataset generation due to mutation failures"
    fi
    
    cd "$BASE_DIR"
    return $([ "$mutation_success" == true ] && [ "$dataset_generated" == "✅" ] && echo 0 || echo 1)
}

echo "=== Testing Individual Mutations ==="
echo
for mutation in "${ALL_MUTATIONS[@]}"; do
    echo "Testing mutation: $mutation"
    
    rm -rf output
    mizan checkout --include-fixed > /dev/null 2>&1
    cd output
    
    mutation_success=true
    if ! mizan mutate -m "$mutation" > /dev/null 2>&1; then
        echo "  ERROR: Failed to apply mutation $mutation"
        echo "| $mutation | N/A | N/A |" >> "$BASE_DIR/mutation-summary.md"
        cd "$BASE_DIR"
        continue
    fi
    
    if [[ -f "mizan_mutations.json" ]]; then
        COUNTS=$(python3 -c "
import json
import sys

try:
    with open('mizan_mutations.json', 'r') as f:
        data = json.load(f)
    
    mutation = '$mutation'
    partial_mutations = data.get('partial_mutations', data.get('partial_applications', {}))
    skipped = data.get('skipped', data.get('failures', {}))
    
    partial_count = len(partial_mutations.get(mutation, []))
    skipped_count = len(skipped.get(mutation, []))
    
    print(f'{partial_count},{skipped_count}')
except Exception as e:
    print('0,0', file=sys.stderr)
    sys.exit(1)
")
        
        if [[ $? -eq 0 ]]; then
            PARTIAL=$(echo "$COUNTS" | cut -d',' -f1)
            SKIPPED=$(echo "$COUNTS" | cut -d',' -f2)
            echo "  Partial: $PARTIAL, Skipped: $SKIPPED"
            
            # Generate individual dataset
            echo "  Generating dataset: mizan-$mutation.json"
            if mizan evaluate prepare-dataset -o "../$DATASETS_DIR/mizan-$mutation.json" > /dev/null 2>&1; then
                echo "  Dataset generated successfully"
            else
                echo "  ERROR: Failed to generate dataset"
            fi
            
            echo "| $mutation | $PARTIAL | $SKIPPED |" >> "$BASE_DIR/mutation-summary.md"
        else
            echo "  Error parsing mutation results"
            echo "| $mutation | N/A | N/A |" >> "$BASE_DIR/mutation-summary.md"
        fi
    else
        echo "  No results file found"
        echo "| $mutation | N/A | N/A |" >> "$BASE_DIR/mutation-summary.md"
    fi
    
    cd "$BASE_DIR"
done

echo
echo "=== Generating Vanilla Dataset ==="
echo

echo "Generating vanilla dataset..."
rm -rf output
mizan checkout --include-fixed > /dev/null 2>&1
cd output
if mizan evaluate prepare-dataset -o "../$DATASETS_DIR/mizan-vanilla.json" > /dev/null 2>&1; then
    echo "✅ Vanilla dataset generated successfully"
else
    echo "❌ Failed to generate vanilla dataset"
fi
cd "$BASE_DIR"

rm -rf output

echo "" >> mutation-summary.md
echo "## Summary" >> mutation-summary.md
echo "" >> mutation-summary.md
echo "- Total mutations tested: ${#ALL_MUTATIONS[@]}" >> mutation-summary.md
echo "- Datasets directory: \`$DATASETS_DIR/\`" >> mutation-summary.md
echo "- Generated datasets:" >> mutation-summary.md

if [[ -d "$DATASETS_DIR" ]]; then
    for dataset in "$DATASETS_DIR"/*.json; do
        if [[ -f "$dataset" ]]; then
            filename=$(basename "$dataset")
            size=$(stat -f%z "$dataset" 2>/dev/null || stat -c%s "$dataset" 2>/dev/null || echo "unknown")
            echo "  - \`$filename\` (${size} bytes)" >> mutation-summary.md
        fi
    done
else
    echo "  - No datasets generated" >> mutation-summary.md
fi

echo
echo "=== Report Complete ==="
echo "Report saved to: mutation-summary.md"
echo "Datasets saved to: $DATASETS_DIR/"
echo
