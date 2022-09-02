# 由 tauri 单例模式 bug “意外修复” 发现的 dangling

## TL;DR
[tauri 单例插件][plugin] 用于区分单例实例的 `productName`的过长会导致[单例功能失效][disfuction]，博主最初确信 `encode_wide` 实现有问题，并提交了[修复][my-fix]。然而在和社区深入研究问题原因后，发现根本原因是使用 `encode_wide` 转码传参时造成了 [dangling] 。

## 所以这个 bug 的现象是怎么样的？
正如博主所说，在 [有问题版本的插件代码][bug-version] 中，博主最初发现，`tauri.conf.json` 中的 `package.productName` 在分别使用五个汉字与六个汉字时，单例模式功能表现出不一致的行为：五个汉字的 `productName` 单例功能运行正常，而六个汉字的 `productName` 单例功能失效，于是针对该问题初步进行了测试：

|测试的 `productName`|单例插件功能是否生效|
|:--                |:-:                |
|六个汉字试试        |x                  |
|随便五个字          |√                  |
|又来了六个字        |x                  |

因为这些汉字测试用例使用 `UTF-8` 编码，又因为是常用字，因此每个汉字对应 3 bytes，因此假设 `productName` 在超过 15 bytes、不超过 18 bytes 时会导致功能失效，进一步补充测试用例：

|测试的 `productName`|单例插件功能是否生效|
|:--                |:-:                |
|z12345678901234    |√                  |
|z123456789012345   |x                  |

看来博主运气不错，刚好踩到了边界的测试用例。那么基本可以确定，`productName` 超过 15 bytes 就会导致单例功能失效。

## PotatoTooLarge: 传递给 Win32 API 的字符串要用 C string 风格的 `\0` 结束
在最初的讨论过程中，因为我们没有仔细留意插件仓库使用的 `encode_wide` 是自行做过封装的，因此我们一开始根据 [以下代码][bug-version] 进行分析：
```rs
pub fn init<R: Runtime>(f: Box<SingleInstanceCallback<R>>) -> TauriPlugin<R> {
    plugin::Builder::new("single-instance")
        .setup(|app| {
            let app_name = &app.package_info().name;
            let class_name = format!("{}-single-instance-class", app_name);
            let window_name = format!("{}-single-instance-window", app_name);

            let hmutex = unsafe {
                CreateMutexW(
                    std::ptr::null(),
                    true.into(),
                    encode_wide("tauri-plugin-single-instance-mutex").as_ptr(),
                )
            };

            if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
                unsafe {
                    let hwnd = FindWindowW(
                        encode_wide(&class_name).as_ptr(),
                        encode_wide(&window_name).as_ptr(),
                    );
            // omitted
                }
        })
        .on_event(|app, event| {
            // omitted
        })
        .build()
}
```

有 Windows 编程经验的 @PotatoTooLarge 指出，`encode_wide`（来自 [std 的 Window扩展][std-encode-wide]）并不会补充 `\0` 结束符：

> Re-encodes an OsStr as a wide character sequence, i.e., potentially ill-formed UTF-16.
>
> This is lossless: calling OsStringExt::from_wide and then encode_wide on the result will yield the original code units. **Note that the encoding does not add a final null terminator.**

于是博主听取建议，把所有会传递到 `encode_wide` 函数的字符串都添加了 `\0`，形成了 [一版修复][my-fix]:

```rs
pub fn init<R: Runtime>(f: Box<SingleInstanceCallback<R>>) -> TauriPlugin<R> {
    plugin::Builder::new("single-instance")
        .setup(|app| {
            let app_name = &app.package_info().name;
            // let class_name = format!("{}-single-instance-class", app_name);
            let class_name = format!("{}-single-instance-class\0", app_name);
            // let window_name = format!("{}-single-instance-window", app_name);
            let window_name = format!("{}-single-instance-window\0", app_name);

            let hmutex = unsafe {
                CreateMutexW(
                    std::ptr::null(),
                    true.into(),
                    // encode_wide("tauri-plugin-single-instance-mutex").as_ptr(),
                    encode_wide("tauri-plugin-single-instance-mutex\0").as_ptr(),
                )
            };

            if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
                unsafe {
                    let hwnd = FindWindowW(
                        encode_wide(&class_name).as_ptr(),
                        encode_wide(&window_name).as_ptr(),
                    );
            // omitted
                }
        })
        .on_event(|app, event| {
            // omitted
        })
        .build()
}
```

然后使用修复前会引起单例功能失效的 `z123456789012345` 作为测试用例，验证单例功能可用了，证明该修改可以修复单例功能失效的问题。

## 但它并不是真的修复

## 那么失效的过程发生了什么？
## 教训与经验


[plugin]: https://github.com/tauri-apps/tauri-plugin-single-instance

[disfuction]: https://github.com/amrbashir/tauri-plugin-single-instance/issues/5

[my-fix]: https://github.com/huangjj27/tauri-plugin-single-instance/commit/6fd3c8c2c518eb5eaa1101eb14a65603ca5e621e

[dangling]: https://github.com/tauri-apps/tauri-plugin-single-instance/pull/6

[bug-version]: https://github.com/tauri-apps/tauri-plugin-single-instance/blob/16e5e9eb59da9ceca3dcf09c81120b37fe108a03/src/platform_impl/windows.rs

[std-encode-wide]: https://doc.rust-lang.org/std/os/windows/ffi/trait.OsStrExt.html#tymethod.encode_wide