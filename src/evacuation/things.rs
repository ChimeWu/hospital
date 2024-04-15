pub mod fire;
pub mod human;
pub mod smoke;

use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct NeededParameters {
    pub k: f32,                        //引燃系数
    pub burning_time: f32,             //单位可燃物燃烧时间
    pub smoking_time: f32,             //可燃物冒烟时间（只要不熄灭就会按这个时间频率冒烟）
    pub head_counts_per_storey: usize, //每层人数
}
