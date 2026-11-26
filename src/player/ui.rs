use bevy::prelude::*;

use crate::combat::Health;
use crate::i18n::I18n;
use crate::player::core::Player;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, update_hud);
    }
}

/// 左上角 HUD 文本实体标记
#[derive(Component)]
struct HudText;

fn spawn_hud(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                left: Val::Px(12.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        ))
        .with_children(|parent| {
            parent.spawn((
                HudText,
                Text::new(""),
                TextFont {
                    font: FontSource::Generic(GenericFontFamily::SansSerif),
                    font_size: 18.0.into(),
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn update_hud(
    i18n: Res<I18n>,
    player_query: Query<(&Player, &Health)>,
    mut text_query: Query<&mut Text, With<HudText>>,
) {
    let Ok((player, health)) = player_query.single() else {
        return;
    };
    let Ok(mut text) = text_query.single_mut() else {
        return;
    };

    let title = i18n.tr_args(
        "hud.title",
        &[
            ("name", player.name().to_string()),
            ("level", player.level().to_string()),
        ],
    );
    let race_class = i18n.tr_args(
        "hud.race_class",
        &[
            ("race", i18n.tr(player.race().name_key())),
            ("class", i18n.tr(player.class().name_key())),
        ],
    );
    let class_desc = i18n.tr(player.class().desc_key());
    let hp = i18n.tr_args(
        "hud.hp",
        &[
            ("current", health.current.to_string()),
            ("max", health.max.to_string()),
            ("mp", player.mp().to_string()),
            ("max_mp", player.max_mp().to_string()),
        ],
    );
    let controls = i18n.tr("hud.controls");
    let language = i18n.tr_args(
        "hud.language",
        &[("language", i18n.language().native_name().to_string())],
    );

    text.0 = format!("{title}\n{race_class}\n{class_desc}\n\n{hp}\n\n{controls}\n{language}");
}
