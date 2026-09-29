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
cargo build --examples --all-features >/dev/null

fail=0
failed=()

for f in examples/*.rs; do
  [[ -e "$f" ]] || continue
  name="${f##*/}"
  name="${name%.rs}"
  echo -e "\n=== Running example: $name ==="
  # showcase ends with an error from main and panicdemo* panic: both on purpose.
  expect_fail=0
  [[ "$name" == showcase || "$name" == panicdemo* ]] && expect_fail=1
  if cargo run --quiet --all-features --example "$name"; then
    (( expect_fail )) && { fail=1; failed+=("$name (expected non-zero exit)"); }
  else
    (( expect_fail )) || { fail=1; failed+=("$name"); }
  fi
done

if (( fail )); then
  echo -e "\nSome examples failed:" >&2
  printf ' - %s\n' "${failed[@]}" >&2
  exit 1
fi

echo -e "\nAll examples ran successfully."

