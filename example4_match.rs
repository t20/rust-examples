fn main () {
    let my_number = 10;
    
    match my_number {
      0     => io::println("zero"),
      1 | 2 => io::println("one or two"),
      3..10 => io::println("three to ten"),
      _     => io::println("something else")
    }
    
    let (a, b) = ret_test_tuple();
    io::println(fmt!("\n%d", my_number));
    io::println(fmt!("\n%d", a));
    io::println(fmt!("\n%d", b));
    
}

fn ret_test_tuple () -> (int, int) {
    return (10, 20);
}