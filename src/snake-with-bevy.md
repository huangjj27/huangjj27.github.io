# 用 `Bevy` 游戏引擎编写贪吃蛇（译）
> 原文：<https://mbuffett.com/posts/bevy-snake-tutorial/#0.3>

[Bevy](https://bevyengine.org/) 最近普及开来了，但是相关学习资料还是很少。这篇文章尝试提供 Bevy 官方书（The Bevy book）的下一步学习。最后产品看起来像这样：

<video autoplay="" loop="" muted="" playsinline="" class="bevy_img">
    <source src="/bevy_snake/new_gifs/game_over.mp4" type="video/mp4">
</video>

这大约是 300 行 Rust 代码；也需要花点时间深入。如果你想快进到成品代码，请点 [这里](https://github.com/marcusbuffett/bevy_snake/tree/tutorial)。每一个小姐开头都有一份代码差异，这应该会在你不是很清晰哪里需要插入代码的时候更加清晰一点。

## 新的空的 Bevy 应用
> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/174c226)

我们现在像 Bevy 官方书那样开始，整一个啥都不干的应用。运行 `cargo new bevy-snake`, 然后把以下代码放到你的 `main.rs` ：

```rs
use bevy::prelude::*;

fn main() {
    App::build().run();
}
```

我们还需要在 `Cargo.toml` 将 Bevy 作为依赖添加，因为我（原文作者，下同）知道这个教程之后要干嘛，我们现在也提前添加 `rand`库吧。

```toml
// ...

[dependencies]
bevy = "0.3.0"
rand = "0.7.3"
```

## 创建窗口

> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/3f7c30b)

我们现在要创建一个2D游戏，需要很多不同的系统；用来创建窗口的，用来做渲染循环的，用来处理输出的，用来处理精灵（sprites)的，等等。幸运的是，Bevy的默认插件给了我们以上所有选项：

```rust
fn main() {
    App::build().add_plugins(DefaultPlugins).run();
}
```

然而 Bevy 的默认插件不包括摄像机（camera），所以我们来插入一个 2D 摄像机，只要我们创建我们第一个系统就可以设置了：

```rust
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
```

[`Cammands`](https://docs.rs/bevy_ecs/0.2.1/bevy_ecs/struct.Commands.html) 通常用来排列命令，来更改游戏世界与资源。在这里，我们创建一个带有 2D 摄像机组件的实体。为Bevy的魔法做点准备吧：

```rust
App::build()
    .add_startup_system(setup.system()) // <--
    .add_plugins(DefaultPlugins)
    .run();
```

我们需要做的只是在我们的函数是调用 `.system()`，然后 Bevy 会神奇地在启动地时候调用 `commands` 参数。再运行一次 app， 你应该能看到一个像这样的空窗口：

![](https://mbuffett.com/bevy_snake/new_pics/empty_window.png)


## 开始编写一条蛇

> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/baaefcc)

我们来写个蛇头放在窗口上吧。我们先定义几个结构体：

```rust
struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}
```

`SnakeHead` 仅仅是一个空结构体，我们会把它当作一个组件来使用，它就是像某种标签，我们会放到一个实体上，之后我们能通过查询带有 `SnakeHead` 组件的实体来找到这个实体。像这样的空结构体在 Bevy 中是一种常见的模式，组件经常不需要他们自己的任何状态。 `Materials` 以后会变成一种资源，用来存储我们给蛇头使用的材质，也会用来存储蛇身和食物的材质。

`head_material` 句柄应该在游戏设置的时候就应该创建好，所以我们接下来要做的是，修改我们的 `setup` 函数：

```rust
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}
```

> **注意：** Bevy要求在注册系统时按照特定的顺序。命令（Commands） -> 资源（Resources） -> 组件（Components）/查询（Queries）。如果你在弄乱一个系统之后获得一个神秘的编译时错误，请检查你的顺序。

`materials.add` 会返回 `Handle<ColorMaterial>`。我们创建了使用这个新建 handle 的 `Materials` 结构体。之后，我们尝试访问类型为 `Materials` 的资源， Bevy会找到我们这个结构体。现在我们来在新的系统里创建我们的蛇头实体，然后你会看到我们如何使用前述资源的：

```rust
fn game_setup(mut commands: Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteComponents {
            material: materials.head_material.clone(),
            sprite: Sprite::new(Vec2::new(10.0, 10.0)),
            ..Default::default()
        })
        .with(SnakeHead);
}
```

现在我们有了新的系统，它会寻找类型为 `Materials` 的资源。它也会创建（spawn）一个新实体，带有 `SpriteComponents` 和 `SnakeHead` 组件。为了创建 `SpriteComponents`, 我们将我们之间创建的颜色的 handle 传入，并且给精灵 10x10 的大小。我们将这个系统添加到我们 app 的构建器：

```rust
.add_startup_system(setup.system())
.add_startup_stage("game_setup") // <--
.add_startup_system_to_stage("game_setup", game_setup.system()) // <--
```

我们需要一个新的场景而不是再一次调用 `add_startup_system` 的原因是，我们需要使用在 `setup` 函数中插入的资源。这次运行后，你应该在屏幕中央看到蛇头：

![](https://mbuffett.com/bevy_snake/new_pics/snake_pixel.png)

好了，可能我们叫它“蛇头”有点过了，你可以看到一个 10x10 的白色精灵。



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
  id: '/posts/snake-with-bevy/',      // Ensure uniqueness and length less than 50
  distractionFreeMode: false  // Facebook-like distraction free mode
})

gitalk.render('gitalk-container')
</script>
