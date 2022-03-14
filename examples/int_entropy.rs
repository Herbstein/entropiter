use entropiter::{BTreeMapHistogram, FrequencyIteratorExt};

fn main() {
    let entropy = [0, 1, 2, 6, 87654, 64, 89234, 234234, 87654, 0]
        .into_iter()
        .shannon::<BTreeMapHistogram<_>>()
        .entropy();
    println!("entropy: {}", entropy);
}
