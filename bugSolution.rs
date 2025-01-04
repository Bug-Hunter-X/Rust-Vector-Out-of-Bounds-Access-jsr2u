fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access the last element using get()
    match vec.get(2) {
        Some(last) => println!("The last element is: {}", last),
        None => println!("The vector is empty!")
    };

    //Alternative using if let
    if let Some(last) = vec.get(2) {
        println!("The last element is: {}", last);
    } else {
        println!("The vector is empty!");
    }
} 