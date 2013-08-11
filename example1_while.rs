fn main() {
    let hi = "hi";
    let mut count = 0; //inmuttable variable

    while count < 10 {
        io::println(hi);
        io::println(fmt!("%s is %d", hi, count));
        count += 1;
    }
}
