// `#[no_mangle]` 关闭混淆功能以让 C 程序找到调用的函数
// `extern` 默认导出为 C ABI
#[no_mangle]
pub extern fn print_hello_from_rust() {
    println!("Hello from rust");
}
