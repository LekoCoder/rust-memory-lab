fn recurse(depth: usize) {
    let local = depth;
    println!("depth {:3}, addr: {:p}", depth, &local);
    recurse(depth + 1);
}

pub fn run() {
    recurse(0)
}