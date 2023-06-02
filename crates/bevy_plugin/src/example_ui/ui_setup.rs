use crate::example_ui::assets::font_handle;
use bevy::prelude::*;

pub(crate) fn ui_setup_plugin(app: &mut App) {
    app.add_system(setup.on_startup());
}

fn setup(mut commands: Commands) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(DIALOG_WIDTH)),
                        min_size: Size::height(Val::Px(180.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    background_color: Color::WHITE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Dialog itself
                    parent.spawn((
                        TextBundle::from_sections(
                            [
                                TextSection { value: "Sara: ".to_string(), style: text_style::name() }, TextSection { value: "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor.Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor.".to_string(), style: text_style::option_text() },]
                        )
                        .with_style(style::dialog()),
                        Label,
                    ));
                }).with_children(|parent| {
                    // Options
                    parent.spawn((
                        TextBundle::from_sections(
                            [
                                TextSection { value: "1: ".to_string(), style: text_style::option_id() }, TextSection { value: "Do stuffs".to_string(), style: text_style::option_text() },]
                        )
                        .with_style(style::dialog()),
                        Label,
                    ));
                });
        });
}

const DIALOG_WIDTH: f32 = 800.0 * 0.7;
const TEXT_BORDER: f32 = 10.0;

mod style {
    use super::*;
    pub(crate) fn dialog() -> Style {
        Style {
            margin: UiRect::all(Val::Px(TEXT_BORDER)),
            max_size: Size {
                width: Val::Px(DIALOG_WIDTH - 2.0 * TEXT_BORDER),
                height: Val::Undefined,
            },
            ..default()
        }
    }
}

mod text_style {
    use super::*;
    pub(crate) fn standard() -> TextStyle {
        TextStyle {
            font: font_handle::MEDIUM.typed(),
            font_size: 20.0,
            color: Color::BLACK,
        }
    }
    pub(crate) fn name() -> TextStyle {
        TextStyle {
            font: font_handle::BOLD.typed(),
            ..standard()
        }
    }

    pub(crate) fn option_id() -> TextStyle {
        TextStyle {
            font: font_handle::BOLD.typed(),
            ..option_text()
        }
    }

    pub(crate) fn option_text() -> TextStyle {
        TextStyle {
            font_size: 18.0,
            ..standard()
        }
    }
}
