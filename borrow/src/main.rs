fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(&m1, &m2);
    let s = format!("{} {}", m1, m2);
    println!("printing {s}");
    println!("---------------------");

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;
    *x += 1;

    let r1: &Box<i32> = &x;
    let b: i32 = **r1;

    let r2: &i32 = &*x;
    let c: i32 = *r2;

    println!("x = {x}");
    println!("a = {a}");
    println!("r1 = {r1}");
    println!("b = {b}");
    println!("c = {c}");

    println!("---------------------");

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);
    let x_abs_2 = x.abs();
    assert_eq!(x_abs1, x_abs_2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r);
    let r_abs2 = r.abs();
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);
    let s_len2 = s.len();
}

fn greet(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2);
}