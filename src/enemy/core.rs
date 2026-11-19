use std::collections::HashMap;

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

use crate::combat::Health;
pub use crate::enemy::ai::*;

pub struct EnemyPlugin;

const COLOR_ENEMY: Color = Color::srgb(0.2, 0.2, 0.2);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (enemy_move_ai, enemy_attack).chain());
    }
}

/// 敌人攻击间隔计时器
#[derive(Component)]
pub struct AttackTimer(pub Timer);

impl AttackTimer {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds.max(0.05), TimerMode::Repeating))
    }
}

/// 敌人模板表，从 `assets/enemy.csv` 加载
#[derive(Resource, Default)]
pub struct EnemyDatabase(pub HashMap<String, EnemyTemplate>);

impl EnemyDatabase {
    pub fn get(&self, enemy_type: &str) -> Option<&EnemyTemplate> {
        self.0.get(enemy_type)
    }

    fn load_from_csv(csv_data: &str) -> Self {
        Self(EnemyTemplate::load_enemy_type_from_csv(csv_data))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct EnemyTemplate {
    pub name: String,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
    pub attack_range: f32,
    pub attack_cooldown: f32,
}

#[derive(Debug, Deserialize)]
struct EnemyRecord {
    enemy_type: String,
    hp: i32,
    atk: i32,
    def: i32,
    spd: i32,
    attack_range: f32,
    attack_cooldown: f32,
}

impl EnemyTemplate {
    fn load_enemy_type_from_csv(csv_data: &str) -> HashMap<String, EnemyTemplate> {
        let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
        let mut map = HashMap::new();
        for result in rdr.deserialize::<EnemyRecord>() {
            match result {
                Ok(record) => {
                    map.insert(
                        record.enemy_type.clone(),
                        EnemyTemplate {
                            name: record.enemy_type,
                            hp: record.hp,
                            atk: record.atk,
                            def: record.def,
                            spd: record.spd,
                            attack_range: record.attack_range,
                            attack_cooldown: record.attack_cooldown,
                        },
                    );
                }
                Err(e) => warn!("跳过一行无效敌人数据: {e}"),
            }
        }
        map
    }
}

/// 出生点表，从 `assets/enemy_spawn.csv` 加载
#[derive(Debug, Deserialize)]
struct EnemySpawn {
    enemy_type: String,
    x: f32,
    y: f32,
}

fn load_enemy_spawns(csv_data: &str) -> Vec<EnemySpawn> {
    let mut rdr = csv::Reader::from_reader(csv_data.as_bytes());
    let mut spawns = Vec::new();
    for result in rdr.deserialize::<EnemySpawn>() {
        match result {
            Ok(spawn) => spawns.push(spawn),
            Err(e) => warn!("跳过一行无效出生点数据: {e}"),
        }
    }
    spawns
}

fn setup(mut commands: Commands) {
    let database = EnemyDatabase::load_from_csv(include_str!("../../assets/enemy.csv"));
    let spawns = load_enemy_spawns(include_str!("../../assets/enemy_spawn.csv"));

    let mut spawned = 0usize;
    for (index, spawn) in spawns.iter().enumerate() {
        let Some(template) = database.get(&spawn.enemy_type) else {
            warn!("出生点引用了未知敌人类型: {}", spawn.enemy_type);
            continue;
        };
        add_enemy(
            &mut commands,
            index as u32,
            template,
            Vec2::new(spawn.x, spawn.y),
        );
        spawned += 1;
    }

    info!(
        "加载了 {} 种敌人模板，生成 {} 个敌人",
        database.0.len(),
        spawned
    );
    commands.insert_resource(database);
}

fn add_enemy(commands: &mut Commands, id: u32, template: &EnemyTemplate, position: Vec2) -> Entity {
    let enemy = Enemy::create_from_template(id, template);
    commands
        .spawn((
            Sprite::from_color(COLOR_ENEMY, Vec2::new(40.0, 40.0)),
            enemy,
            Health::new(template.hp),
            AttackTimer::new(template.attack_cooldown),
            Transform::from_xyz(position.x, position.y, 1.0),
            RigidBody::Dynamic,
            Collider::rectangle(40.0, 40.0),
            LockedAxes::ROTATION_LOCKED,
        ))
        .id()
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Status {
    enemy_type: String,
    hp: i32,
    atk: i32,
    def: i32,
    spd: i32,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Attribute {
    hp: i32,
    atk: i32,
    def: i32,
    spd: i32,
}

#[derive(Clone, PartialEq, Debug, Component)]
pub struct Enemy {
    id: u32,
    name: String,
    status: Status,
    attribute: Option<Attribute>,
    attack_range: f32,
    attack_cooldown: f32,
}

impl Enemy {
    fn create_from_template(id: u32, template: &EnemyTemplate) -> Self {
        Self {
            id,
            name: template.name.clone(),
            status: Status {
                enemy_type: template.name.clone(),
                hp: template.hp,
                atk: template.atk,
                def: template.def,
                spd: template.spd,
            },
            attribute: Some(Attribute {
                hp: template.hp,
                atk: template.atk,
                def: template.def,
                spd: template.spd,
            }),
            attack_range: template.attack_range,
            attack_cooldown: template.attack_cooldown,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn display_name(&self) -> &str {
        &self.name
    }

    pub fn enemy_type(&self) -> &str {
        &self.status.enemy_type
    }

    pub fn spd(&self) -> f32 {
        self.status.spd as f32
    }

    pub fn atk(&self) -> i32 {
        self.status.atk
    }

    pub fn def(&self) -> i32 {
        self.status.def
    }

    pub fn attack_range(&self) -> f32 {
        self.attack_range
    }

    pub fn attack_cooldown(&self) -> f32 {
        self.attack_cooldown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enemy_csv_parses_all_templates() {
        let database = EnemyDatabase::load_from_csv(include_str!("../../assets/enemy.csv"));
        assert!(database.get("empty").is_some());
        assert!(database.get("slime").is_some());
    }

    #[test]
    fn enemy_spawn_csv_parses() {
        let spawns = load_enemy_spawns(include_str!("../../assets/enemy_spawn.csv"));
        assert!(!spawns.is_empty());
    }
}
