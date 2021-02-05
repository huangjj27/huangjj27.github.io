# 总结一次面试
## 最值得总结的三个问题
### 线程同步有哪些方法？如何用这些方法实现一个 RwLock？

参考自：
- [线程同步 -- 百度百科](https://baike.baidu.com/item/%E7%BA%BF%E7%A8%8B%E5%90%8C%E6%AD%A5)
- [<MFC笔记> 四种线程同步（或互斥）方式小结 -- CSDN](https://blog.csdn.net/ebowtang/article/details/29905309)

### 有什么场景是 添加了生命周期标注 还是会出问题的？
'a: 'static ('staic < 'a )

### Waker 如何被唤醒？ Reactor要怎么实现？
  Reactor作为反应器，上面同时挂载了成千上万个待唤醒的事件， 这里使用了mio统一封装了操作系统的多路复用API。在Linux中使用的是Epoll，在Mac中使用的则是Kqueue[^1]

```
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

[^1]: https://rustcc.cn/article?id=e6d50145-4bc2-4f1e-84da-c39c8217640b
### poll epoll

## 一面 -- 手写代码
### 1. 实现一个二分查找函数
```rust
{{#include ../code/interview/binary-search.rs}}
```

- Q: 请分析该函数的算法复杂度？
  - A: 时间复杂度 \\( O(\log n) \\)a，最坏情况下的事件复杂度是 \\( O(n) \\)

- Q: 请优化这个算法？
  - A：一个优化方法是 **插值查找法**，利用如下公式自动根据查找到的元素与目标的距离来修正下一次查找
的区间范围，提高查找速度：

\\[ mid = left + { key - arr[left] \over arr[right] - key] } (right - left) \\]

### 2. 镜像二叉树
请反转二叉树。如给出以下二叉树：
```
     1
   /   \
  2     3
 / \   / \
4   5 6   7
```
反转为：
```
     1
   /   \
  3     2
 / \   / \
7   6 5   4
```

递归解法：
```rust
{{#include ../code/interview/mirroring-tree.rs}}
```

- Q: 请优化这个算法？
  - A：如果不用递归（因为递归会加深调用栈），可以使用 **广度优先搜索算法** 来自根向叶
        逐层反转左右子节点的指针，并将子节点的指针放入到队列中待进行处理。
