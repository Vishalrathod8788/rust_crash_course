fn main() {
    let _x = 10;
    let x = 20;
    let z = _x + x;
    println!("{}", z);

    let xyz: bool = true;
    println!("{}", xyz);

    let xyz: char = 'a';
    println!("{}", xyz);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
}
