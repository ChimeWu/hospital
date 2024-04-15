use crate::components::painting::*;
use crate::evacuation::things::{fire::*, smoke::*};
use crate::systems::paper::{get_x, get_y};
use bevy::prelude::*;

//初始化building_fire
pub fn init_building_fire(mut building_fire: ResMut<BuildingFire>, map_album: ResMut<MapAlbum>) {
    for (marker, map) in map_album.maps.iter() {
        let mut storey_fire = StoreyFire::new(map.tiles.len(), map.tiles[0].len());
        storey_fire.init_from_map(map);
        building_fire.maps.insert(marker.clone(), storey_fire);
    }
}

//初始随即点燃
pub fn init_random_fire(mut building_fire: ResMut<BuildingFire>) {
    for (_, storey_fire) in building_fire.maps.iter_mut() {
        storey_fire.get_p_fire(0.05);
    }
}

//生成火,初始均不可见
pub fn spawn_fire_sprite(
    mut commands: Commands,
    building_fire: ResMut<BuildingFire>,
    papers: Res<PaperAlbum>,
    query: Query<(Entity, &PaperMarker)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, papermarker) in query.iter() {
        let map = &building_fire.maps[&papermarker.map];
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
        let path = building_fire.texture_path.clone();

        commands.entity(entity).with_children(|builder| {
            for i in 0..row as usize {
                for j in 0..column as usize {
                    if map.map[i][j] == Fire::NeverBurn {
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
                        FireTile::new(i, j),
                        papermarker.map.clone(),
                    ));
                }
            }
        });
    }
}

//燃烧时发生的事情：引燃
pub fn ignition(mut building_fire: ResMut<BuildingFire>) {
    for (_, storey_fire) in building_fire.maps.iter_mut() {
        storey_fire.get_neighbours_fire_on();
    }
}

//更新火焰的可见性
pub fn update_fire_visibility(
    mut query: Query<(&FireTile, &mut Sprite, &MapMarker)>,
    building_fire: Res<BuildingFire>,
    map_album: ResMut<MapAlbum>,
) {
    let map_now_marker = map_album.now_map.clone();
    for (fire_tile, mut sprite, marker) in query.iter_mut() {
        if marker != &map_now_marker {
            continue;
        }
        let map = building_fire.maps.get(&map_now_marker).unwrap();
        match map.map[fire_tile.pos.0][fire_tile.pos.1] {
            Fire::On(_) => {
                sprite.color.set_a(1.0);
            }
            Fire::NeverBurn => {
                sprite.color.set_a(0.0);
            }
            _ => {}
        }
    }
}

//燃烧时发生的事情：计时器前进，产生烟
pub fn burning(
    mut building_fire: ResMut<BuildingFire>,
    mut building_smoke: ResMut<BuildingSmoke>,
    time: Res<Time>,
) {
    for (marker, storey_fire) in building_fire.maps.iter_mut() {
        let storey_smoke = building_smoke.maps.get_mut(marker).unwrap();
        for i in 0..storey_fire.map.len() {
            for j in 0..storey_fire.map[0].len() {
                let fire = &mut storey_fire.map[i][j];
                if let Fire::On(timer) = fire {
                    timer.burning_timer.tick(time.delta());
                    timer.smoking_timer.tick(time.delta());
                    if timer.smoking_timer.finished() {
                        storey_smoke.map[i][j].density_up(
                            5.0 * timer.burning_timer.elapsed_secs()
                                / timer.burning_timer.duration().as_secs_f32(),
                        );
                    }
                    if timer.burning_timer.finished() {
                        fire.fire_off();
                    }
                }
            }
        }
    }
}
