# Entropiter

Easily calculate the frequency and probability of symbol occurrence in an iterator, and effortlessly calculate the entropy.

## Usage

The user of the library chooses whether to use a `HashMap` or a `BTreeMap` as a backing store, each with their own tradeoffs as explained in their official documentation.

### Example

```rust
use std::collections::HashMap;

use entropiter::FrequencyIteratorExt;

fn main() {
    let shannon_entropy = "shannon".chars().shannon::<HashMap<_, _>>().entropy();
    println!("entropy of shannon: {}", shannon_entropy);
}
```
