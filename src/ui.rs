use bevy::color::palettes::basic::{GREEN, RED};
use bevy::color::palettes::tailwind::BLUE_400;
use bevy::prelude::*;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
    cmds.spawn_scene(main_screen());
}

fn main_screen() -> impl Scene {
    bsn! {
        Node {
           width: percent(100),
           height: percent(100),

        }
        Children [
            @ingredients_panel()
        ]
    }
}

fn ingredients_panel() -> impl Scene {
    bsn! {
        #Ingredients
        Node {
            border: px(3),
            width: rem(15),
            height: percent(90),
            flex_direction: FlexDirection::Column,
        }
        BorderColor::from(BLUE_400)
        BackgroundColor(Color::hsl(235.0, 0.4, 0.1))
        Children [
            Node {
                height: rem(2),
            }
            Text("ingridients")
            BorderColor{ bottom: Color::BLACK}
            --
            Node {
                margin: px(5)
            }
            @word("test")
            TextColor(GREEN)
            --
            @word("more text")
            TextColor(RED)
    ]
    }
}
fn word(text: impl Into<String>) -> impl Scene {
    bsn! {
        Node {
            margin: px(5)
        }
        Text(text)
        TextShadow {
            offset: vec2(0.5, 0.5)
        }

    }
}
