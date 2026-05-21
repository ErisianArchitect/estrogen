# E<ins>STR</ins>O<ins>GEN</ins>

A library for compile time string generation.

# Examples

### `estrogen::repeat`
Repeat a string literal some number of times.
```rust
use estrogen::repeat;

const SPACES: &'static str = repeat!(8, " ");

assert_eq!(SPACES.len(), 8);
assert_eq!(SPACES, "        ");
```

### `estrogen::join`
Join strings together with a separator.
```rust
use estrogen::join;

const FOOBARBAZ: &'static str = join!("|", ["foo", "bar", "baz"]);

assert_eq!(FOOBARBAZ, "foo|bar|baz");
```
