use bevy::prelude::*;

use drawmymap2_lib::evacuation::things::NeededParameters;
use serde::{Deserialize, Serialize};
use serde_json;

use drawmymap2_lib::evacuation::things::*;

fn main() {
    App::new()
        .insert_resource(NeededParameters::default())
        .add_systems(Startup, get_needed_parameters)
        .run();
}
