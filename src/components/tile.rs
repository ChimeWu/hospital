use std::collections::HashMap;

use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;

//网格可能的类型
#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
    Stair,
    Elevator,
    Door,
    Exit,
    Alarm,
    Hydrant,
    Detector,
    Furniture,
    Stone,
    SavePlace,
}

//用于加载当前的色块套件，每种类型对应一个图片资源，考虑了使用不同套件可以更换图片
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct TilesSet {
    pub tiles: HashMap<TileType, String>,
    pub now_type: TileType,
    pub now_path: String,
}

impl TilesSet {
    pub fn get_path(&self, tile_type: &TileType) -> String {
        format!("./tiles/{}.png", {
            self.tiles.get(tile_type).unwrap().clone()
        })
    }

    pub fn add_tile(&mut self, tile_type: TileType, path: String) {
        self.tiles.insert(tile_type, path);
    }

    pub fn remove_tile(&mut self, tile_type: &TileType) {
        self.tiles.remove(tile_type);
    }

    pub fn save(&self, name: &str) {
        let path = format!("./tiles_set/{}.json", name);
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer(std::io::BufWriter::new(file), self).unwrap();
    }

    pub fn load(name: &str) -> Self {
        let path = format!("./tiles_set/{}.json", name);
        let file = std::fs::File::open(path).unwrap();
        serde_json::from_reader(std::io::BufReader::new(file)).unwrap()
    }
}

impl Default for TilesSet {
    fn default() -> Self {
        let mut tiles = HashMap::new();
        tiles.insert(TileType::Floor, "stone_floor".to_string());
        tiles.insert(TileType::Wall, "brown_wall".to_string());
        tiles.insert(TileType::Stair, "stair1".to_string());
        tiles.insert(TileType::Elevator, "elevator".to_string());
        tiles.insert(TileType::Door, "door1".to_string());
        tiles.insert(TileType::Exit, "exit2".to_string());
        tiles.insert(TileType::Alarm, "alarm".to_string());
        tiles.insert(TileType::Hydrant, "hydrant".to_string());
        tiles.insert(TileType::Detector, "detector".to_string());
        tiles.insert(TileType::Furniture, "bed".to_string());
        tiles.insert(TileType::Stone, "floor3".to_string());
        tiles.insert(TileType::SavePlace, "stone".to_string());
        TilesSet {
            tiles,
            now_type: TileType::Wall,
            now_path: format!("./tiles/{}.png", "wall"),
        }
    }
}

//标记绘图时某一个色块的位置，便于在绘制时找到对应的网格
#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TileMarker {
    pub i: usize,
    pub j: usize,
}
