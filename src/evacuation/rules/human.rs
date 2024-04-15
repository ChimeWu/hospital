
use bevy::ecs::event;
use bevy::ecs::query;
use bevy::prelude::*;
use bevy::transform::commands;

use crate::evacuation::things::human::*;
use crate::evacuation::things::smoke::*;
use crate::components::painting::*;
use crate::systems::paper;

use super::smoke;

pub fn init_building_pass(
    mut building_pass: ResMut<BuildingPass>,
    map_album: Res<MapAlbum>,
) {
    for (marker, map) in map_album.maps.iter() {
        let mut storey_pass = StoreyPass::new(map.tiles.len(), map.tiles[0].len());
        storey_pass.init_from_map(map);
        building_pass.maps.insert(marker.clone(), storey_pass);
    }
}

pub fn init_the_crowd(
    mut the_crowd: ResMut<TheCrowd>,
    map_album: Res<MapAlbum>,
    paper_album: Res<PaperAlbum>,
){
    for (marker, pass) in map_album.maps.iter(){
        the_crowd.extend_from_pass(pass, marker.clone());
    }
    let paper = &paper_album.papers[0];
    the_crowd.element_size = paper.element_size;
    the_crowd.board_width = paper.width;
    the_crowd.board_height = paper.height;
}

pub fn init_crowd_target_and_path(
    mut the_crowd: ResMut<TheCrowd>,
    building_pass: Res<BuildingPass>,
){
    let size = the_crowd.element_size;
    let width = the_crowd.board_width;
    let height = the_crowd.board_height;
    for human in the_crowd.humans.iter_mut(){
        let pass = &building_pass.maps[&human.storey];
        human.find_my_target(pass);
        human.find_my_path(pass);
        human.next_tile = *human.my_path.front().unwrap();
        human.clc_position(size, width, height);
        human.clc_next_position(size, width, height);
        human.change_my_direction();
    }
    let men10 = the_crowd.humans[10].clone();
}

pub fn spawn_people(
    mut commands: Commands,
    the_crowd: Res<TheCrowd>,
    query: Query<(Entity, &PaperMarker)>,
    asset_server: Res<AssetServer>,
){
    for (entity, papermarker) in query.iter(){
        let element_size = the_crowd.element_size;
        let board_width = the_crowd.board_width;
        let board_height = the_crowd.board_height;
        let path = the_crowd.texture_path.clone();

        commands.entity(entity).with_children(|builder|{
            for human in the_crowd.humans.iter(){
                if human.storey != papermarker.map{
                    continue;
                }
                builder.spawn((
                    SpriteBundle{
                        texture: asset_server.load(&path),
                        sprite: Sprite{
                            custom_size: Some(Vec2::new(0.8*element_size, 0.8*element_size)),
                            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                            ..Default::default()
                        },
                        visibility: Visibility::Inherited,
                        transform: Transform{
                            translation: Vec3::new(
                                paper::get_x(element_size, human.now_tile.1, board_width),
                                paper::get_y(element_size, human.now_tile.0, board_height),
                                12.0,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    HumanMarker::from_human(human),
                ));
            }
        });
    }
}



pub fn people_run(
    mut query: Query<(&mut Transform, &HumanMarker)>,
    smoke: Res<BuildingSmoke>,
    mut the_crowd: ResMut<TheCrowd>,
    time: Res<Time>,
    mut event1: EventWriter<Dead>,
    mut event2: EventWriter<Evacuated>,
    mut event3: EventWriter<ChangeStorey>,
){
    let size = the_crowd.element_size;
    let width = the_crowd.board_width;
    let height = the_crowd.board_height;
    for (mut transform, human) in query.iter_mut(){
        let id = human.id;
        let human = &mut the_crowd.humans[id];
        let storey_smoke = &smoke.maps[&human.storey];

        human.smoke_damage(storey_smoke,time.delta_seconds());


        if human.hp <= 0.0{
            human.is_dead = true;
            event1.send(Dead{id: id});
            continue;
        }


        if transform.translation == human.next_position{
            if let Some(p) = human.my_path.pop_front(){
                human.now_tile = p;
                if human.now_tile == human.target_tile{
                    let f1 = MapMarker{name: "f1".to_string()};
                    let f2 = MapMarker{name: "f2".to_string()};
                    if human.is_evacuated == false{
                        if human.storey == f1{
                            event2.send(Evacuated{id: id});
                            human.is_evacuated = true;
                        }else if human.storey == f2{
                            event3.send(ChangeStorey{id: id});
                        }
                    }
                }
            }

            if let Some(next_tile) = human.my_path.front(){;
                human.next_tile = *next_tile;
                human.position = human.next_position;
                human.clc_next_position(size, width, height);
                human.change_my_direction();
            }
        }

        human.change_my_speed();
        let newpos = transform.translation + human.direction *human.speed * time.delta_seconds();
        let dis0 = transform.translation.distance(newpos);
        let dis1 = transform.translation.distance(human.next_position);
        if dis0 >= dis1{
            transform.translation = human.next_position;
        }else{
            transform.translation = newpos;
        }
        
    }
}

pub fn dead(
    mut events: EventReader<Dead>,
    mut query: Query<(&mut Sprite, &HumanMarker)>,
){
    for event in events.read(){
        let id = event.id;
        for (mut sprite, human) in query.iter_mut(){
            if human.id == id{
                sprite.color.set_b(0.0);
                sprite.color.set_g(0.0);
            }
        }

    }
}


pub fn to_get_safe(
    mut the_crowd: ResMut<TheCrowd>,
    map_album: Res<MapAlbum>,
    mut events: EventReader<Evacuated>,
){
    for event in events.read(){
        let id = event.id;
        let size = the_crowd.element_size;
        let width = the_crowd.board_width;
        let height = the_crowd.board_height;
        let mut human = &mut the_crowd.humans[id];
        let storey = &map_album.maps[&human.storey];
        human.find_my_safe_place(storey);
        println!("safe place:{:?}",human.now_tile);
        human.find_my_safe_path(storey);
        println!("safe path:{:?}",human.my_path);
        human.next_tile = *human.my_path.front().unwrap();
        human.clc_position(size, width, height);
        human.clc_next_position(size, width, height);
        human.change_my_direction();
    }
}

pub fn change_storey(
    mut the_crowd: ResMut<TheCrowd>,
    building_pass: Res<BuildingPass>,
    mut events: EventReader<ChangeStorey>,
    mut commands: Commands,
    query: Query<(Entity,&HumanMarker)>,
    the_papers: Query<(Entity, &PaperMarker)>,
    asset_server: Res<AssetServer>,
){
    for event in events.read(){
        let id = event.id;
        let element_size = the_crowd.element_size;
        let board_width = the_crowd.board_width;
        let board_height = the_crowd.board_height;
        let path = the_crowd.texture_path.clone();
        let mut human = &mut the_crowd.humans[id];
        human.storey = MapMarker{name: "f1".to_string()};
        for (entity, humanmarker) in query.iter(){
            if humanmarker.id == id{
                commands.entity(entity).despawn_recursive();
            }
        }

        for (entity, papermarker) in the_papers.iter(){
            if papermarker.map != human.storey{
                continue;
            }    
            commands.entity(entity).with_children(|builder|{
                builder.spawn((
                    SpriteBundle{
                        texture: asset_server.load(&path),
                        sprite: Sprite{
                            custom_size: Some(Vec2::new(0.8*element_size, 0.8*element_size)),
                            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                            ..Default::default()
                        },
                        visibility: Visibility::Inherited,
                        transform: Transform{
                            translation: Vec3::new(
                                paper::get_x(element_size, human.now_tile.1, board_width),
                                paper::get_y(element_size, human.now_tile.0, board_height),
                                12.0,
                            ),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    HumanMarker::from_human(human),
                ));
            });

            let storey = &building_pass.maps[&human.storey];
            human.find_my_target(storey);
            human.find_my_path(storey);
            human.next_tile = *human.my_path.front().unwrap();
            human.clc_position(element_size, board_width, board_height);
            human.clc_next_position(element_size, board_width, board_height);
            human.change_my_direction();
        }
    }
}


pub fn printman10(
    the_crowd: Res<TheCrowd>,
    query: Query<(&Transform, &HumanMarker)>,
){
    for (transform, human) in query.iter(){
        if human.id == 10{
            print!("man10:{:?}",transform.translation);
            println!("man10:{:?}",the_crowd.humans[10].next_position);
            println!("{:?}  {:?}", the_crowd.humans[10].now_tile, the_crowd.humans[10].next_tile);
            println!("{:?}",the_crowd.humans[10].direction);
        }
    }
}