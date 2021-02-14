# 总结一次面试
## 最值得总结的三个问题
### 线程同步有哪些方法？如何用这些方法实现一个 RwLock？
线程同步的目的在于解决多个线程访问关键资源时的竞争状态。一个数据竞争的简单例子如下：
```rust
use std::thread;
fn main() {
  let mut s = String::from("Hello");

  let thread1 = thread::spawn(|| {
    println!("{}", s);
  });

  let thread2 = thread::spawn(|| {
    s.push_str("World!");
    println!("{}", s);
  });

  thread1.join();
  thread2.join();
}
```
上文的代码中 `thread1` 试图打印 `s`, 预期得到输出 `Hello`, 但是 `thread2` 却改变了 `s` 的内容，
那么 `thread1` 最终打印内容将取决于两个线程哪个先完成: 如果 `thread1` 先完成了，那么
将打印 `Hello`; 如果 `thread2` 先完成了，那么将打印 `HelloWorld!`。

实际上得益于 Rust 的所有权系统与生命周期（lifetime）检查，上述示例并不能编译——子线程可能
会在主程序结束后继续运行，导致子线程捕获的 `s` 的引用失效；另外 `thread2` 直接修改了 `s`，
换言之只会允许 `thread2` 独占地持有 `s` 的可变引用（`&mut s`)，而不允许其他线程持有 `s`
的任何引用。

在 Rust 编程中，主要有以下线程同步的方法：

- 互斥锁(Mutex)
  我们可以使用互斥锁 `Mutex<T>` 来控制只能有单独一个线程读取/修改
  对象。通常实践是在外面加上原子引用计数 `Arc` 变成 `Arc<Mutex<T>>`，来减少 `Mutex`
  拷贝的开销。对于多读少写的场景，可以用 `RwLock` 提高并发。
- 条件变量(CondVar)
  条件变量用于“阻塞”线程并使得线程在等待事件时不需要消耗 CPU 时间。通常会与放进互斥锁
  布尔型的预言值（状态值）关联使用，在状态值发生变化时通知条件变量。
- 屏障(Barrier)
  屏障用于在某个需要若干线程 **都完成** 前置操作后再开始计算的操作之前，让所有所需线程
  的状态都达到能开始进行计算的状态。

### 有什么问题是命周期标注无法修正的？请给出一个例子
这道问题最后我也并没有理解，“生命周期标注无法修正的问题”，字面意思是，即使我们按照我们
期望的程序语义来修正了生命周期标注，这个程序仍然不能通过编译，或者再运行时仍然不能得到
期望结果。按此描述，一个可能的例子是，我们尝试从一个较短的引用返回一个较长的引用：

```rust
fn longhten<'a>(s_ref: &'a str) -> &'static str {
    s_ref
}

fn main() {
    let s = String::from("hello");

    let static_ref = longhten(&s);

    println!("{}", static_ref);
}

```

### Waker 如何被唤醒？ Reactor要怎么实现？
Reactor 作为反应器，上面同时挂载了成千上万个待唤醒的事件，这里使用了mio统一封装了操作系统的多路复用API。
在Linux中使用的是Epoll[^3]，在Mac中使用的则是Kqueue[^2]

```ignore
loop {
    // 轮询事件是否超时
    poll.poll(&events, timeout);
    for event in events.iter() {
        if (event.is_readable()) {
            for waker in event.readers.wakers {
                waker.wake();
            }
        }
        if (event.is_writeable()) {
            for waker in event.writers.wakers {
                waker.wake();
            }
        }
    }
}
```

## 一面 -- 手写代码
### 1. 实现一个二分查找函数
{{#playpen ../code/interview/binary-search.rs}}

- Q: 请分析该函数的算法复杂度？
  - A: 时间复杂度 \\( O(\log n) \\)a，最坏情况下的事件复杂度是 \\( O(n) \\)

- Q: 请优化这个算法？
  - A：一个优化方法是 **插值查找法**，利用如下公式自动根据查找到的元素与目标的距离来修正下一次查找
的区间范围，提高查找速度：

\\[ mid = left + { key - arr[left] \over arr[right] - key] } (right - left) \\]

### 2. 镜像二叉树
请反转二叉树。如给出以下二叉树：
```markdown
     1
   /   \
  2     3
 / \   / \
4   5 6   7
```
反转为：
```markdown
     1
   /   \
  3     2
 / \   / \
7   6 5   4
```

递归解法：
{{#playpen ../code/interview/mirroring-tree.rs}}

- Q: 请优化这个算法？
  - A：如果不用递归（因为递归会加深调用栈），可以使用 **广度优先搜索算法** 来自根向叶
        逐层反转左右子节点的指针，并将子节点的指针放入到队列中待进行处理。

---

[^1]: [线程同步 -- 百度百科](https://baike.baidu.com/item/%E7%BA%BF%E7%A8%8B%E5%90%8C%E6%AD%A5)

[^2]: [Rust异步浅谈 -- leaxoy](https://rustcc.cn/article?id=e6d50145-4bc2-4f1e-84da-c39c8217640b)

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
  id: '/posts/rust-interview-1/',      // Ensure uniqueness and length less than 50
  distractionFreeMode: false  // Facebook-like distraction free mode
})

gitalk.render('gitalk-container')
</script>
