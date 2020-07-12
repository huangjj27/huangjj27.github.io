# 构建器（Builder）模式

## 示例
通常在 Rust 中的实现是通过 不断重建 `Builder` 来构造最后的类型：

```rs,no_run
struct Counter {
    counted1: usize,
    counted2: usize,
    done: bool,
}

struct CounterBuilder {
    counted1: usize,
    counted2: usize,
}

impl CounterBuilder {
    // 构建器需要有默认的参数配置，然后从默认配置触发进行构建。
    // 不适用 #[derive(std::default::Default)]，因为默认配置可能不一样
    fn default() -> Self {
        CounterBuiler {
            counted1: 5,
            counted2: 0,
        }
    }

    // 属性定制方法。消耗原本的构建器，修改属性后重新生成新构建器
    fn set_counted1(self, cnt: usize) -> Self {
        self.counted1 = cnt;
        self
    }

    fn set_counted2(self, cnt: usize) -> Self {
        self.counted2 = cnt;
        self
    }

    // 最后通过 `build` 方法生成所需类型
    fn build(self) -> Counter {
        Counter {
            counted1: self.counted1,
            counted2: self.counted2,
            done: false,
        }
    }
}
```

## 个人实践
在设置属性方法的时候，通常的实现是通过消耗原本的构造器后生成新构造器，这使得如果配置构造器的过程不能连续调用属性设置方法时，必须重新捕获构造器：

```rs,no_run
let mut builder = CounterBuilder::default();

// ... 进行一些计算，获得需要配置的值
let cnt1 = operations();

builder = builder.set_counted1(cnt);

// ... 进行一些计算，获得需要配置的值
let cnt2 = operations();

builder = builder.set_counted(cnt2);
```

以上代码通常出现在需要流计算并及时记录参数配置的时候。并且，如果构造器被更大型的数据结构持有时，消耗并重新构建构造器可能会对性能有点影响。因此在博主个人实现时通常采取传递`&mut self` 引用的方法来实现属性设置方法：

```rs, no_run
    // ...
    // 属性定制方法。消耗原本的构建器，修改属性后重新生成新构建器
    fn set_counted1(&mut self, cnt: usize) -> &mut Self {
        self.counted1 = cnt;
        self
    }

    fn set_counted2(&mut self, cnt: usize) -> &mut Self {
        self.counted2 = cnt;
        self
    }

// ...
```
改成如上形式的函数签名，即可 [灵活构造] 目标结构:

```rs,no_run
let mut builder = CounterBuilder::default();

// ... 进行一些计算，获得需要配置的值
let cnt1 = operations();

builder.set_counted1(cnt);

// ... 进行一些计算，获得需要配置的值
let cnt2 = operations();

builder.set_counted(cnt2);

// ... 可能还要等待别的操作完成后再进行构建

let counter = builder.build();
```

[灵活构造]: #灵活构造

## 为什么使用构造器模式
- **构造过程可控**。通常实现构造器模式的时候，我们会将构造器所需要配置的属性设置为私有[^1]，并且只能通过我们提供的属性设置方法进行设置，使得构造过程可控。另外，可以通过属性设置方法提前恐慌（panic）来阻止生成无效对象。
- **设置方法职责专一**。属性设置方法 [职责专一]，只会负责设置一种属性，只有在该属性的设置规则改变时，相应的属性设置方法才需要进行修改；
- **构造灵活**。多个属性设置方法可以自由的组合在一起，也可以分步组合构造。
- **可批量构造**。我们除了使用消耗性的 `build(self)` 方法，也可以使用非消耗性的 `fn build(&self)` 方法，使得构造器可以多次复用。
- **符合开闭原则**。当某一属性的设置方法内部实现发生变化的时候，不影响其他属性的设置方式；而新增属性及其设置方法时，可以通过链式调用很方便地增加新属性的设置。

[职责专一]: https://baike.baidu.com/item/单一职责原则

## 为什么不使用构造器模式
构造器模式由于有以下缺点而在部分场景中不适用：

- **在构造完成前无法使用被构造对象**。在构造完成之前，构造器并不生成被构造对象，因此在整个构造设置完成之前，无法使用被构造对象。
- **构造器与被构造对象使用相同的属性设置方法，造成代码重复并无法复用**。考虑需要只通过属性设置方法来修改对象的场景，当被构造对象在使用过程中需要频繁设置属性，那么就需要编写对应的属性设置方法；而如果还使用构造器进行对象构造，那么属性设置方法就会重复，并且可能造成构造器与被构造对象的属性设置行为不一致的问题[^2]。


[^1]: Rust 语言中默认语言项(Item)的可见性都是私有的，如需公开语言项给其他模块使用，需要使用 `pub` 关键字放开。
[^2]: 一个绕开的行为不一致问题的方法是将属性设置规则抽取为静态函数，但仍然无法避免过度封装的问题。
