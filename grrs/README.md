## Grrs 命令行

通过学习 `grrs` 程序来熟练 `rust` 的 `Shell` 编程方法。

## 用到的包
1. [clap](https://crates.io/crates/clap "clap 命令行解析器")

## 流程
1. 获取命令行参数


## 获取命令行参数

```rust
fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    println!("{},{}", pattern, path);
}  
```
## 定义命令行参数结构体
```rust
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path)
    };

    println!("模式：{}, 文件：{}", args.pattern, args.path.display());
}
```

## Cargo 传参数
使用 `cargo run` 传参数的时候会遇到与`cargo` 本身相冲突的参数，这个时候需要在 `cargo run --` 后面添加两个横杠。
如：`cargo run -- --help` 该命令的作用是查看我们所写程序的帮助信息。
