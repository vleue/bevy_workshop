use bevy::{color::palettes::tailwind, prelude::*};

use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), display_menu)
            .add_systems(Update, button_system.run_if(in_state(GameState::Menu)));
    }
}

fn display_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            StateScoped(GameState::Menu),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Bevy Workshop"),
                TextFont {
                    font_size: 80.0,
                    ..default()
                },
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            p.spawn((
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(tailwind::BLUE_700.into()),
            ))
            .with_child((
                Text::new("Play"),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = tailwind::VIOLET_500.into();
                border_color.0 = tailwind::RED_600.into();
                next.set(GameState::Game);
            }
            Interaction::Hovered => {
                *color = tailwind::BLUE_500.into();
                border_color.0 = tailwind::BLUE_700.into();
            }
            Interaction::None => {
                *color = tailwind::BLUE_700.into();
                border_color.0 = tailwind::BLUE_900.into();
            }
        }
    }
}
