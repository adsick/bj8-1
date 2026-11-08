
use std::collections::HashMap;

use bevy::prelude::*;

use crate::game::Cauldron;
use crate::game::Mode;
use crate::game::UserAction;
use crate::game::input::UserInput;
use crate::game::Product;
use crate::game::Ingredient;
use crate::game::Ingredients;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(pick_ingredient.run_if(in_state(Mode::PickingIngredients)));
    }
}

fn pick_ingredient(input: On<UserInput>, mut cmds: Commands, i9s: Res<Ingredients>) {
    let i9s = i9s.into_inner();
    let total_ingredients = i9s.0.len();
    let UserInput::Num(n @ 1..=9) = input.event() else { return };
    let n = *n as usize - 1;

    if n >= total_ingredients { return };

    cmds.trigger(UserAction::PickIngredient(n));
}

fn get_ingredient() -> Ingredient {
   Ingredient { name: "todo".into() }
}

fn put_to_cauldron(action: On<UserAction>, cauldron: ResMut<Cauldron>) {
    // ?
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
