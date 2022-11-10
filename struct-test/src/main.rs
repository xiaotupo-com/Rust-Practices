/// 定义用户结构体
#[allow(dead_code)]
#[derive(Debug)]
struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

struct Color(i8, f64, i8);

fn main() {
    let mut user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };

    user1.active = false;
    user1.username = "xiaotupo.com";

    println!("{:#?}", user1);

    let c1 = Color(123, 32.045, 44);
    println!("{},{},{}", c1.0, c1.1, c1.2);
}
