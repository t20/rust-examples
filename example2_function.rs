fn main() {
    io::println("Hello functions");
    let x = square(12);
    io::println(fmt!("%d", x));
}


fn square (x: int)  -> int {
    x * x
}