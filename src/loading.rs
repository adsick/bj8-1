use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::csv::{CsvAssetPlugin, LoadedCsv};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

// todo: https://github.com/NiklasEi/bevy_common_assets/blob/main/examples/csv.rs

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CsvAssetPlugin::<WordWithStatistics>::new(&[".csv"]));
        
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                // .load_collection::<AudioAssets>()
                // .load_collection::<TextureAssets>()
                // .load_collection::<Words>(),
        );
        // app.init_asset::<WordWithStatistics>();
        app.add_systems(Update, inspect_loaded);
        app.add_systems(Startup, load_words);
    }
}

#[derive(Resource)]
pub struct WordsHandle(Handle<LoadedCsv<WordWithStatistics>>);

fn load_words(mut cmds: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load::<LoadedCsv<WordWithStatistics>>("word-freq-top5000.csv");
    cmds.insert_resource(WordsHandle(handle));
}
fn inspect_loaded(words: Option<Res<Words>>) {
    
    dbg!(words);
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

#[derive(AssetCollection, Resource, Debug)]
pub struct Words {
    #[asset(path = "word-freq-top5000.csv")]
    pub words: Handle<LoadedCsv<WordWithStatistics>>
}

#[derive(serde::Deserialize, Asset, TypePath, Debug)]
pub struct WordWithStatistics {
    rank: String,
    word: String,
    part_of_speech: String,
    frequency: String,
    dispersion: String,
}

#[derive(Asset, TypePath, Debug)]
pub struct Word(String);