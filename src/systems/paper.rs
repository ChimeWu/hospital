use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::{MouseButton, MouseButtonInput, MouseWheel};
use bevy::input::ButtonState;
use bevy::prelude::*;

use std::collections::BTreeMap;

use crate::components::button::*;
use crate::components::painting::*;
use crate::components::tile::*;

pub fn new_map(
    mut map_album: ResMut<MapAlbum>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::NewMap {
            let inputs = pipe.input.pop_front().unwrap();
            let input_vec = inputs.split(" ").collect::<Vec<&str>>();
            let name = input_vec[0].to_string();
            let row = input_vec[1].parse::<usize>().unwrap();
            let col = input_vec[2].parse::<usize>().unwrap();
            let map = Map::new(row, col);
            map_album.maps.insert(MapMarker { name: name.clone() }, map);
            map_album.now_map = MapMarker { name };
            atomic_command.queue.pop_front();
        }
    }
}

pub fn new_map_album(
    mut map_album: ResMut<MapAlbum>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::NewMapAlbum {
            map_album.maps.clear();
            let inputs = pipe.input.pop_front().unwrap();
            let input_vec = inputs.split(" ").collect::<Vec<&str>>();
            let name = input_vec[0].to_string();
            let row = input_vec[1].parse::<usize>().unwrap();
            let col = input_vec[2].parse::<usize>().unwrap();
            let map = Map::new(row, col);
            let nums = input_vec[3].parse::<usize>().unwrap();
            for i in 0..nums {
                let name = format!("{}_{}", name, i);
                map_album
                    .maps
                    .insert(MapMarker { name: name.clone() }, map.clone());
            }
            atomic_command.queue.pop_front();
        }
    }
}

//生成新的paper纸张，将其加入到album中，将当前paper设置为新生成的paper
//生成paper依赖于当前的map，所以需要先生成map
//还依赖于当前的paperboard，所以需要先生成paperboard
pub fn new_paper(
    map_album: ResMut<MapAlbum>,
    mut paper_album: ResMut<PaperAlbum>,
    query: Query<&PaperBoard>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::NewPaper {
            let board = query.single();
            let map = &map_album.maps[&map_album.now_map];
            let row = map.tiles.len();
            let col = map.tiles[0].len();
            let height = 0.95 * board.height;
            let position = board.position;
            let element_size = height / row as f32;
            let width = col as f32 * element_size;
            let paper = Paper {
                map: map_album.now_map.clone(),
                row,
                col,
                width,
                height,
                element_size,
                position: Vec3::new(0.0, 0.0, 0.0),
                position_global: position,
                scale: 1.0,
            };
            paper_album.papers.push(paper);
            paper_album.now_paper = paper_album.papers.len() - 1;
            atomic_command.queue.pop_front();
        }
    }
}

pub fn new_paper_album(
    mut map_album: ResMut<MapAlbum>,
    mut paper_album: ResMut<PaperAlbum>,
    query: Query<&PaperBoard>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::NewPaperAlbum {
            paper_album.papers.clear();
            let mut bt_map = BTreeMap::new();
            for (marker, map) in map_album.maps.iter() {
                bt_map.insert(marker.clone(), map.clone());
            }
            let board = query.single();
            let height = 0.95 * board.height;
            let position = board.position;
            for map_marker in bt_map.keys() {
                let map = &map_album.maps[map_marker];
                let row = map.tiles.len();
                let col = map.tiles[0].len();
                let element_size = height / row as f32;
                let width = col as f32 * element_size;
                let paper = Paper {
                    map: map_marker.clone(),
                    row,
                    col,
                    width,
                    height,
                    element_size,
                    position: Vec3::new(0.0, 0.0, 0.0),
                    position_global: position,
                    scale: 1.0,
                };
                paper_album.papers.push(paper);
            }
            paper_album.now_paper = 0;
            map_album.now_map = paper_album.papers[0].map.clone();
            atomic_command.queue.pop_front();
        }
    }
}

pub fn new_paper_album_for_show(
    mut map_album: ResMut<MapAlbum>,
    mut paper_album: ResMut<PaperAlbum>,
    query: Query<&PaperBoard>,
) {
    paper_album.papers.clear();
    let mut bt_map = BTreeMap::new();
    for (marker, map) in map_album.maps.iter() {
        bt_map.insert(marker.clone(), map.clone());
    }
    let board = query.single();
    let height = 0.95 * board.height;
    let position = board.position;
    for map_marker in bt_map.keys() {
        let map = &map_album.maps[map_marker];
        let row = map.tiles.len();
        let col = map.tiles[0].len();
        let element_size = height / row as f32;
        let width = col as f32 * element_size;
        let paper = Paper {
            map: map_marker.clone(),
            row,
            col,
            width,
            height,
            element_size,
            position: Vec3::new(0.0, 0.0, 0.0),
            position_global: position,
            scale: 1.0,
        };
        paper_album.papers.push(paper);
    }
    paper_album.now_paper = 0;
    map_album.now_map = paper_album.papers[0].map.clone();
}

//删除the paper纸张实体
pub fn delete_paper(
    mut commands: Commands,
    query: Query<Entity, With<ThePaper>>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::DeSpawnThePaper {
            for entity in query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            atomic_command.queue.pop_front();
        }
    }
}

//生成the paper纸张实体
pub fn spawn_paper(
    mut commands: Commands,
    paper: Res<PaperAlbum>,
    query: Query<Entity, With<PaperBoard>>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::SpawnThePaper {
            let board = query.single();
            let paper = &paper.papers[paper.now_paper];
            commands.entity(board).with_children(|builder| {
                builder.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(paper.width, paper.height)),
                            ..default()
                        },
                        transform: Transform {
                            translation: paper.position,
                            ..default()
                        },
                        ..default()
                    },
                    ThePaper,
                ));
            });
            atomic_command.queue.pop_front();
        }
    }
}

//生成the paper纸张实体,多个同时存在，但只显示一个
pub fn spawn_papers(
    mut commands: Commands,
    paper: Res<PaperAlbum>,
    query: Query<Entity, With<PaperBoard>>,
) {
    let board = query.single();
    for paper in paper.papers.iter() {
        commands.entity(board).with_children(|builder| {
            builder.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(paper.width, paper.height)),
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    transform: Transform {
                        translation: paper.position,
                        ..default()
                    },
                    ..default()
                },
                PaperMarker::new(paper.map.clone()),
            ));
        });
    }
}

//生成地图对应的sprite实体
pub fn spawn_map(
    mut commands: Commands,
    map_album: Res<MapAlbum>,
    papers: Res<PaperAlbum>,
    query: Query<Entity, With<ThePaper>>,
    asset_server: Res<AssetServer>,
    set: Res<TilesSet>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::SpawnTheMap {
            let map = &map_album.maps[&map_album.now_map];
            let now_paper = &papers.papers[papers.now_paper];
            let row = now_paper.row;
            let column = now_paper.col;
            let element_size = now_paper.element_size;
            let board_width = now_paper.width;
            let board_height = now_paper.height;

            for the_paper in query.iter() {
                commands.entity(the_paper).with_children(|builder| {
                    for i in 0..row as usize {
                        for j in 0..column as usize {
                            let path = set.get_path(&map.tiles[i][j]);
                            builder.spawn((
                                SpriteBundle {
                                    texture: asset_server.load(path),
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::new(element_size, element_size)),
                                        ..default()
                                    },
                                    transform: Transform {
                                        translation: Vec3::new(
                                            get_x(element_size, j, board_width),
                                            get_y(element_size, i, board_height),
                                            10.0,
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TileMarker { i, j },
                            ));
                        }
                    }
                });
                atomic_command.queue.pop_front();
            }
        }
    }
}

pub fn spawn_building_png(
    mut commands: Commands,
    papers: Res<PaperAlbum>,
    query: Query<Entity, With<ThePaper>>,
    asset_server: Res<AssetServer>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::SpawnTheBuildingPng {
            let now_paper = &papers.papers[papers.now_paper];
            let _board_width = now_paper.width;
            let board_height = now_paper.height;

            for the_paper in query.iter() {
                commands.entity(the_paper).with_children(|builder| {
                    let path = "./building/1F.png".to_string();
                    builder.spawn(SpriteBundle {
                        texture: asset_server.load(path),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1.43 * board_height, board_height)),
                            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 12.0),
                            ..default()
                        },
                        ..default()
                    });
                });
                atomic_command.queue.pop_front();
            }
        }
    }
}

//生成地图实体，多个同时存在，但只显示一个,用于模拟逃生演示
pub fn spawn_maps(
    mut commands: Commands,
    map_album: Res<MapAlbum>,
    papers: Res<PaperAlbum>,
    query: Query<(Entity, &PaperMarker)>,
    asset_server: Res<AssetServer>,
    set: Res<TilesSet>,
) {
    for (entity, papermarker) in query.iter() {
        let map = &map_album.maps[&papermarker.map];
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
        commands.entity(entity).with_children(|builder| {
            for i in 0..row as usize {
                for j in 0..column as usize {
                    let path = set.get_path(&map.tiles[i][j]);
                    builder.spawn((
                        SpriteBundle {
                            texture: asset_server.load(path),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(element_size, element_size)),
                                ..default()
                            },
                            visibility: Visibility::Inherited,
                            transform: Transform {
                                translation: Vec3::new(
                                    get_x(element_size, j, board_width),
                                    get_y(element_size, i, board_height),
                                    10.0,
                                ),
                                ..default()
                            },
                            ..default()
                        },
                        TileMarker { i, j },
                    ));
                }
            }
        });
    }
}

pub fn get_x(element_size: f32, j: usize, width: f32) -> f32 {
    element_size / 2.0 + (j as f32 * element_size) - width / 2.0
}

pub fn get_y(element_size: f32, i: usize, height: f32) -> f32 {
    -element_size / 2.0 - (i as f32 * element_size) + height / 2.0
}

pub fn next_paper(
    mut paper_album: ResMut<PaperAlbum>,
    mut map_album: ResMut<MapAlbum>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::NextPage {
            paper_album.now_paper = (paper_album.now_paper + 1) % paper_album.papers.len();
            let now = paper_album.papers[paper_album.now_paper].map.clone();
            map_album.now_map = now;
            atomic_command.queue.pop_front();
        }
    }
}

pub fn last_paper(
    mut paper_album: ResMut<PaperAlbum>,
    mut map_album: ResMut<MapAlbum>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::PrePage {
            paper_album.now_paper =
                (paper_album.now_paper + paper_album.papers.len() - 1) % paper_album.papers.len();
            let now = paper_album.papers[paper_album.now_paper].map.clone();
            map_album.now_map = now;
            atomic_command.queue.pop_front();
        }
    }
}

//将应该显示的paper显示出来，将不应该显示的paper隐藏
pub fn show_the_right_paper(
    mut query: Query<(&PaperMarker, &mut Visibility)>,
    map_album: Res<MapAlbum>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::ShowTheRightPaper {
            for (marker, mut visibility) in query.iter_mut() {
                if marker.map == map_album.now_map {
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
            atomic_command.queue.pop_front();
        }
    }
}

pub fn clean_map(mut map_album: ResMut<MapAlbum>, mut atomic_command: ResMut<AtomicCommandQueue>) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::CleanMap {
            let now = map_album.now_map.clone();
            let map = map_album.maps.get_mut(&now).unwrap();
            map.clean();
            atomic_command.queue.pop_front();
        }
    }
}

pub fn delete_map(
    mut paper_album: ResMut<PaperAlbum>,
    mut map_album: ResMut<MapAlbum>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::DeleteMap {
            if paper_album.papers.len() == 1 {
                let now = map_album.now_map.clone();
                let map = map_album.maps.get_mut(&now).unwrap();
                map.clean();
            } else {
                let now = map_album.now_map.clone();
                map_album.maps.remove(&now);
                paper_album.papers.retain(|paper| paper.map != now);
                paper_album.now_paper = 0;
                map_album.now_map = paper_album.papers[0].map.clone();
            }
            atomic_command.queue.pop_front();
        }
    }
}

//利用paper中的信息，更新对应sprite实体的位置和大小
pub fn update_paper(paper: Res<PaperAlbum>, mut query: Query<(&ThePaper, &mut Transform)>) {
    if let Some(paper) = paper.papers.get(paper.now_paper) {
        for (_, mut transform) in query.iter_mut() {
            transform.translation = paper.position;
            transform.scale = Vec3::new(paper.scale, paper.scale, 0.0);
        }
    }
}

//for show版
pub fn update_papers(
    paper: Res<PaperAlbum>,
    mut query: Query<(&PaperMarker, &mut Transform)>,
    map_album: Res<MapAlbum>,
) {
    for (marker, mut transform) in query.iter_mut() {
        if marker.map == map_album.now_map {
            let paper = paper
                .papers
                .iter()
                .find(|paper| paper.map == marker.map)
                .unwrap();
            transform.translation = paper.position;
            transform.scale = Vec3::new(paper.scale, paper.scale, 0.0);
        }
    }
}

//缩放纸张
pub fn scale_paper(mut events: EventReader<MouseWheel>, mut album: ResMut<PaperAlbum>) {
    for event in events.read() {
        let index = album.now_paper;
        let paper = &mut album.papers[index];
        if event.y > 0.0 {
            paper.scale *= 1.1;
        } else {
            paper.scale *= 0.9;
        }
    }
}

//移动纸张
/*
pub fn move_paper_by_element_size(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut album: ResMut<PaperAlbum>,
) {
    let index = album.now_paper;
    let paper = &mut album.papers[index];
    let speed = 3.0;
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        paper.position.y += paper.element_size * speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        paper.position.y -= paper.element_size * speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        paper.position.x -= paper.element_size * speed;
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        paper.position.x += paper.element_size * speed;
    }
}*/

//移动纸张
pub fn move_paper(keyboard_input: Res<ButtonInput<KeyCode>>, mut album: ResMut<PaperAlbum>) {
    let index = album.now_paper;
    if let Some(paper) = album.papers.get_mut(index) {
        let speed = 1.0;
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            paper.position.y += paper.element_size * speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            paper.position.y -= paper.element_size * speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            paper.position.x -= paper.element_size * speed;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            paper.position.x += paper.element_size * speed;
        }
    }
}

//检测鼠标点击事件，并进行坐标转换，如果点击到了色块，写入色块转换消息
pub fn click_tile(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    window: Query<&Window>,
    _map_album: ResMut<MapAlbum>,
    paper_album: Res<PaperAlbum>,
    mut messages: ResMut<MapEditMessage>,
) {
    for event in mouse_button_input_events.read() {
        if event.state == ButtonState::Pressed {
            let window = window.single();
            let board = &paper_album.papers[paper_album.now_paper];
            let width = window.width();
            let height = window.height();
            let cursor_position = window.cursor_position().unwrap();
            let x = board.position.x + board.position_global.x
                - board.col as f32 * board.element_size * board.scale / 2.0;
            let y = board.position.y
                + board.position_global.y
                + board.row as f32 * board.element_size * board.scale / 2.0;
            let cursor_translation = Vec3::new(
                cursor_position.x - width / 2.0,
                -cursor_position.y + height / 2.0,
                0.0,
            );
            let x = (cursor_translation.x - x) / (board.scale * board.element_size);
            let y = (-cursor_translation.y + y) / (board.scale * board.element_size);

            if cursor_position.x >= width - 1.25 * height {
                let row = y as usize;
                let col = x as usize;
                if event.button == MouseButton::Left {
                    messages.change_point.push_back(ChangeMessage { row, col });
                }
                if event.button == MouseButton::Right {
                    messages.change_block.push_back(ChangeMessage { row, col });
                }
            }
        }
    }
}

//改变色块sprite材质
pub fn change_tile(
    mut map_album: ResMut<MapAlbum>,
    mut query: Query<(&TileMarker, &mut Handle<Image>)>,
    set: Res<TilesSet>,
    asset_server: Res<AssetServer>,
    mut messages: ResMut<MapEditMessage>,
) {
    if let Some(message) = messages.change_point.pop_front() {
        for (marker, mut handle) in query.iter_mut() {
            if marker.i == message.row && marker.j == message.col {
                let now = map_album.now_map.clone();
                let map = map_album.maps.get_mut(&now).unwrap();
                map.tiles[marker.i][marker.j] = set.now_type.clone();
                map.tiles[marker.i][marker.j] = set.now_type.clone();
                let path = &set.now_path;
                *handle = asset_server.load(path);
            }
        }
    }
}

//一次性改变一个矩形区域的色块，使用方法是鼠标右键点击两次，第一次左键点击确定左上角，第二次左键点击确定右下角，在同一行或列时，会生成直线
pub fn change_block(
    mut map_album: ResMut<MapAlbum>,
    mut query: Query<(&TileMarker, &mut Handle<Image>)>,
    set: Res<TilesSet>,
    asset_server: Res<AssetServer>,
    mut messages: ResMut<MapEditMessage>,
) {
    if messages.change_block.len() >= 2 {
        let message1 = messages.change_block.pop_front().unwrap();
        let message2 = messages.change_block.pop_front().unwrap();
        let row1 = message1.row;
        let col1 = message1.col;
        let row2 = message2.row;
        let col2 = message2.col;
        let now = map_album.now_map.clone();
        let map = map_album.maps.get_mut(&now).unwrap();
        for i in row1..=row2 {
            for j in col1..=col2 {
                map.tiles[i][j] = set.now_type.clone();
            }
        }
        for (marker, mut handle) in query.iter_mut() {
            if (marker.i >= row1 && marker.i <= row2) && (marker.j >= col1 && marker.j <= col2) {
                let path = &set.now_path;
                *handle = asset_server.load(path);
            }
        }
    }
}
