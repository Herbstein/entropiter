# Entropiter

Easily calculate the frequency and probability of symbol occurrence in an iterator, and effortlessly calculate the entropy.

## Usage

To use this library you only need an iterator containing the symbol of your choice, and the choice of histogram backing store to accumulate into. The generic backing stores available are `BTreeMapHistogram` and `HashMapHistogram`, while for `u8` symbols `ByteHistogram` is also available.

### Example

```rust
use entropiter::{FrequencyIteratorExt, HashMapHistogram};

fn main() {
    let shannon_entropy = "shannon".chars().shannon::<HashMapHistogram<_>>().entropy();
    println!("entropy: {}", shannon_entropy);
}
```
