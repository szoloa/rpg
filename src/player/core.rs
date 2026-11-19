use crate::combat::{AttackCooldown, Health};
use crate::player::bag::*;
use avian2d::prelude::*;
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
    fn base() -> Self {
        Attribute {
            level: 1,
            max_hp: 100,
            max_mp: 20,
            atk: 20,
            def: 5,
            agk: 5,
            mag: 8,
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
    facing: PlayerFaceDirection,
    animationstate: AnimationState,
    mp: i32,
    exp: u32,
}

impl Status {
    fn empty() -> Self {
        Status {
            facing: PlayerFaceDirection::DOWN,
            animationstate: AnimationState::Idle,
            mp: 20,
            exp: 0,
        }
    }
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
            attribute: Attribute::base(),
            status: Status::empty(),
            races: races,
            class: class,
        }
    }
    pub fn empty() -> Self {
        Self::init(0, "Roxy", Races::Elves, Class::Mage)
    }
    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> &'static str {
        self.name
    }
    pub fn level(&self) -> i32 {
        self.attribute.level
    }
    pub fn max_hp(&self) -> i32 {
        self.attribute.max_hp
    }
    pub fn max_mp(&self) -> i32 {
        self.attribute.max_mp
    }
    pub fn atk(&self) -> i32 {
        self.attribute.atk
    }
    pub fn def(&self) -> i32 {
        self.attribute.def
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
    let player = Player::empty();
    let max_hp = player.max_hp();
    let player_id = commands
        .spawn((
            Sprite::default(),
            AseAnimation {
                aseprite: asset_server.load("player.aseprite"),
                animation: Animation::default().with_tag("down"),
            },
            player,
            Health::new(max_hp),
            AttackCooldown::new(0.4),
            ContainerComponent::<CommonItem>::new(16),
            RigidBody::Dynamic,
            Collider::rectangle(31.0, 56.0),
            LockedAxes::ROTATION_LOCKED,
        ))
        .id();
    spawn_health_bar(&mut commands, player_id, COLOR_HP_PLAYER, 40.0);
}

fn player_movement(
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

const COLOR_HP_PLAYER: Color = Color::srgb(0.2, 0.9, 0.2);
const COLOR_HP_BG: Color = Color::srgb(0.2, 0.2, 0.2);
const HP_BAR_WIDTH: f32 = 50.0;
const HP_BAR_HEIGHT: f32 = 6.0;

#[derive(Component)]
struct HealthBar {
    owner: Entity,
    offset_y: f32,
    fill: Entity,
}

fn spawn_health_bar(commands: &mut Commands, owner: Entity, color: Color, offset_y: f32) -> Entity {
    let bar = commands
        .spawn((
            Sprite::from_color(COLOR_HP_BG, Vec2::new(52.0, 8.0)),
            Transform::from_xyz(0.0, 0.0, 100.0),
            Visibility::Visible,
        ))
        .id();

    let mut fill = Entity::PLACEHOLDER;
    commands.entity(bar).with_children(|parent| {
        fill = parent
            .spawn((
                Sprite::from_color(color, Vec2::new(HP_BAR_WIDTH, HP_BAR_HEIGHT)),
                Transform::from_xyz(0.0, 0.0, 0.1),
            ))
            .id();
    });

    commands.entity(bar).insert(HealthBar {
        owner,
        offset_y,
        fill,
    });
    bar
}

fn update_player_health_bar(
    mut commands: Commands,
    health_query: Query<(&Health, &Transform), With<Player>>,
    mut bar_query: Query<(Entity, &HealthBar, &mut Transform, &mut Visibility), Without<Player>>,
    mut bar_fill_query: Query<(&mut Sprite, &mut Transform), (Without<HealthBar>, Without<Player>)>,
) {
    for (bar_entity, bar, mut bar_transform, mut visibility) in bar_query.iter_mut() {
        let Ok((health, owner_transform)) = health_query.get(bar.owner) else {
            commands.entity(bar_entity).despawn();
            continue;
        };

        *visibility = Visibility::Visible;
        bar_transform.translation.x = owner_transform.translation.x;
        bar_transform.translation.y = owner_transform.translation.y + bar.offset_y;

        // 按血量比例缩放血条填充，并保持左端对齐
        let ratio = health.ratio();
        let width = HP_BAR_WIDTH * ratio;
        if let Ok((mut fill_sprite, mut fill_transform)) = bar_fill_query.get_mut(bar.fill) {
            fill_sprite.custom_size = Some(Vec2::new(width.max(0.0), HP_BAR_HEIGHT));
            fill_transform.translation.x = -(HP_BAR_WIDTH - width) * 0.5;
        }
    }
}
