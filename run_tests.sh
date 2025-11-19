#!/bin/bash
# Quick test runner script for find-fast

set -e

echo "========================================="
echo "find-fast Test Suite Runner"
echo "========================================="
echo ""

# Build release binary first
echo "📦 Building release binary..."
cargo build --release
echo "✓ Release binary built"
echo ""

# Run integration tests
echo "🧪 Running integration tests..."
cargo test --release
echo "✓ Integration tests passed"
echo ""

# Run benchmarks (quick mode for CI)
echo "⚡ Running performance benchmarks..."
if [ "$1" == "--quick" ]; then
    echo "   (Quick mode - test only)"
    cargo bench --bench search_benchmark -- --test
else
    echo "   (Full benchmark suite)"
    cargo bench --bench search_benchmark
fi
echo "✓ Benchmarks completed"
echo ""

echo "========================================="
echo "✓ All tests passed successfully!"
echo "========================================="
echo ""
echo "View detailed benchmark results in: target/criterion/"
echo "View test documentation in: TESTING.md"
echo "View latest results in: TEST_RESULTS.md"
