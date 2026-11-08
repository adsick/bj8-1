use bevy::{input::keyboard::{Key, KeyboardInput}, prelude::*};

use crate::Ingredients;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum Mode {
    #[default]
    PickingIngredients,
}

#[derive(Event)]
enum UserInput {
    Num(u8),
}

impl From<u8> for UserInput {
    fn from(value: u8) -> Self {
        Self::Num(value)
    }
}

#[derive(Event)]
enum UserAction {
    PickIngredient(u8)
}

struct Word(String);

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, handle_keyboard_input);

        app.add_observer(pick_ingredient.run_if(in_state(Mode::PickingIngredients)));

        app.init_resource::<Ingredients>();
        app.init_state::<Mode>();
    }
}

fn setup(cmds: Commands) {
    
}


fn handle_keyboard_input(mut cmds: Commands, mut keyboard: MessageReader<KeyboardInput>) {
    for message in keyboard.read() {
        let Some(ref t) = message.text else { continue };
        debug!("Input: {t}");

        let Ok(n) = t.parse::<u8>() else { continue };

        let input = UserInput::from(n);
        cmds.trigger(input);
    }
}


#[deprecated]
fn dispatch_user_input(input: On<UserInput>, mut cmds: Commands, mode: Res<State<Mode>>) {
    match **mode {
        Mode::PickingIngredients => {
            
        },
    }
}


fn pick_ingredient(input: On<UserInput>, mut i9s: ResMut<Ingredients>) {
    
}