fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern provided");
    let path = std::env::args().nth(2).expect("No Path provided");

    println!("Pattern: {:?}, Path: {:?}", pattern, path);
}
