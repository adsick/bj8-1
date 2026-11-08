mod input;
mod logic;
mod ui;

use bevy::{prelude::*};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum Mode {
    #[default]
    PickingIngredients,
}

struct Word(String);


#[derive(Resource, Default)]
pub struct Ingredients(Vec<Ingredient>);

pub struct Ingredient {
    name: String,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ui::GameUiPlugin, input::InputPlugin, logic::LogicPlugin));
        app.add_systems(Startup, setup);

        app.init_resource::<Ingredients>();
        app.init_state::<Mode>();
    }
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d);
}

