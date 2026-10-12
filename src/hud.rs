//! On-screen panels, styled for the bedroom era: beige home-computer plastic and dark ink.

use bevy::prelude::*;

use crate::{
    INK, MUTED, Screen,
    clock::GameClock,
    gameplay::{Identity, Toast},
};

/// Beige case plastic, and its darker edge.
const PANEL: Color = Color::srgb_u8(0xE8, 0xE0, 0xCC);
const PANEL_EDGE: Color = Color::srgb_u8(0xB8, 0xAE, 0x96);
const TOAST_SECS: f32 = 4.0;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Screen::Game), spawn_hud.after(crate::gameplay::spawn_people))
            .add_systems(Update, update_hud.run_if(in_state(Screen::Game)));
    }
}

#[derive(Component)]
struct ClockText;

#[derive(Component)]
struct ToastBox;

#[derive(Component)]
struct ToastText;

fn spawn_hud(mut commands: Commands, assets: Res<AssetServer>, identity: Res<Identity>) {
    let font: Handle<Font> = assets.load("fonts/SourceSans3-Semibold.ttf");
    let text = |size: f32, color: Color| (TextFont { font: font.clone().into(), font_size: FontSize::Px(size), ..default() }, TextColor(color));
    let panel = || (BackgroundColor(PANEL), BorderColor::all(PANEL_EDGE));

    // Top left: day and time, and who you are.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: px(16),
            top: px(16),
            padding: UiRect::axes(px(12), px(8)),
            border: UiRect::all(px(2)),
            flex_direction: FlexDirection::Column,
            row_gap: px(2),
            ..default()
        },
        panel(),
        DespawnOnExit(Screen::Game),
        children![
            (Text::new("Day 1 · 08:00"), text(20.0, INK), ClockText),
            (Text::new(format!("You woke up a {}.", identity.species.name())), text(14.0, MUTED)),
        ],
    ));

    // Bottom right: controls, quiet.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: px(16),
            bottom: px(16),
            padding: UiRect::axes(px(10), px(6)),
            border: UiRect::all(px(2)),
            ..default()
        },
        panel(),
        DespawnOnExit(Screen::Game),
        children![(Text::new("Click to walk or use · Q / E turn the street · T fast time"), text(14.0, MUTED))],
    ));

    // Bottom centre: what just happened. Hidden until there is something to say.
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(48),
            width: percent(100),
            justify_content: JustifyContent::Center,
            ..default()
        },
        DespawnOnExit(Screen::Game),
        children![(
            Node { padding: UiRect::axes(px(16), px(10)), border: UiRect::all(px(2)), max_width: px(560), ..default() },
            panel(),
            Visibility::Hidden,
            ToastBox,
            children![(Text::new(""), text(16.0, INK), ToastText)],
        )],
    ));
}

fn update_hud(
    time: Res<Time>,
    clock: Res<GameClock>,
    mut toast: ResMut<Toast>,
    mut clock_text: Single<&mut Text, (With<ClockText>, Without<ToastText>)>,
    mut toast_text: Single<&mut Text, (With<ToastText>, Without<ClockText>)>,
    mut toast_box: Single<&mut Visibility, With<ToastBox>>,
) {
    let fast = if clock.fast { " · fast" } else { "" };
    clock_text.0 = format!("Day {} · {}{}", clock.day(), clock.time_text(), fast);

    toast.age += time.delta_secs();
    if toast.is_changed() && toast.age < TOAST_SECS && !toast.text.is_empty() {
        toast_text.0 = toast.text.clone();
    }
    **toast_box = if toast.age < TOAST_SECS && !toast.text.is_empty() { Visibility::Inherited } else { Visibility::Hidden };
}
