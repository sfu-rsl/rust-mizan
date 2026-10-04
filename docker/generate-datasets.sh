#!/bin/bash
set -e

cd /app

BENIGN_MUTATIONS=(
    "remove-comments"
    "format-compact"
    "mizan-mut-while-to-loop"
    "mizan-mut-for-to-while"
    "mizan-mut-if-else-reorder"
    "benign-comments"
    "benign-blocks"
    "benign-rename-fn"
    "benign-rename-var"
)

MALIGNANT_MUTATIONS=(
    "remove-comments"
    "malignant-comments"
    "malignant-blocks"
    "malignant-rename-fn"
    "malignant-rename-var"
)

RUST_SPECIFIC_MUTATIONS=(
    "remove-comments"
    "mizan-mut-derive-reorder"
    "mizan-mut-trait-bound-reorder"
    "mizan-mut-use-reorder"
    "mizan-mut-arithmetic-identity"
    "mizan-mut-explicit-where"
    "mizan-mut-rename-lifetime"
    "mizan-mut-extraneous-unsafe"
    "mizan-mut-impl-trait-to-generic"
    "mizan-mut-option-wrap"
    "mizan-mut-maybeuninit-wrap"
    "mizan-mut-manuallydrop-wrap"
    "mizan-mut-explicit-return"
    "mizan-mut-unreachable-panic"
    "mizan-mut-repeated-shadowing"
)

generate_dataset() {
    local tag="$1"
    shift
    local mutations=("$@")

    echo "=== Generating $tag dataset ==="
    rm -rf output
    mizan checkout --include-fixed > /dev/null
    cd output

    for mutation in "${mutations[@]}"; do
        echo "  Applying: $mutation"
        mizan mutate -m "$mutation" > /dev/null
    done

    echo "  Creating dataset with tag: $tag"
    mizan evaluate prepare-dataset --tag "$tag" -o "/app/datasets/mizan-$tag.parquet"

    cd ..
    echo "$tag dataset complete"
    echo
}

generate_dataset "vanilla"
generate_dataset "benign" "${BENIGN_MUTATIONS[@]}"
generate_dataset "malignant" "${MALIGNANT_MUTATIONS[@]}"
generate_dataset "rust-specific" "${RUST_SPECIFIC_MUTATIONS[@]}"

rm -rf output

echo "=== All datasets generated ==="
ls -lh /app/datasets/
