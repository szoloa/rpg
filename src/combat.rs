use bevy::prelude::*;

use crate::enemy::core::Enemy;
use crate::player::core::Player;

/// 全局统一的血量组件，玩家和敌人都使用它。
#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

impl Health {
    pub fn new(max: i32) -> Self {
        let max = max.max(1);
        Self { current: max, max }
    }

    /// 剩余血量比例，范围 0.0..=1.0
    pub fn ratio(&self) -> f32 {
        if self.max <= 0 {
            0.0
        } else {
            (self.current as f32 / self.max as f32).clamp(0.0, 1.0)
        }
    }

    pub fn is_dead(&self) -> bool {
        self.current <= 0
    }

    /// 扣血并返回是否因这次伤害死亡
    pub fn take_damage(&mut self, amount: i32) -> bool {
        self.current = (self.current - amount.max(0)).max(0);
        self.is_dead()
    }
}

/// 玩家攻击冷却
#[derive(Component)]
pub struct AttackCooldown(pub Timer);

impl AttackCooldown {
    pub fn new(seconds: f32) -> Self {
        Self(Timer::from_seconds(seconds, TimerMode::Once))
    }
}

pub struct CombatPlugin;

const PLAYER_ATTACK_RANGE: f32 = 72.0;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_attack);
    }
}

fn player_attack(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut player_query: Query<(&Transform, &Player, &mut AttackCooldown), Without<Enemy>>,
    mut enemy_query: Query<(Entity, &Transform, &Enemy, &mut Health), Without<Player>>,
) {
    let Ok((player_transform, player, mut cooldown)) = player_query.single_mut() else {
        return;
    };

    cooldown.0.tick(time.delta());

    let wants_attack =
        keyboard.just_pressed(KeyCode::KeyJ) || keyboard.just_pressed(KeyCode::Space);
    if !wants_attack || !cooldown.0.is_finished() {
        return;
    }
    cooldown.0.reset();

    let player_position = player_transform.translation.truncate();
    let mut hit_any = false;

    for (entity, transform, enemy, mut health) in enemy_query.iter_mut() {
        let distance = player_position.distance(transform.translation.truncate());
        if distance > PLAYER_ATTACK_RANGE {
            continue;
        }

        hit_any = true;
        let damage = (player.atk() - enemy.def() / 2).max(1);
        let dead = health.take_damage(damage);
        info!(
            "命中 {}：{} 点伤害（{}/{}）",
            enemy.display_name(),
            damage,
            health.current,
            health.max
        );

        if dead {
            info!("{} 已被击杀", enemy.display_name());
            commands.entity(entity).despawn();
        }
    }

    if !hit_any {
        info!("攻击落空");
    }
}
