<!-- 引入font-awesome -->
<link href="https://cdn.bootcdn.net/ajax/libs/font-awesome/6.4.2/css/all.css" rel="stylesheet">

# <i class="fa fa-male" aria-hidden="true"></i> Sunny Huang
<i class="fa fa-phone" aria-hidden="true"></i> 77u/KCs4NikxODgtMTk0OC0xMjYyDQo=

<i class="fa fa-fingerprint" aria-hidden="true"></i> 4237 514E 3573 DB47 EE84  5A2B CC7D A6E6 9494 7EE8

<i class="fa fa-envelope" aria-hidden="true"></i> <huangjj.27@qq.com> ·
<i class="fa fa-github" aria-hidden="true"></i> [huangjj27](https://github.com/huangjj27) ·
<i class="fa fa-gitlab" aria-hidden="true"></i> [huangjj27](https://jihulab.com/huangjj27)

<i class="fa fa-book" aria-hidden="true"></i> tech-blog: <https://huangjj27.gitlab.io> ·
<i class="fa fa-book" aria-hidden="true"></i> 微信技术公众号: 不如学点Rust

## <i class="fa fa-graduation-cap" aria-hidden="true"></i> 教育背景
2013.9 -- 2017.7 **中山大学** 数据科学与计算机学院(原软件学院) **软件工程** **_工学学士_**

## <i class="fa fa-users" aria-hidden="true"></i> 项目经历
### 数字化营业厅 2021.09 - 至今
**_测试工程师_: 业务测试、自动化测试、性能测试**
- 负责数字化营业厅项目下的叫号系统与数据赋能看板项目的功能测试
- 针对热点性能的接口与流程进行性能测试
- 辅助运营人员排查与定位营业员反馈的生产问题

Bonus:
- 提供数字化营业厅项目的密码验证方案，保障营业厅用户密码的隐私性与安全性
- 基于 Rust 与 WebAssembly 开发测试辅助工具
- 编写接口自动化测试用例, 使用 [goose](https://book.goose.rs/)、[locust](https://locust.io/)框架编写性能测试用例
- 制定性能测试需求评估、性能测试报告规范
- review 项目代码

### 银行业客户经理智能推荐与客户反馈收集项目 2020.11 - 2021.09
**_大数据开发工程师: 大数据 EDI 开发_**
- 负责子项目的架构优化、详细设计及部分代码实现
- 使得原本执行需要 40 小时的作业降至平均完成时间 6 小时
- 通过 GitLab 管理项目源代码，进行问题追踪、代码评审、自动化流水构建、自动化测试执行
- 负责子项目程序优化，进行 excel 配置自动化转化为数据库工具的开发
- 负责子项目部分功能的测试，利用了 Python 工具自动化执行测试用例
- 负责子项目中关键词词频分析相关部分程序的维护

### HiveQL 静态代码扫描检查工具 2019.4
**_大数据开发工程师: 大数据 EDI 开发_**
- 自发地将银行客户用于 Hive QL 静态扫描规则的工具 CLI 化改造(基于 Python)
- 与上下游沟通，将该工具部署至 CI 平台
- 该工具成功阻止多次高风险代码提交
- 对相关员工讲演培训

### 客户个人金融业务管理平台 2018.8 -- 2019.4
**_大数据开发工程师: 大数据 EDI 开发_**
- 基于银行客户内 EDI 框架（基于 Hadoop 与 Hive）进项业务项目开发
- 基于 Hive 特性与调度流程优化提高已有项目代码效率
- 组织相关开发经验分享

（下列项目均为业余项目/开源贡献项目）
### [《Rust 中的异步编程》]
- [_Asynchronous Programming in Rust_] 一书翻译
- 该书详细地介绍了在 Rust 中异步编程的基础设施 `Future` trait、`Waker`类型，为了使 `Future`
  正常工作的 `Pin<T>` 智能指针与 `Unpin` trait，以及方便开发而引用的 `async/await` 语法糖
- 该书亦给出了示例构建一个简单的执行器，以及实现一个简单的利用异步优化性能的简单 HTTP 服务器

[《Rust 中的异步编程》]: https://huangjj27.github.io/async-book/index.html
[_Asynchronous Programming in Rust_]: https://rust-lang.github.io/async-book/index.html

### [env_logger]
- 在 std 环境下使用比较广泛的 logger
- 为该库 [实现了基础的 wasm32-unknown-unknown 目标的支持], 让该库支持浏览器环境
- 因为内部结构实现的原因（formatter 格式化后记录丢失了 `log::Level` 信息，writter 直接
使用前述记录写入日志），暂时未实现在浏览器环境中的 log 分级。

[env_logger]: https://github.com/env-logger-rs/env_logger
[实现了基础的 wasm32-unknown-unknown 目标的支持]: https://github.com/env-logger-rs/env_logger/pull/148

### [TLSSigAPI] - 使用 Rust 重写 Tencent Login Service Signature API
- 参考了 [Python] 程序实现
- 补足了单元测试用例、集成测试用例

[Python]: https://github.com/tencentyun/tls-sig-api-python
[TLSSigAPI]: https://github.com/huangjj27/TLSSigAPI

## <i class="fa fa-cogs" aria-hidden="true"></i> 技能
- 数据分析开发
    - 基于 HiveQL 的银行业务数据分析设计与开发
    - 基于 polars/panda 的数据分析
- 后端开发/web 开发
    - 熟悉 Rust-lang，熟悉生命周期约束、所有权系统并对其进行分析
    - 熟悉面向对象编程的概念以及 [SOLID原则]
    - 熟悉基本的算法与数据结构
    - 了解 [RESTful API设计]
- 版本管理
    - 熟练使用 git/github/gitlab进行代码版本管理
    - 具有良好的版本管理意识, 熟悉 [语义化版本] 规则
- 软件测试
    - 有功能测试、单元测试、集成测试、接口测试经验
    - 熟悉 pytest、locust，可基于以上工具编写并部署自动化测试用例
    - 也熟悉使用 Rust 测试套件
- 外语
    - 英语（CET6），可流畅阅读英文技术文档

[RESTful API设计]: http://www.ruanyifeng.com/blog/2014/05/restful_api.html
[SOLID原则]: https://en.wikipedia.org/wiki/SOLID_(object-oriented_design)
[语义化版本]: http://semver.org/lang/zh-CN/

<!--
## <i class="fa fa-heart" aria-hidden="true"></i> 期望
- 工作地点: 广州
- 期望岗位:
    - 测试工程师
        - 负责项目产品的自动化测试
        - 参与开发、维护公司的自动化测试框架
    - rust后端工程师
        - 参与项目软件架构, 源码管理流程方案的决策
        - 抽象, 建模项目需求, 分析关键对象的行为与状态变化
        - 根据架构实现系统代码
        - 对实现的代码编写单元测试, 集成测试, 性能测试用例代码
        - 往架构师/设计师方向发展
-->
