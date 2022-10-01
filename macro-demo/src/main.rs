
/// 加法宏，该例子需要在 解释器中添加个 0
///```rust
/// macro_rules! sum{
///    ($($a: expr), *) => {
///        0 $(+ $a)* // 0 + n + n + n，需要添加 0
///    }
/// }
///
/// fn main() {
///     println!("{}", sum![1,2,3,4,5]);
/// }
///```

macro_rules! sum {
    ($a: expr) => {
        $a
    };
    ($a: expr, $b: expr) => {
        $a + $b
    };
    ($a: expr, $($b: tt)*) => {
        $a + sum!($($b)*)
    }
}

fn main() {
    println!("{}", sum![1,2,3,4,5]);
}
