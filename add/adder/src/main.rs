use add_one::{self, mod_a::test_a};


fn main() {
    let num = 10;
    println!(
        "Hello, World! {} plus one is {}",
        num,
        add_one::add(num, num+1)
    );

    test_a::info();
}
