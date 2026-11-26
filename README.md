# rpg

基于 Bevy 0.20 的俯视角 2D RPG 原型。使用 avian2d 物理、Tiled 地图、Aseprite 动画与 CSV 数据表。

## 运行

```powershell
cargo run
```

> Bevy 0.20 优先用 `CARGO_MANIFEST_DIR` 作为资源根目录，所以 `cargo run` 能直接找到 `assets`。
> 如果直接运行 `target\debug\rpg.exe`，需要把 `assets` 复制到 exe 旁边，或设置 `BEVY_ASSET_ROOT`。

## 操作

| 按键 | 功能 |
| --- | --- |
| `WASD` | 移动 |
| `J` / `Space` | 攻击最近的敌人 |
| `E` | 拾取调试物品 |
| `F1` | 切换语言（中文 / English） |

## 目录

| 路径 | 说明 |
| --- | --- |
| `src/i18n.rs` | 多语言资源、翻译查表、`{{placeholder}}` 替换、F1 切换 |
| `src/combat.rs` | 统一 `Health`、玩家攻击冷却与伤害结算 |
| `src/enemy/` | 敌人数值表、出生点、追击与攻击 AI |
| `src/player/` | 玩家属性、移动动画、背包与 HUD |
| `assets/i18n/` | 各语言的 `key,text` 翻译表 |
| `assets/enemy.csv` | 敌人模板（数值 + 攻击范围/冷却） |
| `assets/enemy_spawn.csv` | 敌人出生点表 |
| `assets/map.tmx` | Tiled 地图 |

## 新增一种语言

1. 在 `src/i18n.rs` 的 `Language` 中加一个枚举值。
2. 在 `SUPPORTED_LANGUAGES` 里注册它，并在 `code()` / `native_name()` 中补上对应信息。
3. 在 `I18n::new()` 中加载新的 `assets/i18n/xx.csv`。
4. 新 CSV 必须与 `zh_cn.csv` 拥有完全相同的 key 集合（`both_languages_define_the_same_keys` 测试会检查）。

代码中的用户可见文本统一通过 `I18n::tr` / `tr_args` 取 key，禁止直接硬编码文案。
