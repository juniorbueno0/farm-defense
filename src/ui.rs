use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
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
                justify_items: JustifyItems::Center,
                justify_content:JustifyContent::Center,
                width:Val::Px(200.),
                height:Val::Percent(100.),
                ..default()
            },
            background_color:BackgroundColor(Color::Rgba{red:0.1,green:0.1,blue:0.1,alpha:0.4}),
            ..default()
        }).with_children(|s|{
            s.spawn(NodeBundle{
                style:Style{display:Display::Flex,width:Val::Px(180.),height:Val::Px(450.),margin:UiRect{left:Val::Px(10.),..default()},..default()},
                background_color:BackgroundColor(Color::Rgba{red:0.6,green:0.2,blue:0.6,alpha:1.}),
                ..default()
            });
        });
    });
}