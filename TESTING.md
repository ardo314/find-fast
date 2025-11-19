# Testing Guide for find-fast

This document describes the testing strategy for the find-fast project, including integration tests and performance benchmarks.

## Overview

The test suite consists of:

- **Integration Tests**: End-to-end tests that verify the release binary works correctly
- **Performance Benchmarks**: Criterion-based benchmarks to measure and track performance

## Integration Tests

### Running Integration Tests

To run all integration tests on the release build:

```bash
cargo test --release
```

### Test Coverage

The integration test suite (`tests/integration_tests.rs`) includes:

1. **Error Handling Tests**:

   - Missing arguments
   - Invalid glob patterns
   - Invalid regex patterns

2. **Search Functionality Tests**:

   - Searching for function declarations in Rust files
   - Searching for TODO comments across multiple file types
   - Searching specific directories
   - Case-sensitive pattern matching
   - Complex regex patterns

3. **Output Format Tests**:

   - Line number accuracy
   - Multi-line match formatting
   - Empty result handling

4. **Glob Pattern Tests**:
   - Wildcard patterns (`**/*.rs`)
   - Single file matching
   - Directory-specific patterns
   - Empty directories

### Test Structure

Each test:

1. Creates a temporary directory with test fixtures
2. Executes the release binary with specific arguments
3. Validates exit codes, stdout, and stderr output
4. Cleans up automatically (tempfile handles cleanup)

## Performance Benchmarks

### Running Benchmarks

To run all performance benchmarks:

```bash
cargo bench --bench search_benchmark
```

To run a specific benchmark group:

```bash
cargo bench --bench search_benchmark -- small_files
cargo bench --bench search_benchmark -- regex_complexity
```

### Benchmark Categories

The benchmark suite (`benches/search_benchmark.rs`) measures:

1. **Small Files** (`benchmark_small_files`):

   - Tests with 10, 50, and 100 files
   - Each file contains 100 lines
   - Measures file processing overhead

2. **Large Files** (`benchmark_large_files`):

   - Tests with 100KB, 500KB, and 1000KB files
   - Measures throughput (MB/s)
   - Tests I/O performance

3. **Regex Complexity** (`benchmark_regex_complexity`):

   - Simple word matching
   - Function declarations (`fn\s+\w+`)
   - Complex patterns with groups
   - Measures regex engine performance

4. **Parallel Scaling** (`benchmark_parallel_scaling`):

   - Tests with 50, 100, 200, and 400 files
   - Verifies parallel processing efficiency
   - Should show minimal time increase with more files (good parallelization)

5. **Match Density** (`benchmark_match_density`):

   - High match rate (many matches per file)
   - Medium match rate (moderate matches)
   - Low match rate (rare matches)
   - Tests output generation overhead

6. **Glob Patterns** (`benchmark_glob_patterns`):
   - All files (`**/*`)
   - Specific extensions (`**/*.rs`)
   - Measures glob matching performance

### Interpreting Results

Criterion automatically:

- Generates HTML reports in `target/criterion/`
- Detects performance regressions
- Shows statistical analysis (outliers, variance)
- Compares against previous runs

Look for:

- **Consistent timings** across file counts (good parallelization)
- **Linear throughput scaling** with file size
- **Low variance** (indicates stable performance)

### Example Output

```
small_files/100         time:   [66.549 ms 67.033 ms 67.612 ms]
large_files/1000KB      time:   [73.067 ms 74.029 ms 75.035 ms]
                        thrpt:  [13.015 MiB/s 13.192 MiB/s 13.365 MiB/s]
```

- Time shows [lower bound, estimate, upper bound]
- Throughput (thrpt) shows data processing rate

## Continuous Integration

For CI/CD pipelines:

```bash
# Run tests with coverage
cargo test --release --all-features

# Run benchmarks (requires more time)
cargo bench --bench search_benchmark --no-fail-fast

# Quick benchmark smoke test
cargo bench --bench search_benchmark -- --test
```

## Adding New Tests

### Integration Test Template

```rust
#[test]
fn test_your_feature() {
    let temp_dir = create_test_files();
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    let mut cmd = Command::cargo_bin("find-fast").unwrap();
    cmd.arg(glob_pattern)
        .arg("your_pattern")
        .assert()
        .success()
        .stdout(predicate::str::contains("expected_output"));
}
```

### Benchmark Template

```rust
fn benchmark_your_feature(c: &mut Criterion) {
    let temp_dir = create_benchmark_files(100, 100);
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    let mut group = c.benchmark_group("your_feature");
    group.bench_function("test_case", |b| {
        b.iter(|| {
            run_find_fast(black_box(&glob_pattern), black_box("pattern"))
        });
    });
    group.finish();
}
```

## Dependencies

Test dependencies (in `Cargo.toml`):

```toml
[dev-dependencies]
assert_cmd = "2.0"    # CLI testing
predicates = "3.0"    # Output assertions
tempfile = "3.8"      # Temporary test directories
criterion = "0.5"     # Benchmarking framework
```

## Troubleshooting

### Tests Fail to Find Binary

Make sure to build the release binary first:

```bash
cargo build --release
cargo test --release
```

### Benchmark Times Out

Increase Criterion's time limits:

```rust
group.measurement_time(Duration::from_secs(10));
```

### Inconsistent Benchmark Results

- Close other applications
- Disable CPU frequency scaling
- Run multiple iterations: `cargo bench --bench search_benchmark -- --sample-size 200`
