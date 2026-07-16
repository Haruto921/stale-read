#!/bin/bash
set -e

# Benchmark 시스템은 workspace 디렉토리에서 eval.sh를 실행
# reward.txt는 eval.sh 실행 디렉토리에 생성되어야 함
WORKSPACE_DIR="$(pwd)"
EVAL_DIR="$(cd "$(dirname "$0")" && pwd)"

cd "$EVAL_DIR/../environment"
export PATH="/usr/local/cargo/bin:/root/.cargo/bin:$PATH"

echo "=============================================="
echo "  Stale Read Expert - Evaluation"
echo "=============================================="
echo ""

echo "[1/2] Building..."
if cargo build --release 2>&1 | tee /tmp/build.log > /dev/null; then
    echo "  Build successful"
else
    echo "  Build failed"
    # reward.txt 파일 생성 (0점) - workspace 디렉토리에
    echo "0" > "$WORKSPACE_DIR/reward.txt"
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

# reward.txt 파일 생성 - benchmark 시스템이 찾는 위치
echo "$SCORE" > "$WORKSPACE_DIR/reward.txt"
echo "Reward saved to: $WORKSPACE_DIR/reward.txt"

[ "$FAILED" = "0" ] && exit 0 || exit 1
