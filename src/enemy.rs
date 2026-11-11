use std::{collections::HashMap, process::id};

use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(mut app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, enemy_ai);
    }
}

fn setup(mut commands: &mut Commands) {
    let enemy = Enemy::empty();
    let enemys = HashMap::new();
    enemys.insert(enemy.id, Some(add_enemy(commands, enemy)));
}

fn add_enemy(mut commands: &mut Commands, enemy: Enemy) -> Entity {
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
    mut enemy_query: Query<(&mut Transform, &mut Enemy), Without<Player>>,
    mut player_query: Query<(&Transform, &mut Health), (With<Player>, Without<Enemy>)>,
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
            enemy_transform.translation += dir * ENEMY_SPEED * time.delta_secs();
        }

        // 近战攻击
        enemy.attack_timer.tick(time.delta());
        if distance < 60.0 && enemy.attack_timer.finished() {
            player_health.current -= ENEMY_ATTACK_DAMAGE;
            enemy.attack_timer.reset();
            println!("敌人攻击！玩家剩余 HP: {}", player_health.current);
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
struct Health {
    current: i32,
    max: i32,
}

struct Status {
    health: Health,
}

struct Attribute;

pub struct Enemy {
    id: u32,
    name: &'static str,
    status: Status,
    attribute: Attribute,
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
            health: Health {
                current: 30,
                max: 30,
            },
        };
        create(0, "enemy", status, Attribute {})
    }
}
