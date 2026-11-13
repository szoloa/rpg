use std::{collections::HashMap, iter::empty};

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::Deserialize;

pub struct EnemyPlugin;

const COLOR_ENEMY: Color = Color::srgb(0.2, 0.2, 0.2);

impl Plugin for EnemyPlugin {
    fn build(&self, mut app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, enemy_ai);
    }
}

fn setup(commands: Commands) {
    let csv_data = include_str!("../assets/enemy.csv");
    let enemy_map = Enemy::load_enemy_type_from_csv(csv_data.into());
    let empty_enemy = enemy_map.get("empty").unwrap();
    let empty_enemy_entity = Enemy::create_from_enemy_template(empty_enemy);
    add_enemy(commands, empty_enemy_entity);
}

fn add_enemy(mut commands: Commands, enemy: Enemy) -> Entity {
    commands
        .spawn((
            Sprite::from_color(COLOR_ENEMY, Vec2::new(40.0, 40.0)),
            enemy,
            Transform::from_xyz(100.0, 150.0, 1.0),
            RigidBody::Dynamic,
            Collider::rectangle(40.0, 40.0),
            LockedAxes::ROTATION_LOCKED,
        ))
        .id()
}

fn enemy_ai(
    mut enemy_query: Query<
        (&mut LinearVelocity, &Transform, &Enemy),
        Without<crate::player::Player>,
    >,
    player_query: Query<&Transform, (With<crate::player::Player>, Without<Enemy>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut velocity, enemy_transform, enemy) in enemy_query.iter_mut() {
        let to_player =
            player_transform.translation.truncate() - enemy_transform.translation.truncate();
        let distance = to_player.length();

        // 追击：朝玩家移动，贴近后停止
        if distance > 45.0 {
            let dir = to_player.normalize();
            velocity.0 = dir * enemy.status.spd as f32;
        } else {
            velocity.0 = Vec2::ZERO;
        }

        // 近战攻击
        // enemy.attack_timer.tick(time.delta());
        // if distance < 60.0 && enemy.attack_timer.finished() {
        //     player_health.current -= 1;
        //     enemy.attack_timer.reset();
        //     println!("敌人攻击！玩家剩余 HP: {}", player_health.current);
        // }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Clone, PartialEq, Eq, Debug, Component)]
struct Status {
    anamy_type: String,
    hp: i32,
    atk: i32,
    def: i32,
    spd: i32,
}

#[derive(Clone, PartialEq, Eq, Debug, Component)]
struct Attribute;

#[derive(Clone, PartialEq, Eq, Debug, Component)]
pub struct Enemy {
    id: u32,
    name: &'static str,
    status: Status,
    attribute: Option<Attribute>,
}

#[derive(Debug, Deserialize)]
struct EnemyRecord {
    enemy_type: String,
    hp: i32,
    atk: i32,
    def: i32,
    spd: i32,
}

#[derive(Debug, Clone)]
pub struct EnemyTemplate {
    pub name: String,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
}

#[derive(Resource, Default)]
pub struct EnemyDatabase(pub HashMap<String, EnemyTemplate>);

impl EnemyDatabase {
    pub fn get(&self, enemy_type: &str) -> Option<&EnemyTemplate> {
        self.0.get(enemy_type)
    }
}

impl Enemy {
    fn create(id: u32, name: &'static str, status: Status, attribute: Attribute) -> Enemy {
        Enemy {
            id: id,
            name: name,
            status: status,
            attribute: Some(attribute),
        }
    }
    fn empty() -> Enemy {
        let status = Status {
            anamy_type: "empty".to_string(),
            hp: 10,
            atk: 1,
            def: 1,
            spd: 200,
        };
        Self::create(0, "enemy", status, Attribute {})
    }
    fn load_enemy_type_from_csv(csv_data: String) -> HashMap<String, EnemyTemplate> {
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
                        },
                    );
                }
                Err(e) => warn!("跳过一行无效敌人数据: {e}"),
            }
        }
        info!("加载了 {} 种敌人模板", map.len());
        // commands.insert_resource(EnemyDatabase(map));
        map
    }
    fn create_from_enemy_template(enemy_template: &EnemyTemplate) -> Self {
        Self {
            id: 10,
            name: "empty",
            status: Status {
                anamy_type: enemy_template.name.clone(),
                hp: enemy_template.hp,
                atk: enemy_template.atk,
                def: enemy_template.def,
                spd: enemy_template.spd,
            },
            attribute: None,
        }
    }
}

#[test]
fn test_enemy() {}
