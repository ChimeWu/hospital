use crate::components::{painting::*, TileType};
use bevy::prelude::*;

use rand::Rng;
use std::collections::{HashMap, VecDeque};

#[derive(Component, Debug, Clone, PartialEq)]
pub struct StoreyFire {
    pub map: Vec<Vec<Fire>>,
    pub burning_time: f32,
    pub smoking_time: f32,
    pub k: f32,
}

impl StoreyFire {
    pub fn new(rows: usize, cols: usize) -> Self {
        StoreyFire {
            map: vec![vec![Fire::NeverBurn; cols]; rows],
            burning_time: 100.0,
            smoking_time: 1.0,
            k: 1.0,
        }
    }

    pub fn new_with(
        rows: usize,
        cols: usize,
        burning_time: f32,
        smoking_time: f32,
        k: f32,
    ) -> Self {
        StoreyFire {
            map: vec![vec![Fire::NeverBurn; cols]; rows],
            burning_time,
            smoking_time,
            k,
        }
    }

    pub fn init_from_map(&mut self, map: &Map) {
        for i in 0..map.tiles.len() {
            for j in 0..map.tiles[0].len() {
                if map.tiles[i][j] == TileType::Furniture {
                    self.map[i][j] = Fire::Off;
                }
            }
        }
    }

    //随机点燃
    pub fn get_p_fire(&mut self, p: f64) {
        let mut rng = rand::thread_rng();
        for i in 0..self.map.len() {
            for j in 0..self.map[0].len() {
                if self.map[i][j] == Fire::Off {
                    if rng.gen_bool(p) {
                        self.map[i][j] =
                            Fire::On(FireTimer::new(self.burning_time, self.smoking_time));
                    }
                }
            }
        }
    }

    //未燃烧的点按一定条件被引燃
    pub fn get_neighbours_fire_on(&mut self) {
        let mut fire_on_queue = VecDeque::new();
        let mut fire_off_queue = VecDeque::new();
        let rows = self.map.len();
        let cols = self.map[0].len();
        for i in 0..rows {
            for j in 0..cols {
                if let Fire::On(_) = &self.map[i][j] {
                    fire_on_queue.push_back((i, j));
                }
                if let Fire::Off = &self.map[i][j] {
                    fire_off_queue.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = fire_off_queue.pop_front() {
            for (x, y) in fire_on_queue.iter() {
                let fire_time = self.map[*x][*y].get_burning_time();
                let distance2 =
                    ((i as i32 - *x as i32).pow(2) + (j as i32 - *y as i32).pow(2)) as f32;
                let distance = distance2.sqrt();
                if fire_time / distance > self.k {
                    self.map[i][j].fire_on(self.burning_time, self.smoking_time);
                    break;
                }
            }
        }
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub enum Fire {
    #[default]
    Off,
    On(FireTimer),
    NeverBurn,
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct FireTile {
    pub pos: (usize, usize),
}

impl FireTile {
    pub fn new(i: usize, j: usize) -> Self {
        FireTile { pos: (i, j) }
    }
}

impl Fire {
    pub fn fire_on(&mut self, burning_time: f32, smoking_time: f32) {
        match self {
            Fire::Off => {
                *self = Fire::On(FireTimer::new(burning_time, smoking_time));
            }
            _ => {}
        }
    }

    pub fn fire_off(&mut self) {
        match self {
            Fire::On(_) => {
                *self = Fire::NeverBurn;
            }
            _ => {}
        }
    }

    pub fn get_burning_time(&self) -> f32 {
        match self {
            Fire::On(fire_timer) => fire_timer.get_burning_time(),
            _ => 0.0,
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct FireTimer {
    pub burning_timer: Timer,
    pub smoking_timer: Timer,
}

impl Default for FireTimer {
    fn default() -> Self {
        FireTimer {
            burning_timer: Timer::from_seconds(60.0, TimerMode::Once),
            smoking_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

impl FireTimer {
    pub fn new(burning_time: f32, smoking_time: f32) -> Self {
        let mut rng = rand::thread_rng();
        let burning_time = rng.gen_range(0.8 * burning_time..1.2 * burning_time);
        let smoking_time = rng.gen_range(0.8 * smoking_time..1.2 * smoking_time);
        FireTimer {
            burning_timer: Timer::from_seconds(burning_time, TimerMode::Once),
            smoking_timer: Timer::from_seconds(smoking_time, TimerMode::Repeating),
        }
    }

    pub fn get_burning_time(&self) -> f32 {
        self.burning_timer.elapsed().as_secs_f32()
    }
}

#[derive(Resource, Debug, Clone)]
pub struct BuildingFire {
    pub maps: HashMap<MapMarker, StoreyFire>,
    pub texture_path: String,
}

impl Default for BuildingFire {
    fn default() -> Self {
        BuildingFire {
            maps: HashMap::new(),
            texture_path: "./movethings/fire.png".to_string(),
        }
    }
}
