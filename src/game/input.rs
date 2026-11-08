use bevy::{
    input::{
        ButtonState,
        keyboard::{Key, KeyboardInput},
    },
    prelude::*,
};

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
    PickIngredient(u8),
}

fn handle_keyboard_input(mut cmds: Commands, kbd: Res<ButtonInput<Key>>) {
    for key in kbd.get_just_pressed() {
        let Key::Character(c) = key else { continue };
        debug!("Input: {c}");

        let Ok(n) = c.parse::<u8>() else { continue };

        let input = UserInput::from(n);
        cmds.trigger(input);
    }
}
