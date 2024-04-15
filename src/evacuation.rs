pub mod rules;
pub mod things;

use bevy::prelude::*;

use crate::evacuation::rules::fire::*;
use crate::evacuation::rules::human::*;
use crate::evacuation::rules::smoke::*;

use crate::systems::button::*;

use crate::systems::paper::*;
use crate::systems::set::*;

use crate::components::*;

use self::things::fire::BuildingFire;
use self::things::human::*;
use self::things::smoke::BuildingSmoke;

pub struct SetUpPlugin;

pub struct PaperPlugin;

pub struct FirePlugin;

impl Plugin for SetUpPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PaperAlbum::default())
            .insert_resource(MapAlbum::load("building"))
            .insert_resource(TilesSet::default())
            .insert_resource(AtomicCommandQueue::new())
            .insert_resource(BuildingFire::default())
            .insert_resource(BuildingSmoke::default())
            .insert_resource(TheCrowd::new())
            .insert_resource(BuildingPass::default())
            .add_event::<Evacuated>()
            .add_event::<Dead>()
            .add_event::<ChangeStorey>()
            .add_event::<ChangeSafe>()
            .add_systems(
                Startup,
                (
                    set_window_frame_for_show,
                    set_leftsidebar_for_show,
                    set_drawing_board,
                    spawn_page_info_box,
                    new_paper_album_for_show,
                    spawn_papers,
                    spawn_maps,
                    init_building_fire,
                    init_random_fire,
                    spawn_fire_sprite,
                    init_building_smoke,
                    spawn_smoke_sprite,
                    init_building_pass,
                    init_the_crowd,
                    init_crowd_target_and_path,
                    spawn_people,
                )
                    .chain(),
            );
    }
}

impl Plugin for PaperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (next_paper, last_paper, show_the_right_paper).chain(),
        )
        .add_systems(
            Update,
            (update_page_info_box, map_edit_button_clicked_for_show, exit),
        );
    }
}

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_fire_visibility,
                ignition,
                burning,
                update_smoke_sprite,
                smoke_diffusion,
                people_run,
                to_get_safe,
                change_storey,
                dead,
                is_safe,
            ),
        );
    }
}
