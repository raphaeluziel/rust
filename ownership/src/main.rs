fn read(y: bool) {
    if y {
        println!("y is true");
    } else {
        println!("y aint true");
    }
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Wheel");
    name
}


fn main() {
    let x = false;
    read(x);

    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}
