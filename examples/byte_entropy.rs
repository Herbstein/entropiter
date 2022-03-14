use entropiter::{ByteHistogram, FrequencyIteratorExt};

fn main() {
    let entropy = [0, 1, 2, 2]
        .into_iter()
        .shannon::<ByteHistogram>()
        .entropy();
    println!("entropy: {}", entropy);
}
