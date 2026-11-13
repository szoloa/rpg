use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tiled::prelude::*;
use rpg::*;

// use rpg::player::Player;

const COLOR_HP_PLAYER: Color = Color::srgb(0.2, 0.9, 0.2);
const COLOR_HP_BG: Color = Color::srgb(0.2, 0.2, 0.2);

fn main() -> AppExit {
    let mut app = build_app();
    app.run()
}

fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window { ..default() }),
        ..default()
    }));

    app.add_systems(Startup, setup);
    app.add_plugins(AsepriteUltraPlugin);
    app.add_plugins(TiledPlugin::default());
    app.add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default());
    // 添加 Avian 物理引擎插件
    app.add_plugins(PhysicsPlugins::default());
    // 俯视角 2D 游戏，关闭重力
    app.insert_resource(Gravity::ZERO);
    app.add_plugins(MapPlugin);
    app.add_plugins(PlayerPlugin);
    app.add_plugins(EnemyPlugin);
    // app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)));
    app
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
}
