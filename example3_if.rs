fn main() {
    io::println("testing sign of a number");
    let x = signum(10);
    io::println(fmt!("%d", x));
    let x = signum(-20);
    io::println(fmt!("%d", x));
    let x = signum(0);
    io::println(fmt!("%d", x));
}

fn signum(x: int) -> int {
    if x < 0 { -1 }
    else if x > 0 { 1 }
    else { 0 }
}