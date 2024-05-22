use bevy::{ecs::bundle, prelude::*};

#[derive(Component, Debug, Clone)]
pub struct ButtonType(ObjectType);

#[derive(Resource, Debug)]
pub struct ObjectSelected(pub ObjectType);

use crate::worldtile::ObjectType;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ObjectSelected(ObjectType::Empty))
            .add_systems(Startup, setup)
            .add_systems(Update, match_btn_sel);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(NodeBundle{
        style:Style{display:Display::Flex,width:Val::Percent(100.),height:Val::Percent(100.),..default()},
        ..default()
    }).with_children(|p|{
        p.spawn(NodeBundle{
            style:Style{width:Val::Percent(85.),height:Val::Percent(100.),..default()},
            ..default()
        });
    }).with_children(|p|{
        p.spawn(NodeBundle{
            style:Style{
                display:Display::Flex,
                flex_direction:FlexDirection::Column,
                justify_content:JustifyContent::Center,
                width:Val::Px(240.),
                height:Val::Percent(100.),
                ..default()
            },
            background_color:BackgroundColor(Color::Rgba{red:0.1,green:0.1,blue:0.1,alpha:0.4}),
            ..default()
        }).with_children(|s|{
            s.spawn(NodeBundle{
                style:Style{display:Display::Flex,flex_wrap:FlexWrap::Wrap,justify_content:JustifyContent::Center,width:Val::Px(220.),height:Val::Px(450.),margin:UiRect::left(Val::Px(5.)),..default()},
                background_color:BackgroundColor(Color::Rgba{red:0.1,green:0.8,blue:0.1,alpha:0.}),
                ..default()
            }).with_children(|b|{
                b.spawn(inventory_button(Color::Rgba { red: 0.4, green: 0.4, blue: 0.8, alpha: 0.5 })).insert(ButtonType(ObjectType::Potato));
            }).with_children(|b|{
                b.spawn(inventory_button(Color::Rgba { red: 0.4, green: 0.4, blue: 0.8, alpha: 0.5 })).insert(ButtonType(ObjectType::TurretA));
            });
        });
    });
}

fn inventory_button(color: Color) -> ButtonBundle {
    ButtonBundle{
        style: Style{
            width: Val::Px(200.),
            height: Val::Px(200.),
            margin: UiRect::new(Val::Px(0.),Val::Px(0.),Val::Px(10.),Val::Px(0.)),
            ..default()
        },
        background_color: BackgroundColor(color),
        ..default()
    }
}

fn match_btn_sel(
    mut btn_int: Query<(&Interaction, &mut BackgroundColor, &ButtonType), With<ButtonType>>,
    mut obj_selected: ResMut<ObjectSelected>
) {
    for mut inter in btn_int.iter_mut() {
        match inter.0 {
            Interaction::Pressed => {
                *inter.1 = BackgroundColor(Color::Rgba { red: 0.4, green: 0.4, blue: 0.8, alpha: 1. });
                obj_selected.0 = inter.2.0.clone();
            },
            Interaction::Hovered => {},
            Interaction::None => {*inter.1 = BackgroundColor(Color::Rgba { red: 0.4, green: 0.4, blue: 0.8, alpha: 0.4  });},
        }
    }
}