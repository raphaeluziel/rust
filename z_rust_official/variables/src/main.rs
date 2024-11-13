fn main() {
    /*
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    */
    /*
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    println!("The value of x is {x}");
    */

    /*
    let spaces = "      ";
    let spaces = spaces.len();
    println!("Spaces: {spaces}");
    */

    /*
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess: {guess}");
    */

    /*
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is {y}");
    */

    /*
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("first = {}, second = {}, third = {}", five_hundred, six_point_four, one);
    */

    /*
    let mut x: (i32, i32) = (1, 2);
    x.0 = 0;
    x.1 += 5;
    println!("x.0 = {}", x.0);
    println!("x.1 = {}", x.1);
    */

    let a: [i32; 5] = [1, 3, 5, 7, 90];
    let b = [457; 11];
    let c = [9, 8, 7];
    println!("a[2] = {}", a[2]);
    println!("b[7] = {}", b[7]);
    println!("c[1] = {}", c[1]);  
}
