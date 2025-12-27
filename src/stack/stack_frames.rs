fn inner() {
    let x = 42;
    println!("inner x addr: {:p}", &x);
}

pub fn run() {
    let y = 7;
    println!("outer y addr: {:p}", &y);
    inner();
}