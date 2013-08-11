fn main () {
    let x = 4;
    let f = fibo(x);
    io::println(fmt!("Fibo(%d) is %d", x, f));
}

fn fibo (x: int) -> int {
    io::println(fmt!("x is %d", x));
    let mut a = 0, b = 1;
    let mut count = 0;
    loop {
        if count >= x { break; }
        let temp = a;
        a = b;
        b = temp + b;
        count = count + 1;
        io::println(fmt!("\nb is %d", b));
    }
    return b;
}