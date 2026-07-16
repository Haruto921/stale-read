#!/bin/bash
set -e

cd "$(dirname "$0")/../environment"
export PATH="/usr/local/cargo/bin:/root/.cargo/bin:$PATH"

echo "=== Running Integration Tests ==="
OUTPUT=$(cargo test --test integration_test --release -- --test-threads=1 2>&1)
echo "$OUTPUT"

PASSED=$(echo "$OUTPUT" | grep -oP '\d+ passed' | grep -oP '\d+')
FAILED=$(echo "$OUTPUT" | grep -oP '\d+ failed' | grep -oP '\d+' || echo "0")
TOTAL=$((PASSED + FAILED))
SCORE=$((PASSED * 100 / TOTAL))

echo ""
echo "=== Results: $PASSED/$TOTAL ($SCORE%) ==="

# reward.txt 생성 - benchmark 시스템이 찾는 위치
echo "$SCORE" > "$(dirname "$0")/../reward.txt"
echo "Reward saved: $SCORE"

[ "$FAILED" = "0" ] && exit 0 || exit 1
