use bevy::prelude::*;

use crate::components::tile::*;

pub fn click_palette_button(
    mut interaction_query: Query<(&Interaction, &TileType), (Changed<Interaction>, With<Button>)>,
    mut res: ResMut<TilesSet>,
) {
    for (interaction, tile) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            res.now_type = tile.clone();
            res.now_path = res.get_path(&res.now_type);
        }
    }
}
