use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::{HashMap, VecDeque};

use super::tile::TileType;

//地图组件，用于保存地图的基本信息，保存每一个网格上的tile类型
#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
}

impl Map {
    pub fn new(row: usize, col: usize) -> Self {
        Map {
            tiles: vec![vec![TileType::Floor; col]; row],
        }
    }

    pub fn get_tile(&self, row: usize, col: usize) -> TileType {
        self.tiles[row][col].clone()
    }

    pub fn set_tile(&mut self, row: usize, col: usize, tile: TileType) {
        self.tiles[row][col] = tile;
    }

    pub fn get_size(&self) -> (usize, usize) {
        (self.tiles.len(), self.tiles[0].len())
    }

    pub fn clean(&mut self) {
        let (row, col) = self.get_size();
        for i in 0..row {
            for j in 0..col {
                self.set_tile(i, j, TileType::Floor);
            }
        }
    }

    pub fn save(&self, name: &str) {
        let path = format!("./assets/mapalbums/maps/{}.json", name);
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer(std::io::BufWriter::new(file), self).unwrap();
    }

    pub fn load(name: &str) -> Self {
        let path = format!("./assets/mapalbums/maps/{}.json", name);
        let file = std::fs::File::open(path).unwrap();
        serde_json::from_reader(std::io::BufReader::new(file)).unwrap()
    }
}

//地图标记组件，用于标记地图的名字
#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MapMarker {
    pub name: String,
}

impl Ord for MapMarker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl PartialOrd for MapMarker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
//地图数据的合集，用于多个地图的显示或者切换，同时承担了绘制地图过程中改变色块消息的传递
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct MapAlbum {
    pub maps: HashMap<MapMarker, Map>,
    pub now_map: MapMarker,
}

impl MapAlbum {
    pub fn new() -> Self {
        MapAlbum {
            maps: HashMap::new(),
            now_map: MapMarker {
                name: "default".to_string(),
            },
        }
    }

    pub fn add_map(&mut self, name: String, map: Map) {
        self.maps.insert(MapMarker { name }, map);
    }

    pub fn remove_map(&mut self, name: &MapMarker) {
        self.maps.remove(name);
    }

    pub fn clean(&mut self) {
        for map in self.maps.values_mut() {
            map.clean();
        }
    }
    pub fn save(&self, name: &str) {
        let path = format!("./assets/mapalbums/albums/{}.json", name);
        let file = std::fs::File::create(path).unwrap();
        let value = self.into_string_map();
        serde_json::to_writer(std::io::BufWriter::new(file), &value).unwrap();
    }

    pub fn load(name: &str) -> Self {
        let path = format!("./assets/mapalbums/albums/{}.json", name);
        println!("{}", path);
        let file = std::fs::File::open(path).unwrap();
        let value: HashMap<String, Map> =
            serde_json::from_reader(std::io::BufReader::new(file)).unwrap();
        MapAlbum::from_string_map(value)
    }

    pub fn into_string_map(&self) -> HashMap<String, Map> {
        let mut string_map = HashMap::new();
        for (marker, map) in self.maps.iter() {
            string_map.insert(marker.name.clone(), map.clone());
        }
        string_map
    }

    pub fn from_string_map(string_map: HashMap<String, Map>) -> Self {
        let mut map_album = MapAlbum::new();
        for (name, map) in string_map.iter() {
            map_album.add_map(name.clone(), map.clone());
        }
        map_album
    }
}

impl Default for MapAlbum {
    fn default() -> Self {
        let marker0 = MapMarker {
            name: "map0".to_string(),
        };
        let map0 = Map {
            tiles: vec![vec![TileType::Floor; 50]; 40],
        };
        let mut album = MapAlbum::new();
        album.maps.insert(marker0.clone(), map0);
        album.now_map = marker0;
        album
    }
}

//改变色块的消息，用于在绘制地图时改变某个网格的颜色
#[derive(Component, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeMessage {
    pub row: usize,
    pub col: usize,
}

//消息队列，用于保存改变色块的消息，change_point用于保存改变单个色块的消息
//change_block用于保存改变一整个矩形的色块的消息
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct MapEditMessage {
    pub change_point: VecDeque<ChangeMessage>,
    pub change_block: VecDeque<ChangeMessage>,
}

impl Default for MapEditMessage {
    fn default() -> Self {
        Self {
            change_point: VecDeque::new(),
            change_block: VecDeque::new(),
        }
    }
}

//用于设置地图的相关属性，一个主要功能是用于保存地图的移动和缩放信息
#[derive(Component, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paper {
    pub map: MapMarker,
    pub row: usize,
    pub col: usize,
    pub element_size: f32,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub position: Vec3,
    pub position_global: Vec3,
}

//用于标记负责地图显示的唯一paper
#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ThePaper;

//用于标记多张地图的显示，用于多张地图的显示或者切换
#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PaperMarker {
    pub map: MapMarker,
}

impl PaperMarker {
    pub fn new(map: MapMarker) -> Self {
        PaperMarker { map }
    }
}

//标记绘图板的位置和大小
#[derive(Component, Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperBoard {
    pub width: f32,
    pub height: f32,
    pub position: Vec3,
}

//标记绘图板的边框
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperBoarder {
    pub width: f32,
    pub height: f32,
    pub position: Vec3,
    pub direction: BoarderType,
}

//绘图板的边框类型，留作可能的扩展，如在边框上添加按钮
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BoarderType {
    Left,
    Right,
    Top,
    Bottom,
}

//纸张信息的合集，用于多张地图的显示或者切换
#[derive(Resource, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperAlbum {
    pub papers: Vec<Paper>,
    pub now_paper: usize,
}

impl Default for PaperAlbum {
    fn default() -> Self {
        PaperAlbum {
            papers: Vec::new(),
            now_paper: 0,
        }
    }
}

//用于显示当前的地图名字和页码
#[derive(Component, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperInfo {
    pub page: usize,
}

impl Default for PaperInfo {
    fn default() -> Self {
        PaperInfo { page: 0 }
    }
}

