use std::{collections::HashMap, error::Error, fs::File, io::{BufReader, Read}, path::Path};

use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};
use serde_json::Value;

use crate::types::{Freeplay, PlacedTower};

mod types;

fn read_json<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let json: Value = serde_json::from_reader(file)?;
    Ok(json)
}

//computes a float to a small unsigned power at comptime
const fn fpowu(f: f64, mut i: u8) -> f64 {
    if i == 0 {
        return 0f64;
    }
    let mut value: f64 = f;
    while i > 1 {
        i -= 1;
        value *= f;
    }
    value
}

const MAX_RANGE: f64 = 84.755;
const RANGE_NO_VILLAGE: f64 = 77.05;
const NINJAGON_RANGE: f64 = 70.0;
const VILLAGE_RANGE: f64 = const {
    let mut base = 55.0;
    base += (0.1 * base) + ((fpowu(1.015, 10) - 1.0) * base) + (0.4 * base) + (0.5 * base);
    base * 1.15
}; // 1.1x oc 1.015^10x for uboost, 1.2x 1.2x 1.15x 1.5x
const HOMELAND_RANGE: f64 = 55.0 + (0.4 * 55.0);

#[derive(Clone, Copy, PartialEq, Eq)]
enum TowerType {
    Other,
    Bloonjitsu,
    Shinobi,
}

#[derive(Clone)]
struct Tower {
    // pub id: i64,
    pub jitsu_value: i32,
    pub shinobi_value: i32,
    pub range: f64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub negotiable: bool,
    pub buffable: bool,
}


impl Tower {
    pub fn new(tower: &PlacedTower, range: f64) -> Self {
        Self {
            // id: tower.id_last_save,
            pos_x: tower.position.x,
            pos_y: tower.position.y,
            negotiable: true,
            buffable: true,
            jitsu_value: 9999999,
            shinobi_value: 9999999,
            range,
        }
    }
    pub fn into_nonnegotiable(mut self) -> Self {
        self.negotiable = false;
        self
    }
    pub fn into_unbuffable(mut self) -> Self {
        self.buffable = false;
        self
    }
    pub fn with_values(mut self, jitsu: i32, shinobi: i32) -> Self {
        self.jitsu_value = jitsu;
        self.shinobi_value = shinobi;
        self
    }
}

fn is_in_range(pos_1: (f64, f64), pos_2: (f64, f64), range: f64) -> bool {
    let dx = pos_1.0 - pos_2.0;
    let dy = pos_1.1 - pos_2.1;
    (dx * dx + dy * dy).sqrt() < range
}

fn tower_is_in_range(tower_1: &Tower, tower_2: &Tower, range: f64) -> bool {
    let dx = tower_1.pos_x - tower_2.pos_x;
    let dy = tower_1.pos_y - tower_2.pos_y;
    (dx * dx + dy * dy).sqrt() < range
}

fn get_attack_speed_buff(stack_count: u8) -> f64 {
    (1.08f64).powi(stack_count.into())
}

fn get_pierce_buff(stack_count: u8) -> f64 {
    1.0 + 0.08 * (stack_count as f64)
}

fn get_setup_dps(towers: &[Tower], tower_types: &[TowerType], stacks: &[u8]) -> f64 {
    let mut setup_dps = 0f64;
    for i in 0..towers.len() {
        let ninja = &towers[i];
        if ninja.negotiable {
            let tower_type = tower_types[i];
            let value = if tower_type == TowerType::Shinobi {
                ninja.shinobi_value
            } else {
                ninja.jitsu_value
            };
            let stack_count = stacks[i].min(20);
            let attack_speed_buff = get_attack_speed_buff(stack_count);
            let pierce_buff = get_pierce_buff(stack_count);
            setup_dps += value as f64 * attack_speed_buff * pierce_buff;
        }
    }
    setup_dps
}

fn full_calc_dps(towers: &[Tower], tower_types: &[TowerType]) -> f64 {
    let mut stacks = vec![0; towers.len()];
    for (i, tower) in towers.iter().enumerate() {
        for (j, other) in towers.iter().enumerate() {
            if tower_is_in_range(other, tower, tower.range) && tower_types[i] == TowerType::Shinobi {
                    stacks[j] += 1;
                }
        }
    }
    get_setup_dps(towers, tower_types, &stacks)
}

fn rand_optimize(towers: &[Tower], inrange_lookup: &[Vec<u8>], base_types: &[TowerType]) -> (f64, Vec<TowerType>, Vec<u8>) {
    let nbits: usize = towers.iter().map(|t| t.negotiable as usize).sum();
    let mut buffer = vec![0u8; nbits.div_ceil(8)];
    File::open("/dev/urandom").unwrap().read_exact(&mut buffer).unwrap();
    let mut i = 0;
    let mut new_types = base_types.to_vec();
    let mut stacks = vec![0; towers.len()];
    for (j, tower) in towers.iter().enumerate() {
        if tower.negotiable {
            if buffer[i / 8] & (1 << (i % 8)) != 0 {
                new_types[j] = TowerType::Bloonjitsu;
            } else {
                new_types[j] = TowerType::Shinobi;
                for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                    *s += *r;
                }
            }
            i += 1;
        }
    }
    max_optimize(towers, inrange_lookup, &new_types, &stacks, 1)
}

fn max_optimize(towers: &[Tower], inrange_lookup: &[Vec<u8>], tower_types: &[TowerType], stacks: &[u8], depth: u8) -> (f64, Vec<TowerType>, Vec<u8>) {
    let (mut score, mut new_types, mut new_stacks) = optimize(towers, inrange_lookup, tower_types, stacks, depth);
    loop {
        let tmp_score;
        (tmp_score, new_types, new_stacks) = optimize(towers, inrange_lookup, &new_types, &new_stacks, depth);
        if tmp_score - score < 0.1 {
            return (tmp_score, new_types, new_stacks);
        }
        score = tmp_score;
    }
}

fn optimize(towers: &[Tower], inrange_lookup: &[Vec<u8>], tower_types: &[TowerType], stacks: &[u8], depth: u8) -> (f64, Vec<TowerType>, Vec<u8>) {
    let base = get_setup_dps(towers, tower_types, stacks);
    let mut best_improvement = (base, tower_types.to_vec(), stacks.to_vec());
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Other => {
                continue;
            },
            TowerType::Bloonjitsu => {
                true
            },
            TowerType::Shinobi => {
                false
            }
        };
            
        let mut new_types = tower_types.to_vec();
        let mut new_stacks = stacks.to_vec();
        if new_shinobi {
            new_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s += *r;
            }
        } else {
            new_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s -= *r;
            }
        }
        if depth == 0 {
        // let dps = if depth == 0 {
            let dps = get_setup_dps(towers, &new_types, &new_stacks);
            if dps > best_improvement.0 {
                best_improvement = (dps, new_types, new_stacks);
            }
            // get_setup_dps(towers, &new_types, &new_stacks)
        } else {
            let (dps, new_types, new_stacks) = optimize(towers, inrange_lookup, &new_types, &new_stacks, depth - 1);
            if dps > best_improvement.0 {
                best_improvement = (dps, new_types, new_stacks);
            }
            // optimize(towers, inrange_lookup, &new_types, &new_stacks, depth - 1).0
        };
        // if dps > best_improvement.0 {
        //     best_improvement = (dps, new_types, new_stacks);
        // }
    }
    best_improvement
}

// fn change_tower(towers: &[Tower], inrange_lookup: &[Vec<u8>], tower_types: &mut [TowerType], stacks: &mut [u8], i: usize) {
//     let tower = &towers[i];
//     if !tower.negotiable {
//         return;
//     }
//     let new_shinobi = match tower_types[i] {
//         TowerType::Other => {
//             return;
//         },
//         TowerType::Bloonjitsu => {
//             true
//         },
//         TowerType::Shinobi => {
//             false
//         }
//     };
//     if new_shinobi {
//         tower_types[i] = TowerType::Shinobi;
//         for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
//             *s += *r;
//         }
//     } else {
//         tower_types[i] = TowerType::Bloonjitsu;
//         for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
//             *s -= *r;
//         }
//     }
// }

fn optimize_mut_inner(towers: &[Tower], inrange_lookup: &[Vec<u8>], tower_types: &mut [TowerType], stacks: &mut [u8], depth: u8) -> f64 {
    let mut best_dps: f64 = get_setup_dps(towers, tower_types, stacks);
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Other => {
                continue;
            },
            TowerType::Bloonjitsu => {
                true
            },
            TowerType::Shinobi => {
                false
            }
        };
        if new_shinobi {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += *r;
            }
        } else {
            tower_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s -= *r;
            }
        }
        if depth == 0 {
            let dps = get_setup_dps(towers, tower_types, stacks);
            if dps > best_dps {
                best_dps = dps;
            }
        } else {
            let dps = optimize_mut_inner(towers, inrange_lookup, tower_types, stacks, depth - 1);
            if dps > best_dps {
                best_dps = dps;
            }
        };
        if new_shinobi {
            tower_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s -= *r;
            }
        } else {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += *r;
            }
        }
    }
    best_dps

}

fn optimize_mut(towers: &[Tower], inrange_lookup: &[Vec<u8>], tower_types: &mut [TowerType], stacks: &mut [u8], depth: u8) -> f64 {
    let mut best_dps = get_setup_dps(towers, tower_types, stacks);
    let mut best_index = None;
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Other => {
                continue;
            },
            TowerType::Bloonjitsu => {
                true
            },
            TowerType::Shinobi => {
                false
            }
        };
        if new_shinobi {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += *r;
            }
        } else {
            tower_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s -= *r;
            }
        }
        if depth == 0 {
            let dps = get_setup_dps(towers, tower_types, stacks);
            if dps > best_dps {
                best_dps = dps;
                best_index = Some(i);
            }
        } else {
            let dps = optimize_mut_inner(towers, inrange_lookup, tower_types, stacks, depth - 1);
            if dps > best_dps {
                best_dps = dps;
                best_index = Some(i);
            }
        };
        if new_shinobi {
            tower_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s -= *r;
            }
        } else {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += *r;
            }
        }
    }
    if let Some(i) = best_index {
        match tower_types[i] {
            TowerType::Other => (),
            TowerType::Bloonjitsu => {
                tower_types[i] = TowerType::Shinobi;
                for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                    *s += *r;
                }
            },
            TowerType::Shinobi => {
                tower_types[i] = TowerType::Bloonjitsu;
                for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                    *s -= *r;
                }
            }
        };
    }
    best_dps
}

const SIZE: (f64, f64) = (900.0, 696.0);
fn game_pos_to_window_pos(pos: (f64, f64)) -> (f64, f64) {
    ((pos.0 * 3.0)+(SIZE.0 / 2.0), (pos.1 * 3.0)+(SIZE.1 / 2.0))
}


fn render(d: &mut RaylibDrawHandle, towers: &[Tower], tower_types: &[TowerType], stacks: &[u8], selected_tower: Option<usize>) {
    d.clear_background(Color::WHITE);
    for (i, tower) in towers.iter().enumerate() {
        let pos = game_pos_to_window_pos((tower.pos_x, tower.pos_y));
        let (radius, colour) = match tower_types[i] {
            TowerType::Other => (33.0, Color::new(0, 200, 200, 255)),
            TowerType::Shinobi => (18.0, Color::new(0, 200, 0, 255)),
            TowerType::Bloonjitsu => (18.0, Color::new(200, 200, 0, 255)),
        };
        d.draw_circle(pos.0 as i32, pos.1 as i32, radius, colour);
    }
    let dps = get_setup_dps(towers, tower_types, stacks).round();
    d.draw_text(&format!("{dps}"), 0, 0, 16, Color::BLACK);
    if let Some(index) = selected_tower {
        let tower = &towers[index];
        let pos = game_pos_to_window_pos((tower.pos_x, tower.pos_y));
        d.draw_circle(pos.0 as i32, pos.1 as i32, tower.range as f32 * 3.0, Color::new(0,0,0,30));
        let mut stack_count = stacks[index];
        if stack_count > 20 {
            d.draw_text(&format!("stacks: 20 ({stack_count})"), 0, 16, 16, Color::new(0, 0, 0, 255));
        } else {
            d.draw_text(&format!("stacks: {stack_count}"), 0, 16, 16, Color::new(0, 0, 0, 255));
        }
        let value = if tower_types[index] == TowerType::Bloonjitsu {
            tower.jitsu_value
        } else {
            tower.shinobi_value
        };
        stack_count = stack_count.min(20);
        d.draw_text(&format!("base value: {}", value), 0, 32, 16, Color::new(0,0,0,255));
        d.draw_text(&format!("value: {}", value as f64 * get_attack_speed_buff(stack_count) * get_pierce_buff(stack_count)), 0, 48, 16, Color::new(0,0,0,255));
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pops_jitsu = read_json("Logs_FreeplaySetupPops.json")?;
    let pops_shinobi = read_json("Logs_FreeplaySetupPopsShi.json")?;
    let actual_setup = read_json("Logs_FreeplaySetup.json")?;
    let pops_jitsu: Freeplay = serde_json::from_value(pops_jitsu)?;
    let pops_shinobi: Freeplay = serde_json::from_value(pops_shinobi)?;
    let actual_setup: Freeplay = serde_json::from_value(actual_setup)?;

    let jitsu_pop_lookup: HashMap<_,_> = pops_jitsu.placed_towers.iter().map(|t| (t.id_last_save, t.damage_dealt)).collect();
    let shinobi_pop_lookup: HashMap<_,_> = pops_shinobi.placed_towers.iter().map(|t| (t.id_last_save, t.damage_dealt)).collect();

    let villages: Vec<_> = actual_setup.placed_towers.iter().filter(|tower| tower.base_id == "MonkeyVillage").map(|tower| Tower::new(tower, if tower.path_one_tier == 5 {VILLAGE_RANGE} else {HOMELAND_RANGE}).into_nonnegotiable().into_unbuffable()).collect();
    
    let mut towers = Vec::new();
    let mut types = Vec::new();
    // let mut tower_index = 0;
    for tower in actual_setup.placed_towers {
        if tower.base_id == "NinjaMonkey" {
            let jitsu_value = jitsu_pop_lookup.get(&tower.id_last_save).copied().unwrap_or_default();
            let shinobi_value = shinobi_pop_lookup.get(&tower.id_last_save).copied().unwrap_or_default();
            if tower.path_one_tier == 6 {
                types.push(TowerType::Shinobi);
                towers.push(Tower::new(&tower, NINJAGON_RANGE).into_nonnegotiable().into_unbuffable());
            }
            else if (4..=5).contains(&tower.path_one_tier) {
            // else if ![2, 3, 4, 5, 6, 7, 8, 15, 16, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 35, 42, 44, 45, 50, 51, 53, 57, 58, 59, 60, 61, 66, 67, 68, 69, 71, 74, 75, 76, 83, 88, 89, 90, 91, 92, 93, 94, 99, 100, 101, 102, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 120, 129, 133, 134, 135, 138, 139].contains(&tower_index) {
                let mut jitsu = Tower::new(&tower, 0.0).with_values(jitsu_value.try_into().unwrap(), shinobi_value.try_into().unwrap());
                let range = if villages
                    .iter()
                    .any(|v| tower_is_in_range(&jitsu, v, v.range))
                {
                    MAX_RANGE
                } else {
                    RANGE_NO_VILLAGE
                };
                jitsu.range = range;
                types.push(TowerType::Bloonjitsu);
                towers.push(jitsu);
            } else if tower.path_two_tier >= 3 {
            // } else {
                let mut shinobi = Tower::new(&tower, 0.0).with_values(jitsu_value.try_into().unwrap(), shinobi_value.try_into().unwrap());
                let range = if villages
                    .iter()
                    .any(|v| tower_is_in_range(&shinobi, v, v.range))
                {
                    MAX_RANGE
                } else {
                    RANGE_NO_VILLAGE
                };
                shinobi.range = range;
                types.push(TowerType::Shinobi);
                towers.push(shinobi);
            }
            // tower_index += 1;
        } else if tower.base_id == "MonkeyVillage" {
            let range = if tower.path_one_tier == 5 {VILLAGE_RANGE} else {HOMELAND_RANGE};
            types.push(TowerType::Other);
            towers.push(Tower::new(&tower, range).into_nonnegotiable().into_unbuffable());
        }
    }

    let mut stacks = vec![0; towers.len()];
    let mut inrange_lookup = vec![vec![0; towers.len()]; towers.len()];

    for (i, tower) in towers.iter().enumerate() {
        for (j, other) in towers.iter().enumerate() {
            if tower_is_in_range(other, tower, tower.range) {
                inrange_lookup[i][j] = 1;
                if types[i] == TowerType::Shinobi {
                    stacks[j] += 1;
                }
                // inrange_lookup[i].push(j);
                // if types[i] == TowerType::Shinobi {
                //     stacks[j] += 1;
                // }
            }
        }
    }

    // let ninjagon = setup_old
    //     .placed_towers
    //     .iter()
    //     .find(|tower| tower.base_id == "NinjaMonkey" && tower.path_one_tier == 6)
    //     .unwrap()
    //     .clone();

    // let mut jitsus: Vec<Tower> = Vec::new();
    // let mut shinobis: Vec<Tower> = vec![
    //     Tower::new(ninjagon, NINJAGON_RANGE)
    //         .into_nonnegotiable()
    //         .into_unbuffable(),
    // ];
    // let towers = setup.placed_towers;
    // let towers_2 = setup_shinobis.placed_towers;
    // let actual_towers = actual_setup.placed_towers;

    // let mut ninjas = Vec::new();
    // let mut ninjas_2 = Vec::new();
    // let mut ninjas_actual = Vec::new();

    // for tower in towers {
    //     if tower.base_id == "NinjaMonkey" {
    //         ninjas.push(tower)
    //     } else if tower.base_id == "MonkeyVillage" {
    //         if tower.path_one_tier == 5 {
    //             villages.push(
    //                 Tower::new(tower, VILLAGE_RANGE)
    //                     .into_nonnegotiable()
    //                     .into_unbuffable(),
    //             );
    //         } else {
    //             villages.push(
    //                 Tower::new(tower, HOMELAND_RANGE)
    //                     .into_nonnegotiable()
    //                     .into_unbuffable(),
    //             );
    //         }
    //     }
    // }

    // for tower in towers_2 {
    //     if tower.base_id == "NinjaMonkey" {
    //         ninjas_2.push(tower);
    //     }
    // }
    // for tower in actual_towers {
    //     if tower.base_id == "NinjaMonkey" {
    //         ninjas_actual.push(tower);
    //     }
    // }

    // let mut jitsu_id_values: HashMap<i64, i64> = HashMap::new();
    // let mut shinobi_id_values: HashMap<i64, i64> = HashMap::new();

    // for ninja in ninjas_actual {
    //     if ninja.path_one_tier >= 4 && ninja.path_one_tier < 6 {
    //         let mut current_range = RANGE_NO_VILLAGE;
    //         let ninja_tower = Tower::new(ninja.clone(), 0.0);
    //         if villages
    //             .iter()
    //             .any(|v| tower_is_in_range(&ninja_tower, v, v.range))
    //         {
    //             current_range = MAX_RANGE;
    //         }
    //         let value = *jitsu_id_values.get(&ninja.id_last_save).unwrap();
    //         jitsus.push(Tower::new(ninja, current_range).with_value(value));
    //     } else if ninja.path_two_tier >= 3 {
    //         let mut current_range = RANGE_NO_VILLAGE;
    //         let ninja_tower = Tower::new(ninja.clone(), 0.0);
    //         if villages
    //             .iter()
    //             .any(|v| tower_is_in_range(&ninja_tower, v, v.range))
    //         {
    //             current_range = MAX_RANGE;
    //         }
    //         let value = *shinobi_id_values.get(&ninja.id_last_save).unwrap();
    //         shinobis.push(Tower::new(ninja, current_range).with_value(value));
    //     }
    // }
    
    // let all_ninjas = [jitsus.as_slice(), shinobis.as_slice()].concat();

    // let mut id_inrange_lookup = HashMap::new();

    // for jitsu in all_ninjas.iter().chain(&villages) {
    //     let mut lookup_entry = Vec::new();
    //     for jitsu2 in &all_ninjas {
    //         if jitsu.id != jitsu2.id && tower_is_in_range(jitsu2, jitsu, jitsu.range){
    //             lookup_entry.push(jitsu2.clone());
    //         }
    //     }
    //     id_inrange_lookup.insert(jitsu.id, lookup_entry);
    // }

    // let ninjagon = shinobis[0].clone();
    // {
    //     let mut lookup_entry = Vec::new();
    //     for jitsu2 in &all_ninjas {
    //         if tower_is_in_range(jitsu2, &ninjagon, ninjagon.range) {
    //             lookup_entry.push(jitsu2.clone());
    //         }
    //     }
    //     id_inrange_lookup.insert(ninjagon.id, lookup_entry);
    // }

    let mut selected_tower: Option<usize> = None;
    let (mut rl, thread) = raylib::init().size(900, 696).title("pop optimizer").build();
    rl.set_exit_key(None);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);
        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_O) {
            let start = std::time::SystemTime::now();
            println!("Lets OPTIMISE!");
            let score;
            (score, types, stacks) = optimize(&towers, &inrange_lookup, &types, &stacks, 1);
            println!("optimisation score: {score}");
            let end = std::time::SystemTime::now();
            match end.duration_since(start) {
                Ok(duration) => {println!("Optimising took {}s", duration.as_secs_f64())},
                Err(_) => {println!("Optimising took < 0s??");}
            }
        }
        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_M) {
            let start = std::time::SystemTime::now();
            println!("Lets OPTIMISE!");
            let score = optimize_mut(&towers, &inrange_lookup, &mut types, &mut stacks, 1);
            println!("optimisation score: {score}");
            let end = std::time::SystemTime::now();
            match end.duration_since(start) {
                Ok(duration) => {println!("Optimising took {}s", duration.as_secs_f64())},
                Err(_) => {println!("Optimising took < 0s??");}
            }
        }
        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_C) {
            let dps = full_calc_dps(&towers, &types);
            println!("dps: {dps}");
        }
        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_R) {
            let start = std::time::SystemTime::now();
            let score;
            let new_types;
            let new_stacks;
            (score, new_types, new_stacks) = rand_optimize(&towers, &inrange_lookup, &types);
            let end = std::time::SystemTime::now();
            let duration = match end.duration_since(start) {
                Ok(duration) => duration.as_secs_f64(),
                Err(_) => 0.0,
            };
            println!("optimized in {duration}s optimisation score: {score}");
            // if score > get_setup_dps(&towers, &types, &stacks) {
                types = new_types;
                stacks = new_stacks;
            // }
        }
        if d.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
            // let pos = window_pos_to_game_pos((mouse_pos.x as f64, mouse_pos.y as f64));
            selected_tower = None;
            let pos = (mouse_pos.x as f64, mouse_pos.y as f64);
            for (i, ninja) in towers.iter().enumerate() {
                let ninja_pos = game_pos_to_window_pos((ninja.pos_x, ninja.pos_y));
                if is_in_range(pos, ninja_pos, 18.0) {
                    selected_tower = Some(i);
                }
            }
        }
        render(&mut d, &towers, &types, &stacks, selected_tower);
    }

    Ok(())
}
