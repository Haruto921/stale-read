#!/bin/bash
set -e

cd "$(dirname "$0")/../environment"
export PATH="/usr/local/cargo/bin:$PATH"

echo "=============================================="
echo "  Stale Read Expert - Evaluation"
echo "=============================================="
echo ""

echo "[1/2] Building..."
if cargo build --release 2>&1 | tee /tmp/build.log > /dev/null; then
    echo "  Build successful"
else
    echo "  Build failed"
    exit 1
fi
echo ""

echo "[2/2] Running tests (--test-threads=1)..."
OUTPUT=$(cargo test --test integration_test --release -- --test-threads=1 2>&1)
echo "$OUTPUT"

PASSED=$(echo "$OUTPUT" | grep -oP '\d+ passed' | grep -oP '\d+')
FAILED=$(echo "$OUTPUT" | grep -oP '\d+ failed' | grep -oP '\d+' || echo "0")
TOTAL=$((PASSED + FAILED))
SCORE=$((PASSED * 100 / TOTAL))

echo ""
echo "=============================================="
echo "  Passed: $PASSED/$TOTAL"
echo "  Score: $SCORE%"
echo "=============================================="

[ "$FAILED" = "0" ] && exit 0 || exit 1
