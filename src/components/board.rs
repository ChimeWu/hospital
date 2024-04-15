use bevy::prelude::*;

#[derive(Component)]
pub struct Board {
    pub width: f32,
    pub height: f32,
    pub position: Vec3,
    pub board_type: BoardType,
    pub color: Color,
}

#[derive(Component, Debug, PartialEq)]
pub enum BoardType {
    LeftSideBar,
    PenPalette,
    PaperBoard,
    Terminal,
    ButtomBar,
}
