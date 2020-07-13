# Rust 安全应用开发51条

> 本文摘自法国国家网络安全局(ASSNI)的《用 Rust 开发安全应用的编程规则》（[_PROGRAMMING RULES TO DEVELOPSECURE APPLICATIONS WITH RUST_](https://www.ssi.gouv.fr/uploads/2020/06/anssi-guide-programming_rules_to_develop_secure_applications_with_rust-v1.0.pdf)）

1. 要使用 stable 编译工具链
2. 要在 cargo 配置文件中将重要变量保持为默认值
3. 要在运行 cargo 时保持编译环境变量为默认值
4. 要周期地使用 linter
5. 要使用 Rust 格式器（rustfmt）
6. 要人工检查自动修复
7. 要检查依赖版本是否过期（cargo-outdated)
8. 要检查依赖的安全脆弱性（vulnerabilities）（cargo-audit)
9. 要遵循命名转换
10. 不要使用 `unsafe` 块
11. 要用合适的算术操作来处理潜在的溢出
12. 推荐实现包含了所有可能错误的自定义错误类型
13. 推荐使用 `?` 操作符且不使用 `try!` 宏
14. 不要使用能导致 `panic!` 的函数
15. 要测试数组索引使用是否正确，或者使用 `get` 方法
16. 要在 FFI 中正确地处理 `panic!`
17. 不要使用 `forget`
18. 推荐使用 clippy 检查 `forget` 的使用
19. 不要泄露内存
20. 要释放包裹在 `ManaullyDrop` 里的值
21. 总是调用 `into_rawed` 值对应的 `from_raw` 函数
22. 不要使用未初始化内存
23. 使用完敏感数据后要将内存清零
24. 推荐校验 `Drop` 实现
25. 不要再 `Drop` 实现内部恐慌（panic）
26. 不允许 `Drop` 的循环引用
27. 推荐不依赖 `Drop` 来保证安全
28. 推荐校验 `Send` 和 `Sync` 实现
29. 要遵循标准库比较特质（trait）的不变之处
30. 推荐使用标准库比较特质的默认实现
31. 推荐尽可能派生（derive）比较特质
32. 要在 FFI 中只使用 C 兼容类型
33. 在 FFI 边界要使用兼容性的类型
34. 推荐使用绑定自动生成工具
35. 在绑定到平台依赖类型时，要使用可移植别名 `c_*`
37. 推荐在 Rust 中检查外部类型
38. 推荐指针类型而不是引用类型[^1]
39. 不要使用未检查的外部引用
40. 检查外部指针
41. 要标记 FFI 中的函数指针类型为 `extern` 和 `unsafe`
42. 检查外部函数指针
43. 建议不在 FFI 边界使用不美容的 Rust `enum` 类型
44. 建议为外部不透明类型使用专门的 Rust 类型
45. 推荐使用不完整的 C/C++ `struct` 指针来使得类型不透明
46. 不要在 FFI 边界使用实现了 `Drop` 的类型
47. 要确保在 FFI 中清除数据所有权
48. 推荐将外部数据包裹在可释放内存的包装类型
49. 要在 FFI 中 正确地处理 `panic!`
50. 推荐为外部库提供安全的包装
51. 推荐只暴露专门的 C 兼容 API
