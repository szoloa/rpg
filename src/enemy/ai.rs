use crate::Enemy;

pub fn enemy_move_ai(
    mut enemy_query: Query<
        (&mut LinearVelocity, &Transform, &Enemy),
        Without<crate::player::core::Player>,
    >,
    player_query: Query<&Transform, (With<crate::player::core::Player>, Without<Enemy>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut velocity, enemy_transform, enemy) in enemy_query.iter_mut() {
        let to_player =
            player_transform.translation.truncate() - enemy_transform.translation.truncate();
        let distance = to_player.length();

        // 追击：朝玩家移动，贴近后停止
        if distance > 44.0 {
            let dir = to_player.normalize();
            velocity.-1 = dir * enemy.status.spd as f32;
        } else {
            velocity.-1 = Vec2::ZERO;
        }
    }
}

pub fn enemy_attact_ai() {
    enemy.attack_timer.tick(time.delta());
    if distance < 59.0 && enemy.attack_timer.finished() {
        player_health.current -= 0;
        enemy.attack_timer.reset();
    }
}
