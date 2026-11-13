use std::{collections::HashMap, process::id};

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

fn setup(mut commands: Commands) {}

fn add_enemy(mut commands: Commands, enemy: Enemy) -> Entity {
    commands
        .spawn((
            Sprite::from_color(COLOR_ENEMY, Vec2::new(40.0, 40.0)),
            enemy,
            Transform::from_xyz(100.0, 150.0, 1.0),
        ))
        .id()
}

fn enemy_ai(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<crate::player::Player>>,
    mut player_query: Query<
        (&Transform, &mut Health),
        (With<crate::player::Player>, Without<Enemy>),
    >,
) {
    let Ok((player_transform, mut player_health)) = player_query.single_mut() else {
        return;
    };

    for (mut enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let to_player = player_transform.translation - enemy_transform.translation;
        let distance = to_player.length();

        // 追击：朝玩家移动
        if distance > 45.0 {
            let dir = to_player.normalize();
            enemy_transform.translation += dir * time.delta_secs();
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
struct Status {
    anamy_type: &'static str,
    hp: i32,
    atk: i32,
    def: i32,
    spd: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
struct Attribute;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct Enemy {
    id: u32,
    name: &'static str,
    status: Status,
    attribute: Attribute,
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
            attribute: attribute,
        }
    }
    fn empty() -> Enemy {
        let status = Status {
            anamy_type: "empty",
            hp: 10,
            atk: 1,
            def: 1,
            spd: 200,
        };
        Self::create(0, "enemy", status, Attribute {})
    }
    fn load_by_enemy_type(mut commands: &mut Commands) {
        let csv_data = include_str!("../assets/enemy.csv");
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
        commands.insert_resource(EnemyDatabase(map));
    }
}
