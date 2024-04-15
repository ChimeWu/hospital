use bevy::prelude::*;

pub mod button;
pub mod palette;
pub mod paper;
pub mod set;
pub mod terminal;

use button::*;
use palette::*;
use paper::*;
use set::*;
use terminal::*;

use crate::components::*;

pub struct SetUpPlugin;

pub struct PaperPlugin;

pub struct MapEditPlugin;

pub struct TerminalPlugin;

impl Plugin for SetUpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PaperAlbum::default())
            .insert_resource(MapAlbum::default())
            .insert_resource(TilesSet::default())
            .insert_resource(MapEditMessage::default())
            .insert_resource(AtomicCommandQueue::default())
            .insert_resource(TerminalIOBuffer::default())
            .insert_resource(CommandIOPipe::default())
            .add_systems(
                Startup,
                (
                    set_window_frame,
                    set_leftsidebar,
                    set_palette_board,
                    set_terminal,
                    set_drawing_board,
                    spawn_page_info_box,
                )
                    .chain(),
            );
    }
}

impl Plugin for PaperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (new_map, new_paper, delete_paper, spawn_paper, spawn_map,spawn_building_png).chain(),
        )
        .add_systems(
            Update,
            (
                new_map_album,
                new_paper_album,
                update_paper,
                scale_paper,
                move_paper,
                click_tile,
                change_tile,
                change_block,
                click_palette_button,
                update_page_info_box,
            ),
        );
    }
}

impl Plugin for MapEditPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                map_edit_button_clicked,
                take_terminal_output,
                get_terminal_input,
                reposition,
                exit,
                load,
                save,
                load_map,
                save_map,
                new,
                load_map_album,
                save_map_album,
                clean_map,
                delete_map,
                next_paper,
                last_paper,
            ),
        );
    }
}

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                input_terminal_buf,
                output_terminal_buf,
                tick_terminal_clock,
                show_terminal_text,
                terminal_enter_and_backspace,
            ),
        );
    }
}
