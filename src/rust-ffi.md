# 在 WSL 中学习 Rust FFI

> 博主最近从新学习 Rust FFI 的使用，但是手头上没有可用的 Linux 环境（Windows 编译c太麻烦了），于是就尝试着使用 WSL来搭建 Rust 环境和简易的 c 编译环境，并记录下中间遇到的一些坑。感谢 Unsafe Rust 群群友 @框框 对本文的首发赞助！感谢 Rust 深水群 @栗子 的 gcc 指导！

## 阅读须知
阅读本文，你可以知道：
- 一些配置 WSL 全局变量的技巧
- 快速配置 Rust 编译运行环境
- 简单的 gcc 编译技巧

但是，本文不涉及：
- [如何安装 WSL?](https://docs.microsoft.com/zh-cn/windows/wsl/wsl2-install)
- [如何解决 WSL 中文乱码问题?](https://www.zhihu.com/question/59714225)
    顺带一提的是，博主通过 VS Code 使用 WSL，因为 Win 10 已经配置成 UTF-8 编码，所以并没有出现乱码问题
- [Rustup 国内镜像有哪些?](https://mp.weixin.qq.com/s?__biz=MzIwMTAxMjg5Ng==&mid=2247483684&idx=2&sn=3cd85509a27b6f74fa220bdb38db6c46&chksm=96f522eba182abfdf3a738e880da8cb1b1d36f8ada87ed5fa9b5ef60a3025082f1ae78abd444#rd)
- [cargo 详细使用教程](https://doc.rust-lang.org/cargo/)
- [甚至不会讲 Rust FFI 是什么](https://doc.rust-lang.org/nomicon/ffi.html)

## WSL Rust 环境搭建
由于 WSL 是新装的，没有 Rust 和 gcc/g++ 环境，因此需要安装：

```shell
sudo apt install gcc -y

# 官方脚本
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<!-- 坑1：共享变量问题 -->
但是由于在国内访问 Rust 官方网站会很慢，因此设置镜像到 Windows 环境变量中：

```
RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

然后，使用 [`WSLENV`环境变量](https://devblogs.microsoft.com/commandline/share-environment-vars-between-wsl-and-windows/)将上述变量共享到 WSL 中：

```
WSLENV=RUSTUP_DIST_SERVER:RUSTUP_UPDATE_ROOT
```

然后重启 WSL 终端，重新执行 Rust 一键脚本。

以下两个项目均来自 《Rust编程之道》一书，源代码仓库在[这里](https://github.com/ZhangHanDong/tao-of-rust-codes)

## Rust 调用 C/C++
Rust 调用 C/C++ 代码可以使用 `cc` crate 配合 `build.rs` 预先编译好 C/C++ 的程序提供给 Rust 调用。

首先，创建一个 binary 项目：

```
cargo new --bin ffi_learn
```

项目目录结构如下：
```
cpp_src
    |-- sorting.h
    |-- sorting.cpp
src
    |-- main.rs
Cargo.toml
build.rs
```

然后编写 `sorting.h` 和 `sorting.cpp`:
```cpp
// sorting.h
{{#include ../code/ffi_call_cpp/cpp_src/sorting.h}}
```

```cpp
// sorting.cpp
{{#include ../code/ffi_call_cpp/cpp_src/sorting.cpp}}
```

然后给 `Cargo.toml` 的 `[build-dependecies]` 加上 `cc` crate 依赖：
```toml
# Cargo.toml
# 其他配置

{{#include ../code/ffi_call_cpp/Cargo.toml:cc}}
```

接着，我们通过 `cc` 调用对应平台的c/c++编译器，因为我们这个项目是 WSL，所以和调用我们刚安装的 `gcc`:
```rs
// build.rs
{{#include ../code/ffi_call_cpp/build.rs}}
```

接着，我们在 Rust 主程序中，通过 `extern` 块引入`sorting.cpp`中的`interop_sort`函数，并调用它:

```rust,no_run
// main.rs
{{#include ../code/ffi_call_cpp/src/main.rs}}
```

然后执行调用：
```shell
$ cargo run
   Compiling ffi_learning v0.1.0 (/mnt/c/Users/huangjj27/Documents/codes/ffi_learning)
warning: `extern` block uses type `[i32]`, which is not FFI-safe
 --> src/main.rs:3:26
  |
3 |     fn interop_sort(arr: &[i32], n: u32);
  |                          ^^^^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes)]` on by default
  = help: consider using a raw pointer instead
  = note: slices have no C equivalent

    Finished dev [unoptimized + debuginfo] target(s) in 4.71s
     Running `target/debug/ffi_learn`
Before sorting...
[10, 42, -9, 12, 8, 25, 7, 13, 55, -1]

After sorting...
[55, 42, 25, 13, 12, 10, 8, 7, -1, -9]
```

我们看到，该函数提示我们 C 中并没有等价于 Rust slice 的类型，原因在于如果我们传递 slice，那么在 C/C++ 中就很容易访问超过数组长度的内存，造成内存不安全问题。但是，我们在 Rust 调用的时候，通过同时传入数组 `arr` 的长度 `arr.len()`, 来保证函数不会访问未经授权的内存。不过在实践中，应该划分模块，只允许确认过 内存安全的 safe Rust 功能跨越模块调用。

## 在 C/C++ 中调用 Rust

接下来我们反过来互操作。项目结构如下：
```
c_src
    |-- main.c
src
    |-- lib.rs
    |-- callrust.h
Cargo.toml
makefile
```

然后配置 Rust 生成两种库——静态库（staticlib）和c动态库（cdylib）：
```toml
# Cargo.toml
# ...

{{#include ../code/c_call_rust/Cargo.toml:lib}}
```

然后添加我们的 Rust 函数：

```rust
// lib.rs

{{#include ../code/c_call_rust/src/lib.rs}}
```

当然，为了给 C 调用我们还需要编写一个头文件：
```c
// callrust.h
{{#include ../code/c_call_rust/src/callrust.h}}
```

在我们的 `main.c` 中库并调用：

```c
// main.c
{{#include ../code/c_call_rust/c_src/main.c}}
```

<!-- 坑2：执行时需要指定LD_LIBRARY_PATH -->
编写 makefile，先调度cargo 编译出我们需要的 Rust 库（动态或链接），然后再运行：
```makefile
{{#include ../code/c_call_rust/makefile}}
```

## 小结
本文通过给出两个简单的示例来展示 Rust 通过 FFI 功能与 C/C++ 生态进行交互的能力, 并且指出几个在实践过程中容易浪费时间的坑：
1. WSL的环境变量不生效 -> 使用 `WSLENV` 变量从 Windows 引入使用。
2. `make share` 的时候提示 `libcallrust.so` 找不到 -> 需要在运行时指定 `LD_LIBRARY_PATH` 变量，引入我们编译的 `libcallrust.so` 路径。
3. `make static`的时候遇到了`pthread_*` `dy*`系列函数未定义问题 -> 通过动态链接系统库来支持运行。


---
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/gitalk@1/dist/gitalk.css">
<script src="https://cdn.jsdelivr.net/npm/gitalk@1/dist/gitalk.min.js"></script>
<div id="gitalk-container"></div>

<script>
const gitalk = new Gitalk({
  clientID: '5af6fa1218b8ad6d12e9',
  clientSecret: '0c226cbc5544c3252c1c0fba0b01ca9b7bf61691',
  repo: 'blog-gitment',      // The repository of store comments,
  owner: 'huangjj27',
  admin: ['huangjj27'],
  id: '/posts/rust-ffi/',      // Ensure uniqueness and length less than 50
  distractionFreeMode: false  // Facebook-like distraction free mode
})

gitalk.render('gitalk-container')
</script>
