use bevy::{input::keyboard::KeyboardInput, prelude::*};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_systems(Update, handle_keyboard_input);
    }
}

#[derive(Event)]
pub enum UserInput {
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

fn handle_keyboard_input(mut cmds: Commands, mut keyboard: MessageReader<KeyboardInput>) {
    for message in keyboard.read() {
        let Some(ref t) = message.text else { continue };
        debug!("Input: {t}");

        let Ok(n) = t.parse::<u8>() else { continue };

        let input = UserInput::from(n);
        cmds.trigger(input);
    }
}