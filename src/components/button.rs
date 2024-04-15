use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PageTurningButton {
    PrePage,
    ShowAll(bool),
    NextPage,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeRoot;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapEditButton {
    New,
    Load,
    Delete,
    CleanPage,
    RePosition,
    Save,
    PrePage,
    NextPage,
    Exit,
}

impl MapEditButton {
    pub fn from_i(i: usize) -> Self {
        match i {
            0 => Self::New,
            1 => Self::Load,
            2 => Self::Delete,
            3 => Self::CleanPage,
            4 => Self::RePosition,
            5 => Self::Save,
            6 => Self::PrePage,
            7 => Self::NextPage,
            8 => Self::Exit,
            _ => panic!("MapEditButton index out of range"),
        }
    }

    pub fn to_i(&self) -> usize {
        match self {
            Self::New => 0,
            Self::Load => 1,
            Self::Delete => 2,
            Self::CleanPage => 3,
            Self::RePosition => 4,
            Self::Save => 5,
            Self::PrePage => 6,
            Self::NextPage => 7,
            Self::Exit => 8,
        }
    }

    pub fn into_path(&self) -> String {
        let name = match self {
            Self::New => "new".to_string(),
            Self::Load => "load".to_string(),
            Self::Delete => "delete".to_string(),
            Self::CleanPage => "redo".to_string(),
            Self::RePosition => "reposition".to_string(),
            Self::Save => "save".to_string(),
            Self::PrePage => "prepage".to_string(),
            Self::NextPage => "nextpage".to_string(),
            Self::Exit => "exit".to_string(),
        };
        format!("./buttons/{}.png", name)
    }

    pub fn path_from_i(i: usize) -> String {
        let button = Self::from_i(i);
        button.into_path()
    }
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AtomicCommand {
    Load,
    Save,
    New,
    Delete,
    Clean,
    MapOrAlbum,
    TerminalOutput(String),
    GetTerminalInput,
    LoadMap,
    LoadMapAlbum,
    SaveMap,
    SaveMapAlbum,
    NewMap,
    NewMapAlbum,
    DeleteMap,
    DeleteMapAlbum,
    CleanMap,
    CleanMapAlbum,
    NewPaper,
    NewPaperAlbum,
    DeSpawnThePaper,
    SpawnThePaper,
    SpawnTheMap,
    RePosition,
    PrePage,
    NextPage,
    Exit,
    NewFire,
    NewSmoke,
    NewHuman,
    SpawnThePapers,
    SpawnTheMaps,
    ShowTheRightPaper,
    SpawnTheBuildingPng,
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AtomicCommandQueue {
    pub queue: VecDeque<AtomicCommand>,
}

impl Default for AtomicCommandQueue {
    fn default() -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(AtomicCommand::NewPaper);
        queue.push_back(AtomicCommand::SpawnThePaper);
        queue.push_back(AtomicCommand::SpawnTheMap);
        Self { queue }
    }
}

impl AtomicCommandQueue {
    pub fn new() -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(AtomicCommand::ShowTheRightPaper);
        Self { queue }
    }
}

#[derive(Resource, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandIOPipe {
    pub input: VecDeque<String>,
    pub output: VecDeque<String>,
}

impl Default for CommandIOPipe {
    fn default() -> Self {
        Self {
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }
}
