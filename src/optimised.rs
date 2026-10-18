use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufReader, Read},
    path::Path,
    sync::atomic::AtomicUsize,
};

use raylib::{
    color::Color, ffi::KeyboardKey, prelude::{RaylibDraw, RaylibDrawHandle},
};
use serde_json::Value;

use crate::types_new::{Freeplay, PlacedTower};

mod apply_threads;
mod types_new;

fn read_json<P: AsRef<Path>>(path: P) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let file = BufReader::new(file);
    let json: Value = serde_json::from_reader(file)?;
    Ok(json)
}

//computes a float to a small unsigned power at comptime
const fn fpowu(f: f64, mut i: u8) -> f64 {
    if i == 0 {
        return 1f64;
    }
    let mut value: f64 = f;
    while i > 1 {
        i -= 1;
        value *= f;
    }
    value
}

const MAX_RANGE_ETIENNE: f64 = const {
    let base = 40.0;
    let village_buff = 0.1;
    let alchemist_buff = 0.15;
    let etienne_buff = 0.2;
    let tsg_mult = 1.4;
    let seeking_shuriken_upgrade = 7.0;
    let ninjagon_upgrade = 10.0;
    base * (1.0 + village_buff + alchemist_buff + etienne_buff) * tsg_mult
        + seeking_shuriken_upgrade
        + ninjagon_upgrade
};
const RANGE_ETIENNE: f64 = const {
    let base = 40.0;
    let alchemist_buff = 0.15;
    let etienne_buff = 0.2;
    let tsg_mult = 1.4;
    let seeking_shuriken_upgrade = 7.0;
    let ninjagon_upgrade = 10.0;
    base * (1.0 + alchemist_buff + etienne_buff) * tsg_mult
        + seeking_shuriken_upgrade
        + ninjagon_upgrade
};
const MAX_RANGE: f64 = const {
    let base = 40.0;
    let village_buff = 0.1;
    let alchemist_buff = 0.15;
    let tsg_mult = 1.4;
    let seeking_shuriken_upgrade = 7.0;
    let ninjagon_upgrade = 10.0;
    base * (1.0 + village_buff + alchemist_buff) * tsg_mult
        + seeking_shuriken_upgrade
        + ninjagon_upgrade
};
const RANGE_NO_VILLAGE: f64 = const {
    let base = 40.0;
    let alchemist_buff = 0.15;
    let tsg_mult = 1.4;
    let seeking_shuriken_upgrade = 7.0;
    let ninjagon_upgrade = 10.0;
    base * (1.0 + alchemist_buff) * tsg_mult + seeking_shuriken_upgrade + ninjagon_upgrade
};
const ETIENNE_RANGE: f64 = const {
    let base = 80.0;
    let desperado_buff = 0.5;
    let village_buff = 0.1;
    let alchemist_buff = 0.15;
    let final_harmonic_buff = 0.15;
    let tsg_mult = 1.4;
    base * (1.0 + desperado_buff + village_buff + alchemist_buff + final_harmonic_buff) * tsg_mult
};
const NINJAGON_RANGE: f64 = 70.0;
const VILLAGE_RANGE: f64 = const {
    let mut base = 55.0;
    base += (0.1 * base) + ((fpowu(1.015, 10) - 1.0) * base) + (0.4 * base) + (0.5 * base);
    base * 1.15
}; // 1.1x oc 1.015^10x for uboost, 1.2x 1.2x 1.15x 1.5x
const HOMELAND_RANGE: f64 = 55.0 + (0.4 * 55.0);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TowerType {
    Bloonjitsu,
    Shinobi,
}

#[derive(Clone, Debug)]
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

#[derive(Clone)]
struct SimTower {
    pub jitsu_value: f64,
    pub shinobi_value: f64,
    pub negotiable: bool,
}

impl From<&Tower> for SimTower {
    fn from(value: &Tower) -> Self {
        Self {
            jitsu_value: value.jitsu_value as f64,
            shinobi_value: value.shinobi_value as f64,
            negotiable: value.negotiable,
        }
    }
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

const fn get_attack_speed_buff(stack_count: u8) -> f64 {
    fpowu(1.08, stack_count)
}

const fn get_pierce_buff(stack_count: u8) -> f64 {
    1.0 + 0.08 * (stack_count as f64)
}

const fn get_damage_buff2(stack_count: u8) -> f64 {
    get_attack_speed_buff(stack_count) * get_pierce_buff(stack_count)
}

fn get_damage_buff(stack_count: u8) -> f64 {
    match stack_count {
        0 => const { get_damage_buff2(0) },
        1 => const { get_damage_buff2(1) },
        2 => const { get_damage_buff2(2) },
        3 => const { get_damage_buff2(3) },
        4 => const { get_damage_buff2(4) },
        5 => const { get_damage_buff2(5) },
        6 => const { get_damage_buff2(6) },
        7 => const { get_damage_buff2(7) },
        8 => const { get_damage_buff2(8) },
        9 => const { get_damage_buff2(9) },
        10 => const { get_damage_buff2(10) },
        11 => const { get_damage_buff2(11) },
        12 => const { get_damage_buff2(12) },
        13 => const { get_damage_buff2(13) },
        14 => const { get_damage_buff2(14) },
        15 => const { get_damage_buff2(15) },
        16 => const { get_damage_buff2(16) },
        17 => const { get_damage_buff2(17) },
        18 => const { get_damage_buff2(18) },
        19 => const { get_damage_buff2(19) },
        _ => const { get_damage_buff2(20) },
    }
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
            let damage_buff = get_damage_buff(stacks[i]);
            setup_dps += value as f64 * damage_buff;
        }
    }
    setup_dps
}

static DPS_CALCS: AtomicUsize = AtomicUsize::new(0);

fn get_sim_setup_dps(towers: &[SimTower], tower_types: &[TowerType], stacks: &[u8]) -> f64 {
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
            let damage_buff = get_damage_buff(stacks[i]);
            setup_dps += value * damage_buff;
        }
    }
    setup_dps
}

fn calc_stacks(towers: &[Tower], tower_types: &[TowerType]) -> Vec<u8> {
    let mut stacks = vec![0; towers.len()];
    for (i, tower) in towers.iter().enumerate() {
        if tower_types[i] == TowerType::Shinobi {
            for (j, other) in towers.iter().enumerate() {
                if tower_is_in_range(other, tower, tower.range) {
                    stacks[j] += 1;
                }
            }
        }
    }
    stacks
}

fn full_calc_dps(towers: &[Tower], tower_types: &[TowerType]) -> f64 {
    let stacks = calc_stacks(towers, tower_types);
    get_setup_dps(towers, tower_types, &stacks)
}

fn rand_optimize(
    towers: &[SimTower],
    inrange_lookup: &[Vec<bool>],
    base_types: &[TowerType],
) -> (f64, Vec<TowerType>, Vec<u8>) {
    let nbits: usize = towers.iter().map(|t| t.negotiable as usize).sum();
    let mut buffer = vec![0u8; nbits.div_ceil(8)];
    File::open("/dev/urandom")
        .unwrap()
        .read_exact(&mut buffer)
        .unwrap();
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
                    *s += (*r) as u8;
                }
            }
            i += 1;
        }
    }
    max_optimize(towers, inrange_lookup, &new_types, &stacks, 1)
}

fn max_optimize(
    towers: &[SimTower],
    inrange_lookup: &[Vec<bool>],
    tower_types: &[TowerType],
    stacks: &[u8],
    depth: u8,
) -> (f64, Vec<TowerType>, Vec<u8>) {
    let (mut score, mut new_types, mut new_stacks) =
        optimize(towers, inrange_lookup, tower_types, stacks, depth);
    loop {
        let tmp_score;
        (tmp_score, new_types, new_stacks) =
            optimize(towers, inrange_lookup, &new_types, &new_stacks, depth);
        if tmp_score - score < 0.1 {
            return (tmp_score, new_types, new_stacks);
        }
        score = tmp_score;
    }
}

fn optimize(
    towers: &[SimTower],
    inrange_lookup: &[Vec<bool>],
    tower_types: &[TowerType],
    stacks: &[u8],
    depth: u8,
) -> (f64, Vec<TowerType>, Vec<u8>) {
    let base = get_sim_setup_dps(towers, tower_types, stacks);
    let mut best_improvement = (base, tower_types.to_vec(), stacks.to_vec());
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Bloonjitsu => true,
            TowerType::Shinobi => false,
        };

        let mut new_types = tower_types.to_vec();
        let mut new_stacks = stacks.to_vec();
        if new_shinobi {
            new_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s += (*r) as u8;
            }
        } else {
            new_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s -= (*r) as u8;
            }
        }
        if depth == 0 {
            let dps = get_sim_setup_dps(towers, &new_types, &new_stacks);
            if dps > best_improvement.0 {
                best_improvement = (dps, new_types, new_stacks);
            }
        } else {
            let (dps, new_types, new_stacks) =
                optimize(towers, inrange_lookup, &new_types, &new_stacks, depth - 1);
            if dps > best_improvement.0 {
                best_improvement = (dps, new_types, new_stacks);
            }
        };
    }
    best_improvement
}

fn optimize_25(
    towers: &[Tower],
    inrange_lookup: &[Vec<bool>],
    tower_types: &[TowerType],
    stacks: &[u8],
    depth: u8,
) -> (f64, Vec<TowerType>, Vec<u8>) {
    let base = get_setup_dps(towers, tower_types, stacks);
    let mut best_improvement = (base, tower_types.to_vec(), stacks.to_vec());
    let mut states = Vec::new();
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Bloonjitsu => true,
            TowerType::Shinobi => false,
        };

        let mut new_types = tower_types.to_vec();
        let mut new_stacks = stacks.to_vec();
        if new_shinobi {
            new_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s += (*r) as u8;
            }
        } else {
            new_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s -= (*r) as u8;
            }
        }
        let dps = get_setup_dps(towers, &new_types, &new_stacks);
        states.push((dps, new_types, new_stacks));
    }
    states.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (_, new_types, new_stacks) in states.iter().rev().take(states.len() / 4) {
        if depth == 0 {
            let dps = get_setup_dps(towers, new_types, new_stacks);
            if dps > best_improvement.0 {
                best_improvement = (dps, new_types.to_vec(), new_stacks.to_vec());
            }
        } else {
            let (dps, new_types, new_stacks) =
                optimize_25(towers, inrange_lookup, new_types, new_stacks, depth - 1);
            if dps > best_improvement.0 {
                best_improvement = (dps, new_types, new_stacks);
            }
        };
    }
    best_improvement
}

fn tabu_get_neighbors(towers: &[Tower], tower_types: &[TowerType]) -> Vec<Vec<TowerType>> {
    let mut neighbors = Vec::new();
    for i in 0..tower_types.len() {
        if !towers[i].negotiable {
            continue;
        }
        let mut new_neighbor = tower_types.to_vec();
        new_neighbor[i] = match new_neighbor[i] {
            TowerType::Bloonjitsu => TowerType::Shinobi,
            TowerType::Shinobi => TowerType::Bloonjitsu,
        };
        neighbors.push(new_neighbor);
    }
    neighbors
}

fn tabu_search(
    towers: &[Tower],
    tower_types: &[TowerType],
    tabu_tenure: usize,
    depth: usize,
) -> Vec<TowerType> {
    let mut best_sol = tower_types.to_vec();
    let mut current_sol = best_sol.clone();
    let mut current_best_dps = full_calc_dps(towers, tower_types);
    let mut tabu_list: HashMap<Vec<TowerType>, usize> = HashMap::new();
    for i in 0..depth {
        let mut neighbours = tabu_get_neighbors(towers, &current_sol)
            .into_iter()
            .map(|n| {
                let dps = full_calc_dps(towers, &n);
                (n, dps)
            })
            .collect::<Vec<_>>();
        neighbours.sort_by(|(_, d1), (_, d2)| {
            // Reverse sort: highest score first.
            d2.total_cmp(d1)
        });
        let mut best_canditate = None;
        for (candidate, dps) in neighbours.clone() {
            let is_tabu = tabu_list.get(&candidate).copied().unwrap_or_default() > i;
            let is_best = dps > current_best_dps;
            if !is_tabu || is_best {
                best_canditate = Some((candidate, dps));
                break;
            }
        }
        let dps;
        (current_sol, dps) = best_canditate.unwrap_or_else(|| {
            neighbours
                .iter()
                .min_by_key(|&(n, _)| tabu_list.get(n).copied().unwrap_or_default())
                .expect("expected state to have neighbors")
                .clone()
        });
        if dps > current_best_dps {
            best_sol = current_sol.clone();
            current_best_dps = dps;
        }
        tabu_list.insert(current_sol.clone(), i + tabu_tenure);
    }
    best_sol
}

fn optimize_thread(
    towers: &[SimTower],
    inrange_lookup: &[Vec<bool>],
    tower_types: &[TowerType],
    stacks: &[u8],
    depth: u8,
) -> (f64, Vec<TowerType>, Vec<u8>) {
    if depth == 0 {
        return optimize(towers, inrange_lookup, tower_types, stacks, depth);
    }
    let depth = depth - 1;
    let mut states = vec![(tower_types.to_vec(), stacks.to_vec())];
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Bloonjitsu => true,
            TowerType::Shinobi => false,
        };

        let mut new_types = tower_types.to_vec();
        let mut new_stacks = stacks.to_vec();
        if new_shinobi {
            new_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s += (*r) as u8;
            }
        } else {
            new_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(new_stacks.iter_mut()) {
                *s -= (*r) as u8;
            }
        }
        states.push((new_types, new_stacks));
    }
    let results = apply_threads::apply_threads(
        12,
        |(towers, inrange_lookup, tower_types, stacks, depth), _| {
            optimize(towers, inrange_lookup, tower_types, stacks, depth)
        },
        &states
            .iter()
            .map(|state| (towers, inrange_lookup, &state.0, &state.1, depth))
            .collect::<Vec<_>>(),
        None,
    )
    .unwrap();
    results
        .iter()
        .max_by(|a, b| a.0.total_cmp(&b.0))
        .unwrap()
        .clone()
}

fn optimize_mut_inner(
    towers: &[Tower],
    inrange_lookup: &[Vec<bool>],
    tower_types: &mut [TowerType],
    stacks: &mut [u8],
    depth: u8,
) -> f64 {
    let mut best_dps: f64 = get_setup_dps(towers, tower_types, stacks);
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Bloonjitsu => true,
            TowerType::Shinobi => false,
        };
        if new_shinobi {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += (*r) as u8;
            }
        } else {
            tower_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s -= (*r) as u8;
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
                *s -= (*r) as u8;
            }
        } else {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += (*r) as u8;
            }
        }
    }
    best_dps
}

fn optimize_mut(
    towers: &[Tower],
    inrange_lookup: &[Vec<bool>],
    tower_types: &mut [TowerType],
    stacks: &mut [u8],
    depth: u8,
) -> f64 {
    let mut best_dps = get_setup_dps(towers, tower_types, stacks);
    let mut best_index = None;
    for i in 0..towers.len() {
        let tower = &towers[i];
        if !tower.negotiable {
            continue;
        }
        let new_shinobi = match tower_types[i] {
            TowerType::Bloonjitsu => true,
            TowerType::Shinobi => false,
        };
        if new_shinobi {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += (*r) as u8;
            }
        } else {
            tower_types[i] = TowerType::Bloonjitsu;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s -= (*r) as u8;
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
                *s -= (*r) as u8;
            }
        } else {
            tower_types[i] = TowerType::Shinobi;
            for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                *s += (*r) as u8;
            }
        }
    }
    if let Some(i) = best_index {
        match tower_types[i] {
            TowerType::Bloonjitsu => {
                tower_types[i] = TowerType::Shinobi;
                for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                    *s += (*r) as u8;
                }
            }
            TowerType::Shinobi => {
                tower_types[i] = TowerType::Bloonjitsu;
                for (r, s) in inrange_lookup[i].iter().zip(stacks.iter_mut()) {
                    *s -= (*r) as u8;
                }
            }
        };
    }
    best_dps
}

const SIZE: (f64, f64) = (900.0, 696.0);
fn game_pos_to_window_pos(pos: (f64, f64)) -> (f64, f64) {
    (
        (pos.0 * 3.0) + (SIZE.0 / 2.0),
        (pos.1 * 3.0) + (SIZE.1 / 2.0),
    )
}

#[derive(Debug, Clone, Copy)]
enum SelectedTower {
    None,
    Ninja(usize),
    Village(usize),
    Etienne,
}

fn render(
    d: &mut RaylibDrawHandle,
    towers: &[Tower],
    villages: &[Tower],
    etienne: &Option<Tower>,
    tower_types: &[TowerType],
    stacks: &[u8],
    selected_tower: SelectedTower,
) {
    let font_size = 16;
    let mut text_y = 1;
    d.clear_background(Color::WHITE);
    for (i, tower) in towers.iter().enumerate() {
        let pos = game_pos_to_window_pos((tower.pos_x, tower.pos_y));
        let (radius, colour) = match tower_types[i] {
            TowerType::Shinobi => (18.0, Color::new(0, 200, 0, 255)),
            TowerType::Bloonjitsu => (18.0, Color::new(200, 200, 0, 255)),
        };
        d.draw_circle(pos.0 as i32, pos.1 as i32, radius, colour);
    }
    for village in villages {
        let pos = game_pos_to_window_pos((village.pos_x, village.pos_y));
        d.draw_circle(
            pos.0 as i32,
            pos.1 as i32,
            33.0,
            Color::new(0, 200, 200, 255),
        );
    }
    if let Some(etienne) = etienne {
        let pos = game_pos_to_window_pos((etienne.pos_x, etienne.pos_y));
        d.draw_circle(
            pos.0 as i32,
            pos.1 as i32,
            18.0,
            Color::new(255, 165, 0, 255),
        );
    }
    let dps = get_setup_dps(towers, tower_types, stacks).round();
    d.draw_text(&format!("value: {dps}"), 1, text_y, font_size, Color::BLACK);
    text_y += font_size;
    d.draw_text(
        &format!(
            "jitsus: {}, shinobis: {}",
            tower_types
                .iter()
                .filter(|&&t| t == TowerType::Bloonjitsu)
                .count(),
            tower_types
                .iter()
                .filter(|&&t| t == TowerType::Shinobi)
                .count()
        ),
        1,
        text_y,
        font_size,
        Color::BLACK,
    );
    text_y += font_size;
    if let Some(tower) = match selected_tower {
        SelectedTower::None => None,
        SelectedTower::Etienne => etienne.as_ref(),
        SelectedTower::Ninja(i) => {
            let tower = &towers[i];
            let mut stack_count = stacks[i];
            if stack_count > 20 {
                d.draw_text(
                    &format!("stacks: 20 ({stack_count})"),
                    1,
                    text_y,
                    font_size,
                    Color::new(0, 0, 0, 255),
                );
                text_y += font_size;
            } else {
                d.draw_text(
                    &format!("stacks: {stack_count}"),
                    1,
                    text_y,
                    font_size,
                    Color::new(0, 0, 0, 255),
                );
                text_y += font_size;
            }
            let value = if tower_types[i] == TowerType::Bloonjitsu {
                tower.jitsu_value
            } else {
                tower.shinobi_value
            };
            stack_count = stack_count.min(20);
            d.draw_text(
                &format!("base value: {}", value),
                1,
                text_y,
                font_size,
                Color::new(0, 0, 0, 255),
            );
            text_y += font_size;
            // safety: stack_count is <= 20
            d.draw_text(
                &format!(
                    "value: {}",
                    value as f64
                        * get_attack_speed_buff(stack_count)
                        * get_pierce_buff(stack_count)
                ),
                1,
                text_y,
                font_size,
                Color::new(0, 0, 0, 255),
            );
            Some(tower)
        }
        SelectedTower::Village(i) => Some(&villages[i]),
    } {
        let pos = game_pos_to_window_pos((tower.pos_x, tower.pos_y));
        d.draw_circle(
            pos.0 as i32,
            pos.1 as i32,
            tower.range as f32 * 3.0,
            Color::new(0, 0, 0, 30),
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // raylib::core::callbacks::set_trace_log_callback(|_, s| eprintln!("{s}")).unwrap();
    let args: Vec<_> = std::env::args().collect();
    let setup_dir = if let Some(x) = args.get(1) {
        Path::new(x)
    } else {
        eprintln!("Usage: ./optimised setup_path optimisation_level");
        return Ok(());
    };
    let optim_level = if let Some(x) = args.get(2) {
        x.parse()?
    } else {
        eprintln!("Usage: ./optimised setup_path optimisation_level");
        return Ok(());
    };
    
    let pops_jitsu: Value = read_json(setup_dir.join("pops.json"))?;
    let pops_shinobi: Value = read_json(setup_dir.join("pops_shinobi.json"))?;
    let actual_setup: Value = read_json(setup_dir.join("actual.json"))?;
    let pops_jitsu: Freeplay = serde_json::from_value(pops_jitsu)?;
    let pops_shinobi: Freeplay = serde_json::from_value(pops_shinobi)?;
    let actual_setup: Freeplay = serde_json::from_value(actual_setup)?;

    let jitsu_pop_lookup: HashMap<_, _> = pops_jitsu
        .placed_towers
        .iter()
        .map(|t| (t.id_last_save, t.damage_dealt))
        .collect();
    let shinobi_pop_lookup: HashMap<_, _> = pops_shinobi
        .placed_towers
        .iter()
        .map(|t| (t.id_last_save, t.damage_dealt))
        .collect();

    let villages: Vec<_> = actual_setup
        .placed_towers
        .iter()
        .filter(|tower| tower.base_id == "MonkeyVillage")
        .map(|tower| {
            Tower::new(
                tower,
                if tower.path_one_tier == 5 {
                    VILLAGE_RANGE
                } else {
                    HOMELAND_RANGE
                },
            )
            .into_nonnegotiable()
            .into_unbuffable()
        })
        .collect();
    let etienne = actual_setup
        .placed_towers
        .iter()
        .find(|tower| tower.base_id == "Etienne")
        .map(|tower| Tower::new(tower, ETIENNE_RANGE));

    let mut towers = Vec::new();
    let mut types = Vec::new();
    // let mut tower_index = 0;
    for tower in actual_setup.placed_towers {
        if tower.base_id == "NinjaMonkey" {
            let jitsu_value = jitsu_pop_lookup
                .get(&tower.id_last_save)
                .copied()
                .unwrap_or_default();
            let shinobi_value = shinobi_pop_lookup
                .get(&tower.id_last_save)
                .copied()
                .unwrap_or_default();
            if tower.path_one_tier == 6 {
                types.push(TowerType::Shinobi);
                towers.push(
                    Tower::new(&tower, NINJAGON_RANGE)
                        .into_nonnegotiable()
                        .into_unbuffable(),
                );
            } else if (4..=5).contains(&tower.path_one_tier) {
                let mut jitsu = Tower::new(&tower, 0.0).with_values(
                    jitsu_value.try_into().unwrap(),
                    shinobi_value.try_into().unwrap(),
                );
                let in_village = villages
                    .iter()
                    .any(|v| tower_is_in_range(&jitsu, v, v.range));
                let in_etienne = etienne
                    .as_ref()
                    .is_some_and(|e| tower_is_in_range(&jitsu, e, e.range));
                let range = match (in_village, in_etienne) {
                    (false, false) => RANGE_NO_VILLAGE,
                    (false, true) => RANGE_ETIENNE,
                    (true, false) => MAX_RANGE,
                    (true, true) => MAX_RANGE_ETIENNE,
                };
                jitsu.range = range;
                types.push(TowerType::Bloonjitsu);
                towers.push(jitsu);
            } else {
                let mut shinobi = Tower::new(&tower, 0.0).with_values(
                    jitsu_value.try_into().unwrap(),
                    shinobi_value.try_into().unwrap(),
                );
                let in_village = villages
                    .iter()
                    .any(|v| tower_is_in_range(&shinobi, v, v.range));
                let in_etienne = etienne
                    .as_ref()
                    .is_some_and(|e| tower_is_in_range(&shinobi, e, e.range));
                let range = match (in_village, in_etienne) {
                    (false, false) => RANGE_NO_VILLAGE,
                    (false, true) => RANGE_ETIENNE,
                    (true, false) => MAX_RANGE,
                    (true, true) => MAX_RANGE_ETIENNE,
                };
                shinobi.range = range;
                types.push(TowerType::Shinobi);
                towers.push(shinobi);
            }
        }
    }
    println!("[INFO] towers.len: {}", towers.len());

    let mut stacks = vec![0; towers.len()];
    let mut inrange_lookup = vec![vec![false; towers.len()]; towers.len()];

    for (i, tower) in towers.iter().enumerate() {
        for (j, other) in towers.iter().enumerate() {
            if tower_is_in_range(other, tower, tower.range) {
                inrange_lookup[i][j] = true;
                if types[i] == TowerType::Shinobi {
                    stacks[j] += 1;
                }
            }
        }
    }

    let sim_towers: Vec<SimTower> = towers.iter().map(Into::into).collect();

    let mut selected_tower = SelectedTower::None;
    let (mut rl, thread) = raylib::init().size(900, 696).title("pop optimizer").build();
    rl.set_exit_key(None);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);
        if d.is_key_pressed(KeyboardKey::KEY_F) {
            let mut start = std::time::SystemTime::now();
            let mut prev_score = 0.0;
            let mut score;
            (score, types, stacks) =
                optimize_thread(&sim_towers, &inrange_lookup, &types, &stacks, optim_level);
            while score > prev_score {
                prev_score = score;
                (score, types, stacks) =
                    optimize_thread(&sim_towers, &inrange_lookup, &types, &stacks, optim_level);
                render(
                    &mut d,
                    &towers,
                    &villages,
                    &etienne,
                    &types,
                    &stacks,
                    selected_tower,
                );
                drop(d);
                rl.window_should_close();
                d = rl.begin_drawing(&thread);
                let end = std::time::SystemTime::now();
                match end.duration_since(start) {
                    Ok(duration) => {
                        println!("New score {score} in {}s", duration.as_secs_f64())
                    }
                    Err(_) => {
                        println!("Optimising took < 0s??");
                    }
                }
                start = end;
            }
        }
        if d.is_key_pressed(KeyboardKey::KEY_O) {
            let start = std::time::SystemTime::now();
            println!("Lets OPTIMISE!");
            let score;
            (score, types, stacks) =
                optimize_thread(&sim_towers, &inrange_lookup, &types, &stacks, optim_level);
            println!("optimisation score: {score}");
            let end = std::time::SystemTime::now();
            match end.duration_since(start) {
                Ok(duration) => {
                    println!("Optimising took {}s", duration.as_secs_f64())
                }
                Err(_) => {
                    println!("Optimising took < 0s??");
                }
            }
        }
        if d.is_key_pressed(KeyboardKey::KEY_L) {
            let start = std::time::SystemTime::now();
            println!("Lets OPTIMISE!");
            let score;
            (score, types, stacks) = optimize_25(&towers, &inrange_lookup, &types, &stacks, optim_level);
            println!("optimisation score: {score}");
            let end = std::time::SystemTime::now();
            match end.duration_since(start) {
                Ok(duration) => {
                    println!("Optimising took {}s", duration.as_secs_f64())
                }
                Err(_) => {
                    println!("Optimising took < 0s??");
                }
            }
        }
        if d.is_key_pressed(KeyboardKey::KEY_T) {
            let start = std::time::SystemTime::now();
            println!("Lets OPTIMISE!");
            types = tabu_search(&towers, &types, 5, 100);
            stacks = calc_stacks(&towers, &types);
            let score = get_setup_dps(&towers, &types, &stacks);
            println!("optimisation score: {score}");
            let end = std::time::SystemTime::now();
            match end.duration_since(start) {
                Ok(duration) => {
                    println!("Optimising took {}s", duration.as_secs_f64())
                }
                Err(_) => {
                    println!("Optimising took < 0s??");
                }
            }
        }
        if d.is_key_pressed(KeyboardKey::KEY_M) {
            let start = std::time::SystemTime::now();
            println!("Lets OPTIMISE!");
            let score = optimize_mut(&towers, &inrange_lookup, &mut types, &mut stacks, optim_level);
            println!("optimisation score: {score}");
            let end = std::time::SystemTime::now();
            match end.duration_since(start) {
                Ok(duration) => {
                    println!("Optimising took {}s", duration.as_secs_f64())
                }
                Err(_) => {
                    println!("Optimising took < 0s??");
                }
            }
        }
        if d.is_key_pressed(KeyboardKey::KEY_C) {
            let dps = full_calc_dps(&towers, &types);
            println!("dps: {dps}");
        }
        if d.is_key_pressed(KeyboardKey::KEY_P) {
            println!(
                "#towers = {}, ({}, {}, {})\ndps-calcs: {}",
                towers.len(),
                inrange_lookup.len(),
                types.len(),
                stacks.len(),
                DPS_CALCS.load(std::sync::atomic::Ordering::SeqCst)
            );
        }
        if d.is_key_pressed(KeyboardKey::KEY_R) {
            let start = std::time::SystemTime::now();
            let score;
            let new_types;
            let new_stacks;
            (score, new_types, new_stacks) = rand_optimize(&sim_towers, &inrange_lookup, &types);
            let end = std::time::SystemTime::now();
            let duration = match end.duration_since(start) {
                Ok(duration) => duration.as_secs_f64(),
                Err(_) => 0.0,
            };
            println!("optimized in {duration}s optimisation score: {score}");
            types = new_types;
            stacks = new_stacks;
        }
        if d.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
            selected_tower = SelectedTower::None;
            let pos = (mouse_pos.x as f64, mouse_pos.y as f64);
            if let Some(etienne) = &etienne {
                let etienne_pos = game_pos_to_window_pos((etienne.pos_x, etienne.pos_y));
                if is_in_range(pos, etienne_pos, 18.0) {
                    selected_tower = SelectedTower::Etienne;
                }
            }
            for (i, ninja) in towers.iter().enumerate() {
                let ninja_pos = game_pos_to_window_pos((ninja.pos_x, ninja.pos_y));
                if is_in_range(pos, ninja_pos, 18.0) {
                    selected_tower = SelectedTower::Ninja(i);
                }
            }
            for (i, village) in villages.iter().enumerate() {
                let village_pos = game_pos_to_window_pos((village.pos_x, village.pos_y));
                if is_in_range(pos, village_pos, 18.0) {
                    selected_tower = SelectedTower::Village(i);
                }
            }
        }
        render(
            &mut d,
            &towers,
            &villages,
            &etienne,
            &types,
            &stacks,
            selected_tower,
        );
    }
    Ok(())
}
