pub mod fire;
pub mod human;
pub mod smoke;

use bevy::prelude::*;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct NeededParameters {
    pub k: f32,            //引燃系数
    pub p: f32,            //初始引燃概率
    pub burning_time: f32, //单位可燃物燃烧时间
    pub smoking_time: f32, //可燃物冒烟时间（只要不熄灭就会按这个时间频率冒烟）
    pub human_seed: f32,   //人员生成概率
    pub s: f32,            //烟雾伤害系数
    pub h: f32,            //人员初始生命值随机算子
    pub v: f32,            //人员移动速度随机算子
    pub c: f32,            //可燃物替换随机算子
}

impl Default for NeededParameters {
    fn default() -> Self {
        Self {
            k: 0.0,
            p: 0.0,
            burning_time: 0.0,
            smoking_time: 0.0,
            human_seed: 0.0,
            s: 0.0,
            h: 0.0,
            v: 0.0,
            c: 0.0,
        }
    }
}

impl NeededParameters {
    pub fn save(&self, name: &str) {
        let path = format!("./assets/parameters/{}.json", name);
        let file = std::fs::File::create(path).unwrap();
        serde_json::to_writer(std::io::BufWriter::new(file), self).unwrap();
    }

    pub fn load(name: &str) -> Self {
        let path = format!("./assets/parameters/{}.json", name);
        let file = std::fs::File::open(path).unwrap();
        serde_json::from_reader(std::io::BufReader::new(file)).unwrap()
    }
}

pub fn get_needed_parameters(mut res: ResMut<NeededParameters>) {
    println!("Please input the k(请输入引燃系数)(0~10):");
    let mut k = String::new();
    std::io::stdin().read_line(&mut k).unwrap();
    let k: f32 = k.trim().parse().unwrap();
    res.k = k;

    println!("Please input the p(请输入初始引燃概率)(0~1):");
    let mut p = String::new();
    std::io::stdin().read_line(&mut p).unwrap();
    let p: f32 = p.trim().parse().unwrap();
    res.p = p;

    println!("Please input the burning time(请输入单位可燃物燃烧时间)(s):");
    let mut burning_time = String::new();
    std::io::stdin().read_line(&mut burning_time).unwrap();
    let burning_time: f32 = burning_time.trim().parse().unwrap();
    res.burning_time = burning_time;

    println!("Please input the smoking time(请输入可燃物冒烟时间)(s):");
    let mut smoking_time = String::new();
    std::io::stdin().read_line(&mut smoking_time).unwrap();
    let smoking_time: f32 = smoking_time.trim().parse().unwrap();
    res.smoking_time = smoking_time;

    println!("Please input the human seed(请输入人员生成概率)(0~1):");
    let mut human_seed = String::new();
    std::io::stdin().read_line(&mut human_seed).unwrap();
    let human_seed: f32 = human_seed.trim().parse().unwrap();
    res.human_seed = human_seed;

    println!("Please input the s(请输入烟雾伤害系数)(正数):");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let s: f32 = s.trim().parse().unwrap();
    res.s = s;

    println!("Please input the h(请输入人员初始生命值随机算子)(0~1):");
    let mut h = String::new();
    std::io::stdin().read_line(&mut h).unwrap();
    let h: f32 = h.trim().parse().unwrap();
    res.h = h;

    println!("Please input the v(请输入人员移动速度随机算子)(0~1):");
    let mut v = String::new();
    std::io::stdin().read_line(&mut v).unwrap();
    let v: f32 = v.trim().parse().unwrap();
    res.v = v;

    println!("Please input the c(请输入可燃物替换随机算子)(0~1):");
    let mut c = String::new();
    std::io::stdin().read_line(&mut c).unwrap();
    let c: f32 = c.trim().parse().unwrap();
    res.c = c;

    res.save("needed_parameters");
}
