use avian2d::prelude::*;
use bevy::prelude::*;

use crate::combat::Health;
use crate::enemy::core::{AttackTimer, Enemy};
use crate::i18n::I18n;
use crate::player::core::Player;

/// 追到离玩家这么近就不再继续挤压
const CHASE_STOP_DISTANCE: f32 = 44.0;

pub fn enemy_move_ai(
    mut enemy_query: Query<(&mut LinearVelocity, &Transform, &Enemy), Without<Player>>,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_position = player_transform.translation.truncate();

    for (mut velocity, enemy_transform, enemy) in enemy_query.iter_mut() {
        let to_player = player_position - enemy_transform.translation.truncate();
        let distance = to_player.length();

        // 追击：朝玩家移动，贴近后停止
        if distance > CHASE_STOP_DISTANCE {
            let direction = to_player.normalize();
            velocity.0 = direction * enemy.spd();
        } else {
            velocity.0 = Vec2::ZERO;
        }
    }
}

pub fn enemy_attack(
    time: Res<Time>,
    i18n: Res<I18n>,
    mut player_query: Query<(&Transform, &mut Health, &Player), Without<Enemy>>,
    mut enemy_query: Query<(&Transform, &mut AttackTimer, &Enemy), Without<Player>>,
) {
    let Ok((player_transform, mut player_health, player)) = player_query.single_mut() else {
        return;
    };

    let player_position = player_transform.translation.truncate();

    for (enemy_transform, mut attack_timer, enemy) in enemy_query.iter_mut() {
        attack_timer.0.tick(time.delta());

        let distance = player_position.distance(enemy_transform.translation.truncate());
        if distance > enemy.attack_range() || !attack_timer.0.just_finished() {
            continue;
        }

        let damage = (enemy.atk() - player.def() / 2).max(1);
        let was_alive = !player_health.is_dead();
        let dead = player_health.take_damage(damage);

        let enemy_name = enemy.localized_name(&i18n);
        info!(
            "{}",
            i18n.tr_args(
                "combat.player_hit",
                &[
                    ("enemy", enemy_name),
                    ("damage", damage.to_string()),
                    ("current", player_health.current.to_string()),
                    ("max", player_health.max.to_string()),
                ]
            )
        );

        if dead && was_alive {
            warn!("{}", i18n.tr("combat.player_death"));
        }
    }
}
