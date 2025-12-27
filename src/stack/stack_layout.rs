pub fn run() {
    let a: u8 = 1;
    let b: u64 = 2;
    let c: u32 = 3;

    println!("a value: {}, address: {:p}", a, &a);
    println!("b value: {}, address: {:p}", b, &b);
    println!("c value: {}, address: {:p}", c, &c);
}