use bevy::prelude::*;
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
    app.add_plugins(PlayerPlugin);
    app.add_plugins(AsepriteUltraPlugin);
    app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.15)));
    app.add_systems(Startup, setup);
    app
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
