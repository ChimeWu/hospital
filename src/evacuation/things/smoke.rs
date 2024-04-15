use bevy::prelude::*;

use crate::components::painting::*;
use crate::components::TileType;

use std::{collections::HashMap, vec};

#[derive(Component, Debug, Clone, PartialEq)]
pub struct StoreySmoke {
    pub map: Vec<Vec<Smoke>>,
}

impl StoreySmoke {
    pub fn new(rows: usize, cols: usize) -> Self {
        StoreySmoke {
            map: vec![vec![Smoke::new_diffusible(); cols]; rows],
        }
    }

    pub fn init_from_map(&mut self, map: &Map) {
        for i in 0..map.tiles.len() {
            for j in 0..map.tiles[0].len() {
                if map.tiles[i][j] == TileType::Wall || map.tiles[i][j] == TileType::Exit {
                    self.map[i][j] = Smoke::InDiffusible;
                }
            }
        }
    }

    pub fn diffuse(&mut self) {
        let rows = self.map.len();
        let cols = self.map[0].len();
        for i in 0..rows {
            for j in 0..cols {
                let mut sum = 0.0;
                let mut count = 0.0;
                let mut vec = Vec::new();
                if let Smoke::Diffusible(density) = &mut self.map[i][j] {
                    vec.push((i, j));
                    sum += density.value;
                    count += 1.0;
                } else {
                    continue;
                }
                if i > 0 {
                    if let Smoke::Diffusible(density) = &mut self.map[i - 1][j] {
                        vec.push((i - 1, j));
                        sum += density.value;
                        count += 1.0;
                    }
                }
                if i < rows - 1 {
                    if let Smoke::Diffusible(density) = &mut self.map[i + 1][j] {
                        vec.push((i + 1, j));
                        sum += density.value;
                        count += 1.0;
                    }
                }
                if j > 0 {
                    if let Smoke::Diffusible(density) = &mut self.map[i][j - 1] {
                        vec.push((i, j - 1));
                        sum += density.value;
                        count += 1.0;
                    }
                }
                if j < cols - 1 {
                    if let Smoke::Diffusible(density) = &mut self.map[i][j + 1] {
                        vec.push((i, j + 1));
                        sum += density.value;
                        count += 1.0;
                    }
                }
                let average = sum / count;
                for (x, y) in vec.iter() {
                    if let Smoke::Diffusible(density) = &mut self.map[*x][*y] {
                        density.value = average;
                    }
                }
            }
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub enum Smoke {
    Diffusible(Density),
    InDiffusible,
}

impl Smoke {
    pub fn new_diffusible() -> Self {
        Smoke::Diffusible(Density::default())
    }

    pub fn density_up(&mut self, value: f32) {
        if let Smoke::Diffusible(density) = self {
            density.density_up(value);
        }
    }

    pub fn get_density(&self) -> f32 {
        if let Smoke::Diffusible(density) = self {
            density.value
        } else {
            0.0
        }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct SmokeTile {
    pub pos: (usize, usize),
}

impl SmokeTile {
    pub fn new(i: usize, j: usize) -> Self {
        SmokeTile { pos: (i, j) }
    }
}

#[derive(Component, Debug, Clone, PartialEq)]
pub struct Density {
    pub value: f32,
}

impl Density {
    pub fn new(value: f32) -> Self {
        Density { value }
    }

    pub fn density_up(&mut self, value: f32) {
        self.value += value;
    }
}

impl Default for Density {
    fn default() -> Self {
        Density { value: 0.0 }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct BuildingSmoke {
    pub maps: HashMap<MapMarker, StoreySmoke>,
    pub texture_path: String,
}

impl Default for BuildingSmoke {
    fn default() -> Self {
        BuildingSmoke {
            maps: HashMap::new(),
            texture_path: "./movethings/smoke.png".to_string(),
        }
    }
}
