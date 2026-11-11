use bevy::image::Image;
use bevy::log::tracing_subscriber::reload::Handle;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Class {
    Warrior,
    Mage,
    Rogue,
}

impl Class {
    fn name(&self) -> &'static str {
        match self {
            Class::Warrior => "战士",
            Class::Mage => "法师",
            Class::Rogue => "盗贼",
        }
    }

    fn desc(&self) -> &'static str {
        match self {
            Class::Warrior => "高生命高防御，物理伤害的主力。",
            Class::Mage => "体质脆弱，但法术伤害惊人。",
            Class::Rogue => "敏捷灵巧，爆发力强且极易闪避。",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Races {
    Human,
    Elves,
}

impl Races {
    fn name(&self) -> &'static str {
        match self {
            Races::Human => "人类",
            Races::Elves => "精灵",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Skill {
    name: &'static str,
    cost: i32,
    always_crit: bool,
    desc: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Attribute {
    level: i32,
    max_hp: i32,
    max_mp: i32,
    atk: i32,
    def: i32,
    agk: i32,
    mag: i32,
}

impl Attribute {
    fn empty() -> Self {
        Attribute {
            level: 0,
            max_hp: 1,
            max_mp: 0,
            atk: 0,
            def: 0,
            agk: 0,
            mag: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PlayerFaceDirection {
    UP,
    LEFT,
    RIGHT,
    DOWN,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum AnimationState {
    Idle,
    Run,
    Attack,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Status {
    health: Health,
    facing: PlayerFaceDirection,
    animationstate: AnimationState,
    mp: i32,
    exp: u32,
}

impl Status {
    fn empty() -> Self {
        Status {
            health: Health {
                current: 1,
                max: 100,
            },
            facing: PlayerFaceDirection::DOWN,
            animationstate: AnimationState::Idle,
            mp: 0,
            exp: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
struct Health {
    current: i32,
    max: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct Player {
    id: u32,
    name: &'static str,
    attribute: Attribute,
    status: Status,
    races: Races,
    class: Class,
}

impl Player {
    fn init(id: u32, name: &'static str, races: Races, class: Class) -> Self {
        Player {
            id: id,
            name: name,
            attribute: Attribute::empty(),
            status: Status::empty(),
            races: races,
            class: class,
        }
    }
    pub fn empty() -> Self {
        Self::init(0, "Roxy", Races::Elves, Class::Mage)
    }
    pub fn get_health_current(&self) -> i32 {
        self.status.health.current
    }
    pub fn get_health_max(&self) -> i32 {
        self.status.health.max
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_player_aesprite);
        app.add_systems(Update, player_movement);
        app.add_systems(Update, update_player_health_bar);
    }
}

fn load_player_aesprite(mut commands: Commands, asset_server: Res<AssetServer>) {
    // sprite animation
    let player_id = commands
        .spawn((
            Sprite::default(),
            AseAnimation {
                aseprite: asset_server.load("player.aseprite"),
                animation: Animation::default().with_tag("down"),
            },
            Player::empty(),
        ))
        .id();
    spawn_health_bar(&mut commands, player_id, COLOR_HP_PLAYER, 10.0);
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    mut animation_assets_query: Query<&mut AseAnimation, With<Player>>,
) {
    let Ok((mut transform, mut player)) = query.single_mut() else {
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
        transform.translation += dir.extend(0.0) * 200.0 * time.delta_secs();

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
    }
}

const COLOR_HP_PLAYER: Color = Color::srgb(0.2, 0.9, 0.2);
const COLOR_HP_BG: Color = Color::srgb(0.2, 0.2, 0.2);

#[derive(Component)]
struct HealthBar {
    owner: Entity,
    offset_y: f32,
}

fn spawn_health_bar(commands: &mut Commands, owner: Entity, color: Color, offset_y: f32) {
    commands
        .spawn((
            Sprite::from_color(COLOR_HP_BG, Vec2::new(52.0, 8.0)),
            Transform::from_xyz(0.0, 0.0, 100.0),
            HealthBar { owner, offset_y },
            // Visibility::Hidden,
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_color(color, Vec2::new(50.0, 6.0)),
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        });
}

fn update_player_health_bar(
    mut commands: Commands,
    health_query: Query<(&Player, &Transform)>,
    mut bar_query: Query<(Entity, &HealthBar, &mut Transform, &mut Visibility), Without<Player>>,
    mut child_sprite_query: Query<&mut Sprite, Without<HealthBar>>,
) {
    for (bar_entity, bar, mut bar_transform, mut visibility) in bar_query.iter_mut() {
        let Ok((player, owner_transform)) = health_query.get(bar.owner) else {
            commands.entity(bar_entity).despawn();
            continue;
        };

        *visibility = Visibility::Visible;
        bar_transform.translation.x = owner_transform.translation.x;
        bar_transform.translation.y = owner_transform.translation.y + bar.offset_y;

        // 更新血条长度
        let ratio = (player.get_health_current() as f32 / player.get_health_max() as f32).max(0.0);
        let target_width = 50.0 * ratio;
    }
}
