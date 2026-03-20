use std::{collections::HashMap, error::Error, fs::File, io::BufReader, path::Path};

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

#[derive(Clone)]
struct Tower {
    pub id: i64,
    pub pos_x: f64,
    pub pos_y: f64,
    pub negotiable: bool,
    pub buffable: bool,
    pub stacks: i64,
    pub value: i64,
    pub fvalue: f64,
    pub range: f64,
    pub full_data: PlacedTower,
}

impl Tower {
    pub fn new(tower: PlacedTower, range: f64) -> Self {
        Self {
            id: tower.id_last_save,
            pos_x: tower.position.x,
            pos_y: tower.position.y,
            negotiable: true,
            buffable: true,
            stacks: 0,
            value: 9999999,
            fvalue: 9999999.0,
            range,
            full_data: tower,
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
    pub fn with_value(mut self, value: i64) -> Self {
        self.value = value;
        self
    }
}

const RANDOM_INIT: bool = false;
const JITSU_INIT: bool = true;
const NORMAL_INIT: bool = true;

// fn is_in_range(tower_1: &PlacedTower, tower_2: &PlacedTower, range: f64) -> bool {
//     let dx = tower_1.position.x - tower_2.position.x;
//     let dy = tower_1.position.y - tower_2.position.y;
//     (dx * dx + dy * dy).sqrt() < range
// }

fn is_in_range(pos_1: (f64, f64), pos_2: (f64, f64), range: f64) -> bool {
    let dx = pos_1.0 - pos_2.0;
    let dy = pos_1.1 - pos_2.1;
    (dx * dx + dy * dy).sqrt() < range
}

fn tower_is_in_range(tower_1: &PlacedTower, tower_2: &PlacedTower, range: f64) -> bool {
    let dx = tower_1.position.x - tower_2.position.x;
    let dy = tower_1.position.y - tower_2.position.y;
    (dx * dx + dy * dy).sqrt() < range
}

fn get_attack_speed_buff(stack_count: i64) -> f64 {
    (1.08f64).powi(stack_count.try_into().unwrap())
}

fn get_pierce_buff(stack_count: i64) -> f64 {
    // 1.0 + 0.08 * (stack_count as f64)
    1.08 * stack_count as f64
}

fn get_setup_dps(jitsus: &[Tower], shinobis: &[Tower], id_inrange_lookup: &HashMap<i64, Vec<Tower>>) -> f64 {
    let mut jitsus = jitsus.to_vec();
    let mut shinobis = shinobis.to_vec();
    let mut setup_dps = 0f64;

    for shinobi in &mut shinobis {
        let jitsus_in_range = id_inrange_lookup.get(&shinobi.id).unwrap();
        for jitsu in jitsus_in_range {
            for jitsu2 in &mut jitsus {
                if jitsu.id == jitsu2.id {
                    jitsu2.stacks = (jitsu2.stacks + 1).min(20);
                }
            }
        }
        if shinobi.buffable {
            shinobi.stacks = (shinobi.stacks + 1).min(20);
        }
    }

    for ninja in &mut shinobis {
        let attack_speed_buff = get_attack_speed_buff(ninja.stacks);
        let pierce_buff = get_pierce_buff(ninja.stacks);
        ninja.fvalue = (ninja.value as f64) * attack_speed_buff * pierce_buff;
    }
    for ninja in &mut jitsus {
        let attack_speed_buff = get_attack_speed_buff(ninja.stacks);
        let pierce_buff = get_pierce_buff(ninja.stacks);
        ninja.fvalue = (ninja.value as f64) * attack_speed_buff * pierce_buff;
    }
    for ninja in shinobis.iter().chain(&jitsus) {
        if ninja.negotiable {
            setup_dps += ninja.fvalue;
        }
    }

    setup_dps
}

fn optimize(jitsus: &[Tower], shinobis: &[Tower], id_inrange_lookup: &HashMap<i64, Vec<Tower>>, jitsu_id_values: &HashMap<i64, i64>, shinobi_id_values: &HashMap<i64, i64>) -> (f64, (Vec<Tower>, Vec<Tower>)) {
    let mut best_improvement = (0f64, (jitsus.to_vec(), shinobis.to_vec()));
    let base = get_setup_dps(jitsus, shinobis, id_inrange_lookup);
    let mut swap_results = HashMap::new();
    for i in 0..jitsus.len() {
        let jitsu = &jitsus[i];
        if jitsu.negotiable {
            let mut shinobis = shinobis.to_vec();
            let mut jitsus = jitsus.to_vec();
            let mut new_jitsu = jitsu.clone();
            new_jitsu.value = *shinobi_id_values.get(&new_jitsu.id).unwrap();
            shinobis.push(new_jitsu);
            jitsus.remove(i);
            let result = get_setup_dps(&jitsus, &shinobis, id_inrange_lookup);
            swap_results.insert(jitsu.id, (result - base, (jitsus, shinobis)));
        }
    }

    for i in 0..shinobis.len() {
        let shinobi = &shinobis[i];
        if shinobi.negotiable {
            let mut shinobis = shinobis.to_vec();
            let mut jitsus = jitsus.to_vec();
            let mut new_shinobi = shinobi.clone();
            new_shinobi.value = *jitsu_id_values.get(&new_shinobi.id).unwrap();
            jitsus.push(new_shinobi);
            shinobis.remove(i);
            let result = get_setup_dps(&jitsus, &shinobis, id_inrange_lookup);
            swap_results.insert(shinobi.id, (result - base, (jitsus, shinobis)));
        }
    }

    for (_, result) in swap_results {
        if result.0 > best_improvement.0 {
            best_improvement = result;
        }
    }
    best_improvement

}

fn rand_byte() -> u8 {
    let start = std::time::SystemTime::now();
    let time = start.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    let seed = time.subsec_nanos();
    (seed % 256) as u8
}

// const GAME_SIZE: (f64, f64) = (300.0, 232.0);
const SIZE: (f64, f64) = (900.0, 696.0);

fn game_pos_to_window_pos(pos: (f64, f64)) -> (f64, f64) {
    ((pos.0 * 3.0)+(SIZE.0 / 2.0), (pos.1 * 3.0)+(SIZE.1 / 2.0))
}

// fn window_pos_to_game_pos(pos: (f64, f64)) -> (f64, f64) {
//     ((pos.0 - (SIZE.0 / 2.0)) / 3.0, (pos.1 - (SIZE.1 / 2.0)) / 3.0)
// }

fn render(d: &mut RaylibDrawHandle, shinobis: &[Tower], jitsus: &[Tower], villages: &[Tower], id_inrange_lookup: &HashMap<i64, Vec<Tower>>, selected_tower: Option<i64>) {
    d.clear_background(Color::WHITE);
    for shinobi in shinobis {
        let pos = game_pos_to_window_pos((shinobi.pos_x, shinobi.pos_y));
        d.draw_circle(pos.0 as i32, pos.1 as i32, 18.0, Color::new(0, 200, 0, 255));
    }
    for jitsu in jitsus {
        let pos = game_pos_to_window_pos((jitsu.pos_x, jitsu.pos_y));
        d.draw_circle(pos.0 as i32, pos.1 as i32, 18.0, Color::new(200, 200, 0, 255));
    }
    for village in villages {
        let pos = game_pos_to_window_pos((village.pos_x, village.pos_y));
        d.draw_circle(pos.0 as i32, pos.1 as i32, 33.0, Color::new(0, 200, 200, 255));
    }
    let dps = get_setup_dps(jitsus, shinobis, id_inrange_lookup);
    d.draw_text(&format!("{dps}"), 0, 0, 16, Color::BLACK);
    if let Some(id) = selected_tower {
        for ninja in shinobis.iter().chain(jitsus) {
            if id == ninja.id {
                let pos = game_pos_to_window_pos((ninja.pos_x, ninja.pos_y));
                d.draw_circle(pos.0 as i32, pos.1 as i32, ninja.range as f32 * 3.0, Color::new(0,0,0,30));
                let mut stack_count = 0;
                if ninja.buffable {
                    for shinobi in shinobis {
                        if tower_is_in_range(&ninja.full_data, &shinobi.full_data, shinobi.range) {
                            stack_count += 1;
                        }
                    }
                }
                if stack_count > 20 {
                    d.draw_text(&format!("stacks: 20 ({stack_count})"), 0, 16, 16, Color::new(0, 0, 0, 255));
                } else {
                    d.draw_text(&format!("stacks: {stack_count}"), 0, 16, 16, Color::new(0, 0, 0, 255));
                }
                stack_count = stack_count.min(20);
                d.draw_text(&format!("base value: {}", ninja.value), 0, 32, 16, Color::new(0,0,0,255));
                d.draw_text(&format!("value: {}", ninja.value as f64 * get_attack_speed_buff(stack_count) * get_pierce_buff(stack_count)), 0, 48, 16, Color::new(0,0,0,255));
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let setup = read_json("Logs_FreeplaySetupPops.json")?;
    let actual_setup = read_json("Logs_FreeplaySetup.json")?;
    let setup_shinobis = read_json("Logs_FreeplaySetupPopsShi.json")?;
    let setup_old = read_json("Logs_FreeplaySetup.json")?;
    let setup: Freeplay = serde_json::from_value(setup)?;
    let actual_setup: Freeplay = serde_json::from_value(actual_setup)?;
    let setup_shinobis: Freeplay = serde_json::from_value(setup_shinobis)?;
    let setup_old: Freeplay = serde_json::from_value(setup_old)?;

    let ninjagon = setup_old
        .placed_towers
        .iter()
        .find(|tower| tower.base_id == "NinjaMonkey" && tower.path_one_tier == 6)
        .unwrap()
        .clone();

    let mut jitsus: Vec<Tower> = Vec::new();
    let mut shinobis: Vec<Tower> = vec![
        Tower::new(ninjagon, NINJAGON_RANGE)
            .into_nonnegotiable()
            .into_unbuffable(),
    ];
    let towers = setup.placed_towers;
    let towers_2 = setup_shinobis.placed_towers;
    let actual_towers = actual_setup.placed_towers;

    let mut ninjas = Vec::new();
    let mut ninjas_2 = Vec::new();
    let mut ninjas_actual = Vec::new();
    let mut villages = Vec::new();

    for tower in towers {
        if tower.base_id == "NinjaMonkey" {
            ninjas.push(tower)
        } else if tower.base_id == "MonkeyVillage" {
            if tower.path_one_tier == 5 {
                villages.push(
                    Tower::new(tower, VILLAGE_RANGE)
                        .into_nonnegotiable()
                        .into_unbuffable(),
                );
            } else {
                villages.push(
                    Tower::new(tower, HOMELAND_RANGE)
                        .into_nonnegotiable()
                        .into_unbuffable(),
                );
            }
        }
    }

    for tower in towers_2 {
        if tower.base_id == "NinjaMonkey" {
            ninjas_2.push(tower);
        }
    }
    for tower in actual_towers {
        if tower.base_id == "NinjaMonkey" {
            ninjas_actual.push(tower);
        }
    }

    let mut jitsu_id_values: HashMap<i64, i64> = HashMap::new();
    let mut shinobi_id_values: HashMap<i64, i64> = HashMap::new();

    for ninja in ninjas {
        if ninja.path_one_tier == 4 {
            let mut current_range = RANGE_NO_VILLAGE;
            for village in &villages {
                if tower_is_in_range(&ninja, &village.full_data, village.range) {
                    current_range = MAX_RANGE;
                    break;
                }
            }
            jitsu_id_values.insert(ninja.id_last_save, ninja.damage_dealt);
            if !NORMAL_INIT && (RANDOM_INIT || JITSU_INIT) {
                jitsus.push(Tower::new(ninja, current_range));
            }
        }
    }
    for ninja in ninjas_2 {
        if ninja.path_two_tier == 2 {
            let mut current_range = RANGE_NO_VILLAGE;
            for village in &villages {
                if tower_is_in_range(&ninja, &village.full_data, village.range) {
                    current_range = MAX_RANGE;
                    break;
                }
            }
            shinobi_id_values.insert(ninja.id_last_save, ninja.damage_dealt);
            if !NORMAL_INIT && (RANDOM_INIT || !JITSU_INIT) {
                shinobis.push(Tower::new(ninja, current_range));
            }
        }
    }
    if NORMAL_INIT {
        for ninja in ninjas_actual {
            if ninja.path_one_tier >= 4 && ninja.path_one_tier < 6 {
                let mut current_range = RANGE_NO_VILLAGE;
                let ninja_tower = Tower::new(ninja.clone(), 0.0);
                if villages
                    .iter()
                    .any(|v| tower_is_in_range(&ninja_tower.full_data, &v.full_data, v.range))
                {
                    current_range = MAX_RANGE;
                }
                let value = *jitsu_id_values.get(&ninja.id_last_save).unwrap();
                jitsus.push(Tower::new(ninja, current_range).with_value(value));
            } else if ninja.path_two_tier >= 3 {
                let mut current_range = RANGE_NO_VILLAGE;
                let ninja_tower = Tower::new(ninja.clone(), 0.0);
                if villages
                    .iter()
                    .any(|v| tower_is_in_range(&ninja_tower.full_data, &v.full_data, v.range))
                {
                    current_range = MAX_RANGE;
                }
                let value = *shinobi_id_values.get(&ninja.id_last_save).unwrap();
                shinobis.push(Tower::new(ninja, current_range).with_value(value));
            }
        }
    }

    if RANDOM_INIT {
        let mut shinobis_temp = Vec::new();
        let mut jitsus_temp = Vec::new();
        for (s, j) in shinobis.into_iter().skip(1).zip(jitsus) {
            if (rand_byte() & 1) != 0 {
                shinobis_temp.push(s);
            } else {
                jitsus_temp.push(j);
            }
        }
        shinobis = shinobis_temp;
        jitsus = jitsus_temp;
    }

    let all_ninjas = [jitsus.as_slice(), shinobis.as_slice()].concat();

    let mut id_inrange_lookup = HashMap::new();

    for jitsu in all_ninjas.iter().chain(&villages) {
        let mut lookup_entry = Vec::new();
        for jitsu2 in &all_ninjas {
            if jitsu.id != jitsu2.id && tower_is_in_range(&jitsu2.full_data, &jitsu.full_data, jitsu.range){
                lookup_entry.push(jitsu2.clone());
            }
        }
        id_inrange_lookup.insert(jitsu.id, lookup_entry);
    }

    let ninjagon = shinobis[0].clone();
    {
        let mut lookup_entry = Vec::new();
        for jitsu2 in &all_ninjas {
            if tower_is_in_range(&jitsu2.full_data, &ninjagon.full_data, ninjagon.range) {
                lookup_entry.push(jitsu2.clone());
            }
        }
        id_inrange_lookup.insert(ninjagon.id, lookup_entry);
    }

    let mut selected_tower: Option<i64> = None;
    let (mut rl, thread) = raylib::init().size(900, 696).title("pop optimizer").build();
    rl.set_exit_key(None);
    rl.set_target_fps(60);
    while !rl.window_should_close() {
        let mouse_pos = rl.get_mouse_position();
        let mut d = rl.begin_drawing(&thread);
        if d.is_key_pressed(raylib::ffi::KeyboardKey::KEY_O) {
            let start = std::time::SystemTime::now();
            println!("Lets OPTIMISE!");
            let (score, results) = optimize(&jitsus, &shinobis, &id_inrange_lookup, &jitsu_id_values, &shinobi_id_values);
            println!("optimisation score: {score}");
            let end = std::time::SystemTime::now();
            match end.duration_since(start) {
                Ok(duration) => {println!("Optimising took {}s", duration.as_secs_f64())},
                Err(_) => {println!("Optimising took < 0s??");}
            }
            (jitsus, shinobis) = results;
        }
        if d.is_mouse_button_pressed(raylib::ffi::MouseButton::MOUSE_BUTTON_LEFT) {
            // let pos = window_pos_to_game_pos((mouse_pos.x as f64, mouse_pos.y as f64));
            selected_tower = None;
            let pos = (mouse_pos.x as f64, mouse_pos.y as f64);
            for ninja in shinobis.iter().chain(&jitsus) {
                let ninja_pos = game_pos_to_window_pos((ninja.pos_x, ninja.pos_y));
                if is_in_range(pos, ninja_pos, 18.0) {
                    selected_tower = Some(ninja.id);
                }
            }
        }
        render(&mut d, &shinobis, &jitsus, &villages, &id_inrange_lookup, selected_tower);
    }

    Ok(())
}

//934,975,479
