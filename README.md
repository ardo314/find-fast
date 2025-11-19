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

```bash
cargo test
```
