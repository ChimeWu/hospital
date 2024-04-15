use bevy::prelude::*;

use rand::Rng;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

use crate::components::painting::*;
use crate::components::*;

use crate::systems::paper::{get_x, get_y};

use super::smoke::*;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct StoreyPass {
    pub map: Vec<Vec<Pass>>,
    pub exit: Vec<(usize, usize)>,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Pass {
    Passable(Danger),
    Impassable,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Danger {
    pub value: f32,
}

impl Default for Danger {
    fn default() -> Self {
        Danger { value: 0.0 }
    }
}

//逃生地图：
//每个点储存应该通向的下一个点
//人在逃生时，根据当前位置和目标位置，找到下一个位置
//逃生地图不断更新，根据烟雾浓度采用加权最短路径算法更新

impl StoreyPass {
    pub fn new(rows: usize, cols: usize) -> Self {
        StoreyPass {
            map: vec![vec![Pass::Passable(Danger::default()); cols]; rows],
            exit: Vec::new(),
        }
    }

    pub fn init_from_map(&mut self, map: &Map) {
        let mut temp = Vec::new();
        for i in 0..map.tiles.len() {
            for j in 0..map.tiles[0].len() {
                if map.tiles[i][j] == TileType::Wall
                    || map.tiles[i][j] == TileType::Furniture
                    || map.tiles[i][j] == TileType::Stone
                {
                    self.map[i][j] = Pass::Impassable;
                }

                if map.tiles[i][j] == TileType::Exit {
                    temp.push((i, j));
                }
            }
        }
        //按照j值从小到大排序
        temp.sort_by(|a, b| a.1.cmp(&b.1));
        self.exit = temp;
    }

    pub fn update_from_smoke(&mut self, smoke: &StoreySmoke) {
        let rows = self.map.len();
        let cols = self.map[0].len();
        for i in 0..rows {
            for j in 0..cols {
                if let Pass::Passable(danger) = &mut self.map[i][j] {
                    if let Smoke::Diffusible(density) = &smoke.map[i][j] {
                        danger.value = density.value;
                    }
                }
            }
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct BuildingPass {
    pub maps: HashMap<MapMarker, StoreyPass>,
}

impl Default for BuildingPass {
    fn default() -> Self {
        BuildingPass {
            maps: HashMap::new(),
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
    Stay,
}

impl MoveDirection {
    pub fn into_vec3(&self) -> Vec3 {
        match self {
            MoveDirection::Up => Vec3::new(0.0, 1.0, 0.0),
            MoveDirection::Down => Vec3::new(0.0, -1.0, 0.0),
            MoveDirection::Left => Vec3::new(-1.0, 0.0, 0.0),
            MoveDirection::Right => Vec3::new(1.0, 0.0, 0.0),
            MoveDirection::Stay => Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn from_ij(i: i32, j: i32) -> Self {
        match (i, j) {
            (i, j) if i == 0 && j == 1 => MoveDirection::Up,
            (i, j) if i == 0 && j == -1 => MoveDirection::Down,
            (i, j) if i == 1 && j == 0 => MoveDirection::Left,
            (i, j) if i == -1 && j == 0 => MoveDirection::Right,
            _ => MoveDirection::Stay,
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Evacuate {
    Move(MoveDirection),
    CanNotArrive,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct StoreyEvacuation {
    pub map: Vec<Vec<Evacuate>>,
}

impl StoreyEvacuation {
    pub fn init_from_pass(pass: &StoreyPass) -> Self {
        let rows = pass.map.len();
        let cols = pass.map[0].len();
        let mut map = vec![vec![Evacuate::CanNotArrive; cols]; rows];
        for i in 0..rows {
            for j in 0..cols {
                if let Pass::Passable(_) = pass.map[i][j] {
                    map[i][j] = Evacuate::Move(MoveDirection::Stay);
                }
            }
        }
        StoreyEvacuation { map }
    }

    //计算最佳逃生路线，根据pass（danger值取决于smoke浓度）更新，采用加权最短路径Dijkstra算法，每点储存应该通向的下一个点
    pub fn update_from_pass(&mut self, pass: &StoreyPass, exit: (usize, usize)) {
        let map = &pass.map;
        let rows = map.len();
        let cols = map[0].len();
        let mut queue = BinaryHeap::new();
        let mut cost = vec![vec![usize::MAX; cols]; rows];

        cost[exit.0][exit.1] = 0;
        queue.push(State {
            cost: 0,
            position: exit,
        });

        while let Some(State {
            cost: now_cost,
            position: (i, j),
        }) = queue.pop()
        {
            if now_cost > cost[i][j] {
                continue;
            }
            if let Evacuate::Move(_) = self.map[i][j] {
                for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let new_i = i as i32 + di;
                    let new_j = j as i32 + dj;
                    if new_i >= 0 && new_i < rows as i32 && new_j >= 0 && new_j < cols as i32 {
                        let new_i = new_i as usize;
                        let new_j = new_j as usize;
                        if let Pass::Passable(danger) = &map[new_i][new_j] {
                            let new_cost = now_cost + danger.value as usize;
                            if new_cost < cost[new_i][new_j] {
                                cost[new_i][new_j] = new_cost;
                                queue.push(State {
                                    cost: new_cost,
                                    position: (new_i, new_j),
                                });
                                self.map[new_i][new_j] = Evacuate::Move(MoveDirection::from_ij(
                                    i as i32 - new_i as i32,
                                    j as i32 - new_j as i32,
                                ));
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct BuildingEvacuation {
    pub maps: HashMap<MapMarker, StoreyEvacuation>,
    pub now_map: MapMarker,
}

#[derive(Component, Debug, Clone)]
pub struct Human {
    pub id: usize,
    pub position: Vec3,
    pub next_position: Vec3,
    pub speed: f32,
    pub max_speed: f32,
    pub direction: Vec3,
    pub now_tile: (usize, usize),
    pub next_tile: (usize, usize),
    pub target_tile: (usize, usize),
    pub my_path: VecDeque<(usize, usize)>,
    pub hp: f32,
    pub max_hp: f32,
    pub storey: MapMarker,
    pub is_evacuated: bool,
    pub is_dead: bool,
    pub is_safe: bool,
}

impl PartialEq for Human {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Human {}

impl Hash for Human {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Human {
    pub fn clc_position(&mut self, size: f32, width: f32, height: f32) {
        let (i, j) = self.now_tile;
        self.position = Vec3::new(get_x(size, j, width), get_y(size, i, height), 12.0);
    }

    pub fn clc_next_position(&mut self, size: f32, width: f32, height: f32) {
        let (i, j) = self.next_tile;
        self.next_position = Vec3::new(get_x(size, j, width), get_y(size, i, height), 12.0);
    }

    pub fn find_my_target(&mut self, evacuation: &StoreyPass) {
        let now = self.now_tile;
        let mut min_dis = usize::MAX;
        let mut temp_target = (0, 0);
        for &(i, j) in &evacuation.exit {
            let dis = ((i as i32 - now.0 as i32).abs() + (j as i32 - now.1 as i32).abs()) as usize;
            if dis < min_dis {
                min_dis = dis;
                temp_target = (i, j);
            }
        }
        self.target_tile = temp_target;
    }

    pub fn find_my_path(&mut self, evacuation: &StoreyPass) {
        let now = self.now_tile;
        let target = self.target_tile;
        let mut queue = VecDeque::new();
        //Dijkstra算法
        let mut cost = vec![vec![usize::MAX; evacuation.map[0].len()]; evacuation.map.len()];
        let mut path = vec![vec![(0, 0); evacuation.map[0].len()]; evacuation.map.len()];
        cost[now.0][now.1] = 0;
        queue.push_back(now);
        while let Some((i, j)) = queue.pop_front() {
            for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_i = i as i32 + di;
                let new_j = j as i32 + dj;
                if new_i >= 0
                    && new_i < evacuation.map.len() as i32
                    && new_j >= 0
                    && new_j < evacuation.map[0].len() as i32
                {
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;
                    if let Pass::Passable(_) = evacuation.map[new_i][new_j] {
                        let new_cost = cost[i][j] + 1;
                        if new_cost < cost[new_i][new_j] {
                            cost[new_i][new_j] = new_cost;
                            path[new_i][new_j] = (i, j);
                            queue.push_back((new_i, new_j));
                        }
                    }
                }
            }
        }
        let mut my_path = VecDeque::new();
        let mut now = target;
        while now != self.now_tile {
            my_path.push_front(now);
            now = path[now.0][now.1];
        }
        self.my_path = my_path;
    }

    pub fn find_my_safe_place(&mut self, evacuation: &Map) {
        let now = self.now_tile;
        let mut min_dis = usize::MAX;
        let mut temp_target = (0, 0);
        for i in 0..evacuation.tiles.len() {
            for j in 0..evacuation.tiles[0].len() {
                if evacuation.tiles[i][j] == TileType::SavePlace {
                    let dis = ((i as i32 - now.0 as i32).abs() + (j as i32 - now.1 as i32).abs())
                        as usize;
                    if dis < min_dis {
                        min_dis = dis;
                        temp_target = (i, j);
                    }
                }
            }
        }
        self.target_tile = temp_target;
    }

    pub fn find_my_safe_path(&mut self, evacuation: &Map) {
        let now = self.now_tile;
        let target = self.target_tile;
        //用广度优先搜索算法
        //只有TileType::Stone 和Tiletype：：Saveplace是可通行的
        let mut queue = VecDeque::new();
        let mut cost = vec![vec![usize::MAX; evacuation.tiles[0].len()]; evacuation.tiles.len()];
        let mut path = vec![vec![(0, 0); evacuation.tiles[0].len()]; evacuation.tiles.len()];
        cost[now.0][now.1] = 0;
        queue.push_back(now);
        while let Some((i, j)) = queue.pop_front() {
            for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let new_i = i as i32 + di;
                let new_j = j as i32 + dj;
                if new_i >= 0
                    && new_i < evacuation.tiles.len() as i32
                    && new_j >= 0
                    && new_j < evacuation.tiles[0].len() as i32
                {
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;
                    if evacuation.tiles[new_i][new_j] == TileType::Stone
                        || evacuation.tiles[new_i][new_j] == TileType::SavePlace
                        || evacuation.tiles[new_i][new_j] == TileType::Exit
                    {
                        let new_cost = cost[i][j] + 1;
                        if new_cost < cost[new_i][new_j] {
                            cost[new_i][new_j] = new_cost;
                            path[new_i][new_j] = (i, j);
                            queue.push_back((new_i, new_j));
                        }
                    }
                }
            }
        }
        let mut my_path = VecDeque::new();
        let mut now = target;
        while now != self.now_tile {
            my_path.push_front(now);
            now = path[now.0][now.1];
        }
        self.my_path = my_path;
    }

    pub fn smoke_damage(&mut self, smoke: &StoreySmoke, time: f32) {
        let (i, j) = self.now_tile;
        if let Smoke::Diffusible(density) = &smoke.map[i][j] {
            self.hp -= time * density.value;
            if self.hp <= 0.0 {
                self.is_dead = true;
            }
        }
    }

    pub fn change_my_speed(&mut self) {
        if self.hp <= 0.0 {
            self.speed = 0.0;
        } else {
            self.speed = self.max_speed * self.hp / self.max_hp;
        }
    }

    pub fn change_my_direction(&mut self) {
        self.direction = (self.next_position - self.position).normalize();
    }
}

impl Default for Human {
    fn default() -> Self {
        Human {
            id: 0,
            position: Vec3::new(0.0, 0.0, 0.0),
            next_position: Vec3::new(0.0, 0.0, 0.0),
            speed: 0.0,
            max_speed: 10.0,
            direction: Vec3::new(0.0, 0.0, 0.0),
            now_tile: (0, 0),
            next_tile: (0, 0),
            target_tile: (0, 0),
            my_path: VecDeque::new(),
            hp: 100.0,
            max_hp: 100.0,
            storey: MapMarker {
                name: "default".to_string(),
            },
            is_evacuated: false,
            is_dead: false,
            is_safe: false,
        }
    }
}

impl Human{
    pub fn rand_my_hp(&mut self, h: f32) {
        let mut rng = rand::thread_rng();
        self.hp = self.max_hp*rng.gen_range((1.0-h)..1.0);
    }

    pub fn rand_my_speed(&mut self, v: f32) {
        let mut rng = rand::thread_rng();
        self.speed = self.max_speed*rng.gen_range((1.0-v)..(1.0+v));
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct HumanMarker {
    pub id: usize,
    pub storey: MapMarker,
}

impl HumanMarker {
    pub fn from_human(human: &Human) -> Self {
        HumanMarker {
            id: human.id,
            storey: human.storey.clone(),
        }
    }
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct TheCrowd {
    pub humans: Vec<Human>,
    pub texture_path: String,
    pub element_size: f32,
    pub board_width: f32,
    pub board_height: f32,
}

impl TheCrowd {
    pub fn new() -> Self {
        TheCrowd {
            humans: Vec::new(),
            texture_path: "./movethings/mango.png".to_string(),
            element_size: 1.0,
            board_width: 50.0,
            board_height: 40.0,
        }
    }

    pub fn extend_from_pass(&mut self, pass: &Map, storey: MapMarker,human_seed: f32,v: f32,h: f32) {
        let mut rng = rand::thread_rng();
        for i in 0..pass.tiles.len() {
            for j in 0..pass.tiles[0].len() {
                if TileType::Floor == pass.tiles[i][j] {
                    if rng.gen_bool(human_seed as f64) {
                        let mut human = Human::default();
                        human.id = self.humans.len();
                        human.rand_my_hp(h);
                        human.rand_my_speed(v);
                        human.now_tile = (i, j);
                        human.storey = storey.clone();
                        self.humans.push(human);
                    }
                }
            }
        }
    }

    pub fn add_human(&mut self, human: Human) {
        self.humans.push(human);
    }

    pub fn remove_human(&mut self, human: &Human) {
        self.humans.retain(|x| x != human);
    }
}

#[derive(Event)]
pub struct Evacuated {
    pub id: usize,
}

#[derive(Event)]
pub struct Dead {
    pub id: usize,
}

#[derive(Event)]
pub struct ChangeStorey {
    pub id: usize,
}

#[derive(Event)]
pub struct ChangeSafe {
    pub id: usize,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct DeadBox;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct EvacuatedBox;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct TimeBox;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct ManNumBox;

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Crowd {
    pub humans: HashSet<Human>,
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub struct BuildingCrowdGroup {
    pub maps: HashMap<MapMarker, Crowd>,
    pub now_map: MapMarker,
}
