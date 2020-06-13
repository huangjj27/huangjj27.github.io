# 在天河二号上配置 Rust 运行环境

> 受朋友委托，需要帮忙在“天河二号”超级计算机上配置 Rust 编程语言运行环境，并配置安装 [`rust-overlaps`]。

## 阅读须知
本文将不涉及：
- [如何申请天河二号计算资源]
- [如何分布式运行程序]

## 通过 Rust 独立包安装适合 天河二号的 Rust 运行环境
0. ssh 远程登录到天河二号[^1]：
    ```shell
    $ ssh -i${YOUR_CERTIFICATE_ID} -P${SSH_PORT} ${YOUR_USERNAME}@server.ip.in.vpn
    ```
1. 获取超算的服务器平台架构：
    ```shell
    [you@tainhe2-H ~]$ uname -r
    ```
2. 了解平台架构后，获取对应平台的[Rust 独立安装包], 并上传至超算。此处以`x8_64`架构为例：
    ```shell
    $ scp -i${YOUR_CERTIFICATE_ID} -P${SSH_PORT} rust-1.44.0-x86_64-unknown-linux-gnu.tar.gz you@server.ip.in.vpn:~
    ```
3. 解压安装压缩包：
    ```shell
    [you@tainhe2-H ~]$ tar -zxvf rust-1.44.0-x86_64-unknown-linux-gnu.tar.gz
    ```
4. 切换到解压缩目录，并执行安装命令:
    ```shell
    [you@tainhe2-H ~]$ cd rust-1.44.0-x86_64-unknown-linux-gnu
    [you@tainhe2-H rust-1.44.0-x86_64-unknown-linux-gnu]$ ./install.sh --prefix=~/rust --disable-ldconfig --verbose
    ```
    此命令会将 Rust 安装在 `~/rust` 文件夹中，rust 的 可执行文件将会放在 `~/rust/bin`文件夹中。
5. 编辑`~/.bashrc`, 增加下面这一行配置：
    ```shell
    export PATH=$HOME/rust/bin:$PATH
    ```
6. 使`~/.bashrc`生效：
    ```shell
    [you@tainhe2-H ~]$ source ~/.bashrc
    ```
7. 检查 Rust 是否成功安装：
    ```shell
    [you@tainhe2-H ~]$ cargo --version
    cargo 1.44.0 (05d080faa 2020-05-06)
    ```

## 离线安装 `rust-overlaps`
0. 在本地联网环境拷贝源代码：
    ```shell
    git clone https://github.com/sirkibsirkib/rust-overlaps.git
    ```
1. 修复源码的 `Cargo.toml` 的`version`[^2]:
    ```toml
    version = "1.1.0"
    ```
2. 在代码仓库目录下执行 `cargo vendor`，获取依赖的源码[^3]：
    ```shell
    rust-overlaps$ cargo vendor --respect-source-config
    ```
    下载好的依赖将会存放到 `vendor`文件夹中。
3. 在 `rust-overlaps` 文件夹中添加 `.cargo/config` 文件，以便在超算的离线环境中使用本地缓存好的依赖源码进行编译：
    ```toml
    [source.crates-io]
    replace-with = "vendored-sources"

    [source.vendored-sources]
    directory = "vendor"
    ```
4. 将源码文件夹打包成 `.zip` 包，然后上传到超算:
    ```shell
    $ scp -i${YOUR_CERTIFICATE_ID} -P${SSH_PORT} rust-overlaps.zip you@server.ip.in.vpn:~
    ```
5. 在超算中解压：
    ```shell
    [you@tainhe2-H ~]$ unzip rust-overlaps.zip
    ```
6. 离线安装[^3]:
    ```shell
    [you@tainhe2-H ~]$ cd rust-overlaps
    [you@tainhe2-H rust-overlaps]$ cargo install --path . --offline
    ```
7. 检查是否安装成功：
    ```shell
    [you@tainhe2-H ~]$ rust-overlaps --version
    ASPOPsolver 1.0
    ```

[`rust-overlaps`]: https://github.com/sirkibsirkib/rust-overlaps

[如何申请天河二号计算资源]: http://www.nscc-gz.cn/Service/Igotto.html

[如何分布式运行程序]: https://tlanyan.me/tianhe-ii-guide/

[Rust 独立安装包]: https://forge.rust-lang.org/infra/other-installation-methods.html#standalone

[^1]: ssh登陆前还需登录VPN环境，账号密码为管理员提供的账号密码。

[^2]: Rust仓库的版本号遵循[语义化版本](https://semver.org/lang/zh-CN/)，因此必须为`x.y.z`的形式。

[^3]: `cargo`编译中断，可以重新运行命令继续安装，直到安装完成。
