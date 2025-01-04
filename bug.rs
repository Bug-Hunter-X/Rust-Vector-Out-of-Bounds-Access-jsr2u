fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // This will panic if you try to access vec[3]!
    println!("The last element is: {}", vec[2]);
}