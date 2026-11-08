use bevy::color::palettes::basic::{GREEN, RED};
use bevy::color::palettes::tailwind::BLUE_400;
use bevy::prelude::*;

use crate::game::Ingredient;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.insert_resource(GlobalUiDebugOptions { enabled: true, ..default()});
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
            width: rem(13),
            height: percent(90),
            flex_direction: FlexDirection::Column,
        }
        BorderColor::from(BLUE_400)
        BackgroundColor(Color::hsl(235.0, 0.4, 0.13))
        Children [
            Node {
                height: rem(2),
            }
            Text("ingredients")
            BorderColor{ bottom: Color::BLACK}
            --
            Node {
                justify_content: JustifyContent::Start, // todo: doesn't do anything?
                align_content: AlignContent::Start, // same
                justify_items: JustifyItems::Start,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                // 
            }
            Children [{ ingredients_list(&[Ingredient{name: "what".into()}, Ingredient{name: "am".into()}, Ingredient{name: "doing".into()}]) }]
    ]
    }
}


fn ingredients_list(i9s: &[Ingredient]) -> Vec<Box<dyn Scene>> {
    i9s.iter().enumerate().map(|(n, i)| Box::new(word(format!("{}. {}", n + 1, &i.name))) as _).collect()
}

fn word(text: impl Into<String>) -> impl Scene {
    bsn! {
        Node {
            margin: em(3)
        }
        Text(text)
        TextShadow {
            offset: vec2(0.5, 0.5)
        }
        
    }
}
