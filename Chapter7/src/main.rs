extern crate communicator;

enum Light {
    Red,
    Green,
    Yellow,
}

fn main() {
    communicator::client::connect();
    communicator::network::server::connect();

    let red = Light::Red; // Long
    // Short but risky. Use catiously.
    use Light::*; let red2 = Red; // Short
}