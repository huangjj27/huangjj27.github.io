# sprint1-Doro

> 7.有活动清单
> 9.专注于某个活动，或者置顶最多一个活动

## 需求细化
本次 sprint 将实现初版的活动清单，以及从活动清单中置顶最多一项活动。活动（Doro）是番茄工作法的核心对象，活动清单应当可以新增、编辑、删除（可回收）活动，也应该允许查询全部的活动。而作为番茄工作法应用，应当也具有专注/置顶于活动的功能。

## 核心对象分析
### 活动（Doro）
每一项活动的开展，都必然有其开始时间与结束时间。结束时间对应的是该项活动完成的时间，而且番茄工作法中，活动应当置顶后才进行开始，因此要记录最后置顶时间。此外，一些活动会有预期的截止时间，这与完成时间有区别。每一项活动创建时都必须有简短的描述来确定活动的主题，每一项活动都能够被标记为完成与未完成状态。

```mermaid
classDiagram

    class Doro {
        description: String
        due_at: Option~Datetime~
        last_pinned_at: Option~Datetime~
        done_at: Option~Datetime~
        with_description(desc: &str) Doro$
        with_desc(&mut self, desc: &str) &mut Self
        with_due(&mut self, due: Datetime) &mut Self
    }
```

### 活动清单(Doros)
活动清单包含新增、编辑、删除（可回收）、查询活动的功能，也即活动清单的目的是管理活动。活动清单全局唯一。
```mermaid
classDiagram

    class Doros {
        inner: Arc~Mutex~Vec~Doro~~~
        add(&mut self, doro: Doro)
        edit(&mut self, idx: usize) &mut Doro
        remove(&mut self, idx: usize) Doro
        all(&self) &[Doro]
    }
```

### 置顶（DoroPin）
置顶是番茄工作才会存在的概念，其目的是从活动清单中挑选一项活动保持专注，直到活动完成。置顶项也是全局唯一的。

```mermaid
classDiagram

    class DoroPin {
        innner: Arc~Mutex~Option~Doro~~~
        pin(&mut self, doro: Doro)
        unpin() Option~Doro~
    }
```

## 对象关系分析
活动清单由单个的活动聚合而成，而置顶依赖被选定的活动而工作：

```mermaid
classDiagram
    direction LR
    Doro "*" --o Doros
    Doro "0..1" <.. DoroPin

    class Doro {
        description: String
        due_at: Option~Datetime~
        last_pinned_at: Option~Datetime~
        done_at: Option~Datetime~
        with_description(desc: &str) Doro$
        with_desc(&mut self, desc: &str) &mut Self
        with_due(&mut self, due: Datetime) &mut Self
    }

    class Doros {
        inner: Arc~Mutex~Vec~Doro~~~
        add(&mut self, doro: Doro)
        edit(&mut self, idx: usize) &mut Doro
        remove(&mut self, idx: usize) Doro
        all(&self) &[Doro]
    }

    class DoroPin {
        innner: Arc~Mutex~Option~Doro~~~
        pin(&mut self, doro: Doro)
        unpin() Option~Doro~
    }
```

## 时序分析
1. 活动清单与置顶容器是最
