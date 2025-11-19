# find-fast

A fast grep-like tool that searches files matching a glob pattern for regex patterns.

## Usage

```bash
find-fast '<glob-pattern>' '<regex-pattern>'
```

**Important:** Always quote your glob pattern to prevent shell expansion!

### Examples

```bash
# Search all Rust files for functions named 'main'
find-fast '**/*.rs' 'fn.*main'

# Search all text files for the word 'TODO'
find-fast '**/*.txt' 'TODO'

# Search all files in src/ directory for 'Config'
find-fast 'src/**/*' 'Config'
```

## Why Quote the Glob Pattern?

Without quotes, your shell (bash/zsh) will expand the glob pattern before passing it to the program:

```bash
# ❌ BAD - Shell expands *.rs before the program sees it
find-fast *.rs 'pattern'
# The program receives: find-fast main.rs lib.rs utils.rs 'pattern'

# ✅ GOOD - The program receives the pattern and does the expansion
find-fast '*.rs' 'pattern'
# The program receives: find-fast '*.rs' 'pattern'
```

## Building

```bash
cargo build --release
```

## Testing

### Integration Tests

Run comprehensive integration tests on the release build:

```bash
cargo test --release
```

The test suite includes:

- Error handling (invalid patterns, missing arguments)
- Search functionality across different file types
- Output format verification
- Glob pattern matching
- Regex complexity tests

### Performance Benchmarks

Run performance benchmarks to measure search speed:

```bash
cargo bench --bench search_benchmark
```

Benchmarks measure:

- Small vs large file performance
- Regex complexity impact
- Parallel processing efficiency
- Match density effects
- Throughput metrics

Results are saved in `target/criterion/` with detailed HTML reports.

See [TESTING.md](TESTING.md) for detailed testing documentation.

## Related Implementations

This is part of a group of implementations. Specification can be found in the Elixir repository by pherklotz.

Other implementations of this in other languages can be found here:

- [Haskell provided by mgrosser3](https://github.com/mgrosser3/findfast/)
- [Elixir provided by pherklotz](https://github.com/pherklotz/find-fast-elixir/)
