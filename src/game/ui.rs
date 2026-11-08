use bevy::color::palettes::tailwind::BLUE_400;
use bevy::prelude::*;
use std::fmt::Write;

use crate::game::{Cauldron, Ingredient, Ingredients};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateUi;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (update_ingredients_list, update_cauldron_list).run_if(resource_changed::<Ingredients>),
        );
        app.insert_resource(GlobalUiDebugOptions {
            enabled: true,
            ..default()
        });
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
           justify_content: JustifyContent::SpaceBetween,
           align_items: AlignItems::Center,
        }
        Children [
            @ingredients_panel()
            --
            @cauldron_panel()
            --
            @product_panel()
        ]
    }
}

fn ingredients_panel() -> impl Scene {
    bsn! {
        #IngredientsPanel
        Node {
            border: px(4),
            width: rem(16),
            height: percent(90),
            flex_direction: FlexDirection::Column,
        }
        BorderColor::from(BLUE_400)
        BackgroundColor(Color::hsla(200.0, 0.4, 0.13, 1.0))
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
                        // Ingredient{name: "what".into()},
                        // Ingredient{name: "am".into()},
                        // Ingredient{name: "I".into()},
                        // Ingredient{name: "doing".into()},
                        // Ingredient{name: "here".into()},
                    ])
            }]
    ]
    }
}

fn cauldron_panel() -> impl Scene {
    bsn! {
        #CauldronPanel
        Node {
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            width: px(400),
            height: px(300),

            border: px(7),
            border_radius: BorderRadius::MAX,
        }
        BackgroundColor(Color::hsla(150.0, 0.5, 0.3, 0.6))
        BorderColor::from(Color::hsl(120.0, 0.4, 0.09))
        Children [
            Text("cauldron")
            TextLayout{ justify: Justify::Center }
            BorderColor{ bottom: Color::BLACK}
            --
            #CauldronList

        ]
    }
}

fn product_panel() -> impl Scene {
    bsn! {
        #ProductPanel
        Node {
            width: rem(16),
            height: rem(30)
        }
        Children [
            Text("product orders")
            TextLayout{ justify: Justify::Center }
            BorderColor{ bottom: Color::BLACK}
            --
            #ProductsList
        ]
    }
}

fn ingredients_list(ingredients: &[Ingredient]) -> Vec<Box<dyn Scene>> {
    ingredients
        .iter()
        .enumerate()
        .map(|(n, i)| Box::new(ingredient_word(n + 1, i)) as _)
        .collect()
}

fn cauldron_list(i9s: &[Ingredient]) -> impl Scene {
    let node = bsn! { Node {
        height: percent(70),
        align_items: AlignItems::Center, // todo: doesn't center
        justify_content: JustifyContent::Center,
    }};
    if i9s.is_empty() {
        return bsn! {  @node Children[@centered_word("...".to_string())]};
    }

    let mut s = i9s.join(" + ");

    let _ = write!(&mut s, "= ...");
    bsn! {
        @node
        Children[ @centered_word(s) ]
    }
}

fn word(text: impl Into<String>) -> impl Scene {
    bsn! {
        Node {
            margin: UiRect {
                left: px(7),
                top: em(0.6)
            },
        }
        Text(text)
        TextShadow {
            offset: vec2(0.5, 0.5)
        }
    }
}

fn centered_word(text: impl Into<String>) -> impl Scene {
    bsn! {
        @word(text)
        TextLayout {
            justify: Justify::Center
        }
    }
}

fn ingredient_word(n: usize, i: &Ingredient) -> impl Scene {
    bsn! {
        @word(format!("{}. {}", n, i.name))
    }
}

fn update_ingredients_list(
    mut cmds: Commands,
    ingredients: Res<Ingredients>,
    list_query: Query<(Entity, &Name)>,
) {
    let Some((e, _n)) = list_query
        .iter()
        .find(|(_e, n)| n == &&Name::new("IngredientsList"))
    else {
        return;
    };
    let mut entity = cmds.entity(e);
    entity.despawn_children();
    entity.queue_spawn_related_scenes::<Children>(ingredients_list(&ingredients.0));
    // let Some((_e, _n, c)) = list_query
    //     .iter()
    //     .find(|(_e, n, c)| n == &&Name::new("IngredientsList"))
    // else {
    //     return;
    // };
    //
    // let i = &ingredients.0[*picked_ingredient];
    // // I assume that children are ordered
    // let Some(entity) = c.get(*picked_ingredient) else {
    //     debug!("tried to pick an ingredient which doesn't exist: {picked_ingredient}");
    //     return;
    // };
    //
    // // todo: insert new ingredient from the pool here instead
    // cmds.entity(*entity)
    //     .apply_scene(ingredient_word(picked_ingredient + 1, i));
}

fn update_cauldron_list(
    mut cmds: Commands,
    cauldron: Res<Cauldron>,
    list_query: Query<(Entity, &Name)>,
) {
    let Some((e, _n)) = list_query
        .iter()
        .find(|(_e, n)| n == &&Name::new("CauldronList"))
    else {
        return;
    };
    let mut entity = cmds.entity(e);

    entity.apply_scene(cauldron_list(&cauldron.content));
}
