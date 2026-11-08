use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::Cauldron;
use crate::game::Ingredient;
use crate::game::Ingredients;
use crate::game::Mode;
use crate::game::PickIngredient;
use crate::game::Product;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(pick_ingredient.run_if(in_state(Mode::PickingIngredients)));
    }
}

fn pick_ingredient(
    input: On<PickIngredient>,
    mut ingredients: ResMut<Ingredients>,
    mut cauldron: ResMut<Cauldron>,
) {
    if input.0 >= ingredients.0.len() {
        return;
    }
    let removed_ingredient = ingredients.0.remove(input.0);
    cauldron.content.push(removed_ingredient);
}

fn get_ingredient() -> Ingredient {
    Ingredient {
        name: "todo".into(),
    }
}

fn combine_ingredients(i9s: &[Ingredient], product: Product) -> Result<u32, ()> {
    let mut count = HashMap::new();
    for i in i9s {
        count_letters(&i.name, &mut count);
    }
    let mut prod_count = HashMap::new();
    count_letters(&product.name, &mut prod_count);

    for (char, n) in prod_count {
        let Some(incoming) = count.get_mut(&char) else {
            return Err(());
        };

        if *incoming < n {
            return Err(());
        }
        *incoming -= n;
    }

    // todo: or return the count map?
    let extra: u32 = count.values().sum();
    Ok(extra)
}

fn count_letters(word: &str, count: &mut HashMap<char, u32>) {
    word.chars().for_each(|c| *count.entry(c).or_default() += 1);
}
