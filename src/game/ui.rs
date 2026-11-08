use bevy::color::palettes::basic::{GREEN, RED};
use bevy::color::palettes::tailwind::BLUE_400;
use bevy::prelude::*;

use crate::game::{Cauldron, Ingredient, Ingredients, UserAction};

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_observer(update_ingredients_list);
        app.add_systems(Update, (update_cauldron_list));
        app.insert_resource(GlobalUiDebugOptions { enabled: true, ..default()});
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn_scene(main_screen());
}

fn main_screen() -> impl Scene {
    bsn! {
        #MainScreen
        Node {
           width: percent(100),
           height: percent(100),
           flex_direction: FlexDirection::Row,
           justify_content: JustifyContent::Center,
           align_items: AlignItems::Center,

        }
        Children [
            @ingredients_panel()
            --
            @cauldron()
        ]
    }
}

fn ingredients_panel() -> impl Scene {
    bsn! {
        #IngredientsPanel
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
            TextLayout{ justify: Justify::Center } 
            BorderColor{ bottom: Color::BLACK}
            --
            #IngredientsList
            Node {
                flex_direction: FlexDirection::Column,
            }
            Children [{ 
                ingredients_list(
                    &[
                        Ingredient{name: "what".into()},
                        Ingredient{name: "am".into()},
                        Ingredient{name: "I".into()},
                        Ingredient{name: "doing".into()},
                        Ingredient{name: "here".into()},
                    ])
            }]
    ]
    }
}

fn cauldron() -> impl Scene {
    bsn! {
        #CauldronPanel
        Node {
            justify_content: JustifyContent::Start,
            align_content: AlignContent::Center,
            // justify_items: JustifyItems::Start,
            // align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            width: px(300),
            height: px(200)
        }
        Children [
            Text("cauldron")
            --
            #CauldronList
            Node
        ]
    }
}

fn ingredients_list(i9s: &[Ingredient]) -> Vec<Box<dyn Scene>> {
    i9s.iter().enumerate().map(|(n, i)| Box::new(ingredient_word(n + 1, i)) as _).collect()
}

fn cauldron_list(i9s: &[Ingredient]) -> Vec<Box<dyn Scene>> {
    i9s.iter().map(|i| Box::new(word(format!("{}", &i.name))) as _).collect()
}

fn word(text: impl Into<String>) -> impl Scene {
    bsn! {
        Node {
            margin: UiRect {
                left: px(7),
                top: em(0.6)
            },
        }
        // BackgroundColor(RED)
        Text(text)
        TextShadow {
            offset: vec2(0.5, 0.5)
        }
    }
}


fn ingredient_word(n: usize, i: &Ingredient) -> impl Scene {
    bsn! {
        @word(format!("{}. {}", n, i.name))
    }
}

fn update_ingredients_list(action: On<UserAction>, mut cmds: Commands, i9s: Res<Ingredients>, panel_query: Query<(Entity, &Name, &Children)>) {
    let UserAction::PickIngredient(picked_ingredient) = action.event() else { return };
    let Some((_e, _n, c)) = panel_query.iter().find(|(_e, n, c)| n == &&Name::new("IngredientsList")) else { return };

    let i = &i9s.0[*picked_ingredient];
    // I assume that children are ordered
    let Some(entity) = c.get(*picked_ingredient) else { debug!("tried to pick an ingredient which doesn't exist: {picked_ingredient}"); return; };


    // todo: insert new ingredient from the pool here instead
    cmds.entity(*entity).apply_scene(ingredient_word(picked_ingredient + 1, i));
    
}

fn update_cauldron_list(mut cmds: Commands, cauldron: Res<Cauldron>, panel_query: Query<(Entity, &Name)>) {
    if !cauldron.is_changed() {
        return
    }
    let Some((e, _n)) = panel_query.iter().find(|(_e, n)| n == &&Name::new("CauldronList")) else { return };
    let mut entity = cmds.entity(e);
    
    entity.despawn_children();
    entity.queue_spawn_related_scenes::<Children>(cauldron_list(&cauldron.content));
}

