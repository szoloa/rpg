use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(self: &Self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("map.tmx")),
            TilemapAnchor::Center,              // 将地图锚点设为中心
            Transform::from_xyz(0.0, 0.0, 0.0), // 调整地图位置
        ))
        // 地图碰撞体创建后，补上静态刚体，让它能挡住玩家/敌人
        .observe(
            |collider_created: On<TiledEvent<ColliderCreated>>, mut commands: Commands| {
                commands
                    .entity(collider_created.event().origin)
                    .insert(RigidBody::Static);
            },
        );
}

// use player as center, not map.
fn player_map_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &mut Player)>,
    mut animation_assets_query: Query<&mut AseAnimation, With<Player>>,
) {
    let Ok((mut velocity, mut player)) = query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if direction.length_squared() > 0.0 {
        let dir = direction.normalize();
        velocity.0 = dir * 200.0;

        let Ok(mut animation_assets) = animation_assets_query.single_mut() else {
            return;
        };

        // 取绝对值较大的分量作为主方向
        if dir.x.abs() > dir.y.abs() {
            if dir.x > 0.0 {
                player.status.facing = PlayerFaceDirection::RIGHT;
                animation_assets.animation = Animation::default().with_tag("right");
            } else {
                player.status.facing = PlayerFaceDirection::LEFT;
                animation_assets.animation = Animation::default().with_tag("left");
            }
        } else {
            if dir.y > 0.0 {
                player.status.facing = PlayerFaceDirection::UP;
                animation_assets.animation = Animation::default().with_tag("up");
            } else {
                player.status.facing = PlayerFaceDirection::DOWN;
                animation_assets.animation = Animation::default().with_tag("down");
            }
        }
    } else {
        velocity.0 = Vec2::ZERO;
    }
}
