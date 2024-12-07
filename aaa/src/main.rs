
fn main() {
    /*
    struct Person { name: String, birth: i32, }

    let mut composers = Vec::new();

    composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
    composers.push(Person { name: "Dowland".to_string(), birth: 1563 });
    composers.push(Person { name: "Lully".to_string(), birth: 1632 });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
    */

    /*
    for i in 0..3 {
        println!("{} born in {}", composers[i].name, composers[i].birth);
    }
    println!("{}", composers[0].name);
    

    // Page 106
    let x = 14;
    let mut y = 78;

    let rx = &x;
    let sx = &x;
    let tx = &mut y;

    *tx = 9999;

    println!("rx = {:p}", rx);
    println!("sx = {:p}", sx);
    println!("tx = {:p}", tx);

    println!("y = {}", y);
    */

    // page 116 removing the * in *r < *s
    fn smallest(v: &[i32]) -> &i32 {
        let mut s = &v[0];
        for r in &v[1..] {
            if r < s { s = r; }
        }
        s
    }
    let s;
    {
        let parabola = [16, 9, 4, 2, 4, 9, 16];
        s = smallest(&parabola);
        println!("{}", *s);
    }
}
