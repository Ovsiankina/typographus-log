#!/usr/bin/env bash
set -euo pipefail

# Run all Rust examples in the examples/ folder.
# Usage: scripts/run_examples.sh

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root_dir"

if [[ ! -d examples ]]; then
  echo "No examples/ directory found." >&2
  exit 1
fi

echo "Building examples..."
cargo build --examples >/dev/null

fail=0
failed=()

for f in examples/*.rs; do
  [[ -e "$f" ]] || continue
  name="${f##*/}"
  name="${name%.rs}"
  echo "\n=== Running example: $name ==="
  if ! cargo run --quiet --example "$name"; then
    fail=1
    failed+=("$name")
  fi
done

if (( fail )); then
  echo "\nSome examples failed:" >&2
  printf ' - %s\n' "${failed[@]}" >&2
  exit 1
fi

echo "\nAll examples ran successfully."

