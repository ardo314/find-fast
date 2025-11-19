use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Create a temporary directory with a specific number of files
fn create_benchmark_files(num_files: usize, lines_per_file: usize) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let base_path = temp_dir.path();

    for i in 0..num_files {
        let file_path = base_path.join(format!("file_{}.rs", i));
        let mut content = String::new();

        for j in 0..lines_per_file {
            if j % 10 == 0 {
                content.push_str(&format!("fn function_{}() {{\n", j));
                content.push_str("    // Some comment\n");
                content.push_str("    let x = 42;\n");
                content.push_str("}\n");
            } else if j % 5 == 0 {
                content.push_str(&format!("// TODO: Implement feature {}\n", j));
            } else {
                content.push_str(&format!("let variable_{} = {};\n", j, j));
            }
        }

        fs::write(file_path, content).unwrap();
    }

    temp_dir
}

/// Create files with various sizes for throughput testing
fn create_large_file(size_kb: usize) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large_file.txt");

    let mut content = String::new();
    let line = "The quick brown fox jumps over the lazy dog. TODO: Add more content here.\n";
    let line_size = line.len();
    let num_lines = (size_kb * 1024) / line_size;

    for _ in 0..num_lines {
        content.push_str(line);
    }

    fs::write(file_path, content).unwrap();
    temp_dir
}

/// Run find-fast command and return execution time
fn run_find_fast(glob_pattern: &str, regex: &str) {
    let output = Command::new("target/release/find-fast")
        .arg(glob_pattern)
        .arg(regex)
        .output()
        .expect("Failed to execute find-fast");

    // Ensure command succeeded
    assert!(output.status.success(), "Command failed: {:?}", output);
}

fn benchmark_small_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_files");

    for num_files in [10, 50, 100].iter() {
        let temp_dir = create_benchmark_files(*num_files, 100);
        let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

        group.bench_with_input(BenchmarkId::from_parameter(num_files), num_files, |b, _| {
            b.iter(|| run_find_fast(black_box(&glob_pattern), black_box(r"fn\s+\w+")));
        });
    }

    group.finish();
}

fn benchmark_large_files(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_files");

    for size_kb in [100, 500, 1000].iter() {
        let temp_dir = create_large_file(*size_kb);
        let glob_pattern = format!("{}/*.txt", temp_dir.path().display());

        group.throughput(Throughput::Bytes((*size_kb * 1024) as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}KB", size_kb)),
            size_kb,
            |b, _| {
                b.iter(|| run_find_fast(black_box(&glob_pattern), black_box("TODO")));
            },
        );
    }

    group.finish();
}

fn benchmark_regex_complexity(c: &mut Criterion) {
    let temp_dir = create_benchmark_files(20, 200);
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    let mut group = c.benchmark_group("regex_complexity");

    // Simple pattern
    group.bench_function("simple_word", |b| {
        b.iter(|| run_find_fast(black_box(&glob_pattern), black_box("TODO")));
    });

    // Medium complexity pattern
    group.bench_function("function_declaration", |b| {
        b.iter(|| run_find_fast(black_box(&glob_pattern), black_box(r"fn\s+\w+")));
    });

    // Complex pattern
    group.bench_function("complex_pattern", |b| {
        b.iter(|| run_find_fast(black_box(&glob_pattern), black_box(r"(fn|let)\s+\w+.*\{")));
    });

    group.finish();
}

fn benchmark_parallel_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_scaling");

    // Create many files to test parallel processing
    for num_files in [50, 100, 200, 400].iter() {
        let temp_dir = create_benchmark_files(*num_files, 100);
        let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_files", num_files)),
            num_files,
            |b, _| {
                b.iter(|| run_find_fast(black_box(&glob_pattern), black_box(r"fn\s+\w+")));
            },
        );
    }

    group.finish();
}

fn benchmark_match_density(c: &mut Criterion) {
    let mut group = c.benchmark_group("match_density");

    let temp_dir = create_benchmark_files(50, 100);
    let glob_pattern = format!("{}/**/*.rs", temp_dir.path().display());

    // High match rate - matches every 5th line
    group.bench_function("high_match_rate", |b| {
        b.iter(|| run_find_fast(black_box(&glob_pattern), black_box("TODO")));
    });

    // Medium match rate - matches every 10th line
    group.bench_function("medium_match_rate", |b| {
        b.iter(|| run_find_fast(black_box(&glob_pattern), black_box(r"fn\s+function")));
    });

    // Low match rate - rare matches
    group.bench_function("low_match_rate", |b| {
        b.iter(|| run_find_fast(black_box(&glob_pattern), black_box("RAREPATTERN")));
    });

    group.finish();
}

fn benchmark_glob_patterns(c: &mut Criterion) {
    let temp_dir = create_benchmark_files(100, 100);
    let base_path = temp_dir.path().display().to_string();

    let mut group = c.benchmark_group("glob_patterns");

    // All files
    group.bench_function("all_files", |b| {
        let pattern = format!("{}/**/*", base_path);
        b.iter(|| run_find_fast(black_box(&pattern), black_box("fn")));
    });

    // Specific extension
    group.bench_function("specific_extension", |b| {
        let pattern = format!("{}/**/*.rs", base_path);
        b.iter(|| run_find_fast(black_box(&pattern), black_box("fn")));
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_small_files,
    benchmark_large_files,
    benchmark_regex_complexity,
    benchmark_parallel_scaling,
    benchmark_match_density,
    benchmark_glob_patterns
);
criterion_main!(benches);
