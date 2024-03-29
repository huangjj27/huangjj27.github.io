# WASI探索（一） -- WASI简介与Wasmtime配置

## 什么是WASI?
WASI[^1]是一个新的API体系, 由[Wasmtime项目]设计, 目的是为WASM设计一套引擎无关(engine-indepent), 面向非Web系统(non-Web system-oriented)的API标准. 目前, WASI核心API(WASI Core)在做覆盖文件, 网络等等模块的API, 但这些实现都是刚刚开始实现, 离实用还是有很长路要走.

## 关于WASM runtime
在了解了WASI之后, 博主最后选定两个WASM运行时进行探索: WASMER 与 Wasmtime. 这两款运行时都号称开始支持了WASI标准, 但博主使用[rust-wasi-tutorial]对两款运行时进行试验后, 发现[WASMER对于文件读取还是有些问题], 而Wasmtime则是通过了规格测试(基于specs testsuite), 因此本文接下来着重于Wasmtime的配置介绍.

[rust-wasi-tutorial]: https://github.com/kubkon/rust-wasi-tutorial
[WASMER对于文件读取还是有些问题]: https://github.com/wasmerio/wasmer/issues/356

## Wasmtime与rust环境配置
由于目前Wasmtime与WASMER均只支持Unix-like环境, 接下来楼主将演示如何在WSL(Ubuntu 18.04)下配置Wasmtime. 而在目前比较方便生成wasm的编程语言中, 博主选择使用自带wasi目标的[rust编程语言], 可以"零代价"配置wasm相关工具链.

### 配置rust
1. 下载并安装rustup: `curl https://sh.rustup.rs -sSf | sh`, 安装时使用默认 `stable-x86_64-unknown-linux-gnu`工具链, 后面我们还会自行添加用于编译wasm的`nightly`工具链.
2. 为cargo配置ustc反代, 提高crates(rust库)下载速度[^2]
3. 安装rustfmt: `rustup component add rustfmt --toolchain stable-x86_64-unknown-linux-gnu`. Wasmtime的test脚本需要用到该组件.
4. 安装rust nightly工具链: `rustup toolchain add nightly-x86_64-unknown-linux-gnu`. 当前rust的WASI目标还在开发中, 尚未稳定[^3].
5. 安装rust WASI目标: `rustup target add wasm32-unknown-wasi`[^3].

### 配置Wasmtime
1. 安装cmake与clang: `sudo apt install cmake clang`, 用于编译Wasmtime. Wasmtime目前尚未有正式发布版本, 故需要我们自行编译.
2. 拷贝Wasmtime源码: `git clone --recursive git@github.com:CraneStation/wasmtime.git ~/wasmtime`.
3. 切换到Wasm源码目录: `cd ~/wasmtime`
4. 执行测试脚本: `./scripts/test-all.sh`. 当脚本执行完毕并通过测试后, 说明wasmtime已经正常编译并且能在当前WSL环境下正常工作, 可以使用生成的wasmtime可执行文件.
5. 将生成的wasmtime拷贝到`/usr/bin`目录中: `cp ~/wasmtime/target/release/wasmtime /usr/bin`, 以便在整个WSL环境中任意目录执行wasmtime. wasmtime是个单文件(stand alone)运行时.
6. 执行`wasmtime --help`命令, 确认wasmtime成功安装.


## 试验
GitHub上面已经有了比较简单的[试验](https://github.com/kubkon/rust-wasi-tutorial), 大家按照上面的说明
去试验即可. 下一篇文章, 博主将会把[猜数字]编译成WASI目标并执行, 同时会尝试把一些常用的库尝试编译, 来探究
当前社区对WASI支持的程度.

[Wasmtime项目]: https://github.com/CraneStation/wasmtime
[rust编程语言]: https://rust-lang.org
[猜数字]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

[^1]: [WASI概要](https://github.com/CraneStation/wasmtime/blob/master/docs/WASI-overview.md#wasi-webassembly-system-interface)

[^2]: [rust crates 镜像使用帮助 - 中国科学技术大学](https://lug.ustc.edu.cn/wiki/mirrors/help/rust-crates#rust_crates_镜像使用帮助)

[^3]: 截至2020年1月19日，WASI目标已经稳定并重命名为`wasm32-wasi`
