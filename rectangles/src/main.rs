fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn main() {
    let rect1 = (30, 50);

    println!("The area of rectangle is {} square pixels", 
             area(rect1));
}
