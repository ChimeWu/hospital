use bevy::prelude::*;
use bevy::{render::camera::RenderTarget, window::WindowRef};

use crate::components::board::*;
use crate::components::button::*;
use crate::components::painting::*;
use crate::components::terminal::*;
use crate::components::tile::*;

use crate::evacuation::things::human::*;

//设置窗口的基本框架
pub fn set_window_frame(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.width();
    let height = window.height();
    commands.spawn(Camera2dBundle::default());

    //绘图板
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.8 * width, 0.8 * height)),
                color: Color::rgb(0.15, 0.15, 0.15),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.1 * width, 0.1 * height, 0.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: 0.8 * width,
            height: 0.8 * height,
            position: Vec3::new(0.1 * width, 0.1 * height, 10.0),
            board_type: BoardType::PaperBoard,
            color: Color::rgb(0.15, 0.15, 0.15),
        },
    ));

    //终端
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.8 * width, 0.16 * height)),
                color: Color::rgb(0.1, 0.1, 0.1),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.1 * width, -0.38 * height, 20.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: 0.8 * width,
            height: 0.16 * height,
            position: Vec3::new(0.1 * width, -0.38 * height, 20.0),
            board_type: BoardType::Terminal,
            color: Color::rgb(0.1, 0.1, 0.1),
        },
    ));

    //左栏（操作按钮）
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.05 * width, 0.96 * height)),
                color: Color::DARK_GRAY,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-0.475 * width, 0.02 * height, 20.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: 0.05 * width,
            height: 0.96 * height,
            position: Vec3::new(-0.475 * width, 0.02 * height, 20.0),
            board_type: BoardType::LeftSideBar,
            color: Color::DARK_GRAY,
        },
    ));

    //中栏（选色板）
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.15 * width, 0.96 * height)),
                color: Color::rgb(0.2, 0.2, 0.2),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-0.375 * width, 0.02 * height, 20.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: 0.15 * width,
            height: 0.96 * height,
            position: Vec3::new(-0.375 * width, 0.02 * height, 20.0),
            board_type: BoardType::PenPalette,
            color: Color::rgb(0.2, 0.2, 0.2),
        },
    ));

    //底栏
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, 0.04 * height)),
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -0.48 * height, 20.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: width,
            height: 0.04 * height,
            position: Vec3::new(0.0, -0.48 * height, 0.0),
            board_type: BoardType::ButtomBar,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));

    //左栏和中栏的node根节点
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(0.96 * height),
                flex_direction: FlexDirection::Row,
                margin: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        NodeRoot,
    ));
}

pub fn set_window_frame_for_show(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.width();
    let height = window.height();
    commands.spawn(Camera2dBundle::default());

    //绘图板
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.96 * width, 0.96 * height)),
                color: Color::rgb(0.15, 0.15, 0.15),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.02 * width, 0.02 * height, 0.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: 0.96 * width,
            height: 0.96 * height,
            position: Vec3::new(0.02 * width, 0.02 * height, 10.0),
            board_type: BoardType::PaperBoard,
            color: Color::rgb(0.15, 0.15, 0.15),
        },
    ));

    //左栏（操作按钮）
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(0.04 * width, 0.96 * height)),
                color: Color::DARK_GRAY,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-0.48 * width, 0.02 * height, 20.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: 0.04 * width,
            height: 0.96 * height,
            position: Vec3::new(-0.48 * width, 0.02 * height, 20.0),
            board_type: BoardType::LeftSideBar,
            color: Color::DARK_GRAY,
        },
    ));

    //底栏
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, 0.04 * height)),
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, -0.48 * height, 20.0),
                ..default()
            },
            ..default()
        },
        Board {
            width: width,
            height: 0.04 * height,
            position: Vec3::new(0.0, -0.48 * height, 0.0),
            board_type: BoardType::ButtomBar,
            color: Color::rgb(0.9, 0.9, 0.9),
        },
    ));

    //左栏的node根节点
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                flex_direction: FlexDirection::Column,
                margin: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        NodeRoot,
    ));
}

//设置操作按钮
pub fn set_leftsidebar(
    mut commands: Commands,
    boards: Query<&Board>,
    asset_server: Res<AssetServer>,
    root: Query<Entity, With<NodeRoot>>,
) {
    let root = root.single();
    for board in boards.iter() {
        if board.board_type == BoardType::LeftSideBar {
            let width = board.width;
            let height = board.height;
            commands.entity(root).with_children(|commands| {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(height),
                            width: Val::Px(width),
                            display: Display::Grid,
                            padding: UiRect::all(Val::Px(5.0)),
                            grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(15, 1.0),
                            row_gap: Val::Percent(2.0),
                            margin: UiRect {
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        for i in 0..9 as usize {
                            builder.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height: Val::Percent(100.0),
                                        aspect_ratio: Some(1.0),
                                        align_self: AlignSelf::Center,
                                        justify_self: JustifySelf::Center,
                                        ..default()
                                    },
                                    image: UiImage {
                                        texture: asset_server.load(MapEditButton::path_from_i(i)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                MapEditButton::from_i(i),
                            ));
                        }
                    });
            });
        }
    }
}

//用于演示而非画图的左侧按钮
pub fn set_leftsidebar_for_show(
    mut commands: Commands,
    boards: Query<&Board>,
    asset_server: Res<AssetServer>,
    root: Query<Entity, With<NodeRoot>>,
) {
    let root = root.single();
    for board in boards.iter() {
        if board.board_type == BoardType::LeftSideBar {
            let width = board.width;
            let height = board.height;
            commands.entity(root).with_children(|commands| {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(height),
                            width: Val::Px(width),
                            display: Display::Grid,
                            padding: UiRect::all(Val::Px(5.0)),
                            grid_template_columns: RepeatedGridTrack::flex(1, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(15, 1.0),
                            row_gap: Val::Percent(2.0),
                            margin: UiRect {
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|builder| {
                        let vec: Vec<usize> = vec![6, 7, 8];
                        for i in vec.into_iter() {
                            builder.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height: Val::Percent(100.0),
                                        aspect_ratio: Some(1.0),
                                        align_self: AlignSelf::Center,
                                        justify_self: JustifySelf::Center,
                                        ..default()
                                    },
                                    image: UiImage {
                                        texture: asset_server.load(MapEditButton::path_from_i(i)),
                                        ..default()
                                    },
                                    ..default()
                                },
                                MapEditButton::from_i(i),
                            ));
                        }
                    });
            });
        }
    }
}

//设置调色板
pub fn set_palette_board(
    mut commands: Commands,
    query: Query<&Board>,
    tiles: Res<TilesSet>,
    assert_server: Res<AssetServer>,
    root: Query<Entity, With<NodeRoot>>,
) {
    let root = root.single();
    for board in query.iter() {
        if board.board_type == BoardType::PenPalette {
            let width = board.width;
            let height = 5.0 * width;
            commands.entity(root).with_children(|commands| {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(width),
                            height: Val::Px(height),
                            flex_direction: FlexDirection::Column,
                            margin: UiRect {
                                left: Val::Px(0.0),
                                top: Val::Px(0.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|command| {
                        command
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    display: Display::Grid,
                                    padding: UiRect::all(Val::Px(5.0)),
                                    grid_template_columns: RepeatedGridTrack::flex(2, 1.0),
                                    grid_template_rows: RepeatedGridTrack::flex(8, 1.0),
                                    row_gap: Val::Percent(2.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|builder| {
                                for tile in tiles.tiles.keys() {
                                    builder.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Percent(80.0),
                                                aspect_ratio: Some(1.0),
                                                align_self: AlignSelf::Center,
                                                justify_self: JustifySelf::Center,
                                                ..default()
                                            },
                                            image: UiImage {
                                                texture: assert_server.load(&tiles.get_path(&tile)),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        tile.clone(),
                                    ));
                                }
                            });
                    });
            });
        }
    }
}

//生成终端
pub fn set_terminal(
    mut commands: Commands,
    boards: Query<&Board>,
    asset_server: Res<AssetServer>,
    root: Query<Entity, With<NodeRoot>>,
    buffer: Res<TerminalIOBuffer>,
) {
    for board in boards.iter() {
        if board.board_type == BoardType::Terminal {
            let capacity = buffer.capacity;
            let width = board.width;
            let height = board.height;
            let _position = board.position;
            let root = root.single();
            commands.entity(root).with_children(|commands| {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Px(width),
                            height: Val::Px(height),
                            flex_direction: FlexDirection::Column,
                            margin: UiRect {
                                left: Val::Px(0.0),
                                top: Val::Px(5.0 * height),
                                ..default()
                            },
                            padding: UiRect::all(Val::Percent(1.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|commands| {
                        for i in 0..capacity as usize {
                            commands.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            color: Color::WHITE,
                                            ..default()
                                        },
                                    ),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0 / capacity as f32),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TextPipeId(i),
                            ));
                        }
                    });
            });
        }
    }
}

//设置绘图板，包括绘图板和绘图板的边框
pub fn set_drawing_board(mut commands: Commands, boards: Query<&Board>) {
    for board in boards.iter() {
        if board.board_type == BoardType::PaperBoard {
            let width = 0.9 * board.width;
            let height = 0.9 * board.height;
            let position = board.position;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(width, height)),
                        color: Color::rgb(0.87, 0.78, 0.66),
                        ..default()
                    },
                    transform: Transform {
                        translation: position,
                        ..default()
                    },
                    ..default()
                },
                PaperBoard {
                    width,
                    height,
                    position,
                },
            ));

            //边框的z轴位置设置为20.0，使其在绘图板上方，这样移动纸张时纸张不会超出边框
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(board.width, 0.05 * board.height)),
                        color: board.color,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y + 0.475 * board.height, 20.0),
                        ..default()
                    },
                    ..default()
                },
                PaperBoarder {
                    width: board.width,
                    height: 0.05 * board.height,
                    position: Vec3::new(position.x, position.y + 0.475 * board.height, 20.0),
                    direction: BoarderType::Top,
                },
            ));

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(board.width, 0.05 * board.height)),
                        color: board.color,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y - 0.475 * board.height, 20.0),
                        ..default()
                    },
                    ..default()
                },
                PaperBoarder {
                    width: board.width,
                    height: 0.05 * board.height,
                    position: Vec3::new(position.x, position.y - 0.475 * board.height, 20.0),
                    direction: BoarderType::Bottom,
                },
            ));

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(0.05 * board.width, height)),
                        color: board.color,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(position.x - 0.475 * board.width, position.y, 20.0),
                        ..default()
                    },
                    ..default()
                },
                PaperBoarder {
                    width: 0.05 * board.width,
                    height,
                    position: Vec3::new(position.x - 0.475 * board.width, position.y, 20.0),
                    direction: BoarderType::Left,
                },
            ));

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(0.05 * board.width, height)),
                        color: board.color,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(position.x + 0.475 * board.width, position.y, 20.0),
                        ..default()
                    },
                    ..default()
                },
                PaperBoarder {
                    width: 0.05 * board.width,
                    height,
                    position: Vec3::new(position.x + 0.475 * board.width, position.y, 20.0),
                    direction: BoarderType::Right,
                },
            ));
        }
    }
}

pub fn spawn_page_info_box(
    mut commands: Commands,
    boards: Query<&Board>,
    asset_server: Res<AssetServer>,
    paper_album: Res<PaperAlbum>,
    query: Query<Entity, With<NodeRoot>>,
) {
    let root = query.single();
    for board in boards.iter() {
        if board.board_type == BoardType::ButtomBar {
            let width = board.width;
            let height = board.height;
            let page = paper_album.now_paper;

            commands.entity(root).with_children(|commands| {
                commands
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Px(height),
                            width: Val::Px(width),
                            display: Display::Grid,
                            padding: UiRect::all(Val::Px(5.0)),
                            grid_template_columns: RepeatedGridTrack::flex(10, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(1, 1.0),
                            row_gap: Val::Percent(2.0),
                            margin: UiRect {
                                right: Val::Px(0.0),
                                bottom: Val::Px(0.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|commands| {
                        commands.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    format!("storey: {}", page),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        color: Color::BLACK,
                                        font_size: 0.5 * height,
                                    },
                                ),
                                style: Style {
                                    width: Val::Px(0.1 * width),
                                    height: Val::Px(height),
                                    margin: UiRect {
                                        right: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        ..default()
                                    },
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                transform: Transform {
                                    translation: board.position,
                                    ..default()
                                },
                                ..default()
                            },
                            PaperInfo { page },
                        ));

                        commands.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    format!("Dead {}", 0),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        color: Color::BLACK,
                                        font_size: 0.5 * height,
                                    },
                                ),
                                style: Style {
                                    width: Val::Px(0.1 * width),
                                    height: Val::Px(height),
                                    margin: UiRect {
                                        right: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        ..default()
                                    },
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                transform: Transform {
                                    translation: board.position,
                                    ..default()
                                },
                                ..default()
                            },
                            DeadBox,
                        ));

                        commands.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    format!("Evacuated {}", 0),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        color: Color::BLACK,
                                        font_size: 0.5 * height,
                                    },
                                ),
                                style: Style {
                                    width: Val::Px(0.1 * width),
                                    height: Val::Px(height),
                                    margin: UiRect {
                                        right: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        ..default()
                                    },
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                transform: Transform {
                                    translation: board.position,
                                    ..default()
                                },
                                ..default()
                            },
                            EvacuatedBox,
                        ));

                        commands.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    format!("Time {}", 0.0),
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        color: Color::BLACK,
                                        font_size: 0.5 * height,
                                    },
                                ),
                                style: Style {
                                    width: Val::Px(0.1 * width),
                                    height: Val::Px(height),
                                    margin: UiRect {
                                        right: Val::Px(0.0),
                                        bottom: Val::Px(0.0),
                                        ..default()
                                    },
                                    align_self: AlignSelf::Center,
                                    ..default()
                                },
                                transform: Transform {
                                    translation: board.position,
                                    ..default()
                                },
                                ..default()
                            },
                            TimeBox,
                        ));
                    });
            });
        }
    }
}

pub fn update_page_info_box(
    mut query1: Query<(&mut Text, &mut PaperInfo)>,
    paper_album: Res<PaperAlbum>,
) {
    let (mut text, mut page) = query1.single_mut();
    text.sections[0].value = format!("Storey: {}", paper_album.now_paper + 1);
    page.page = paper_album.now_paper;
}

pub fn update_dead_num(mut query: Query<(&mut Text, &DeadBox)>, human: Res<TheCrowd>) {
    let (mut text, _) = query.single_mut();
    let dead = human.humans.iter().filter(|&x| x.is_dead == true).count();
    text.sections[0].value = format!("Dead: {}/{}", dead, human.humans.len());
}

pub fn update_evacuated_num(mut query: Query<(&mut Text, &EvacuatedBox)>, human: Res<TheCrowd>) {
    let (mut text, _) = query.single_mut();
    let evacuated = human
        .humans
        .iter()
        .filter(|&x| x.is_evacuated == true)
        .count();
    text.sections[0].value = format!("Evacuated: {}/{}", evacuated, human.humans.len());
}

pub fn update_time(mut query: Query<(&mut Text, &TimeBox)>, time: Res<Time>) {
    let (mut text, _) = query.single_mut();
    text.sections[0].value = format!("Time: {:.2}", time.elapsed_seconds());
}
