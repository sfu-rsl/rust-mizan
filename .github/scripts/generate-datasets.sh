#!/bin/bash
# Script to generate 4 core datasets: vanilla, neutral, benign, malignant

set -e

cd "$(dirname "$0")/../.."

# Create output directory
DATASETS_DIR="datasets"
mkdir -p "$DATASETS_DIR"

# Define mutation sets
NEUTRAL_MUTATIONS=(
    "remove-comments"
    "format-compact"
    "mizan-mut-for-to-while"
    "mizan-mut-while-to-loop"
    "mizan-mut-if-else-reorder"
    "mizan-mut-derive-reorder"
    "mizan-mut-trait-bound-reorder"
    "mizan-mut-use-reorder"
    "mizan-mut-arithmetic-identity"
)

BENIGN_MUTATIONS=(
    "${NEUTRAL_MUTATIONS[@]}"
    "benign-comments"
    "benign-blocks"
    "benign-rename-fn"
    "benign-rename-var"
)

MALIGNANT_MUTATIONS=(
    "${NEUTRAL_MUTATIONS[@]}"
    "malignant-comments"
    "malignant-blocks"
    "malignant-rename-fn"
    "malignant-rename-var"
)

generate_dataset() {
    local tag="$1"
    shift
    local mutations=("$@")

    echo "=== Generating $tag dataset ==="
    rm -rf output
    mizan checkout --include-fixed > /dev/null
    cd output

    # Apply mutations if any
    for mutation in "${mutations[@]}"; do
        echo "  Applying: $mutation"
        mizan mutate -m "$mutation" > /dev/null
    done

    # Generate dataset with tag
    echo "  Creating dataset with tag: $tag"
    mizan evaluate prepare-dataset --tag "$tag" -o "../$DATASETS_DIR/mizan-$tag.parquet"

    cd ..
    echo "$tag dataset complete"
    echo
}

# Generate all datasets
generate_dataset "vanilla"
generate_dataset "neutral" "${NEUTRAL_MUTATIONS[@]}"
generate_dataset "benign" "${BENIGN_MUTATIONS[@]}"
generate_dataset "malignant" "${MALIGNANT_MUTATIONS[@]}"

# Cleanup
rm -rf output

echo "=== All datasets generated ==="
ls -lh "$DATASETS_DIR"
