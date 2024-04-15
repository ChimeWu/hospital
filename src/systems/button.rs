use bevy::app::AppExit;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use crate::components::button::*;

use crate::components::painting::*;

const INPUT_MESSAGE: [&str; 2] = [
    "Please input the name of the map or the mapalbum:",
    "Please input the name ,rows and cols of the map(and nums of pages if needed):",
];

fn format_output(action: &str) -> String {
    format! {"Will you {} the map or the mapalbum? m for map,a for mapalbum:",action}
}

//检测地图编辑按钮点击事件，如果点击到了按钮，将对应命令分解成原子命令，加入原子命令队列
pub fn map_edit_button_clicked(
    mut interaction_query: Query<
        (&Interaction, &MapEditButton),
        (Changed<Interaction>, With<Button>),
    >,
    _mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let command = match button {
                MapEditButton::RePosition => vec![AtomicCommand::RePosition],
                MapEditButton::Exit => vec![AtomicCommand::Exit],
                MapEditButton::Load => {
                    vec![
                        AtomicCommand::TerminalOutput(format_output("load")),
                        AtomicCommand::GetTerminalInput,
                        AtomicCommand::Load,
                        AtomicCommand::TerminalOutput(INPUT_MESSAGE[0].to_string()),
                        AtomicCommand::GetTerminalInput,
                    ]
                }
                MapEditButton::Save => {
                    vec![
                        AtomicCommand::TerminalOutput(format_output("save")),
                        AtomicCommand::GetTerminalInput,
                        AtomicCommand::Save,
                        AtomicCommand::TerminalOutput(INPUT_MESSAGE[0].to_string()),
                        AtomicCommand::GetTerminalInput,
                    ]
                }
                MapEditButton::New => {
                    vec![
                        AtomicCommand::TerminalOutput(format_output("new")),
                        AtomicCommand::GetTerminalInput,
                        AtomicCommand::New,
                        AtomicCommand::TerminalOutput(INPUT_MESSAGE[1].to_string()),
                        AtomicCommand::GetTerminalInput,
                    ]
                }
                MapEditButton::Delete => {
                    vec![
                        AtomicCommand::DeleteMap,
                        AtomicCommand::DeSpawnThePaper,
                        AtomicCommand::SpawnThePaper,
                        AtomicCommand::SpawnTheMap,
                    ]
                }
                MapEditButton::CleanPage => {
                    vec![
                        AtomicCommand::CleanMap,
                        AtomicCommand::DeSpawnThePaper,
                        AtomicCommand::SpawnThePaper,
                        AtomicCommand::SpawnTheMap,
                    ]
                }
                MapEditButton::PrePage => {
                    vec![
                        AtomicCommand::PrePage,
                        AtomicCommand::DeSpawnThePaper,
                        AtomicCommand::SpawnThePaper,
                        AtomicCommand::SpawnTheMap,
                        AtomicCommand::SpawnTheBuildingPng,
                    ]
                }
                MapEditButton::NextPage => {
                    vec![
                        AtomicCommand::NextPage,
                        AtomicCommand::DeSpawnThePaper,
                        AtomicCommand::SpawnThePaper,
                        AtomicCommand::SpawnTheMap,
                        AtomicCommand::SpawnTheBuildingPng,
                    ]
                }
            };
            atomic_command.queue.extend(command);
        }
    }
}

pub fn map_edit_button_clicked_for_show(
    mut interaction_query: Query<
        (&Interaction, &MapEditButton),
        (Changed<Interaction>, With<Button>),
    >,
    _mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut atomic_command: ResMut<AtomicCommandQueue>,
) {
    for (interaction, button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            let command = match button {
                MapEditButton::Exit => vec![AtomicCommand::Exit],
                MapEditButton::PrePage => {
                    vec![AtomicCommand::PrePage, AtomicCommand::ShowTheRightPaper]
                }
                MapEditButton::NextPage => {
                    vec![AtomicCommand::NextPage, AtomicCommand::ShowTheRightPaper]
                }
                _ => vec![],
            };
            atomic_command.queue.extend(command);
        }
    }
}

//原子命令处理：load
pub fn load(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut pipe: ResMut<CommandIOPipe>,
    _map_album: ResMut<MapAlbum>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::Load {
            let choice = pipe.input.pop_front().unwrap();
            if choice == "m" {
                atomic_command.queue.push_back(AtomicCommand::LoadMap);
            } else if choice == "a" {
                atomic_command.queue.push_back(AtomicCommand::LoadMapAlbum);
            }
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：save
pub fn save(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut pipe: ResMut<CommandIOPipe>,
    _map_album: ResMut<MapAlbum>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::Save {
            let choice = pipe.input.pop_front().unwrap();
            if choice == "m" {
                atomic_command.queue.push_back(AtomicCommand::SaveMap);
            } else if choice == "a" {
                atomic_command.queue.push_back(AtomicCommand::SaveMapAlbum);
            }
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：new
pub fn new(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut pipe: ResMut<CommandIOPipe>,
    _map_album: ResMut<MapAlbum>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::New {
            let choice = pipe.input.pop_front().unwrap();
            if choice == "m" {
                atomic_command.queue.extend(
                    vec![
                        AtomicCommand::NewMap,
                        AtomicCommand::NewPaper,
                        AtomicCommand::DeSpawnThePaper,
                        AtomicCommand::SpawnThePaper,
                        AtomicCommand::SpawnTheMap,
                        AtomicCommand::SpawnTheBuildingPng,
                    ]
                    .into_iter(),
                );
            } else if choice == "a" {
                atomic_command.queue.extend(
                    vec![
                        AtomicCommand::NewMapAlbum,
                        AtomicCommand::NewPaperAlbum,
                        AtomicCommand::DeSpawnThePaper,
                        AtomicCommand::SpawnThePaper,
                        AtomicCommand::SpawnTheMap,
                    ]
                    .into_iter(),
                );
            }
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：loadmap
pub fn load_map(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut map_album: ResMut<MapAlbum>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::LoadMap {
            let name = pipe.input.pop_front().unwrap();
            let map = Map::load(&name);
            map_album.maps.insert(MapMarker { name: name.clone() }, map);
            map_album.now_map = MapMarker { name };
            atomic_command.queue.extend(
                vec![
                    AtomicCommand::NewPaper,
                    AtomicCommand::DeSpawnThePaper,
                    AtomicCommand::SpawnThePaper,
                    AtomicCommand::SpawnTheMap,
                    AtomicCommand::SpawnTheBuildingPng,
                ]
                .into_iter(),
            );
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：loadmapalbum
pub fn load_map_album(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut map_album: ResMut<MapAlbum>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::LoadMapAlbum {
            let name = pipe.input.pop_front().unwrap();
            let album = MapAlbum::load(&name);
            map_album.maps = album.maps;
            map_album.now_map = album.now_map;
            atomic_command.queue.extend(
                vec![
                    AtomicCommand::NewPaperAlbum,
                    AtomicCommand::DeSpawnThePaper,
                    AtomicCommand::SpawnThePaper,
                    AtomicCommand::SpawnTheMap,
                    AtomicCommand::SpawnTheBuildingPng,
                ]
                .into_iter(),
            );
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：savemap
pub fn save_map(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    map_album: ResMut<MapAlbum>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::SaveMap {
            let name = pipe.input.pop_front().unwrap();
            let now = map_album.now_map.clone();
            let map = map_album.maps.get(&now).unwrap();
            map.save(&name);
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：savemapalbum
pub fn save_map_album(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    map_album: ResMut<MapAlbum>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::SaveMapAlbum {
            let name = pipe.input.pop_front().unwrap();
            map_album.save(&name);
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：output
pub fn take_terminal_output(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if let AtomicCommand::TerminalOutput(s) = command.clone() {
            pipe.output.push_back(s.clone());
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：getinput
pub fn get_terminal_input(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    pipe: ResMut<CommandIOPipe>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if let AtomicCommand::GetTerminalInput = command.clone() {
            while let Some(_input) = pipe.input.front() {
                atomic_command.queue.pop_front();
                break;
            }
        }
    }
}

//原子命令处理：reposition
pub fn reposition(
    mut atomic_command: ResMut<AtomicCommandQueue>,
    mut paper_album: ResMut<PaperAlbum>,
) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::RePosition {
            let now = paper_album.now_paper.clone();
            let paper = &mut paper_album.papers[now];
            paper.position = Vec3::new(0.0, 0.0, 0.0);
            paper.scale = 1.0;
            atomic_command.queue.pop_front();
        }
    }
}

//原子命令处理：exit
pub fn exit(atomic_command: ResMut<AtomicCommandQueue>, mut exit_events: EventWriter<AppExit>) {
    if let Some(command) = atomic_command.queue.front() {
        if command == &AtomicCommand::Exit {
            exit_events.send(AppExit);
        }
    }
}
