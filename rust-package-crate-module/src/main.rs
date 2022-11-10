mod mod_a;

use crate::mod_a::{a_test, b_test};

struct AlwaysEqual;

impl AlwaysEqual {
    fn info() {
        println!("Hello Rust!");
    }
}

fn main() {
    println!("{}", a_test::test_add(1,2));
    println!("{}", b_test::test_sub(1,2));

    let c1 = b_test::Color::new(121,32,35);
    println!("{:#?}", c1);
    c1.info();

    let mut c2 = b_test::Color {
        r: 20,
        g: 30,
        b: 40
    };
    c2.info();

    AlwaysEqual::info();

}
