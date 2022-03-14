use std::collections::HashMap;

use entropiter::FrequencyIteratorExt;

fn main() {
    let shannon_entropy = "shannon".chars().shannon::<HashMap<_, _>>().entropy();
    println!("entropy of shannon: {}", shannon_entropy);
}
