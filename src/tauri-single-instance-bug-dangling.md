# 由 tauri 单例模式 bug “意外修复” 发现的 dangling

## TL;DR
[tauri 单例插件][plugin] 用于区分单例实例的 `productName`的过长会导致[单例功能失效][disfuction]，博主最初确信 `encode_wide` 实现有问题，并提交了[修复][my-fix]。然而在和社区深入研究问题原因后，发现根本原因是使用 `encode_wide` 转码传参时造成了 [dangling] 。

PS: 为方便读者理解，博主花费一天时间重新梳理分析步骤，按照演绎法展示定位 bug 地过程，实现发生的分析过程要比博文的过程更加曲折，对分析理解问题无意义因此略过。

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

                // omitted
            }

            // omitted
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

                // omitted
            }

            // omitted
        })
        .on_event(|app, event| {
            // omitted
        })
        .build()
}
```

然后使用修复前会引起单例功能失效的 `z123456789012345` 作为测试用例，验证单例功能可用了，证明该修改可以修复单例功能失效的问题。

## 但它并不是真的修复
在博主提交了修复后，插件仓库作者提醒，前文所述的代码使用的 `encode_wide` 是[封装拼接了 `\0`][null-concated] 后再传递参数的:

```rs
pub fn encode_wide(string: impl AsRef<std::ffi::OsStr>) -> Vec<u16> {
    std::os::windows::prelude::OsStrExt::encode_wide(string.as_ref())
        .chain(std::iter::once(0))
        .collect()
}
```

这意味着，并不是 `\0` 导致问题的失效，因为该函数在 windows 环境下执行是能够补足 `\0` 的：

```rs
fn encode_wide(string: impl AsRef<std::ffi::OsStr>) -> Vec<u16> {
    std::os::windows::prelude::OsStrExt::encode_wide(string.as_ref())
        .chain(std::iter::once(0))
        .collect()
}

fn main() {
    let product_name = "z123456789012345";

    // output: [122, 49, 50, 51, 52, 53, 54, 55, 56, 57, 48, 49, 50, 51, 52, 53, 0]
    //                                                                           ^
    //                              null concated here so it's null-terminated --|
    println!("{:?}", encode_wide(product_name));
}
```

## 那么失效的过程发生了什么？
为了分析问题详细过程，我将插件仓库代码切换到了 [问题代码版本][bug-version]：

```sh
git checkout 16e5e9eb59da9ceca3dcf09c81120b37fe108a03
```

然后添加了一些 `dbg` 宏：

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
            dbg!(hmutex);  // windows.rs:43 debug here!

            if unsafe { GetLastError() } == ERROR_ALREADY_EXISTS {
                unsafe {
                    let hwnd = FindWindowW(
                        encode_wide(&class_name).as_ptr(),
                        encode_wide(&window_name).as_ptr(),
                    );
                    dbg!(hwnd);  // windows.rs:51 debug here!

                    // omitted
                }
            } else {
                app.manage(MutexHandle(hmutex));

                let hwnd = create_event_target_window::<R>(&class_name, &window_name);
                dbg!(hwnd);  // windows.rs:76 debug here!

                // omitted
            }

            Ok(())
        })
        .on_event(|app, event| {
            // omitted
        })
        .build()
}
```

然后，将代码仓库  `examples\emit-event\src-tauri\tauri.conf.json` 分别改成 `z12345678901234` 与 `z123456789012345`，然后执行：

```
# process1
> cd examples\emit-event
examples\emit-event> cargo tauri build --debug
examples\emit-event> src-tauri\target\debug\z12345678901234.exe
[tauri-plugin-single-instance\src\platform_impl\windows.rs:43] hmutex = 548
[tauri-plugin-single-instance\src\platform_impl\windows.rs:76] hwnd = 40113446

# process2
> examples\emit-event\src-tauri\target\debug\z12345678901234.exe
[tauri-plugin-single-instance\src\platform_impl\windows.rs:43] hmutex = 548
[tauri-plugin-single-instance\src\platform_impl\windows.rs:51] hwnd = 40113446
```

```
# process1
> cd examples\emit-event
examples\emit-event> cargo tauri build --debug
examples\emit-event> src-tauri\target\debug\z123456789012345.exe
[tauri-plugin-single-instance\src\platform_impl\windows.rs:43] hmutex = 552
[tauri-plugin-single-instance\src\platform_impl\windows.rs:76] hwnd = 0

# process2
> examples\emit-event\src-tauri\target\debug\z123456789012345.exe
[tauri-plugin-single-instance\src\platform_impl\windows.rs:43] hmutex = 548
[tauri-plugin-single-instance\src\platform_impl\windows.rs:51] hwnd = 0

# process3
> examples\emit-event\src-tauri\target\debug\z123456789012345.exe
[tauri-plugin-single-instance\src\platform_impl\windows.rs:43] hmutex = 548
[tauri-plugin-single-instance\src\platform_impl\windows.rs:51] hwnd = 0
```

由上面的结果我们可以知道：在用例 `z12345678901234`，我们在创建实实例时返回了有效的 `hwnd` 值，并且在检查 `hwnd` 时确认已经创建窗口；而在用例 `z123456789012345`，我们创建窗口的函数 `create_event_target_window` 返回的 `hwnd` 是无效的！所以导致问题的代码，应该在 `create_event_target_window` 的逻辑中！再次添加 `dbg` ，重新编译后继续 debug：

```rs
fn create_event_target_window<R: Runtime>(class_name: &str, window_name: &str) -> HWND {
    unsafe {
        let class = WNDCLASSEXW {
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
            style: 0,
            lpfnWndProc: Some(single_instance_window_proc::<R>),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(std::ptr::null()),
            hIcon: 0,
            hCursor: 0,
            hbrBackground: 0,
            lpszMenuName: std::ptr::null(),
            lpszClassName: encode_wide(&class_name).as_ptr(),
            hIconSm: 0,
        };
        dbg!(class.lpszClassName);  // windows.rs:153 debug here
        dbg!(*class.lpszClassName);  // windows.rs:154 debug here

        RegisterClassExW(&class);

        let hwnd = CreateWindowExW(
            WS_EX_NOACTIVATE
            | WS_EX_TRANSPARENT
            | WS_EX_LAYERED
            // WS_EX_TOOLWINDOW prevents this window from ever showing up in the taskbar, which
            // we want to avoid. If you remove this style, this window won't show up in the
            // taskbar *initially*, but it can show up at some later point. This can sometimes
            // happen on its own after several hours have passed, although this has proven
            // difficult to reproduce. Alternatively, it can be manually triggered by killing
            // `explorer.exe` and then starting the process back up.
            // It is unclear why the bug is triggered by waiting for several hours.
            | WS_EX_TOOLWINDOW,
            dbg!(encode_wide(&class_name).as_ptr()),  // windows.rs:170 debug here
            dbg!(encode_wide(&window_name).as_ptr()),  // windows.rs:171 debug here
            WS_OVERLAPPED,
            0,
            0,
            0,
            0,
            0,
            0,
            GetModuleHandleW(std::ptr::null()),
            std::ptr::null(),
        );
        SetWindowLongPtrW(
            hwnd,
            GWL_STYLE,
            // The window technically has to be visible to receive WM_PAINT messages (which are used
            // for delivering events during resizes), but it isn't displayed to the user because of
            // the LAYERED style.
            (WS_VISIBLE | WS_POPUP) as isize,
        );
        hwnd
    }
}
```

`z12345678901234`:
```
examples\emit-event> src-tauri\target\debug\z12345678901234.exe
[tauri-plugin-single-instance\src\platform_impl\windows.rs:43] hmutex = 556
[tauri-plugin-single-instance\src\platform_impl\windows.rs:153] class.lpszClassName = 0x0000021d099eddc0
[tauri-plugin-single-instance\src\platform_impl\windows.rs:154] *class.lpszClassName = 122
[tauri-plugin-single-instance\src\platform_impl\windows.rs:170] encode_wide(&class_name).as_ptr() = 0x0000021d099ee180
[tauri-plugin-single-instance\src\platform_impl\windows.rs:171] encode_wide(&window_name).as_ptr() = 0x0000021d099dfe20
[tauri-plugin-single-instance\src\platform_impl\windows.rs:76] hwnd = 28841288
```

`z123456789012345`:
```
examples\emit-event> src-tauri\target\debug\z123456789012345.exe
[tauri-plugin-single-instance\src\platform_impl\windows.rs:43] hmutex = 548
[tauri-plugin-single-instance\src\platform_impl\windows.rs:153] class.lpszClassName = 0x0000017259ca6be0
[tauri-plugin-single-instance\src\platform_impl\windows.rs:154] *class.lpszClassName = 43920
[tauri-plugin-single-instance\src\platform_impl\windows.rs:170] encode_wide(&class_name).as_ptr() = 0x0000017259cc6b30
[tauri-plugin-single-instance\src\platform_impl\windows.rs:171] encode_wide(&window_name).as_ptr() = 0x0000017259cc6970
[tauri-plugin-single-instance\src\platform_impl\windows.rs:76] hwnd = 0
```

对比我们之前 `encode_wide` 函数[返回的结果](#但它并不是真的修复)，`class_name` 开头的字符应该是 ASCII 字符 `z`（ASCII 码 122），因此通过 `encode_wide(&class_name).as_ptr()` 传参的 `class.lpszClassName`，应当指向值为 "z123456789012345-single-instance-class" 的字符串，这在用例 `z12345678901234` 中行为符合预期；但在 `z123456789012345` 的用例中，`class.lpszClassName` 指向的却发生了变化（`*class.lpszClassName = 43920`），反推可以得知， `encode_wide(&class_name).as_ptr()` 并没有成功地把指针传递给 `class.lpszClassName`。

## 一语惊醒梦中人：悬垂指针（dangling）！

[@Berrysoft] 指出， `encode_wide(&class_name).as_ptr()` 这种写法由于直接对临时变量直接取指针，而临时变量 `encode_wide(&class_name)` 会在执行完之后被马上释放结束生命周期，因此指向该临时变量的指针也会变成悬垂指针！临时变量的这一行为在 [reference] 中有说明：

> When using a value expression in most place expression contexts, a temporary unnamed memory location is created and initialized to that value. The expression evaluates to that location instead, except if promoted to a static. **The drop scope of the temporary is usually the end of the enclosing statement.**

而解决该问题，只需要把提升变量的 lifetime，把要用到的变量提取出来，使其 lifetime 可以覆盖要用到的函数而不至于在语句执行完之后马上被回收。于是有了解决问题的 [PR][bug-fix]。

## 那为什么在 `format` 的时候手动添加 `\0` 后，问题“修复”了呢？
这个问题依然悬而未决。有 TG 群友提出，可能是由于堆栈被破坏 “碰巧” 又指向了正确的字符串位置，而 `format` 后的变量又是 `'static` 的，因此能达到“修复”的效果，然而这依然是基于 bug/undefined behavior 的修复方案，因此仍然不可靠。后续原因排查出来后会更新博客~

## 教训与经验
1. 实际上的排查过程，是分析过一次 `create_event_target_window` 的，然而当时由于需求紧急而找到了临时绕开的实现方案（把 `productName` 砍短），因此搁置了，也没有留下相关的排查记录文档，以致于后续在需求变更而变得必须排查清楚该问题时，走向了排查 `encode_wide` 的错误方向，虽然有了“修复”方案，但该方案仍然不可靠，因此可以视作浪费了实践。 **形成记录首先方便的是以后的自己。**
2. 凡是 `unsafe` 多查几遍。像本文涉及到的悬垂指针问题，在 safe rust 中因为 lifetime 不够长而会阻止编译，而 `unsafe` 块中使用裸指针是不会被编译器检查的，因此相关操作都要相当慎重。
3. 多借助社区的力量。比起一个人钻牛角尖，多与社区讨论才容易跳出原本的死胡同，从而理解意识到原来思路的局限性。


[plugin]: https://github.com/tauri-apps/tauri-plugin-single-instance

[disfuction]: https://github.com/amrbashir/tauri-plugin-single-instance/issues/5

[my-fix]: https://github.com/huangjj27/tauri-plugin-single-instance/commit/6fd3c8c2c518eb5eaa1101eb14a65603ca5e621e

[dangling]: https://github.com/tauri-apps/tauri-plugin-single-instance/pull/6

[bug-version]: https://github.com/tauri-apps/tauri-plugin-single-instance/blob/16e5e9eb59da9ceca3dcf09c81120b37fe108a03/src/platform_impl/windows.rs

[std-encode-wide]: https://doc.rust-lang.org/std/os/windows/ffi/trait.OsStrExt.html#tymethod.encode_wide

[null-concated]: https://github.com/tauri-apps/tauri-plugin-single-instance/blob/16e5e9eb59da9ceca3dcf09c81120b37fe108a03/src/platform_impl/windows.rs#L189

[@Berrysoft]: https://github.com/Berrysoft

[reference]: https://doc.rust-lang.org/stable/reference/expressions.html#temporaries

[bug-fix]: https://github.com/tauri-apps/tauri-plugin-single-instance/pull/6