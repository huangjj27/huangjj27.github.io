# 用 `Bevy` 游戏引擎编写贪吃蛇（译）
> 原文：<https://mbuffett.com/posts/bevy-snake-tutorial/#0.3>

[Bevy](https://bevyengine.org/) 最近普及开来了，但是相关学习资料还是很少。这篇文章尝试提供 Bevy 官方书（The Bevy book）的下一步学习。最后产品看起来像这样：

<video autoplay="" loop="" muted="" playsinline="" class="bevy_img">
    <source src="https://mbuffett.com/bevy_snake/new_gifs/game_over.mp4" type="video/mp4">
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

```rs
fn main() {
    App::build().add_plugins(DefaultPlugins).run();
}
```

然而 Bevy 的默认插件不包括摄像机（camera），所以我们来插入一个 2D 摄像机，只要我们创建我们第一个系统就可以设置了：

```rs
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dComponents::default());
}
```

[`Cammands`](https://docs.rs/bevy_ecs/0.2.1/bevy_ecs/struct.Commands.html) 通常用来排列命令，来更改游戏世界与资源。在这里，我们创建一个带有 2D 摄像机组件的实体。为Bevy的魔法做点准备吧：

```rs
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

```rs
struct SnakeHead;
struct Materials {
    head_material: Handle<ColorMaterial>,
}
```

`SnakeHead` 仅仅是一个空结构体，我们会把它当作一个组件来使用，它就是像某种标签，我们会放到一个实体上，之后我们能通过查询带有 `SnakeHead` 组件的实体来找到这个实体。像这样的空结构体在 Bevy 中是一种常见的模式，组件经常不需要他们自己的任何状态。 `Materials` 以后会变成一种资源，用来存储我们给蛇头使用的材质，也会用来存储蛇身和食物的材质。

`head_material` 句柄应该在游戏设置的时候就应该创建好，所以我们接下来要做的是，修改我们的 `setup` 函数：

```rs
fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    });
}
```

> **注意：** Bevy要求在注册系统时按照特定的顺序。命令（Commands） -> 资源（Resources） -> 组件（Components）/查询（Queries）。如果你在弄乱一个系统之后获得一个神秘的编译时错误，请检查你的顺序。

`materials.add` 会返回 `Handle<ColorMaterial>`。我们创建了使用这个新建 handle 的 `Materials` 结构体。之后，我们尝试访问类型为 `Materials` 的资源， Bevy会找到我们这个结构体。现在我们来在新的系统里创建我们的蛇头实体，然后你会看到我们如何使用前述资源的：

```rs
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

```rs
.add_startup_system(setup.system())
.add_startup_stage("game_setup") // <--
.add_startup_system_to_stage("game_setup", game_setup.system()) // <--
```

我们需要一个新的场景而不是再一次调用 `add_startup_system` 的原因是，我们需要使用在 `setup` 函数中插入的资源。这次运行后，你应该在屏幕中央看到蛇头：

![](https://mbuffett.com/bevy_snake/new_pics/snake_pixel.png)

好了，可能我们叫它“蛇头”有点过了，你可以看到一个 10x10 的白色精灵。

## 移动小蛇

> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/fde99c9)

如果小蛇不运动，那么游戏很无趣，所以我们先让蛇头动起来。我们之后再担心输入，现在我们的目标是让蛇头移动。所以我们来创建一个系统来移动所有的蛇头：

```rs
fn snake_movement(mut head_positions: Query<(&SnakeHead, &mut Transform)>) {
    for (_head, mut transform) in head_positions.iter_mut() {
        *transform.translation.y_mut() += 2.;
    }
}
```

这里有个新概念， `Query` 类型。我们用它来迭代所有拥有 `SnakeHead` 组件以及 `Transform` 组件的实体。我们不需要担心实际上如何创建查询类型， bevy 会帮我们创建好并用它调用我们的函数，算是 ECS 魔法的一部分。所以我们来加上这个系统， 然后看看会发生些什么：

```rs
.add_startup_system_to_stage("game_setup", game_setup.system())
.add_system(snake_movement.system()) // <--
.add_plugins(DefaultPlugins)
```

这是我们看到的，一头蛇移出了屏幕：

<video controls="" loop="" muted="" playsinline="" class="bevy_img">
    <source src="https://mbuffett.com/bevy_snake/new_gifs/moving_up.mp4" type="video/mp4">
</video>

你可能再思考 Transform 组件。当我们生成 `SnakeHead` 时，我们并没有给它 `Transform`，所以我们怎么就能找到一个同事拥有 `SnakeHead` 和 `Transform` 组件的实体呢？实际上 `SpriteComponents` 是一捆组件。就 `SpriteComponents` 来说，它包含了 `Transform` 组件，以及一堆其他组件（如 `Sprite`, `Mesh`, `Draw`, `Rotation`, `Sale`）。

## 控制小蛇

我们来修改我们小蛇的移动系统，使得我们可以控制小蛇：
```rs
fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<With<SnakeHead, &mut Transform>>,
) {
    for mut transform in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            *transform.translation.x_mut() -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            *transform.translation.x_mut() += 2.;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            *transform.translation.y_mut() -= 2.;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            *transform.translation.y_mut() += 2.;
        }
    }
}
```

留意到我们的查询 `Query<(&SnakeHead, &mut Transform)>` 改为了 `Query<With<SnakeHead, &mut Transform>>`，其实当前版本没有必要更改，旧的查询依然能很好地工作。我想，第一个系统的类型签名可能简单些，但是现在我们用正确的方式编写类型。这写法更正确是因为我们其实不需要 SnakeHead 组件。所以 `With` 类型允许我们说，“我们需要那些有蛇头的实体，但是我不关心蛇头组件，只给我 transform 组件就好。”每个系统访问的组件越少，bevy就能并行越多的系统。例如，如果另外一个系统正在修改 `SnakeHead` 组件，那这个系统旧不能在用旧写法的时候并行了。

现在，我们能控制小蛇了，尽管它动起来不那么像蛇：

<video autoplay="" loop="" muted="" playsinline="" class="bevy_img">
    <source src="https://mbuffett.com/bevy_snake/new_gifs/game_over.mp4" type="video/mp4">
</video>

## 码格子
> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/d1f4225)

到现在我们一直在用窗口的坐标，但这种方法只能在 (0, 0) 坐标在窗口正中央，并且单位是像素的时候有效。贪吃蛇游戏通常用格子，所以如果我们把我们的贪吃蛇设置成 10x10，那我们的窗口会 __真的__ 很小。我们让日子变得轻松些吧，我们选择用我们自己的位置和尺寸。然后，我们用系统来处理变换到窗口的坐标。

我们先定义格子为 10x10。在程序文件开头定义如下变量：

```rs
const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;
```

以及我们用于处理位置/尺寸的结构体：

```rs
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Size {
    width: f32,
    height: f32,
}
impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
```

相对直接地，有一个辅助方法来获取一个有相等长宽的 `Size`. Position 派生了一些很有用的 trait，所以我们不必不停地回顾这个结构体。 `Size` 可以仅仅包含一个浮点数，因为所有的对象最后都有相等的长度和宽度，但是我给它长度和宽度好像有点不对。我们现在把这些组件添加到我们生成的蛇头上：

```rs
commands
    .spawn(SpriteComponents {
        material: materials.head_material.clone(),
        sprite: Sprite::new(Vec2::new(10.0, 10.0)),
        ..Default::default()
    })
    .with(SnakeHead)
    .with(Position { x: 3, y: 3 }) // <--
    .with(Size::square(0.8)); // <--
```

这些组件暂时不做任何事情，我们现在就来将我们的尺寸映射到精灵的尺寸：

```rs
fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}
```

这个尺寸变换逻辑是这样的：如果某个对象有一个单位格子宽度，格子宽40，然后窗口现在 400px 宽，那么它应该有10哥宽度。下面我们做位置系统：

```rs
fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}
```

<!-- TODO: 翻译结果混乱，需要重新翻译 -->
位置变换：如果项目的 X 坐标在我们的系统中是 5，宽度是 10，并且窗口宽度是200，那么坐标应该是 5/10 * 200 - 200 / 2。我们减去一半的窗口宽度，因为我们的做消息是从左下角开始，然后替换到正中央。然后我们再加上半个格子，因为我们想要我们精灵的左下角对齐格子的左下角，而不是精灵中心对齐。

然后我们把这些系统加到我们的应用构建器上：
```rs
.add_system(snake_movement.system())
.add_system(position_translation.system()) <--
.add_system(size_scaling.system()) <--
.add_plugins(DefaultPlugins)
.run();
```

> **注意：** 现在最明显的问题是小蛇被压扁了。另外一个问题是我们破环了我们的输入处理。我们先修复输入处理，然后我们得记得回来处理我们被压扁的小蛇，把它恢复原状。

## 使用我们的格子

> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/0f39c51)

我们现在配置好了格子坐标，现在我们需要更新我们的 `snake_movement` 系统。之前我们使用 `Transform` 的地方，现在替换成 `Position`：

```rs
fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut head_positions: Query<With<SnakeHead, &mut Position>>,
) {
    for mut pos in head_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}
```

## 调整窗口大小

> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/dce7a53)

我们上一步中的小蛇被压扁了，是因为默认的窗口尺寸并不是方形的，然而我们的格子是，所以我们每个格坐标会宽度长于高度。我们修复它最简单的方法，是在构建 app 的时候创建一个 `WindowDescriptor` 资源：

```rs
    App::build()
        .add_resource(WindowDescriptor { // <--
            title: "Snake!".to_string(), // <--
            width: 200,                 // <--
            height: 200,                // <--
            ..Default::default()         // <--
        })
        .add_startup_system(setup.system())
```
同时，我们改一下背景颜色，插入这个 `use` 语句来引入 `ClearColor` 结构体：

```rs
use bevy::render::pass::ClearColor;
```

然后在 app 构建器增加资源：

```rs
.add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
```
## 生成食物
现在我们的小蛇可以到处移动了，该喂点东西给它了。现在我们给 `Materials` 加一个 `food_materials` 字段：

```rs
struct Materials {
    head_material: Handle<ColorMaterial>,
    food_material: Handle<ColorMaterial>, // <--
}
```

然后把这个新材质加到我们的 `setup` 函数里：

```rs
commands.insert_resource(Materials {
    head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()), // <--
});
```

然后我们需要 `Duration` 给要创建的定时器使用，而且我们还需要 `random` 来随机分配食物的位置。先在程序里引入这些：

```rs
use rand::prelude::random;
use std::time::Duration;
```

然后我们因素两个新结构体： `Food` 组件让我们知道哪个实体是食物，以及一个定时制造食物的定时器：

```rs
struct Food;

struct FoodSpawnTimer(Timer);
impl Default for FoodSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(1000), true))
    }
}
```

至于实现 `Default` 的原因，会在我解释下面的系统的时候说明：

```
fn food_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    time: Res<Time>,
    mut timer: Local<FoodSpawnTimer>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        commands
            .spawn(SpriteComponents {
                material: materials.food_material.clone(),
                ..Default::default()
            })
            .with(Food)
            .with(Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
            })
            .with(Size::square(0.8));
    }
}
```

我们引入了局部资源概念，具体而言是 `timer` 参数。 Bevy 会看到这个参数并且实例化一个 `FoodSpawnTimer` 类型的值，用的是我们的 `Default` 实现。这会在这个系统第一次运行是发生，之后这个系统会一直重用相同的定时器。像这样使用局部资源要比手动注册资源更贴近工程化。这个定时器会一直重复，所以我们只需要调用 `tick` 函数，然后无论这个系统在定时器完成后什么时候跑，我们就随机创建一些食物。

你可能知道下一步是什么了，把这个系统加到应用构建器上：

```rs
.add_system(food_spawner.system())
```

现在我们的程序看起来像这样：

<video controls="" loop="" muted="" playsinline="" class="bevy_img">
    <source src="/bevy_snake/new_gifs/food_spawning.mp4" type="video/mp4">
</video>

## 更像蛇的移动

> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/f4e6100)

我们现在准备定时触发小蛇移动。具体说来，我们想小蛇一直在移动，无论我们是否按下按键；并且我们想要它每隔 X 秒移动一次，而不是每一帧都移动。我们会改动几个地方，所以如果你不太清楚要改动哪里，查看这一小节的差异吧。

首先，我们需要加一个方向枚举：

```rs
#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}
```

然后把这个方向枚举加到我们的 `SnakeHead` 结构体，使得它知道应该要往哪里移动：

```rs
struct SnakeHead {
    direction: Direction,
}
```

我们也得在实例化 `SnakeHead` 组件的时候给定初始方向，例如我们让它一开始往上走：

```rs
.with(SnakeHead {
    direction: Direction::Up,
})
```

小蛇通常移动不是很流畅，是一种一步步来的行动。就行我们生成食物的时候，我们需要使用定时器来让系统没每隔 X秒/毫秒才跑一次。我们需要创建一个结构体来持有定时器：

```rs
struct SnakeMoveTimer(Timer);
```

然后我们把它当成资源加到我们的 app 构建器：

```rs
.add_resource(SnakeMoveTimer(Timer::new(
    Duration::from_millis(150. as u64),
    true,
)))
```

我们之所以不把这个定时器像生成食物的时候把定时器看成局部资源，是因为我们将会在几个系统里用上它，所以我帮你节约了一些重构的工作。因为我们需要在几个系统里使用它，我们需要创建一个新系统来触发这个定时器：

```rs
fn snake_timer(time: Res<Time>, mut snake_timer: ResMut<SnakeMoveTimer>) {
    snake_timer.0.tick(time.delta_seconds);
}
```

我们也可以把这段触发逻辑直接放到 `snake_movement` 系统里，但是我比较喜欢整洁地吧它放到一个单独的系统中，因为这个定时器会用在几个地方。我们把这个系统也加到 app上：

```rs
.add_system(snake_timer.system())
```

现在我们可以做方向逻辑的核心部分，也就是 `snake_movement` 系统，以下是更新后的版本：

```rs
fn snake_movement(
    keyboard_input: Res<Input<KeyCode>>,
    snake_timer: ResMut<SnakeMoveTimer>,
    mut heads: Query<(Entity, &mut SnakeHead)>,
    mut positions: Query<&mut Position>,
) {
    if let Some((head_entity, mut head)) = heads.iter_mut().next() {
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
        if !snake_timer.0.finished {
            return;
        }
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
    }
}
```

这里没有什么新概念，仅仅是游戏逻辑。你可能在想为什么我们需要获取拥有 `SankeHead` 组件的 `Entity`， 然后用另外一个独立的查询来获取位置， 而不是用像 `Query<Entity, &SnakeHead, &mut Position>` 这样的参数。原因在于，我们之后可能需要其他实体的位置，而分开两个查询访问相同的组件是不会允许放在 Bevy app 构建器上的。这样改了之后，你会获得一个蛇头移动的稍微……像蛇一样：

<video controls="" loop="" muted="" playsinline="" class="bevy_img">
    <source src="/bevy_snake/new_gifs/moving_snake_like.mp4" type="video/mp4">
</video>

## 加个尾巴
> [点击查看差异](https://github.com/marcusbuffett/bevy_snake/commit/7c8e2f7)

小蛇的尾巴有点复杂。对于每蛇尾的分段，我们需要知道它下一步需要到哪里。我们准备这样实现：将这些分段放到 `Vec`，然后存储为资源。这样，当我们更新分段的位置时，我们能够迭代所有的分段并且设置每个分段的位置为前一个分段的位置。

我们加一个 `segment_material` 字段到我们趁手的 `Materials` 结构体：
```rs
struct Materials {
    head_material: Handle<ColorMaterial>,
    segment_material: Handle<ColorMaterial>, // <--
    food_material: Handle<ColorMaterial>,
}
```

老调重弹，把 `segment_material` 加到 `setup` 中：

```rs
commands.insert_resource(Materials {
    head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
    segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()), // <--
    food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
});
```

然后一个给蛇身分段的组件：

```rs
struct SnakeSegment;
```

然后我们再加上我们说到的，用来存储分段列表的资源：

```rs
#[derive(Default)]
struct SnakeSegments(Vec<Entity>);
```

再把它作为资源加到我们的 app 上：

```rs
.add_resource(SnakeSegments::default())
```

我们我们需要从几个地方生成分段（当你吃食物或者你初始化小蛇的时候），我们需要先创建一个辅助函数：

```rs
fn spawn_segment(
    commands: &mut Commands,
    material: &Handle<ColorMaterial>,
    position: Position,
) -> Entity {
    commands
        .spawn(SpriteComponents {
            material: material.clone(),
            ..SpriteComponents::default()
        })
        .with(SnakeSegment)
        .with(position)
        .with(Size::square(0.65))
        .current_entity()
        .unwrap()
}
```
这看上去非常像我们生成 `SnakeHead` 的函数，但是替换了 `SnakeHead` 组件，我们用的是 `SnakeSegment` 组件。这里要说的新知识点，就是我们最后通过 `current_entity` 函数，获取了生成的 `Entity` （其实只是个 id），然后将它返回给调用者以便使用它。现在，我们需要修改我们的游戏配置函数。并非只是生成一个蛇头，它现在要生成一个蛇身的分段：

```rs
fn spawn_snake(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    segments.0 = vec![
        commands
            .spawn(SpriteComponents {
                material: materials.head_material.clone(),
                ..Default::default()
            })
            .with(SnakeHead {
                direction: Direction::Up,
            })
            .with(SnakeSegment)
            .with(Position { x: 3, y: 3 })
            .with(Size::square(0.8))
            .current_entity()
            .unwrap(),
        spawn_segment(
            &mut commands,
            &materials.segment_material,
            Position { x: 3, y: 2 },
        ),
    ];
}
```

我们第一个分段是头部，现在我们多加了一个 `with(SnakeSegment)`。第二个分段来自我们的 `spawn_segment` 函数。我先现在得到了一条小小的尾巴：

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
