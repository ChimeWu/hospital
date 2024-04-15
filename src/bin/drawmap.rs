use bevy::prelude::*;

use drawmymap2_lib::systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SetUpPlugin)
        .add_plugins(PaperPlugin)
        .add_plugins(TerminalPlugin)
        .add_plugins(MapEditPlugin)
        .run();
}
