# WASI探索（二） -- WASI版猜数字

> 猜数字作为入门Rust时第一次编写并具有实际功能的程序，适合让读者快速掌握rust的基本概念。同时，为了让程序更加有趣，博主在原本的猜数字程序上添加了日志和从运行时参数传递游戏难度的功能。此外，由于博主偏好改变，本文还会涉及到另外一款WASI运行时Wasmer，以及他们为了丰富WASI生态而推出的wasm包管理器wapm。

## 阅读须知
学习外部资料更有助于读者了解相关生态，因此本文将不赘述：
- [WASI是什么？](/wasi/wasi_and_wasmtime.html#什么是wasi)
- [Wasmer与wapm如何安装？](https://wapm.io/help/install)
- [rust是什么？](https://www.rust-lang.org/zh-CN/) [如何安装Rust？](https://www.rust-lang.org/zh-CN/tools/install) [如何学习rust？](https://www.rust-lang.org/zh-CN/learn)
- [cargo是什么？要怎样使用？](https://doc.rust-lang.org/cargo/index.html)
- [猜数字原版程序在哪？](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
- [log crate](https://docs.rs/log/0.4.6/log/)与[env_log crate](https://docs.rs/env_logger/0.6.1/env_logger/)详细如何使用？
- [structopt](https://docs.rs/releases/search?query=structopt)如何使用？

而阅读本文，你将了解：
- 如何用日志debug的一些原则
- 一个简单的配置文件的设计
- 读者对Wasmer的一些浅薄看法

## 原版猜数字
我们从[官方书](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)拷贝了一份猜数字程序：

```rust,no_run
{{#include ../../code/guess/main.rs:all}}
```

```toml
{{#include ../../code/guess/Cargo.toml:show}}
```

## 一次游戏只猜一个数
我们可以看到，这个程序每次运行，只能猜一个数字，如果要继续玩就只能重新启动。但是博主想让这个游戏，能在一次运行时
可以生成不同难度关卡，因此首先我们将“猜一个数字”逻辑抽取成可复用函数
```rust,no_run
// main.rs
fn guess_a_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess_str = String::new();

        io::stdin().read_line(&mut guess_str)
            .expect("Failed to read line");

        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

再将配置文件修改一下：

```toml
# guess_wasi/Cargo.toml
{{#include ../../code/guess_wasi/Cargo.toml:rand}}
```

此外，猜数字游戏的难度取决于随机生成数字的范围, 为了生成不同的难度关卡，我们需要`guess_a_number`接受一组控制
生成数字范围的参数：
```rust
// main.rs
/// 生成熟悉范围的下界（lower bound，lb)与上界（higher bound，hb）在主函数中读取配置文件得到
fn guess_a_number((lb, hb): (u32, u32)) {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(lb, hb + 1);

    // ...
}
```
这里在传入参数时，直接[解构](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html?highlight=destruct#destructuring-to-break-apart-values)了[元组](https://doc.rust-lang.org/book/ch03-02-data-types.html#compound-types), 这样后面就可以直接使用传入的上界与下界来控制生成数范围

然后，博主发现， 原版猜数字如果解析数字错误的话会直接跳过，博主觉得这里应该至少提醒一下用户输入错误了：
```rust
// main.rs
fn guess_a_number((lb, hb): (u32, u32)) {
    // ...
        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input not a number! please input a number");
                continue;
            },
        };
    // ...
}
```

## 加上log追踪生成的数据情况
使用log去追踪数据与可能产生bug的代码有以下好处：
- 了解运行时所关注的数据情况， 方便定位bug
- 清晰地知道实际运行流程是否如期望那样执行
- 即便使用release版目标， 仍然可以获得需要的分析信息
- 区分产生信息的层级，以便将精力集中在优先需要处理的信息中

回到猜数字游戏上，博主想要知道每一次游戏中知道生成的`secret_number`是多少， 并且根据运行时输入的日志层级的参数
决定是否显示这个数字，需求相对简单，因此使用rust生态中比较常用的[log crate](https://docs.rs/log/0.4.6/log/)与[env_log crate](https://docs.rs/env_logger/0.6.1/env_logger/)。在`Cargo.toml`中加入两个新依赖：

```toml
# guess_wasi/Cargo.toml

# ...

{{#include ../../code/guess_wasi/Cargo.toml:log}}
```

加入追踪日志代码：
```rust
// main.rs
use log::{trace, debug};

fn main() {
    // 别忘了初始化日志生成器， 才能获取日志！
    env_log::init();
    guess_a_number((1, 100));
}

fn guess_a_number((lb, hb): (u32, u32)) {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(lb, hb + 1);
    trace!("secret number: {}", secret);

    loop {
        println!("Please input your guess.");

        let mut guess_str = String::new();
        io::stdin().read_line(&mut guess_str)
            .expect("Failed to read line");
        debug!("scaned string: {:?}", guess_str);

        // ...
    }
}
```

## 向高难度挑战！
现在我们来到最后了一个需求：通过运行时参数来给每次游戏输入多个游戏难度，这个难度由随机数生成的范围决定-- 随机数
生成的范围越大，一次猜中这个数的概率就越小。为方便地写出输入参数的命令，我们需要引入`structopt`库（crate）,
最后获得类似`--levels=10 100 1000`这样的参数输入方式, 参数中每个数字表示每次生成随机数的生成范围上界。

配置文件追加：
```toml
# guess_wasi/Cargo.toml

# ...

{{#include ../../code/guess_wasi/Cargo.toml:structopt}}
```

编写参数代码。
```rust
// main.rs

// ...
{{#include ../../code/guess_wasi/main.rs:structopt}}
```

## 完整代码
```toml
{{#include ../../code/guess_wasi/Cargo.toml:all}}
```

```rust
{{#include ../../code/guess_wasi/main.rs:all}}
```

读到这里，读者可以发现前文**根本没涉及到WASI，甚至没有涉及WASM**。这因为WASI作为应用与运行时交互的接口，被rust编译器封装成为编译目标，读者只需要编译到对应目标即可让自己的程序在对应平台上运行. 这是Rust编程语言现代化与工程学的体现：
一般应用研发工程师可以通过使用已经适配所需平台的底层库(这些底层库通常已经针对所有支持平台做了最优化适配)，就能让自己的应用支持对应的平台而无需重新编写针对某平台的特化版本源码！

## 是时候编译成WASI目标了
我们还需要添加对应的编译目标：
```
rustup target add wasm32-wasi
```

编译到`wasm32-wasi`目标上：
```
$ cargo build --target=wasm32-wasi --release
   Compiling proc-macro2 v1.0.18
   Compiling version_check v0.9.2
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.30
   Compiling cfg-if v0.1.10
   Compiling memchr v2.3.3
   Compiling getrandom v0.1.14
   Compiling wasi v0.9.0+wasi-snapshot-preview1
   Compiling lazy_static v1.4.0
   Compiling bitflags v1.2.1
   Compiling atty v0.2.14
   Compiling unicode-width v0.1.7
   Compiling unicode-segmentation v1.6.0
   Compiling log v0.4.8
   Compiling quick-error v1.2.3
   Compiling ansi_term v0.11.0
   Compiling ppv-lite86 v0.2.8
   Compiling regex-syntax v0.6.18
   Compiling strsim v0.8.0
   Compiling vec_map v0.8.2
   Compiling termcolor v1.1.0
   Compiling thread_local v1.0.1
   Compiling textwrap v0.11.0
   Compiling proc-macro-error-attr v1.0.2
   Compiling proc-macro-error v1.0.2
   Compiling humantime v1.3.0
   Compiling heck v0.3.1
   Compiling quote v1.0.7
   Compiling rand_core v0.5.1
   Compiling clap v2.33.1
   Compiling regex v1.3.9
   Compiling rand_chacha v0.2.2
   Compiling env_logger v0.7.1
   Compiling syn-mid v0.5.0
   Compiling rand v0.7.3
   Compiling structopt-derive v0.4.7
   Compiling structopt v0.3.14
   Compiling guess_wasi v0.1.0 (C:\Users\huangjj27\Documents\codes\huangjj27.github.io\code\guess_wasi)
    Finished release [optimized] target(s) in 4m 58s
```

现在，我们来运行一下程序吧：
```
$ wasmer --version
wasmer 0.13.1
$ wasmer run .\target\wasm32-wasi\release\guess.wasm --env RUST_LOG=trace -- --levels 10 100 1000
given number range 0~10
[2020-06-09T14:55:58Z TRACE guess] secret number: 10
Please input your guess.
5
[2020-06-09T14:56:02Z DEBUG guess] scaned string: "5\r\n"
You guessed: 5
too small!
Please input your guess.
8
[2020-06-09T14:56:04Z DEBUG guess] scaned string: "8\r\n"
You guessed: 8
too small!
Please input your guess.
9
[2020-06-09T14:56:07Z DEBUG guess] scaned string: "9\r\n"
You guessed: 9
too small!
Please input your guess.
10
[2020-06-09T14:56:09Z DEBUG guess] scaned string: "10\r\n"
You guessed: 10
You get it!
given number range 0~100
[2020-06-09T14:56:09Z TRACE guess] secret number: 60
Please input your guess.
60
[2020-06-09T14:56:25Z DEBUG guess] scaned string: "60\r\n"
You guessed: 60
You get it!
given number range 0~1000
[2020-06-09T14:56:25Z TRACE guess] secret number: 715
Please input your guess.
300
[2020-06-09T14:56:32Z DEBUG guess] scaned string: "300\r\n"
You guessed: 300
too small!
Please input your guess.
720
[2020-06-09T14:56:38Z DEBUG guess] scaned string: "720\r\n"
You guessed: 720
too big!
Please input your guess.
716
[2020-06-09T14:56:41Z DEBUG guess] scaned string: "716\r\n"
You guessed: 716
too big!
Please input your guess.
714
[2020-06-09T14:56:46Z DEBUG guess] scaned string: "714\r\n"
You guessed: 714
too small!
Please input your guess.
715
[2020-06-09T14:56:48Z DEBUG guess] scaned string: "715\r\n"
You guessed: 715
You get it!
$
```

调试后，确认我们的程序可以正常执行了， 去掉`--env RUST_LOG=trace`参数，享受自己制作的这个小游戏吧！

<!-- TODO：待更新 -->
## Wasmer与Wapm
[Wasmer](https://github.com/wasmerio/wasmer)可以是说在WASI生态中响应速度仅次于Mozilla的组织，他们号称打造了
一款可以让代码“一次构建，处处运行”（Build Once, Run Anywhere.)的运行时环境，该环境可以运行ECMAScripten标准与
WASI标准的wasm栈机码。并且方便为wasm代码分发，该组织开发了类似于nodejs生态中npm的包管理工具wapm，这样用户就可以
很轻松地发布自己的程序，以及利用他人的程序了--这促进了WASM生态的发展，同时作为生态底层的领导者，Wasmer也将拥有
更多发言权。

作为边缘人士（稍微知道WASM生态但没很深入了解），博主看到这项目背后的布局很像上世纪Sun公司的Java和JVM（尽管WASM并不是Wasmer的发明，但这样反而不必为WASM这样可以作为主流编程语言编译目标工具投入过多精力宣传，可以集中精力去优化wasmer与wapm）同时因为wasmer是使用MIT协议授权，不会产生类似OracleJDK专利权所属的问题，相信随着生态的进一步发展，在虚拟机运行时领域会逐步替代JVM成为主流，届时将解放程序员更多生产力 -- 不必要求掌握Java而是通过自己熟悉的编程语言（c/c++/rust/python/...)通过统一的标准相互调用（进一步微型化的微服务）。

而这个在服务器/PC桌面应用占主导地位的标准，就是WASI。
