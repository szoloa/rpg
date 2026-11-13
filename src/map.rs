use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(self: &Self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TiledMap(asset_server.load("map.tmx")),
        TilemapAnchor::Center,               // 将地图锚点设为中心
        Transform::from_xyz(0.0, 0.0, -1.0), // 调整地图位置
    ));
}
