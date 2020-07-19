# 抽象工厂（Abstract Factory）模式

## 场景需求
我们在设计游戏的时候，经常会遇到设计类似关卡的需求，例如 RPG 游戏中的副本（dungeon），每种副本都循序类似的模式、类似的行为，但是采用资源不同，行为的细节可能有所差异，为了方便我们新增新的副本，同时使得原有副本的修改尽可能少影响游戏的运行机制（所谓符合开闭原则），我们可以采用抽象工厂模式来统一管理游戏副本的生成。

## 场景设计
我们先设计一个简单的副本模式，包含了一个 Boss 和若干小怪，小怪数量在范围内随机波动。小怪和 Boss 都拥有血量，但是只有Boss 能攻击。我们把这个模式定义为结构：

```rs,no_run
struct Dungeon {
    boss: Box<dyn Boss>,
    monsters: Vec<Box<dyn Monster>>,
}

trait Monster {
    // 获知当前血量
    fn life(&self) -> u64;

    // 怪物血量可以因受到攻击降低，也可自行回复
    fn change_life(&mut self, diff: i32);
}

trait Boss: Monster {
    // Boss 能攻击玩家（Hero）
    fn attack(&self target: &mut Box<dyn Hero>);
}
```

上面在运行时已经不关心具体是什么副本（当然，副本的元数据可以添加为 `Dungeon` 结构的字段），我们只需要知道它有一个 `Boss` 和若干 `Monster`。

然后，我们建立抽象工厂：

```
use rand::random;
trait DungeonFactory {
    fn create_boss(&self) -> Box<dyn Boss>
    fn create_mosters(&self, amount: usize) -> Vec<Box<dyn Monster>>

    fn generate_dungeon(&self) -> Dungeon {
        Dungeon {
            boss: self.create_boss(),
            monsters: self.create_monsters(random::<u8>() % 2 + 2), // 2 ~ 3 只小怪
        }
    }
}
```

然后，我们创建新手副本的具体工厂:

```rs,no_run
struct NewbieDungeonFactory;

impl DungeonFactory for NewBieDungeonFactory {
    fn create_boss(&self) -> Box<dyn Boss> {
        NewBieBoss::new()
    }

    // 新手副本甚至没有小怪，
    fn create_mosters(&self, amount: usize) -> Vec<Box<dyn Monster>> {
        Vec::new()
    }

    // generate_dungeon 已有默认实现，不需要修改。
}

// 新手副本，boss极弱
struct NewbieBoss {
    max_life: u8,
    life: u8,
    attack: u8,
}

impl NewbieBoss {
    fn new() -> Self {
        NewBieBoss {
            max_life: 5,
            life: 5,
            attack: 1,
        }
    }
}

impl Monster for NewBieBoss {
    fn life(&self) -> u64 {
        self.life as u64
    }

    fn change_life(&mut self, diff: i32) {
        self.life = match self.life + diff {
            n if n < 0 => 0,
            n if n > self.max_life => self.max_life
            n => n
        };
    }
}

impl Boss for NewbieBoss {
    fn attack(&self, target: &mut Box<dyn Hero>) {
        target.attacked_wtih(self.attack);
    }
}
```

上面的工厂看上去好像很多内容，主要是因为 `Boss`特质需要一个新载体 `NewBieBoss`，同时实现 `Boss` 载体的 `NewBieBoss` 也需要实现 `Monster` 特质（继承关系组合化）, 这部分的代码作为示例让读者理解设定的 `Dungeon` 模式的行为。

接下来，我们可以新增一个新副本，新副本的小怪和 Boss 是新手副本的 `NewbieBoss`，但是小怪已经不能攻击玩家了（通过动态分发为 `dyn Monster` 对象来屏蔽 `Boss` 的攻击行为：

```rs,no_run
struct JuniorDungeonFactory;

impl DungeonFactory for JuniorDungeonFactory {
    fn create_boss(&self) -> Box<dyn Boss> {
        NewBieBoss::new()
    }

    // 新手副本甚至没有小怪，
    fn create_mosters(&self, amount: usize) -> Vec<Box<dyn Monster>> {
        vec![NewBieBoss::new(); amount]
    }

    // generate_dungeon 已有默认实现，不需要修改。
}
```

我们可以看到，抽象工厂模式很方便地为我们重用了已有资源并确保副本的行为效果。

## 为什么使用抽象工厂模式
- **模式统一**。通过抽象工厂可以快速地制作符合相同模式，但是资源细节稍有差异的工厂，生成具有同样属性的产品。
- **职责专一**。每个具体工厂只有一个原因需要被修改——该工厂对应的副本细节变化了。
- **符合开闭原则**。因为我们统一出了抽象工厂的接口，当我们修改具体工厂的具体实现细节时，并不会影响到接口调用；而新增具体工厂时只需要提供对应的接口，也可以方便地接入副本生成系统。
