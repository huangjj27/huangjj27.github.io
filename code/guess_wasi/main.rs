/* ANCHOR: all */
// guess_wasi/main.rs
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use log::{debug, trace};

/* ANCHOR: structopt */
use structopt::StructOpt;

// 定义参数只需要把他们的名字和类型写在一个参数结构体中即可！
#[derive(StructOpt)]
#[structopt(name="guess_wasi")]
struct Opt {
    #[structopt(long="levels")]
    levels: Vec<u32>,
}

fn main() {
    env_logger::init();

    // 获取并访问levels参数, 只需要访问参数结构体的对应成员即可， 细节处理可以方便地交给库执行！
    let opt = Opt::from_args();
    for &lv in &opt.levels {
        println!("given number range 0~{}", lv);
        guess_a_number((0, lv));
    }
}
/* ANCHOR_END: structopt */

// 一场游戏有多个难度，我们每个难度只猜一个数字，然后变难
fn guess_a_number((lb, hb): (u32, u32)) {
    let secret = rand::thread_rng().gen_range(lb, hb + 1);
    trace!("secret number: {}", secret);

    loop {
        println!("Please input your guess.");

        let mut guess_str = String::new();
        io::stdin().read_line(&mut guess_str)
            .expect("Failed to read line");
        debug!("scaned string: {:?}", guess_str);

        let guess: u32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input not a number! please input a number");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("You get it!");
                break;
            }
        }
    }
}
/* ANCHOR_END: all */
