use bevy::prelude::*;

use drawmymap2_lib::evacuation::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetUpPlugin)
        .add_plugins(PaperPlugin)
        .add_plugins(FirePlugin)
        .run();
}
