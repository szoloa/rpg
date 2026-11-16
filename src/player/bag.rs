use bevy::prelude::*;

pub struct BagPlugin;

pub trait BagItem: Send + Sync + Clone + 'static {
    /// 唯一 ID，用于判断是否为同类物品（未来做堆叠/合成可用）
    fn get_id(&self) -> u32;
    /// 显示名称
    fn get_name(&self) -> &'static str;
    /// 最大堆叠数
    fn max_stack(&self) -> u32 {
        1
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemType {
    Wood,
    Stone,
    Potion,
}

impl BagItem for ItemType {
    fn get_id(&self) -> u32 {
        match self {
            ItemType::Wood => 1,
            ItemType::Stone => 2,
            ItemType::Potion => 3,
        }
    }
    fn get_name(&self) -> &'static str {
        match self {
            ItemType::Wood => "Wood",
            ItemType::Stone => "Stone",
            ItemType::Potion => "Potion",
        }
    }
    fn max_stack(&self) -> u32 {
        match self {
            ItemType::Wood | ItemType::Stone => 99,
            ItemType::Potion => 16,
        }
    }
}

pub struct Inventory<I> {
    slots: Vec<Option<I>>,
    capacity: usize,
}

impl<I: BagItem> Inventory<I> {
    pub fn new(capacity: usize) -> Self {
        Self {
            slots: vec![None; capacity],
            capacity,
        }
    }

    /// 背包是否已满
    fn is_full(&self) -> bool {
        self.slots.iter().all(|s| s.is_some())
    }

    /// 添加物品，成功返回 true
    fn add_item(&mut self, item: I) -> bool {
        if self.is_full() {
            return false;
        }
        if let Some(idx) = self.slots.iter().position(|s| s.is_none()) {
            self.slots[idx] = Some(item);
            true
        } else {
            false
        }
    }

    /// 按槽位索引取出物品
    fn remove_at(&mut self, idx: usize) -> Option<I> {
        self.slots.get_mut(idx).and_then(|s| s.take())
    }

    /// 已占用槽位数量
    fn count(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }

    /// 遍历已有物品
    fn iter(&self) -> impl Iterator<Item = &I> {
        self.slots.iter().filter_map(|s| s.as_ref())
    }
}

#[derive(Component)]
pub struct ContainerComponent<I: BagItem>(Inventory<I>);

impl<I: BagItem> ContainerComponent<I> {
    /// 外部模块用这个构造，而不是直接 ContainerComponent(...)
    pub fn new(capacity: usize) -> Self {
        Self(Inventory::new(capacity))
    }
    pub fn inventory(&self) -> &Inventory<I> {
        &self.0
    }
    pub fn inventory_mut(&mut self) -> &mut Inventory<I> {
        &mut self.0
    }
}

impl Plugin for BagPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, debug_pickup);
    }
}

fn debug_pickup(
    mut query: Query<&mut ContainerComponent<ItemType>, With<crate::Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyE) {
        return;
    }

    let item = match pseudo_rand() {
        0 => ItemType::Wood,
        1 => ItemType::Stone,
        _ => ItemType::Potion,
    };
    let name = item.get_name();

    // 只处理一个玩家，item 只被 move 一次，没有循环问题
    let Ok(mut container) = query.single_mut() else {
        return;
    };
    let inv = &mut container.0;

    if inv.add_item(item) {
        println!(
            "[Bag] Picked up {} ({}/{})",
            name,
            inv.count(),
            inv.capacity
        );
    } else {
        println!("[Bag] Inventory is full!");
    }
}

fn pseudo_rand() -> u32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
        % 3
}
