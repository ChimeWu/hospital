use bevy::prelude::*;

use crate::components::painting::*;
use crate::evacuation::things::smoke::*;

use crate::systems::paper::{get_x, get_y};

//初始化building_smoke
pub fn init_building_smoke(mut building_smoke: ResMut<BuildingSmoke>, map_album: ResMut<MapAlbum>) {
    for (marker, map) in map_album.maps.iter() {
        let mut storey_smoke = StoreySmoke::new(map.tiles.len(), map.tiles[0].len());
        storey_smoke.init_from_map(map);
        building_smoke.maps.insert(marker.clone(), storey_smoke);
    }
}

//生成烟雾
pub fn spawn_smoke_sprite(
    mut commands: Commands,
    building_smoke: ResMut<BuildingSmoke>,
    papers: Res<PaperAlbum>,
    query: Query<(Entity, &PaperMarker)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, papermarker) in query.iter() {
        let map = &building_smoke.maps[&papermarker.map];
        let now_paper = papers
            .papers
            .iter()
            .find(|paper| paper.map == papermarker.map)
            .unwrap();
        let row = now_paper.row;
        let column = now_paper.col;
        let element_size = now_paper.element_size;
        let board_width = now_paper.width;
        let board_height = now_paper.height;
        let path = building_smoke.texture_path.clone();

        commands.entity(entity).with_children(|builder| {
            for i in 0..row as usize {
                for j in 0..column as usize {
                    if let Smoke::InDiffusible = map.map[i][j] {
                        continue;
                    }
                    builder.spawn((
                        SpriteBundle {
                            texture: asset_server.load(&path),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(element_size, element_size)),
                                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                                ..default()
                            },
                            visibility: Visibility::Inherited,
                            transform: Transform {
                                translation: Vec3::new(
                                    get_x(element_size, j, board_width),
                                    get_y(element_size, i, board_height),
                                    11.0,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        SmokeTile::new(i, j),
                        papermarker.map.clone(),
                    ));
                }
            }
        });
    }
}

//更新烟雾sprite的透明度
pub fn update_smoke_sprite(
    mut query: Query<(&mut Sprite, &SmokeTile, &MapMarker)>,
    building_smoke: Res<BuildingSmoke>,
    map_album: Res<MapAlbum>,
) {
    let marker_now = map_album.now_map.clone();
    for (mut sprite, tile, marker) in query.iter_mut() {
        if marker != &marker_now {
            continue;
        }
        let storey_smoke = &building_smoke.maps[&marker_now];
        let smoke = &storey_smoke.map[tile.pos.0][tile.pos.1];
        let mut value = smoke.get_density();
        if value > 1.0 {
            value = 1.0;
        }
        sprite.color.set_a(value);
    }
}

//烟雾扩散
pub fn smoke_diffusion(mut building_smoke: ResMut<BuildingSmoke>) {
    for (_, storey_smoke) in building_smoke.maps.iter_mut() {
        storey_smoke.diffuse();
    }
}
