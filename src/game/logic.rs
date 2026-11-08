use bevy::prelude::*;

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

fn pick_ingredient(input: On<UserInput>, mut i9s: ResMut<Ingredients>) {
    
}

fn get_ingredient() -> Ingredient {
   todo!() 
}