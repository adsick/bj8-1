mod input;
mod ui;

use bevy::{prelude::*};
use crate::{Ingredients, game::input::UserInput};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum Mode {
    #[default]
    PickingIngredients,
}

struct Word(String);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ui::GameUiPlugin, input::InputPlugin));
        
        app.add_systems(Startup, setup);

        app.add_observer(pick_ingredient.run_if(in_state(Mode::PickingIngredients)));

        app.init_resource::<Ingredients>();
        app.init_state::<Mode>();
    }
}

fn setup(cmds: Commands) {
    
}



fn pick_ingredient(input: On<UserInput>, mut i9s: ResMut<Ingredients>) {
    
}