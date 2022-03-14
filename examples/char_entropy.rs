use entropiter::{FrequencyIteratorExt, HashMapHistogram};

fn main() {
    let shannon_entropy = "shannon".chars().shannon::<HashMapHistogram<_>>().entropy();
    println!("entropy: {}", shannon_entropy);
}
