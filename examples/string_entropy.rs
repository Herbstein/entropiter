use entropiter::{ByteHistogram, FrequencyIteratorExt};

fn main() {
    let entropy = [0, 1, 0, 0, 0]
        .into_iter()
        .shannon::<ByteHistogram>()
        .entropy();
    println!("entropy: {}", entropy);
}
