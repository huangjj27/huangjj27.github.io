# 尝试在单 HTML 文件中嵌入 WASM 模块的错误操作
## TL;DR
1. 当项目是需要 WASM 与 JavaScript 相互交互的时候，请尽可能在统一的 JavaScript 入口中定义所有的功能；
2. wasm-pack 生成的胶水 JavaScript 与 WASM 可以稍作修改即可嵌入到 HTML 文件中。

## 项目背景
当博主兴高采烈地使用 HTML 与 JavaScript 迅速开发好 UI 界面与交互功能的时候，发现核心的功能的 JavaScript 库只支持 npm 环境而无法应用到前述 UI 界面上，博主迫于无奈只能抓起以前做过的 Rust 版本库，尝试改造成 WASM 模块以复用界面代码。因为博主的这个项目是属于不对外开放的项目，因此本文中使用的项目是简化后的 demo，但不影响博主记录以及提醒上述遇到的两个问题（这两个坑每一个都坑掉了我几个小时，但愿会有读者看到我这篇文章抢救一下自己的时间）。

## demo 项目架构
```
demo
  |-- Cargo.toml
  |-- src
        |-- lib.rs
  |-- assets
        |-- demo.html
```

```toml
# Cargo.toml
[package]
name = "demo"
authors = ["huangjj27 <huangjj.27@qq.com>"]
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2.63"
```

```rs
// lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

```html
<!-- demo.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>just a demo</title>
    <script id="indexscript" type="module">
        import init, { add } from "../pkg/demo.js";
        async function run() {
            await init();
        }

        run();
    </script>
    <script>
        function js_add() {
            let a = document.getElementById("a");
            let b = document.getElementById("b");
            let sum = document.getElementById("sum");
            sum.value = add(a.value, b.value);
        }
    </script>
</head>
<body>
    <textarea id="a" autofocus oninput="js_add()">40</textarea>
    <textarea id="b" oninput="js_add()">2</textarea>
    <textarea id="sum" readonly></textarea>
</body>
</html>
```

项目在编译的时候还需要 [wasm-pack]，用来生成胶水 JavaScript `demo/pkg/demo.js` 和 wasm 文件 `demo/pkg/demo_bg.wasm`：
```sh
wasm-pack build --target=web
```

[wasm-pack]: https://rustwasm.github.io/wasm-pack/installer/

## 薛定谔的 JavaScript 函数
当我们打开 `demo.html` 并且尝试修改 `a` 和 `b` 的值时，我们会从控制台遇到了如下报错：
```log
13:33:19.672 Uncaught ReferenceError: add is not defined
    js_add file:///demo/assets/demo.html:23
    oninput file:///demo/assets/demo.html:1
2 demo.html:22:13
```

这里的 `add` 函数就是我们从 WASM 模块中加载的，来自 rust 实现的 `add` 函数。此时，如果我们在 `run` 下属下方，直接调用 `add` 则是可以执行成功的：
```html
    <script id="indexscript" type="module">
        import init, { add } from "../pkg/demo.js";
        async function run() {
            await init();
            console.log(`add函数已加载，add(1, 2) = ${add(1, 2)}`);
        }

        run();
    </script>
```

执行结果:
```log
14:02:58.360 add函数已加载，add(1, 2) = 3 demo.html:13:21
14:03:11.895 Uncaught ReferenceError: add is not defined
    js_add file:///demo/assets/demo.html:23
    oninput file:///demo/assets/demo.html:1
2 demo.html:23:13
```

出现以上现象的原因是，`type="module"` 限制了 `indexscript` 内部项目的作用域只能在该 `<script>` 代码块中有效。解决方法有两种：
1. 将需要导出的给其他 `<script>` 块使用的功能，挂载在页面全局的 `window` 对象上，模拟 `export` 的效果，缺点是很可能无意中覆盖了挂载对象。
2. 将所有js 功能都集中在 `indexscript` 代码块中，将所有的功能统一管理。此时，要注意将DOM 元素的事件通过监听事件的方式来管理：
```html
<!-- demo.html -->
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>just a demo</title>
    <script id="indexscript" type="module">
        import init, { add } from "../pkg/demo.js";
        function js_add() {
            let a = document.getElementById("a");
            let b = document.getElementById("b");
            let sum = document.getElementById("sum");
            sum.value = add(a.value, b.value);
        }

        async function run() {
            await init();

            document.getElementById("a").addEventListener("input", js_add);
            document.getElementById("b").addEventListener("input", js_add);
        }

        run();
    </script>
</head>
<body>
    <textarea id="a" autofocus>40</textarea>
    <textarea id="b">2</textarea>
    <textarea id="sum" readonly></textarea>
</body>
</html>
```

## 然而我还是想包容你的，我的 WASM
我们来分析一下 `wasm-pack` 生成的 `demo.js`：
```js
// demo.js

let wasm;

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
export function add(a, b) {
    const ret = wasm.add(a, b);
    return ret;
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('demo_bg.wasm', import.meta.url);
    }
    const imports = {};


    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;
```

我们看到，`demo.js:175` 对来待加载的 `module` 做了判断：如果不是从响应获取的数据，则直接视作 wasm bytes 来进行加载。于是我们可以对生成的 `demo_bg.wasm` 文件通过 base64 转码，嵌入到 HTML 文件中，然后转换成 `Uint8Array` 传递给 `demo.js:init` 函数进行加载:

```html
    <script id="indexscript" type="module">
// demo.js 完全嵌入到 html 文件中，并且不需要 export 语句
let wasm;

/**
* @param {number} a
* @param {number} b
* @returns {number}
*/
function add(a, b) {
    const ret = wasm.add(a, b);
    return ret;
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('demo_bg.wasm', import.meta.url);
    }
    const imports = {};


    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

// demo.js 嵌入结束

        // wasm 二进制，base64编码：
        const WASM_B64 = "AGFzbQEAAAABBwFgAn9/AX8DAgEABQMBABEHEAIGbWVtb3J5AgADYWRkAAAKCQEHACAAIAFqCwB7CXByb2R1Y2VycwIIbGFuZ3VhZ2UBBFJ1c3QADHByb2Nlc3NlZC1ieQMFcnVzdGMdMS42MS4wIChmZTViMTNkNjggMjAyMi0wNS0xOCkGd2FscnVzBjAuMTkuMAx3YXNtLWJpbmRnZW4SMC4yLjgwICg0Y2FhOTgxNjUp";

        function b64toBytes(b64) {
            let binary = atob(b64);
            let bytes = new Uint8Array(binary.length);
            for (let i = 0; i < bytes.length; i++) {
                bytes[i] = binary.charCodeAt(i);
            }
            return bytes;
        }

        function js_add() {
            let a = document.getElementById("a");
            let b = document.getElementById("b");
            let sum = document.getElementById("sum");
            sum.value = add(a.value, b.value);
        }

        async function run() {
            await init(b64toBytes(WASM_B64));  // 直接通过页面加载编码

            document.getElementById("a").addEventListener("input", js_add);
            document.getElementById("b").addEventListener("input", js_add);
        }

        run();
    </script>
```

完成以上步骤，我们就得到了一个带有 WASM 功能的 standalone html 文件啦~。
