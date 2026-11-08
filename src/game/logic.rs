use bevy::prelude::*;

use crate::game::Mode;
use crate::game::UserAction;
use crate::game::input::UserInput;
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