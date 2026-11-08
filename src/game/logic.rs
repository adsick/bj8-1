use bevy::prelude::*;

use crate::game::Cauldron;
use crate::game::Mode;
use crate::game::input::UserInput;
use crate::game::Ingredient;
use crate::game::Ingredients;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(pick_ingredient.run_if(in_state(Mode::PickingIngredients)));
    }
}

fn pick_ingredient(input: On<UserInput>, i9s: ResMut<Ingredients>, mut cauldron: ResMut<Cauldron>) {
    let i9s = i9s.into_inner();
    let total = i9s.0.len();
    let UserInput::Num(n @ 1..=9) = input.event() else { return };
    let n = *n as usize - 1;

    if n >= total { return };

    let ingredient = std::mem::replace(&mut i9s.0[n], get_ingredient());
    (cauldron).content.push(ingredient);
}

fn get_ingredient() -> Ingredient {
   Ingredient { name: "todo".into() }
}