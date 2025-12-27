fn recurse_forever() {
    let x = 0;
    println!("{:p}", &x);
    recurse_forever();
}

pub fn run() {
    recurse_forever();
}